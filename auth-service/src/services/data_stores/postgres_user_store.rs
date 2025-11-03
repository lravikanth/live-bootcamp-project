use std::{error::Error, panic::panic_any};

use argon2::{
    password_hash::SaltString, Algorithm, Argon2, Params, PasswordHash, PasswordHasher,
    PasswordVerifier, Version,
};

use crate::domains::{
    data_stores::{UserStore, UserStoreError},
    email::{self, Email},
    password::{self, Password},
    user::{self, User},
};
use sqlx::PgPool;
use sqlx::Row;

#[derive(Clone)]
pub struct PostgresUserStore {
    pool: PgPool,
}

impl PostgresUserStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

async fn verify_password_hash(
    expected_password_hash: &str,
    password_candidate: &str,
) -> Result<(), Box<dyn Error>> {
    let expected_password_hash: PasswordHash<'_> = PasswordHash::new(expected_password_hash)?;

    Argon2::default()
        .verify_password(password_candidate.as_bytes(), &expected_password_hash)
        .map_err(|e| e.into())
}

#[async_trait::async_trait]
impl UserStore for PostgresUserStore {
    async fn add_user(&mut self, user: user::User) -> Result<(), UserStoreError> {
        let password_hash = compute_password_hash(user.password.as_ref())
            .await
            .map_err(|_| UserStoreError::UnexpectedError)?;

        // compile-time verified query
        let result = sqlx::query!(
            r#"
        INSERT INTO users (email, password_hash, requires_2fa)
        VALUES ($1, $2, $3)
        "#,
            user.email.as_ref(), // $1
            password_hash,       // $2
            user.requires_2fa    // $3
        )
        .execute(&self.pool)
        .await
        .map_err(|_| UserStoreError::UnexpectedError)?;
        let n = result.rows_affected();
        println!("users insert rows_affected = {}", n);
        if result.rows_affected() == 1 {
            Ok(())
        } else {
            Err(UserStoreError::UnexpectedError)
        }
    }

    async fn get_user(&self, email: &Email) -> Result<user::User, UserStoreError> {
        let result = sqlx::query!(
            r#"
    SELECT email, password_hash, requires_2fa
    FROM users
    WHERE email = $1
    "#,
            email.as_ref()
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|_| UserStoreError::UnexpectedError)?;

        match result {
            Some(record) => {
                // Access fields directly; query! returns an anonymous struct with named fields
                let email =
                    Email::parse(record.email).map_err(|_| UserStoreError::UnexpectedError)?;

                // You probably want the stored hash, not a hardcoded password:
                let password_hash = Password::parse(record.password_hash)
                    .map_err(|_| UserStoreError::UnexpectedError)?;

                // If the column is nullable, the macro types it as Option<bool>
                let requires_2fa = record.requires_2fa;

                Ok(User::new(email, password_hash, requires_2fa))
            }
            None => Err(UserStoreError::UserNotFound),
        }
    }

    async fn validate_user(&self, email: &Email, password: &str) -> Result<(), UserStoreError> {
        let result = sqlx::query("select * from users where email = $1")
            .bind(email.as_ref())
            .fetch_one(&self.pool)
            .await
            .map_err(|_| UserStoreError::UnexpectedError)?;
        println!("got user from validation");
        if result.is_empty() {
            return Err(UserStoreError::InvalidCredentials);
        }

        let email =
            Email::parse(result.get("email")).map_err(|_| UserStoreError::UnexpectedError)?;
        let expected_password_hash: String = result.get("password_hash");
        let requires_2fa: bool = result.get("requires_2fa");
        println!("Expected Hash  {}", expected_password_hash);
        verify_password_hash(&expected_password_hash, password)
            .await
            .map_err(|_| UserStoreError::InvalidCredentials)?;

        Ok(())
    }
}

async fn compute_password_hash(password: &str) -> Result<String, Box<dyn Error>> {
    let salt: SaltString = SaltString::generate(&mut rand::thread_rng());
    let password_hash = Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        Params::new(15000, 2, 1, None)?,
    )
    .hash_password(password.as_bytes(), &salt)?
    .to_string();

    Ok(password_hash)
}

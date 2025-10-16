use crate::domains::data_stores::{UserStore, UserStoreError};
use crate::domains::email::Email;
use crate::domains::user;
use async_trait::async_trait;
use std::collections::{hash_map, HashMap};

#[derive(Default, Clone)]
pub struct HashMapUserStore {
    users: HashMap<Email, user::User>,
}
#[async_trait]

impl UserStore for HashMapUserStore {
    async fn add_user(&mut self, user: user::User) -> Result<(), UserStoreError> {
        if (self.users.contains_key(&user.email)) {
            Err(UserStoreError::UserAlreadyExists)
        } else {
            self.users.insert(user.email.clone(), user);
            Ok(())
        }
    }

    async fn get_user(&self, email: &Email) -> Result<user::User, UserStoreError> {
        match self.users.get(email) {
            Some(a) => Ok(a.clone()),
            None => Err(UserStoreError::UserNotFound),
        }
    }

    async fn validate_user(&self, email: &Email, password: &str) -> Result<(), UserStoreError> {
        let r = self.get_user(email).await.unwrap();
        if r.password.as_ref() == password {
            Ok(())
        } else {
            Err(UserStoreError::InvalidCredentials)
        }
    }
}
#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{
        domains::{email::Email, password::Password},
        services::hashmap_user_store,
    };

    use super::*;

    #[tokio::test]
    async fn test_add_user() {
        let email = Email::parse("Ravi@gmailk.ocm".to_string()).unwrap();
        let password = Password::parse("Password123".to_string()).unwrap();

        let u1 = user::User {
            email: email,
            password: password,
            requires_2fa: true,
        };

        let mut store = HashMapUserStore::default();
        let first = store.add_user(u1).await;
        assert_eq!(first, Ok(()));
    }

    #[tokio::test]
    async fn test_get_user() {
        let email = Email::parse("Ravi@gmailk.ocm".to_string()).unwrap();
        let password = Password::parse("Password123".to_string()).unwrap();

        let u1 = user::User {
            email: email,
            password: password,
            requires_2fa: true,
        };

        let mut store = HashMapUserStore::default();
        let first = store.add_user(u1).await;

        let g = store
            .get_user(&Email::parse("Ravi@gmailk.ocm".to_string()).unwrap())
            .await
            .unwrap();
        assert_eq!(g.email.as_ref(), "Ravi@gmailk.ocm");
        assert_eq!(g.password.as_ref(), "Password123");

        let failed = store
            .get_user(&Email::parse("Ravi1@gmail.com".to_string()).unwrap())
            .await;
        assert!(matches!(failed, Err(UserStoreError::UserNotFound)));
    }

    #[tokio::test]
    async fn test_validate_user() {
        let email = Email::parse("Ravi@gmailk.ocm".to_string()).unwrap();
        let password = Password::parse("Password123".to_string()).unwrap();

        let u1 = user::User {
            email,
            password,
            requires_2fa: true,
        };

        let mut store = HashMapUserStore::default();
        let _first = store.add_user(u1).await;

        let email = Email::parse("Ravi@gmailk.ocm".to_string()).unwrap();
        let password = Password::parse("Password123".to_string()).unwrap();

        let res = store.validate_user(&email, password.as_ref()).await;
        assert_eq!(res, Ok(()));
    }
}

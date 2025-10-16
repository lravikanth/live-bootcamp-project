use crate::domains::user;
use std::collections::{hash_map, HashMap};

#[derive(Default)]
pub struct HashMapUserStore {
    users: HashMap<String, user::User>,
}

#[derive(Debug, PartialEq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError,
}

impl HashMapUserStore {
    pub fn add_user(&mut self, user: user::User) -> Result<(), UserStoreError> {
        if (self.users.contains_key(&user.email)) {
            Err(UserStoreError::UserAlreadyExists)
        } else {
            self.users.insert(user.email.clone(), user);
            Ok(())
        }
    }

    pub fn get_user(&self, email: &str) -> Result<user::User, UserStoreError> {
        match &self.users.get(email).cloned() {
            Some(a) => Ok(a.clone()),
            None => Err(UserStoreError::UserNotFound),
        }
    }

    fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError> {
        let r = self.get_user(email).unwrap();
        if r.password == password {
            Ok(())
        } else {
            Err(UserStoreError::InvalidCredentials)
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::services::hashmap_user_store;

    use super::*;

    #[tokio::test]
    async fn test_add_user() {
        let u1 = user::User {
            email: "Ravi@gmail.com".to_string(),
            password: "password".to_string(),
            requires_2fa: true,
        };

        let mut store = HashMapUserStore::default();
        let first = store.add_user(u1);
        assert_eq!(first, Ok(()));
    }

    #[tokio::test]
    async fn test_get_user() {
        let u1 = user::User {
            email: "Ravi@gmail.com".to_string(),
            password: "password".to_string(),
            requires_2fa: true,
        };

        let mut store = HashMapUserStore::default();
        let first = store.add_user(u1);

        let g = store.get_user("Ravi@gmail.com").unwrap();
        assert_eq!(g.email, "Ravi@gmail.com".to_string());
        assert_eq!(g.password, "password".to_string());

        let failed = store.get_user("Ravi1@gmail.com");
        assert!(matches!(failed, Err(UserStoreError::UserNotFound)));
    }

    #[tokio::test]
    async fn test_validate_user() {
        let u1 = user::User {
            email: "Ravi@gmail.com".to_string(),
            password: "password".to_string(),
            requires_2fa: true,
        };

        let mut store = HashMapUserStore::default();
        let first = store.add_user(u1);
        let res = store.validate_user("Ravi@gmail.com", "password");
        assert_eq!(res, Ok(()));
    }
}

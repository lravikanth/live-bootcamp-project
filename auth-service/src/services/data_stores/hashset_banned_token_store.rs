use std::collections::HashSet;

use axum::async_trait;
use uuid::serde::simple::serialize;

use crate::domains::data_stores::{BannedTokenError, BannedTokenStore};
#[derive(Default, Clone)]
pub struct HashsetBannedTokenStore {
    banned_token: HashSet<String>,
}
#[async_trait]
impl BannedTokenStore for HashsetBannedTokenStore {
    async fn add_banned_token(&mut self, banned_token: String) -> Result<(), BannedTokenError> {
        let current = self.banned_token.get(&banned_token);
        match current {
            Some(t) => Err(BannedTokenError::TokenExists),
            None => {
                self.banned_token.insert(banned_token);
                Ok(())
            }
        }
    }

    async fn does_token_exist(&self, banned_token: String) -> bool {
        match self.banned_token.get(&banned_token) {
            Some(t) => true,
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::domains::data_stores::{BannedTokenError, BannedTokenStore};
    use crate::services::data_stores::hashset_banned_token_store::HashsetBannedTokenStore;

    #[tokio::test]
    async fn test_add_token() {
        let mut b_tokens = HashsetBannedTokenStore {
            banned_token: HashSet::new(),
        };

        let res = b_tokens.add_banned_token("Ravi Lukkani".to_owned()).await;
        assert_eq!(res.is_ok(), true);
    }

    #[tokio::test]
    async fn test_add_same_token() {
        let mut b_tokens = HashsetBannedTokenStore {
            banned_token: HashSet::new(),
        };

        let res = b_tokens.add_banned_token("Ravi Lukkani".to_owned()).await;
        let res1 = b_tokens.add_banned_token("Ravi Lukkani".to_owned()).await;
        assert!(matches!(res1, Err(BannedTokenError::TokenExists)));
    }

    #[tokio::test]
    async fn test_check_token_exists() {
        let mut b_tokens = HashsetBannedTokenStore {
            banned_token: HashSet::new(),
        };

        let res = b_tokens.add_banned_token("Ravi Lukkani".to_owned()).await;
        let exist = b_tokens.does_token_exist("Ravi Lukkani".to_owned()).await;

        assert!(exist);
    }
}

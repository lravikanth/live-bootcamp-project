use std::collections::HashMap;

use axum::async_trait;

use crate::domains::data_stores::{LoginAttemptId, TwoFACode, TwoFACodeStore, TwoFACodeStoreError};
use crate::domains::email::Email;
#[derive(Default, Clone)]
pub struct HashmapTwoFACodeStore {
    codes: HashMap<Email, (LoginAttemptId, TwoFACode)>,
}

#[async_trait]
impl TwoFACodeStore for HashmapTwoFACodeStore {
    async fn add_code(
        &mut self,
        email: Email,
        login_attempt_id: LoginAttemptId,
        code: TwoFACode,
    ) -> Result<(), TwoFACodeStoreError> {
        self.codes.insert(email, (login_attempt_id, code));

        Ok(())
    }

    async fn remove_code(&mut self, email: &Email) -> Result<(), TwoFACodeStoreError> {
        match self.codes.get(email) {
            Some(a) => {
                self.codes.remove(email);
                Ok(())
            }
            None => Err(TwoFACodeStoreError::LoginAttemptIdNotFound),
        }
    }

    async fn get_code(
        &self,
        email: &Email,
    ) -> Result<&(LoginAttemptId, TwoFACode), TwoFACodeStoreError> {
        match self.codes.get(email) {
            Some(a) => Ok((a)),
            None => Err(TwoFACodeStoreError::LoginAttemptIdNotFound),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::domains::data_stores::{
        LoginAttemptId, TwoFACode, TwoFACodeStore, TwoFACodeStoreError,
    };
    use crate::domains::email::Email;
    use crate::services::hashmap_two_fa_code_store::HashmapTwoFACodeStore;

    #[tokio::test]
    async fn add_valid_codes_test() {
        let email = Email::parse("ravi@gmail.com".to_string()).unwrap();
        let login_attempt_id = LoginAttemptId::default();
        let code = TwoFACode::default();

        let mut hash_map_store = HashmapTwoFACodeStore {
            codes: HashMap::default(),
        };

        let resp = hash_map_store.add_code(email, login_attempt_id, code).await;
        assert_eq!(resp, Ok(()));
    }

    #[tokio::test]
    async fn remove_codes_test() {
        let email = Email::parse("ravi@gmail.com".to_string()).unwrap();
        let login_attempt_id = LoginAttemptId::default();
        let code = TwoFACode::default();

        let mut hash_map_store = HashmapTwoFACodeStore {
            codes: HashMap::default(),
        };

        let resp = hash_map_store
            .add_code(email.clone(), login_attempt_id, code)
            .await;

        let resp_remove = hash_map_store
            .remove_code(&Email::parse("ravi111@gmail.com".to_string()).unwrap())
            .await;

        assert_eq!(
            resp_remove,
            Err(TwoFACodeStoreError::LoginAttemptIdNotFound)
        );

        let resp_remove = hash_map_store.remove_code(&email).await;

        assert_eq!(resp_remove, Ok(()));
    }
}

use std::sync::Arc;
use tokio::sync::RwLock;

use crate::domains::data_stores::{BannedTokenStore, TwoFACodeStore, UserStore};
use crate::domains::EmailClient;

// Using a type alias to improve readability!
pub type UserStoreType<T> = Arc<RwLock<T>>;
pub type EmailClientType<T> = Arc<RwLock<T>>;

#[derive(Clone)]
pub struct AppState<T: UserStore, T1: BannedTokenStore, T2: TwoFACodeStore, T3: EmailClient> {
    pub user_store: UserStoreType<T>,
    pub banned_token_store: UserStoreType<T1>,
    pub two_fa_store: UserStoreType<T2>,
    pub email_client: UserStoreType<T3>,
}

impl<T: UserStore, T1: BannedTokenStore, T2: TwoFACodeStore, T3: EmailClient>
    AppState<T, T1, T2, T3>
{
    pub fn new(
        user_store: UserStoreType<T>,
        banned_token_store: UserStoreType<T1>,
        two_fa_store: UserStoreType<T2>,
        email_client: EmailClientType<T3>,
    ) -> Self {
        Self {
            user_store,
            banned_token_store,
            two_fa_store,
            email_client,
        }
    }
}

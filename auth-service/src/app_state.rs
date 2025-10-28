use std::sync::Arc;
use tokio::sync::RwLock;

use crate::domains::data_stores::{BannedTokenStore, TwoFACodeStore, UserStore};

// Using a type alias to improve readability!
pub type UserStoreType<T> = Arc<RwLock<T>>;

#[derive(Clone)]
pub struct AppState<T: UserStore, T1: BannedTokenStore, T2: TwoFACodeStore> {
    pub user_store: UserStoreType<T>,
    pub banned_token_store: UserStoreType<T1>,
    pub two_fa_store: UserStoreType<T2>,
}

impl<T: UserStore, T1: BannedTokenStore, T2: TwoFACodeStore> AppState<T, T1, T2> {
    pub fn new(
        user_store: UserStoreType<T>,
        banned_token_store: UserStoreType<T1>,
        two_fa_store: UserStoreType<T2>,
    ) -> Self {
        Self {
            user_store,
            banned_token_store,
            two_fa_store,
        }
    }
}

use std::sync::Arc;
use tokio::sync::RwLock;

use crate::domains::data_stores::UserStore;

// Using a type alias to improve readability!
pub type UserStoreType<T> = Arc<RwLock<T>>;

#[derive(Clone)]
pub struct AppState<T: UserStore + Clone + Send + Sync> {
    pub user_store: UserStoreType<T>,
}

impl<T: UserStore + Clone + Send + Sync> AppState<T> {
    pub fn new(user_store: T) -> Self {
        Self {
            user_store: Arc::new(RwLock::new(user_store)),
        }
    }
}

use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{domains::data_stores::UserStore, services::hashmap_user_store::HashMapUserStore};

// Using a type alias to improve readability!
pub type UserStoreType<T: UserStore + Clone + Send + Sync> = Arc<RwLock<T>>;

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

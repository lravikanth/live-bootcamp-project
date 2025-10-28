use auth_service::app_state::AppState;
use auth_service::services;
use auth_service::services::hashmap_two_fa_code_store::HashmapTwoFACodeStore;
use auth_service::services::hashset_banned_token_store::HashsetBannedTokenStore;
use auth_service::Application;
use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    let users_store = services::hashmap_user_store::HashMapUserStore::default();
    let banned_token_store = HashsetBannedTokenStore::default();
    let two_fa_store = HashmapTwoFACodeStore::default();

    let app_state = AppState::new(
        Arc::new(RwLock::new(users_store)),
        Arc::new(RwLock::new(banned_token_store)),
        Arc::new(RwLock::new(two_fa_store)),
    );

    let app = Application::build(app_state, "0.0.0.0:3000")
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to run app");
}

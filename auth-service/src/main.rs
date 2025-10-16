use auth_service::app_state::AppState;
use auth_service::services;
use auth_service::Application;

#[tokio::main]
async fn main() {
    let users_store = services::hashmap_user_store::HashMapUserStore::default();
    let app_state = AppState::new(users_store);

    let app = Application::build(app_state, "0.0.0.0:3000")
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to run app");
}

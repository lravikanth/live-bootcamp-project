use auth_service::app_state::AppState;
use auth_service::domains::EmailClient;
use auth_service::services::data_stores::hashmap_user_store;
use auth_service::services::data_stores::postgres_user_store::PostgresUserStore;
use auth_service::services::data_stores::{
    hashmap_two_fa_code_store::HashmapTwoFACodeStore,
    hashset_banned_token_store::HashsetBannedTokenStore,
};
use auth_service::services::mock_email_client::MockEmailClient;
use auth_service::services::{self};
use auth_service::utils::constants::DATABASE_URL;
use auth_service::{get_postgres_pool, Application};
use sqlx::{PgPool, Pool, Postgres};
use std::cell::OnceCell;
use std::net::IpAddr;
use std::sync::Arc;
use tokio::sync::RwLock;

async fn sql_db(pool: PgPool) {
    use sqlx::Row;

    let row = sqlx::query(
        r#"
    SELECT
      current_database()     AS db,
      current_user           AS user,
      inet_server_addr()::text     AS server_ip,
      inet_server_port()     AS server_port,
      version()              AS version
    "#,
    )
    .fetch_one(&pool)
    .await
    .expect("conn info");

    println!(
        "Connected to db='{}' user='{}' at {}:{}\n{}",
        row.get::<String, _>("db"),
        row.get::<String, _>("user"),
        row.get::<String, _>("server_ip"),
        row.get::<i32, _>("server_port"),
        row.get::<String, _>("version"),
    );
}
#[tokio::main]
async fn main() {
    let pg_pool = configure_postgresql().await;
    sql_db(pg_pool.clone()).await;
    let users_store = PostgresUserStore::new(pg_pool);
    let banned_token_store = HashsetBannedTokenStore::default();
    let two_fa_store = HashmapTwoFACodeStore::default();
    let email_client = MockEmailClient;

    let app_state = AppState::new(
        Arc::new(RwLock::new(users_store)),
        Arc::new(RwLock::new(banned_token_store)),
        Arc::new(RwLock::new(two_fa_store)),
        Arc::new(RwLock::new(email_client)),
    );

    let app = Application::build(app_state, "0.0.0.0:3000")
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to run app");
}
async fn configure_postgresql() -> PgPool {
    // Create a new database connection pool
    let pg_pool = get_postgres_pool(&DATABASE_URL)
        .await
        .expect("Failed to create Postgres connection pool!");

    // Run database migrations against our test database!
    sqlx::migrate!()
        .run(&pg_pool)
        .await
        .expect("Failed to run migrations");

    pg_pool
}

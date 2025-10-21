use crate::helpers::{self, TestApp};
use auth_service::domains::email::Email;
use auth_service::utils::auth::{generate_auth_token, validate_token};

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;
    let body = serde_json::json!({
        "toke": "string"
    });

    let resp = app.post_verify_token(&body).await;
    assert_eq!(resp.status().as_u16(), 422);
}

#[tokio::test]
async fn should_return_200_valid_token() {
    let app = TestApp::new().await;
    let email = Email::parse(helpers::TestApp::get_random_email()).unwrap();

    let token = generate_auth_token(&email).unwrap();

    let body = serde_json::json!({
        "token": token
    });

    let resp = app.post_verify_token(&body).await;
    assert_eq!(resp.status().as_u16(), 200);
}

#[tokio::test]
async fn should_return_401_if_invalid_token() {
    let app = TestApp::new().await;
    let email = Email::parse(helpers::TestApp::get_random_email()).unwrap();

    let token = generate_auth_token(&email).unwrap();

    let body = serde_json::json!({
        "token": token + "12"
    });
    let resp = app.post_verify_token(&body).await;
    assert_eq!(resp.status().as_u16(), 401);
}

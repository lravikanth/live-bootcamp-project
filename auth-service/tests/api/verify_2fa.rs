use auth_service::domains::data_stores::{LoginAttemptId, TwoFACode, TwoFACodeStore};
use auth_service::domains::email::Email;
use auth_service::utils::constants::JWT_COOKIE_NAME;
use axum::Json;

use crate::helpers::TestApp;

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;

    let body = serde_json::json!({
            "email": "user@example.com",
            "loginAttemptId": "string",
            "FACode": "string"
    });

    let resp = app.post_verify_2fa(&body).await;
    assert_eq!(resp.status().as_u16(), 422);
}

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    let app = TestApp::new().await;

    let body = serde_json::json!({
            "email": "user@example.com",
            "loginAttemptId": "550e8400-e29b-41d4-a716-446655440000",
            "2FACode": "string11"
    });

    let resp = app.post_verify_2fa(&body).await;
    assert_eq!(resp.status().as_u16(), 400);
}

#[tokio::test]
async fn should_return_401_if_incorrect_credentials() {
    let app = TestApp::new().await;

    let body = serde_json::json!({
            "email": "user@example.com",
            "loginAttemptId": "550e8400-e29b-41d4-a716-446655440000",
            "2FACode": "stri"
    });

    let resp = app.post_verify_2fa(&body).await;
    assert_eq!(resp.status().as_u16(), 401);
}

#[tokio::test]
async fn should_return_401_if_old_code() {
    let app: TestApp = TestApp::new().await;
    let email = TestApp::get_random_email();

    let signup_body = serde_json::json!({
        "email": email,
        "password": "password123" ,
        "requires2FA": true,
    });

    let login_body = serde_json::json!({
        "email": email,
        "password": "password123" ,
    });

    let email_obj = Email::parse(email.clone()).unwrap();
    let response = app.post_signup(&signup_body).await;
    let response = app.post_login(&login_body).await;
    let login_attempt_id: LoginAttemptId;
    let first_code: TwoFACode;
    {
        let read_lock = app.two_fa_code.read().await;
        login_attempt_id = read_lock.get_code(&email_obj).await.unwrap().0.clone();
        first_code = read_lock.get_code(&email_obj).await.unwrap().1.clone();
    }

    let response = app.post_login(&login_body).await;
    assert_eq!(response.status().as_u16(), 206);

    let body = serde_json::json!({
            "email": email,
            "loginAttemptId": login_attempt_id.as_ref(),
            "2FACode": first_code.as_ref()
    });

    let response = app.post_verify_2fa(&body).await;

    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn should_return_200_if_correct_code() {
    let app: TestApp = TestApp::new().await;
    let email = TestApp::get_random_email();

    let signup_body = serde_json::json!({
        "email": email,
        "password": "password123" ,
        "requires2FA": true,
    });

    let login_body = serde_json::json!({
        "email": email,
        "password": "password123" ,
    });

    let email_obj = Email::parse(email.clone()).unwrap();
    let response = app.post_signup(&signup_body).await;
    let response = app.post_login(&login_body).await;

    let login_attempt_id: LoginAttemptId;
    let first_code: TwoFACode;
    {
        let read_lock = app.two_fa_code.read().await;
        login_attempt_id = read_lock.get_code(&email_obj).await.unwrap().0.clone();
        first_code = read_lock.get_code(&email_obj).await.unwrap().1.clone();
    }

    let body = serde_json::json!({
            "email": email,
            "loginAttemptId": login_attempt_id.as_ref(),
            "2FACode": first_code.as_ref()
    });

    let response = app.post_verify_2fa(&body).await;

    assert_eq!(response.status().as_u16(), 200);

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    assert!(!auth_cookie.value().is_empty());
}

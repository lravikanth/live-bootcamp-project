use crate::helpers::TestApp;
use crate::login::SigninResponse;

use auth_service::domains::email::{self, Email};
use auth_service::utils::auth::generate_auth_cookie;
use auth_service::utils::constants::JWT_COOKIE_NAME;

use reqwest::Url;

#[tokio::test]
async fn should_return_400_if_jwt_cookie_missing() {
    let app = TestApp::new().await;
    let resp = app.post_logout().await;
    assert_eq!(resp.status().as_u16(), 400);
}

#[tokio::test]
async fn should_return_401_if_invalid_token() {
    let app = TestApp::new().await;

    // add invalid cookie
    app.cookie_jar.add_cookie_str(
        &format!(
            "{}=invalid; HttpOnly; SameSite=Lax; Secure; Path=/",
            JWT_COOKIE_NAME
        ),
        &Url::parse("http://127.0.0.1").expect("Failed to parse URL"),
    );

    let resp = app.post_logout().await;
    assert_eq!(resp.status().as_u16(), 401);
}

#[tokio::test]
async fn should_return_200_if_valid_jwt_cookie() {
    let app = TestApp::new().await;
    let email = Email::parse("lravikanth@gmail.com".to_string()).unwrap();
    let cookie = generate_auth_cookie(&email).unwrap();

    app.cookie_jar.add_cookie_str(
        &cookie.to_string(),
        &Url::parse("http://127.0.0.1").expect("Failed to parse URL"),
    );

    // app.cookie_jar.add_cookie_str(
    //     &format!(
    //         "{}=invalid; HttpOnly; SameSite=Lax; Secure; Path=/",
    //         JWT_COOKIE_NAME
    //     ),
    //     &Url::parse("http://127.0.0.1").expect("Failed to parse URL"),
    // );

    let resp = app.post_logout().await;
    assert_eq!(resp.status().as_u16(), 200);
}

#[tokio::test]
async fn should_return_400_if_logout_called_twice_in_a_row() {
    let app = TestApp::new().await;
    let email = Email::parse("lravikanth@gmail.com".to_string()).unwrap();
    let cookie = generate_auth_cookie(&email).unwrap();

    app.cookie_jar.add_cookie_str(
        &cookie.to_string(),
        &Url::parse("http://127.0.0.1").expect("Failed to parse URL"),
    );

    let resp = app.post_logout().await;
    assert_eq!(resp.status().as_u16(), 200);

    let resp2 = app.post_logout().await;
    assert_eq!(resp2.status().as_u16(), 400);
}

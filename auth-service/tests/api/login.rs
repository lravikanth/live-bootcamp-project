use auth_service::utils::constants::JWT_COOKIE_NAME;
use axum::Json;
use serde::Serialize;

use crate::helpers::TestApp;

// #[tokio::test]
// async fn login_returns_auth_ui() {
//     let app = TestApp::new().await;

//     let test_case = serde_json::json!({
//       "email": "user@example.com",
//       "password": "password",
//     });

//     let response = app.post_login(&test_case).await;
//     println!("status code: {}", response.status().as_u16());

//    // assert_eq!(response.status().as_u16(), 200);
//     //  assert_eq!(response.headers().get("content-type").unwrap(), "text/html");
// }
#[tokio::test]
async fn should_return_422_if_malformed_credentials() {
    let app = TestApp::new().await;
    let test_case = serde_json::json!({
        "email": "ravi@gmail.com",
        "passwor": "lukkani",
    });

    let resp = app.post_login(&test_case).await;
    assert_eq!(resp.status().as_u16(), 422);
}

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    // Call the log-in route with invalid credentials and assert that a
    // 400 HTTP status code is returned along with the appropriate error message.
    let app = TestApp::new().await;
    let test_case = serde_json::json!({
        "email": "ravigmail.com",
        "password": "lukkani",
    });

    let msg = SigninResponse {
        message: "Invalid Credentials".to_string(),
    };

    let resp = app.post_login(&test_case).await;
    assert_eq!(resp.status().as_u16(), 400);
    assert_eq!(
        resp.json::<SigninResponse>()
            .await
            .expect("Could not deserialize response body to ErrorResponse"),
        msg
    );
}

#[tokio::test]
async fn should_return_401_if_incorrect_credentials() {
    let app = TestApp::new().await;
    let test_case = serde_json::json!({
        "email": "ravi@gmail.com",
        "password": "lukkani12",
    });

    let msg = SigninResponse {
        message: "Unauthorized".to_string(),
    };

    let resp = app.post_login(&test_case).await;
    assert_eq!(resp.status().as_u16(), 401);
    assert_eq!(
        resp.json::<SigninResponse>()
            .await
            .expect("Could not deserialize response body to ErrorResponse"),
        msg
    );
}

#[tokio::test]
async fn should_return_200_if_valid_credentials_and_2fa_disabled() {
    let app: TestApp = TestApp::new().await;
    let email = TestApp::get_random_email();

    let signup_body = serde_json::json!({
        "email": email,
        "password": "password123" ,
        "requires2FA": false,
    });

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);

    let login_body = serde_json::json!({
        "email": email,
        "password": "password123",
    });

    let response = app.post_login(&login_body).await;
    assert_eq!(response.status().as_u16(), 200);

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    assert!(!auth_cookie.value().is_empty());
}

#[derive(Serialize, PartialEq, Debug, serde::Deserialize)]
pub struct SigninResponse {
    pub message: String,
}

use std::fmt::Display;

use axum::{
    body::Body,
    extract::State,
    http::{Response, StatusCode},
    response::IntoResponse,
    Json,
};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};

use crate::{
    app_state::AppState,
    domains::{
        data_stores::{BannedTokenStore, LoginAttemptId, TwoFACode, TwoFACodeStore, UserStore},
        email::Email,
        error::AuthAPIError,
        password::Password,
        user,
    },
    utils::auth::generate_auth_cookie,
};
#[derive(Deserialize, Debug)]
pub struct LoginInfo {
    email: String,
    password: String,
}

pub(crate) async fn login<
    T: UserStore + Clone + Send + Sync,
    T1: BannedTokenStore + Clone + Send + Sync,
    T2: TwoFACodeStore + Clone + Send + Sync,
>(
    State(state): State<AppState<T, T1, T2>>,
    jar: CookieJar,
    Json(request): Json<LoginInfo>,
) -> (CookieJar, Result<Response<Body>, AuthAPIError>) {
    let email: Email = match Email::parse(request.email) {
        Ok(e) => e,
        Err(_) => return (jar, Err(AuthAPIError::InvalidCredentials)),
    };

    let password: Password = match Password::parse(request.password) {
        Ok(p) => p,
        Err(_) => return (jar, Err(AuthAPIError::InvalidCredentials)),
    };

    let validation_result = state
        .user_store
        .read()
        .await
        .validate_user(&email, password.as_ref())
        .await;

    if validation_result.is_err() {
        return (jar, Err(AuthAPIError::IncorrectCredentials));
    }

    let user_lock = state.user_store.read().await;
    let user = match user_lock.get_user(&email).await {
        Ok(u) => u,
        Err(_) => return (jar, Err(AuthAPIError::IncorrectCredentials)),
    };

    let auth_cookie = match generate_auth_cookie(&email) {
        Ok(c) => c,
        Err(_) => return (jar, Err(AuthAPIError::UnexpectedError)),
    };

    if user.requires_2fa {
        handle_2fa(jar, &state, &email).await
    } else {
        handle_no_2fa(jar, &email).await
    }

    // Ok((jar.add(auth_cookie), StatusCode::OK.into_response()))
}

async fn handle_no_2fa(
    jar: CookieJar,
    email: &Email,
) -> (CookieJar, Result<Response<Body>, AuthAPIError>) {
    let auth_cookie = match generate_auth_cookie(email) {
        Ok(c) => c,
        Err(_) => return (jar, Err(AuthAPIError::UnexpectedError)),
    };

    let updated_jar = jar.add(auth_cookie);
    let response = StatusCode::OK.into_response();

    (updated_jar, Ok(response))
}

async fn handle_2fa<
    T: UserStore + Clone + Send + Sync,
    T1: BannedTokenStore + Clone + Send + Sync,
    T2: TwoFACodeStore + Clone + Send + Sync,
>(
    jar: CookieJar,
    state: &AppState<T, T1, T2>,
    email: &Email,
) -> (CookieJar, Result<Response<Body>, AuthAPIError>) {
    let login_attempt_id = LoginAttemptId::default();
    let code = TwoFACode::default();
    {
        let mut s = state.two_fa_store.try_write().unwrap();
        s.add_code(email.clone(), login_attempt_id.clone(), code)
            .await;
    }
    let response = Json(TwoFactorAuthResponse {
        message: "2FA required".to_string(),
        login_attempt_id: login_attempt_id.as_ref().to_string(),
    });
    (
        jar,
        Ok((StatusCode::PARTIAL_CONTENT, response).into_response()),
    )
}

// The login route can return 2 possible success responses.
// This enum models each response!
#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum LoginResponse {
    RegularAuth,
    TwoFactorAuth(TwoFactorAuthResponse),
}

// If a user requires 2FA, this JSON body should be returned!
#[derive(Debug, Serialize, Deserialize)]
pub struct TwoFactorAuthResponse {
    pub message: String,
    #[serde(rename = "loginAttemptId")]
    pub login_attempt_id: String,
}

use std::fmt::Display;

use axum::{
    body::Body,
    extract::State,
    http::{Response, StatusCode},
    response::IntoResponse,
    Json,
};
use axum_extra::extract::CookieJar;
use serde::Deserialize;

use crate::{
    app_state::AppState,
    domains::{data_stores::UserStore, email::Email, error::AuthAPIError, password::Password},
    utils::auth::generate_auth_cookie,
};
#[derive(Deserialize, Debug)]
pub struct LoginInfo {
    email: String,
    password: String,
}

pub(crate) async fn login<T: UserStore + Clone + Send + Sync>(
    State(state): State<AppState<T>>,
    jar: CookieJar,
    Json(request): Json<LoginInfo>,
) -> (CookieJar, Result<Response<Body>, AuthAPIError>) {
    let email: Email;
    let password: Password;

    match Email::parse(request.email) {
        Ok(value) => email = value,
        Err(s) => return (jar, Err(AuthAPIError::InvalidCredentials)),
    }
    match Password::parse(request.password) {
        Ok(value) => password = value,
        Err(s) => return (jar, Err(AuthAPIError::InvalidCredentials)),
    }

    let valid = state
        .user_store
        .read()
        .await
        .validate_user(&email, password.as_ref())
        .await;

    if valid.is_err() {
        (jar, Err(AuthAPIError::IncorrectCredentials))
    } else {
        let auth_cookie = generate_auth_cookie(&email);
        match auth_cookie {
            Ok(c) => {
                return (jar.add(c), Ok(StatusCode::OK.into_response()));
            }
            Err(e) => (jar, Err(AuthAPIError::UnexpectedError)),
        }
    }
}

use axum::{http::StatusCode, response::IntoResponse};
use axum_extra::extract::CookieJar;

use crate::{
    domains::error::AuthAPIError,
    utils::{auth::validate_token, constants::JWT_COOKIE_NAME},
};

pub(crate) async fn logout(jar: CookieJar) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    let cookie = match jar.get(JWT_COOKIE_NAME) {
        Some(v) => v,
        None => return (jar, Err(AuthAPIError::MissingToken)),
    };
    let token = cookie.value().to_owned();

    let result = validate_token(&token).await;

    //if the token is valid. Remove token and return 200.
    match result {
        Ok(v) => (jar.remove(JWT_COOKIE_NAME), Ok(StatusCode::OK)),
        Err(e) => (jar, Err(AuthAPIError::InvalidToken)),
    }
}

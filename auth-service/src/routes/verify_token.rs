use axum::extract::State;
use axum::Json;
use axum::{http::StatusCode, response::IntoResponse};
use serde::Deserialize;

use crate::app_state::AppState;
use crate::domains::data_stores::UserStore;
use crate::domains::email::Email;
use crate::domains::error::AuthAPIError;
use crate::utils::auth;

pub(crate) async fn verify_token(
    Json(request): Json<VerifyTokenString>,
) -> Result<impl IntoResponse, AuthAPIError> {
    match auth::validate_token(&request.token).await {
        Ok(s) => Ok(StatusCode::OK.into_response()),
        Err(e) => Err(AuthAPIError::InvalidToken),
    }
}
#[derive(Deserialize)]
pub struct VerifyTokenString {
    token: String,
}

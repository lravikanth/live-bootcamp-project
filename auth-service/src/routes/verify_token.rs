use axum::extract::State;
use axum::Json;
use axum::{http::StatusCode, response::IntoResponse};
use serde::Deserialize;

use crate::app_state::AppState;
use crate::domains::data_stores::{BannedTokenStore, TwoFACodeStore, UserStore};
use crate::domains::email::Email;
use crate::domains::error::AuthAPIError;
use crate::utils::auth;

pub(crate) async fn verify_token<
    T: UserStore + Send + Sync + Clone,
    T1: BannedTokenStore + Send + Sync + Clone,
    T2: TwoFACodeStore + Clone + Send + Sync,
>(
    State(app_state): State<AppState<T, T1, T2>>,
    Json(request): Json<VerifyTokenString>,
) -> Result<impl IntoResponse, AuthAPIError> {
    match auth::validate_token(&request.token, app_state.banned_token_store).await {
        Ok(s) => Ok(StatusCode::OK.into_response()),
        Err(e) => Err(AuthAPIError::InvalidToken),
    }
}
#[derive(Deserialize)]
pub struct VerifyTokenString {
    token: String,
}

use ::serde::{Deserialize, Serialize};
use axum::extract::State;
use axum::Json;
use axum::{http::StatusCode, response::IntoResponse};
use axum_extra::extract::CookieJar;
use uuid::serde;

use crate::app_state::AppState;
use crate::domains::data_stores::{
    BannedTokenStore, LoginAttemptId, TwoFACode, TwoFACodeStore, UserStore,
};
use crate::domains::email::Email;
use crate::domains::error::AuthAPIError;
use crate::domains::EmailClient;
use crate::utils::auth::generate_auth_cookie;

pub(crate) async fn verify_2fa<
    T: UserStore + Clone + Send + Sync,
    T1: BannedTokenStore + Clone + Send + Sync,
    T2: TwoFACodeStore + Clone + Send + Sync,
    T3: EmailClient + Clone + Send + Sync,
>(
    jar: CookieJar,
    State(state): State<AppState<T, T1, T2, T3>>,
    Json(request): Json<Verify2FARequest>,
) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    let email: Email = match Email::parse(request.email) {
        Ok(e) => e,
        Err(_) => return (jar, Err(AuthAPIError::InvalidCredentials)),
    };

    let login_attempt_id = match LoginAttemptId::parse(request.login_attempt_id) {
        Ok(e) => e,
        Err(_) => return (jar, Err(AuthAPIError::InvalidCredentials)),
    };

    let two_fa_code = match TwoFACode::parse(request.two_fa_Code) {
        Ok(e) => e,
        Err(_) => return (jar, Err(AuthAPIError::InvalidCredentials)),
    };
    let mut write_lock = state.two_fa_store.write().await;
    let stored_login_attempt_id = match write_lock.get_code(&email).await {
        Ok(e) => e,
        Err(_) => return (jar, Err(AuthAPIError::IncorrectCredentials)),
    };

    if !(login_attempt_id == stored_login_attempt_id.0 && two_fa_code == stored_login_attempt_id.1)
    {
        return (jar, Err(AuthAPIError::IncorrectCredentials));
    }

    write_lock.remove_code(&email).await;
    let auth_cookie = match generate_auth_cookie(&email) {
        Ok(c) => c,
        Err(_) => return (jar, Err(AuthAPIError::UnexpectedError)),
    };

    let updated_jar = jar.add(auth_cookie);

    (updated_jar, Ok(StatusCode::OK.into_response()))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Verify2FARequest {
    email: String,
    #[serde(rename = "loginAttemptId")]
    pub login_attempt_id: String,
    #[serde(rename = "2FACode")]
    two_fa_Code: String,
}

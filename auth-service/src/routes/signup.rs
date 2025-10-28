use crate::{
    domains::{
        data_stores::{BannedTokenStore, TwoFACodeStore, UserStore},
        email::Email,
        error::AuthAPIError,
        password::{self, Password},
        user::User,
    },
    AppState,
};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

pub async fn signup<
    T: UserStore + Clone + Send + Sync,
    T1: BannedTokenStore + Clone + Send + Sync,
    T2: TwoFACodeStore + Clone + Send + Sync,
>(
    State(state): State<AppState<T, T1, T2>>,
    Json(request): Json<SignupRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let email;
    let password;

    match Email::parse(request.email) {
        Ok(value) => email = value,
        Err(s) => return Err(AuthAPIError::InvalidCredentials),
    }
    match Password::parse(request.password) {
        Ok(value) => password = value,
        Err(s) => return Err(AuthAPIError::InvalidCredentials),
    }

    let user = User {
        email,
        password,
        requires_2fa: request.requires_2fa,
    };

    let mut user_store = state.user_store.write().await;
    let existing_user = user_store.get_user(&user.email).await;
    if existing_user.is_ok() {
        return Err(AuthAPIError::UserAlreadyExists);
    }

    let add_user = user_store.add_user(user).await;
    if (add_user.is_err()) {
        return Err(AuthAPIError::UnexpectedError);
    }

    let response = Json(SignupResponse {
        message: "User created successfully!".to_string(),
    });

    Ok((StatusCode::CREATED, response))
}

#[derive(Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}

#[derive(Serialize, PartialEq, Debug, serde::Deserialize)]
pub struct SignupResponse {
    pub message: String,
}

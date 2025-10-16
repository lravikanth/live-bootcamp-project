use crate::{
    domains::{error::AuthAPIError, user::User},
    AppState,
};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

pub async fn signup(
    State(state): State<AppState>,
    Json(request): Json<SignupRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let user = User {
        email: request.email,
        password: request.password,
        requires_2fa: request.requires_2fa,
    };

    if (!user.email.contains("@") || user.email.len() == 0 || user.password.len() < 8) {
        let response = Json(SignupResponse {
            message: "User created successfully!".to_string(),
        });
        return Err(AuthAPIError::InvalidCredentials);
    }

    let mut user_store = state.user_store.write().await;
    let existing_user = user_store.get_user(&user.email);
    if existing_user.is_ok() {
        return Err(AuthAPIError::UserAlreadyExists);
    }

    let add_user = user_store.add_user(user);
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

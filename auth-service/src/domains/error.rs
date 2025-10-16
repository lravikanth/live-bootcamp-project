use axum::{
    http::{Response, StatusCode},
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};

pub enum AuthAPIError {
    UserAlreadyExists,
    InvalidCredentials,
    UnexpectedError,
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    message: String,
}

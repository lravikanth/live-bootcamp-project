use serde::{Deserialize, Serialize};

pub enum AuthAPIError {
    UserAlreadyExists,
    InvalidCredentials,
    UnexpectedError,
    IncorrectCredentials,
    MissingToken,
    InvalidToken,
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    message: String,
}

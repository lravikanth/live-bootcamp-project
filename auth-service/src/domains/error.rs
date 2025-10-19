use serde::{Deserialize, Serialize};

pub enum AuthAPIError {
    UserAlreadyExists,
    InvalidCredentials,
    UnexpectedError,
    IncorrectCredentials,
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    message: String,
}

use axum::{http::StatusCode, response::IntoResponse};

pub(crate) async fn verify_token() -> impl IntoResponse {
    StatusCode::OK.into_response()
}

use axum::{http::StatusCode, response::IntoResponse};

pub(crate) async fn verify_2fa() -> impl IntoResponse {
    StatusCode::OK.into_response()
}

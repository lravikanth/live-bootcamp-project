use axum::{http::StatusCode, response::IntoResponse};

pub(crate) async fn login() -> impl IntoResponse {
    StatusCode::OK.into_response()
}

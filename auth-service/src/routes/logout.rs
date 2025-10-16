use axum::{http::StatusCode, response::IntoResponse};

pub(crate) async fn logout() -> impl IntoResponse {
    StatusCode::OK.into_response()
}

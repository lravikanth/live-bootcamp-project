pub mod app_state;
mod domains;
pub mod routes;
pub mod services;

use crate::{domains::error::AuthAPIError, routes::*};

use std::error::Error;

use app_state::AppState;
use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    serve::Serve,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tower_http::services::ServeDir;

pub struct Application {
    server: Serve<Router, Router>,
    pub address: String,
}

impl Application {
    pub async fn build(app_state: AppState, address: &str) -> Result<Self, Box<dyn Error>> {
        // Move the Router definition from `main.rs` to here.
        // Also, remove the `hello` route.
        // We don't need it at this point!
        let router = Router::new()
            .nest_service("/", ServeDir::new("assets"))
            .route("/login", post(login))
            .route("/verify-2fa", post(verify_2fa))
            .route("/logout", post(logout))
            .route("/verify-token", post(verify_token))
            .route("/signup", post(signup))
            .with_state(app_state);

        let listener = tokio::net::TcpListener::bind(address).await?;
        let address = listener.local_addr()?.to_string();
        let server = axum::serve(listener, router);

        // Create a new Application instance and return it
        Ok(Application { server, address })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        println!("listening on {}", &self.address);
        self.server.await
    }
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub message: String,
}

impl IntoResponse for AuthAPIError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            AuthAPIError::UserAlreadyExists => (StatusCode::CONFLICT, "User Already exists"),
            AuthAPIError::InvalidCredentials => (StatusCode::BAD_REQUEST, "Invalid Credentials"),
            AuthAPIError::UnexpectedError => (StatusCode::INTERNAL_SERVER_ERROR, "Unexpcted Error"),
        };

        let body = Json(ErrorResponse {
            message: error_message.to_string(),
        });
        (status, body).into_response()
    }
}

#![allow(dead_code, unused_imports, unused_variables, unused_mut)]
pub mod app_state;
pub mod domains;
pub mod routes;
pub mod services;
pub mod utils;

use crate::{
    domains::{data_stores::UserStore, error::AuthAPIError},
    routes::*,
};

use std::error::Error;

use app_state::AppState;
use axum::{
    http::{Method, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    serve::Serve,
    Json, Router,
};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};
use tower_http::{cors::CorsLayer, services::ServeDir};

pub struct Application {
    server: Serve<Router, Router>,
    pub address: String,
}

impl Application {
    pub async fn build<T: UserStore + Clone + Send + Sync + 'static>(
        app_state: AppState<T>,
        address: &str,
    ) -> Result<Self, Box<dyn Error>> {
        // Move the Router definition from `main.rs` to here.
        // Also, remove the `hello` route.
        // We don't need it at this point!
        let allowed_origins = [
            "http://localhost:8000".parse()?,
            // TODO: Replace [YOUR_DROPLET_IP] with your Droplet IP address
            "http://localhost:3000".parse()?,
        ];

        let cors = CorsLayer::new()
            // Allow GET and POST requests
            .allow_methods([Method::GET, Method::POST])
            // Allow cookies to be included in requests
            .allow_credentials(true)
            .allow_origin(allowed_origins);

        let router = Router::new()
            .nest_service("/", ServeDir::new("assets"))
            .route("/signup", post(signup))
            .route("/login", post(login))
            .route("/verify-2fa", post(verify_2fa))
            .route("/logout", post(logout))
            .route("/verify-token", post(verify_token))
            .with_state(app_state)
            .layer(cors); // Add CORS config to our Axum router

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
            AuthAPIError::IncorrectCredentials => (StatusCode::UNAUTHORIZED, "Unauthorized"),
            AuthAPIError::MissingToken => (StatusCode::BAD_REQUEST, "Invalid Credentials"),
            AuthAPIError::InvalidToken => (StatusCode::UNAUTHORIZED, "Unauthorized"),
        };

        let body = Json(ErrorResponse {
            message: error_message.to_string(),
        });
        (status, body).into_response()
    }
}

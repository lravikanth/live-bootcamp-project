use axum::{extract::State, http::StatusCode, response::IntoResponse};
use axum_extra::extract::CookieJar;

use crate::{
    app_state::{self, AppState},
    domains::{
        data_stores::{BannedTokenStore, TwoFACodeStore, UserStore},
        error::AuthAPIError,
    },
    utils::{auth::validate_token, constants::JWT_COOKIE_NAME},
};

pub(crate) async fn logout<
    T: UserStore + Clone + Send + Sync,
    T1: BannedTokenStore + Clone + Send + Sync,
    T2: TwoFACodeStore + Clone + Send + Sync,
>(
    jar: CookieJar,
    State(state): State<AppState<T, T1, T2>>,
) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    let cookie = match jar.get(JWT_COOKIE_NAME) {
        Some(v) => v,
        None => return (jar, Err(AuthAPIError::MissingToken)),
    };
    let token = cookie.value().to_owned();
    let cloned_banned_token_store = state.banned_token_store.clone();

    let result = validate_token(&token, cloned_banned_token_store).await;

    //if the token is valid. Remove token and return 200.
    match result {
        Ok(v) => {
            let mut banned_store = state.banned_token_store.write().await;
            let res = banned_store.add_banned_token(token).await;
            (jar.remove(JWT_COOKIE_NAME), Ok(StatusCode::OK))
        }
        Err(e) => (jar, Err(AuthAPIError::InvalidToken)),
    }
}

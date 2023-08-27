pub mod dto;

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

use axum::extract::{Query, State};
use axum::response::Redirect;
use axum::routing::{get, post};
use axum::{Json, Router};

use crate::{
    common::{Result, ValidatedJson},
    config::oauth::{OAuthSharedState, OAuthState},
    services::auth::oauth::oauth_google_return,
    services::auth::{login, oauth::oauth_google, register},
};

use self::dto::{AuthPayload, LoginDto};
use super::users::dto::CreateUserDto;

pub fn create_auth_routes() -> Router {
    let oauth_state = Arc::new(Mutex::new(OAuthState::new()));

    let router = Router::new()
        .route("/login", post(login_route))
        .route("/register", post(register_route))
        .route("/google-oauth", get(oauth_google_route))
        .route("/google-oauth-return", get(oauth_google_return_route))
        .with_state(oauth_state);
    Router::new().nest("/auth", router)
}

async fn register_route(
    ValidatedJson(dto): ValidatedJson<CreateUserDto>,
) -> Result<Json<AuthPayload>> {
    let res = register(dto).await?;
    Ok(Json(res))
}

async fn login_route(ValidatedJson(dto): ValidatedJson<LoginDto>) -> Result<Json<AuthPayload>> {
    let res = login(dto).await?;
    Ok(Json(res))
}

async fn oauth_google_route(
    Query(params): Query<HashMap<String, String>>,
    State(state): State<OAuthSharedState>,
) -> Result<Redirect> {
    let res = oauth_google(params, state).await?;
    Ok(res)
}

async fn oauth_google_return_route(
    Query(params): Query<HashMap<String, String>>,
    State(state): State<OAuthSharedState>,
) -> Result<Redirect> {
    let res = oauth_google_return(params, state).await?;
    Ok(res)
}

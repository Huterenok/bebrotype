pub mod dto;

use axum::routing::post;
use axum::{Json, Router};

use crate::services::auth::{login, register};

use crate::config::Result;

use self::dto::{AuthPayload, LoginDto};

use super::users::dto::CreateUserDto;

pub fn create_auth_routes() -> Router {
    let router = Router::new()
        .route("/login", post(login_route))
        .route("/register", post(register_route));
    Router::new().nest("/auth", router)
}

async fn register_route(Json(dto): Json<CreateUserDto>) -> Result<Json<AuthPayload>> {
    match register(dto).await {
        Ok(data) => Ok(Json(data)),
        Err(err) => Err(err),
    }
}

async fn login_route(Json(dto): Json<LoginDto>) -> Result<Json<AuthPayload>> {
    match login(dto).await {
        Ok(data) => Ok(Json(data)),
        Err(err) => Err(err),
    }
}

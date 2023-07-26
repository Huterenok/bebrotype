pub mod dto;

use axum::routing::post;
use axum::{Json, Router};

use crate::services::auth::{login, register};

use crate::repositories::error::Result;

use self::dto::{AuthPayload, LoginDto};

use super::user::dto::CreateUserDto;

pub fn create_auth_routes() -> Router {
    Router::new()
				.route("/login", post(login_route))
        .route("/register", post(register_route))
        
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

pub mod dto;

use axum::routing::post;
use axum::{Json, Router};

use crate::services::auth::{login, register};

use crate::config::{Result, ValidatedJson};

use self::dto::{AuthPayload, LoginDto};

use super::users::dto::CreateUserDto;

pub fn create_auth_routes() -> Router {
    let router = Router::new()
        .route("/login", post(login_route))
        .route("/register", post(register_route));
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

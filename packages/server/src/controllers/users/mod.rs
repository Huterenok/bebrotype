pub mod dto;

use axum::extract::{Multipart, Path, Query};
use axum::routing::{get, patch};
use axum::{Extension, Json, Router};

use crate::{
    common::Result,
    entities::User,
    services::users::{get_all_users, get_user_by_id, update_user},
};

use self::dto::{AllUsersQuery, UserResponseDto};

pub fn create_public_user_routes() -> Router {
    let router = Router::new()
        .route("/all", get(get_all_users_route))
        .route("/:id", get(get_user_route));
    Router::new().nest("/user", router)
}

pub fn create_private_user_routes() -> Router {
    let router = Router::new()
        .route("/edit", patch(update_user_route))
        .route("/whoami", get(whoami));
    Router::new().nest("/user", router)
}

pub async fn whoami(Extension(user): Extension<User>) -> Result<Json<UserResponseDto>> {
    Ok(Json(user.into()))
}

async fn get_user_route(Path(id): Path<i64>) -> Result<Json<UserResponseDto>> {
    let res = get_user_by_id(id).await?.into();
    Ok(Json(res))
}

async fn get_all_users_route(query: Query<AllUsersQuery>) -> Result<Json<Vec<UserResponseDto>>> {
    let res = get_all_users(query.0)
        .await?
        .into_iter()
        .map(|user| user.into())
        .collect();
    Ok(Json(res))
}

async fn update_user_route(
    Extension(user): Extension<User>,
    data: Multipart,
) -> Result<Json<UserResponseDto>> {
    let res = update_user(data, user).await?.into();
    Ok(Json(res))
}

pub mod dto;

use axum::extract::{Path, Query};
use axum::routing::get;
use axum::{middleware, Extension, Json, Router};
use axum_macros::debug_handler;

use crate::entities::user::FilteredUser;
use crate::entities::User;

use crate::services::auth::jwt::authentication_middleware;
use crate::services::user::{get_all_users, get_user_by_id};

use crate::repositories::error::Result;

use self::dto::AllUsersQuery;

pub fn create_user_routes() -> Router {
    let user_router = Router::new()
        .route("/all", get(get_all_users_route))
        .route_layer(middleware::from_fn(authentication_middleware))
        .route("/:id", get(get_user_route));

    user_router
}

async fn get_user_route(Path(id): Path<i32>) -> Result<Json<FilteredUser>> {
    let res = get_user_by_id(id).await?.into();
    Ok(Json(res))
}

#[debug_handler]
async fn get_all_users_route(
    Extension(_): Extension<User>,
    query: Query<AllUsersQuery>,
) -> Result<Json<Vec<FilteredUser>>> {
    let res: Vec<FilteredUser> = get_all_users(query.0)
        .await?
        .into_iter()
        .map(|user| user.into())
        .collect();
    Ok(Json(res))
}

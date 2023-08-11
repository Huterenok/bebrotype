pub mod dto;

use axum::extract::Path;
use axum::routing::{get, patch, post};
use axum::{Extension, Json, Router};

use crate::entities::{Text, User};

use crate::repositories::error::Result;

use crate::services::texts::{
    create_text, get_text_by_id, get_texts_by_user_id, toggle_favourite_text, get_favourite_texts,
};

use self::dto::CreateTextDto;

pub fn create_public_text_routes() -> Router {
    let router = Router::new()
        .route("/user/:id", get(get_texts_by_user_id_route))
        .route("/:id", get(get_text_by_id_route));
    Router::new().nest("/text", router)
}

pub fn create_private_text_routes() -> Router {
    let router = Router::new()
        .route("/", post(create_text_route))
        .route("/favourite/:id", patch(toggle_favourite_text_route))
        .route("/favourite", get(get_favourite_texts_route));
    Router::new().nest("/text", router)
}

async fn get_text_by_id_route(Path(id): Path<i32>) -> Result<Json<Text>> {
    let texts = get_text_by_id(id).await?;
    Ok(Json(texts))
}

async fn get_texts_by_user_id_route(Path(id): Path<i32>) -> Result<Json<Vec<Text>>> {
    let texts = get_texts_by_user_id(id).await?;
    Ok(Json(texts))
}

async fn create_text_route(
    Extension(user): Extension<User>,
    Json(text): Json<CreateTextDto>,
) -> Result<Json<Text>> {
    let res = create_text(text, user.id).await?;
    Ok(Json(res))
}

async fn get_favourite_texts_route(
    Extension(user): Extension<User>
) -> Result<Json<Vec<Text>>> {
    let res = get_favourite_texts(user.favourite_texts).await?;
    Ok(Json(res))
}

async fn toggle_favourite_text_route(
    Extension(user): Extension<User>,
    Path(id): Path<i32>,
) -> Result<Json<bool>> {
    let res = toggle_favourite_text(id, user).await?;
    Ok(Json(res))
}

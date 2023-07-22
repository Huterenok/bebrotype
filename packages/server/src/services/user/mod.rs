use axum::extract::Path;
use axum::routing::{get};
use axum::{Json, Router};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use hyper::StatusCode;
use serde::Deserialize;

use crate::entities::schema::users;
use crate::entities::schema::users::dsl::*;
use crate::entities::User;
use crate::repositories::database::DB;
use crate::repositories::error::{Error, Result};

pub fn create_user_service() -> Router {
    let user_router = Router::new().route("/:user_id", get(get_user));
    user_router
}

async fn get_user(Path(user_id): Path<i32>) -> Result<Json<User>> {
    let mut conn = DB.get().await.pool.get().await.unwrap();
    println!("{user_id}");
    let result = users
        .find(user_id)
        .select(User::as_select())
        .first(&mut conn)
        .await;
    match result {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err((StatusCode::BAD_REQUEST, Json(Error::UserNotFound))),
    }
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct CreateUserRequest {
    username: String,
    email: String,
		password: String
}

pub async fn create_user(user: CreateUserRequest) -> Result<User> {
    let mut conn = DB.get().await.pool.get().await.unwrap();
    let result = diesel::insert_into(users::table)
        .values(&user)
        .returning(User::as_returning())
        .get_result(&mut conn)
        .await;

    match result {
        Ok(user) => Ok(user),
        Err(_) => {
            Err((StatusCode::BAD_REQUEST, Json(Error::UserAlreadyExist)))
        }
    }
}

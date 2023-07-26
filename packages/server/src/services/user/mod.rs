use axum::Json;

use hyper::StatusCode;

use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use crate::entities::schema::users::{self, dsl::*, email};
use crate::entities::User;

use crate::controllers::user::dto::{AllUsersQuery, CreateUserDto};

use crate::repositories::crypto::CR;
use crate::repositories::database::DB;
use crate::repositories::error::{Error, Result};

pub async fn get_user_by_id(user_id: i32) -> Result<User> {
    let mut conn = DB.get().await.get_conn().await?;

    let result = users
        .find(user_id)
        .select(User::as_select())
        .first(&mut conn)
        .await;

    match result {
        Ok(user) => Ok(user),
        Err(_) => Err((StatusCode::BAD_REQUEST, Json(Error::UserNotFound))),
    }
}

pub async fn get_user_by_email(user_email: String) -> Result<User> {
    let mut conn = DB.get().await.get_conn().await?;

    let result = users
        .filter(email.eq(user_email))
        .select(User::as_select())
        .first(&mut conn)
        .await;

    match result {
        Ok(user) => Ok(user),
        Err(_) => Err((StatusCode::BAD_REQUEST, Json(Error::UserNotFound))),
    }
}

pub async fn get_all_users(query: AllUsersQuery) -> Result<Vec<User>> {
    let mut conn = DB.get().await.get_conn().await?;

    let result = users
        .limit(query.limit.unwrap_or(20) as i64)
        .select(User::as_select())
        .load(&mut conn)
        .await;

    match result {
        Ok(user) => Ok(user),
        Err(_) => Err((StatusCode::BAD_REQUEST, Json(Error::UserNotFound))),
    }
}

pub async fn create_user(mut dto: CreateUserDto) -> Result<User> {
    let mut conn = DB.get().await.get_conn().await?;

    dto.password = CR.mc_encrypt(&dto.password);
    let result = diesel::insert_into(users::table)
        .values(&dto)
        .returning(User::as_returning())
        .get_result(&mut conn)
        .await;

    match result {
        Ok(user) => Ok(user),
        Err(_) => Err((StatusCode::BAD_REQUEST, Json(Error::UserAlreadyExist))),
    }
}

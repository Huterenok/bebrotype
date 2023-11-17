use axum::response::IntoResponse;

use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use crate::entities::schema::users::{
    avatar as avatar_column, dsl::users as users_table, email as email_column, id as id_column,
    near_address as near_address_column, password as password_column, username as username_column,
};

use crate::{
    common::{Error, Result},
    config::DB,
    controllers::users::dto::{CreateUserDto, UpdateUserDto},
    entities::User,
};

pub async fn create(dto: CreateUserDto) -> Result<User> {
    let mut conn = DB().await.get_conn().await?;

    let result = diesel::insert_into(users_table)
        .values((
            username_column.eq(dto.username),
            email_column.eq(dto.email),
            password_column.eq(dto.password),
            avatar_column.eq(dto.avatar),
        ))
        .returning(User::as_returning())
        .get_result(&mut conn)
        .await;

    match result {
        Ok(user) => Ok(user),
        Err(err) => {
            println!("------------------- {:?} --------", err);
            Err(Error::UserAlreadyExist.into_response())
        }
    }
}

pub async fn get_by_id(id: i64) -> Result<User> {
    let mut conn = DB().await.get_conn().await?;

    let result = users_table
        .find(id)
        .select(User::as_select())
        .first(&mut conn)
        .await;

    match result {
        Ok(user) => Ok(user),
        Err(_) => Err(Error::UserByIdNotFound(id).into_response()),
    }
}

pub async fn get_by_email(email: String) -> Result<User> {
    let mut conn = DB().await.get_conn().await?;

    let result = users_table
        .filter(email_column.eq(&email))
        .select(User::as_select())
        .first(&mut conn)
        .await;

    match result {
        Ok(user) => Ok(user),
        Err(_) => Err(Error::UserByEmailNotFound(email).into_response()),
    }
}

pub async fn get_all(limit: i64) -> Result<Vec<User>> {
    let mut conn = DB().await.get_conn().await?;

    let result = users_table
        .limit(limit)
        .select(User::as_select())
        .load(&mut conn)
        .await;

    match result {
        Ok(user) => Ok(user),
        Err(_) => Err(Error::UsersNotFound.into_response()),
    }
}

pub async fn update(id: i64, dto: UpdateUserDto) -> Result<User> {
    let mut conn = DB().await.get_conn().await?;

    let res = diesel::update(users_table.filter(id_column.eq(id)))
        .set((
            avatar_column.eq(dto.avatar),
            near_address_column.eq(dto.near_address),
        ))
        .get_result::<User>(&mut conn)
        .await;

    match res {
        Ok(user) => Ok(user.into()),
        //TODO
        Err(_) => Err(Error::NearUserAlreadyExist.into_response()),
    }
}

// pub async fn update_favourites_texts(id: i64, texts: Vec<Option<i64>>) -> Result<Vec<Option<i64>>> {
//     let mut conn = DB().await.get_conn().await?;

//     let res = diesel::update(users_table.filter(id_column.eq(id)))
//         .set(favourite_texts_column.eq(texts))
//         .get_result::<User>(&mut conn)
//         .await;

//     match res {
//         Ok(user) => Ok(user.favourite_texts),
//         //TODO
//         Err(_) => Err(Error::UserAlreadyExist.into_response()),
//     }
// }

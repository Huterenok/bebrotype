use axum::response::IntoResponse;

use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use crate::config::{Error, Result, DB};
use crate::entities::schema::texts::{
    content as content_column, dsl::texts as texts_table, id as text_id_column,
    likes as likes_column, title as title_column, user_id as user_id_column,
};
use crate::entities::{Text, User};

use crate::controllers::texts::dto::{TextDto, UpdateTextDto};

pub async fn create(dto: TextDto) -> Result<Text> {
    let mut conn = DB().await.get_conn().await?;

    let result = diesel::insert_into(texts_table)
        .values((
            title_column.eq(dto.title),
            content_column.eq(dto.content),
            user_id_column.eq(dto.user_id),
        ))
        .returning(Text::as_returning())
        .get_result(&mut conn)
        .await;

    match result {
        Ok(text) => Ok(text),
        Err(_) => Err(Error::UserAlreadyExist.into_response()),
    }
}

pub async fn get_by_id(id: i64) -> Result<Text> {
    let mut conn = DB().await.get_conn().await?;

    let result = texts_table
        .find(id)
        .select(Text::as_select())
        .first(&mut conn)
        .await;

    match result {
        Ok(text) => Ok(text),
        Err(_) => Err(Error::TextByIdNotFound(id).into_response()),
    }
}

pub async fn get_by_user(user: User) -> Result<Vec<Text>> {
    let mut conn = DB().await.get_conn().await.unwrap();

    let data = Text::belonging_to(&user)
        .select(Text::as_select())
        .load(&mut conn)
        .await;

    match data {
        Ok(texts) => match texts.len() {
            0 => Err(Error::TextByUserIdNotFound(user.id).into_response()),
            _ => Ok(texts),
        },
        Err(_) => Err(Error::TextByUserIdNotFound(user.id).into_response()),
    }
}

pub async fn update(id: i64, dto: UpdateTextDto) -> Result<Text> {
    let mut conn = DB().await.get_conn().await?;

    let res = diesel::update(texts_table.filter(text_id_column.eq(id)))
        .set((title_column.eq(dto.title), content_column.eq(dto.content)))
				.returning(Text::as_returning())
        .get_result(&mut conn)
        .await;

    match res {
        Ok(text) => Ok(text),
        //TODO
        Err(_) => Err(Error::TextByIdNotFound(id).into_response()),
    }
}

pub async fn update_likes(id: i64, likes: i32) -> Result<Text> {
    let mut conn = DB().await.get_conn().await?;

    let res = diesel::update(texts_table.filter(text_id_column.eq(id)))
        .set(likes_column.eq(likes))
        .returning(Text::as_returning())
        .get_result(&mut conn)
        .await;

    match res {
        Ok(text) => Ok(text),
        //TODO
        Err(_) => Err(Error::TextByIdNotFound(id).into_response()),
    }
}

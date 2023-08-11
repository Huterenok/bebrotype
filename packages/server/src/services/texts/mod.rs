use axum::response::IntoResponse;

use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use crate::entities::schema::texts::dsl::{
    id as text_id_column, likes as likes_column, texts as texts_table,
};
use crate::entities::schema::users::dsl::{
    favourite_texts as favourite_texts_column, id as user_id_column, users as users_table,
};
use crate::entities::{Text, User};

use crate::controllers::texts::dto::CreateTextDto;

use crate::repositories::database::DB;
use crate::repositories::error::Error;
use crate::repositories::error::Result;

use super::users::get_user_by_id;

pub async fn get_text_by_id(text_id: i32) -> Result<Text> {
    let mut conn = DB().await.get_conn().await?;

    let result = texts_table
        .find(text_id)
        .select(Text::as_select())
        .first(&mut conn)
        .await;

    match result {
        Ok(text) => Ok(text),
        Err(_) => Err(Error::TextByIdNotFound(text_id).into_response()),
    }
}

pub async fn get_texts_by_user_id(user_id: i32) -> Result<Vec<Text>> {
    let mut conn = DB().await.get_conn().await.unwrap();

    let user = get_user_by_id(user_id).await?;
    let data = Text::belonging_to(&user)
        .select(Text::as_select())
        .load(&mut conn)
        .await;

    match data {
        Ok(texts) => match texts.len() {
            0 => Err(Error::TextByUserIdNotFound(user_id).into_response()),
            _ => Ok(texts),
        },
        Err(_) => Err(Error::TextByUserIdNotFound(user_id).into_response()),
    }
}

pub async fn create_text(dto: CreateTextDto, user_id: i32) -> Result<Text> {
    let mut conn = DB().await.get_conn().await?;
    let text = dto.into_text(user_id);

    let result = diesel::insert_into(texts_table)
        .values(&text)
        .returning(Text::as_returning())
        .get_result(&mut conn)
        .await;

    match result {
        Ok(text) => Ok(text),
        Err(_) => Err(Error::UserAlreadyExist.into_response()),
    }
}

//TODO: id shouldn't be Option<i32>
pub async fn get_favourite_texts(text_ids: Vec<Option<i32>>) -> Result<Vec<Text>> {
    let mut res: Vec<Text> = Vec::with_capacity(text_ids.len());

    for id in text_ids {
        res.push(get_text_by_id(id.unwrap()).await?);
    }

    Ok(res)
}

//TODO: favourite_text_id shouldn't be Option<i32>
//TODO: structure of route
pub async fn toggle_favourite_text(text_id: i32, user: User) -> Result<bool> {
    let mut conn = DB().await.get_conn().await?;

    let mut favourite_texts = user.favourite_texts;
    let likes = get_text_by_id(text_id).await?.likes;

    if favourite_texts.contains(&Some(text_id)) {
        favourite_texts = favourite_texts
            .into_iter()
            .filter(|el| el.unwrap() != text_id)
            .collect::<Vec<Option<i32>>>();
        diesel::update(users_table.filter(user_id_column.eq(user.id)))
            .set(favourite_texts_column.eq(favourite_texts))
            .execute(&mut conn)
            .await
            .unwrap();
        diesel::update(texts_table.filter(text_id_column.eq(text_id)))
            .set(likes_column.eq(likes - 1))
            .execute(&mut conn)
            .await
            .unwrap();

        Ok(false)
    } else {
        favourite_texts.push(Some(text_id));
        diesel::update(users_table.filter(user_id_column.eq(user.id)))
            .set(favourite_texts_column.eq(favourite_texts))
            .execute(&mut conn)
            .await
            .unwrap();
        diesel::update(texts_table.filter(text_id_column.eq(text_id)))
            .set(likes_column.eq(likes + 1))
            .execute(&mut conn)
            .await
            .unwrap();
        Ok(true)
    }
}

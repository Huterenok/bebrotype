use axum::response::IntoResponse;

use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use crate::entities::liked_text::LikedText;
use crate::entities::schema::liked_texts::{
    dsl::liked_texts as liked_texts_table, text_id as text_id_liked_column,
    user_id as user_id_liked_column,
};
use crate::entities::schema::texts::{
    content as content_column, dsl::texts as texts_table, id as text_id_column,
    likes as likes_column, title as title_column, user_id as user_id_column,
};

use crate::{
    common::{Error, Result},
    config::DB,
    controllers::texts::dto::{TextDto, UpdateTextDto},
    entities::{Text, User},
};

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

pub async fn get_all(limit: i64) -> Result<Vec<Text>> {
    let mut conn = DB().await.get_conn().await?;

    let result = texts_table
        .limit(limit)
        .select(Text::as_select())
        .load(&mut conn)
        .await;

    match result {
        Ok(texts) => Ok(texts),
        Err(_) => Err(Error::NoTextsFound.into_response()),
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
            0 => Err(Error::TextsByUserIdNotFound(user.id).into_response()),
            _ => Ok(texts),
        },
        Err(_) => Err(Error::TextsByUserIdNotFound(user.id).into_response()),
    }
}

pub async fn get_liked(user: &User) -> Result<Vec<Text>> {
    let mut conn = DB().await.get_conn().await.unwrap();

    let res = match LikedText::belonging_to(user)
        .inner_join(texts_table)
        .select(Text::as_select())
        .load(&mut conn)
        .await
    {
        Err(_) => return Err(Error::DontHaveLikedTexts.into_response()),
        Ok(texts) => texts,
    };

    Ok(res)
}

pub async fn update(text: &Text, dto: UpdateTextDto) -> Result<Text> {
    let mut conn = DB().await.get_conn().await?;

    let res = diesel::update(text)
        .set((title_column.eq(dto.title), content_column.eq(dto.content)))
        .returning(Text::as_returning())
        .get_result(&mut conn)
        .await;

    match res {
        Ok(text) => Ok(text),
        //TODO
        Err(_) => Err(Error::TextByIdNotFound(text.id).into_response()),
    }
}

pub async fn like(id: i64, user_id: i64) -> Result<Text> {
    let mut conn = DB().await.get_conn().await?;
    let text = get_by_id(id).await?;

    match diesel::insert_into(liked_texts_table)
        .values((
            user_id_liked_column.eq(user_id),
            text_id_liked_column.eq(id),
        ))
        .execute(&mut conn)
        .await
    {
        Ok(_) => (),
        //TODO
        Err(_) => return Err(Error::TextByIdNotFound(id).into_response()),
    };

    let res = diesel::update(&text)
        .set(likes_column.eq(text.likes + 1))
        .returning(Text::as_returning())
        .get_result(&mut conn)
        .await;

    match res {
        Ok(text) => Ok(text),
        //TODO
        Err(_) => Err(Error::TextByIdNotFound(id).into_response()),
    }
}

pub async fn dislike(text: Text) -> Result<Text> {
    let mut conn = DB().await.get_conn().await?;

    //TODO
    match diesel::delete(liked_texts_table.filter(text_id_liked_column.eq(text.id)))
        .execute(&mut conn)
        .await
    {
        Ok(_) => (),
        //TODO
        Err(_) => return Err(Error::TextByIdNotFound(text.id).into_response()),
    };

    let res = diesel::update(texts_table.filter(text_id_column.eq(text.id)))
        .set(likes_column.eq(text.likes - 1))
        .returning(Text::as_returning())
        .get_result(&mut conn)
        .await;

    match res {
        Ok(text) => Ok(text),
        //TODO
        Err(_) => Err(Error::TextByIdNotFound(text.id).into_response()),
    }
}

pub async fn delete(id: i64) -> Result<Text> {
    let mut conn = DB().await.get_conn().await?;

    let res = diesel::delete(texts_table.filter(text_id_column.eq(id)))
        .returning(Text::as_returning())
        .get_result(&mut conn)
        .await;

    match res {
        Ok(text) => Ok(text),
        Err(_) => Err(Error::TextByIdNotFound(id).into_response()),
    }
}

use axum::response::IntoResponse;
use rand::Rng;

use crate::{
    common::{Error, Result},
    controllers::texts::dto::{CreateTextDto, UpdateTextDto},
    entities::{Text, User},
    repositories::texts::{
        create, delete, dislike, get_all, get_by_id, get_by_user, get_liked, like, update,
    },
};

use super::users::get_user_by_id;

pub async fn create_text(user_id: i64, dto: CreateTextDto) -> Result<Text> {
    let text = dto.into_text(user_id);

    let res = create(text).await?;
    Ok(res)
}

pub async fn get_all_texts(limit: i64) -> Result<Vec<Text>> {
    let res = get_all(limit).await?;
    Ok(res)
}

pub async fn get_text_by_id(id: i64) -> Result<Text> {
    let res = get_by_id(id).await?;
    Ok(res)
}

pub async fn get_texts_by_user_id(user_id: i64) -> Result<Vec<Text>> {
    let user = get_user_by_id(user_id).await?;

    let res = get_by_user(user).await?;
    Ok(res)
}

pub async fn get_liked_texts(user: User) -> Result<Vec<Text>> {
    let res = get_liked(&user).await?;
    Ok(res)
}

//TODO
pub async fn get_random_text() -> Result<Text> {
    let total_texts_count = std::fs::read_dir("static/random_texts").unwrap().count();
    let mut rng = rand::thread_rng();
    let random_id = rng.gen_range(1..=total_texts_count);
    let text_path = format!("static/random_texts/{}.txt", random_id);

    match std::fs::read_to_string(text_path) {
        //TODO: is it good idea to make empty text?
        Ok(text) => Ok(Text {
            content: text,
            id: 0,
            likes: 0,
            title: "".into(),
            user_id: 0,
        }),
        Err(err) => {
            tracing::error!("{:?}", err);
            Err(Error::InternalServerError.into_response())
        }
    }
}

pub async fn update_text(id: i64, user_id: i64, dto: UpdateTextDto) -> Result<Text> {
    let text = get_text_by_id(id).await?;

    match text.user_id == user_id {
        true => {
            let res = update(&text, dto).await?;
            Ok(res)
        }
        false => Err(Error::NotAllowed.into_response()),
    }
}

pub async fn toggle_like(id: i64, user: User) -> Result<bool> {
    let liked_text = get_liked(&user)
        .await?
        .into_iter()
        .find(|text| text.id == id);

    if let Some(text) = liked_text {
        dislike(text).await?;
        Ok(false)
    } else {
        like(id, user.id).await?;
        Ok(true)
    }
}

pub async fn delete_text(id: i64, user_id: i64) -> Result<Text> {
    let text = get_text_by_id(id).await?;

    match text.user_id == user_id {
        true => {
            let res = delete(id).await?;
            Ok(res)
        }
        false => Err(Error::NotAllowed.into_response()),
    }
}

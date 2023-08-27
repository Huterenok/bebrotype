use axum::response::IntoResponse;

use crate::{
	entities::{Text, User},

	controllers::texts::dto::{CreateTextDto, UpdateTextDto},
	repositories::texts::{create, delete, get_by_id, get_by_user, get_liked, update, like, dislike},

	common::{Error, Result}
};

use super::users::get_user_by_id;

pub async fn create_text(user_id: i64, dto: CreateTextDto) -> Result<Text> {
    let text = dto.into_text(user_id);

    let res = create(text).await?;
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

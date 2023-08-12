use axum::response::IntoResponse;

use crate::entities::Text;

use crate::config::{Result, Error};

use crate::controllers::texts::dto::{CreateTextDto, UpdateTextDto};

use super::users::{get_user_by_id, update_favourites};

use crate::repositories::texts::{create, get_by_id, get_by_user, update, update_likes};

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

//TODO: id shouldn't be Option<i32>
pub async fn get_favourite_texts(text_ids: Vec<Option<i64>>) -> Result<Vec<Text>> {
    let mut res: Vec<Text> = Vec::with_capacity(text_ids.len());

    for id in text_ids {
        res.push(get_by_id(id.unwrap()).await?);
    }

    Ok(res)
}

pub async fn update_text(id: i64, user_id: i64, dto: UpdateTextDto) -> Result<Text> {
		let text = get_text_by_id(id).await?;
		match text.user_id == user_id {
			true => (),
			false => return Err(Error::NotAllowed.into_response())
		};

    let res = update(id, dto).await?;
    Ok(res)
}

pub async fn toggle_favourites(id: i64, user_id: i64) -> Result<bool> {
    let likes = get_by_id(id).await?.likes;
    let mut favourite_texts = get_user_by_id(user_id).await?.favourite_texts;

    //TODO
    match favourite_texts.contains(&Some(id)) {
        true => {
            update_likes(id, likes - 1).await?;
            favourite_texts = favourite_texts
                .into_iter()
                .filter(|text_id| text_id.unwrap() != id)
                .collect::<Vec<Option<i64>>>();
            update_favourites(user_id, favourite_texts).await?;

            return Ok(false);
        }
        false => {
            update_likes(id, likes + 1).await?;
            favourite_texts.push(Some(id));
            update_favourites(user_id, favourite_texts).await?;

            return Ok(true);
        }
    };
}
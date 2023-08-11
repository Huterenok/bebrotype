use crate::entities::schema::texts;
use diesel::prelude::*;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateTextDto {
    pub title: String,
    pub content: String,
}

impl CreateTextDto {
    pub fn into_text(self, user_id: i32) -> TextDto {
        TextDto {
            user_id,
            content: self.content,
            title: self.title,
        }
    }
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = texts)]
pub struct TextDto {
    pub title: String,
    pub content: String,
    pub user_id: i32,
}


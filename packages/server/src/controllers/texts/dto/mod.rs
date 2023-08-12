use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct CreateTextDto {
    #[validate(length(
        min = 6,
        max = 24,
        message = "Title must consist of 6 to 24 characters"
    ))]
    pub title: String,
    pub content: String,
}

impl CreateTextDto {
    pub fn into_text(self, user_id: i64) -> TextDto {
        TextDto {
            user_id,
            content: self.content,
            title: self.title,
        }
    }
}

#[derive(Deserialize, Validate)]
pub struct UpdateTextDto {
    #[validate(length(
        min = 6,
        max = 24,
        message = "Title must consist of 6 to 24 characters"
    ))]
    pub title: String,
    pub content: String,
}

#[derive(Deserialize, Validate)]
pub struct TextDto {
    #[validate(length(
        min = 6,
        max = 24,
        message = "Title must consist of 6 to 24 characters"
    ))]
    pub title: String,
    pub content: String,
    pub user_id: i64,
}

use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateTextDto {
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

#[derive(Deserialize)]
pub struct UpdateTextDto {
    pub title: String,
    pub content: String
}

#[derive(Deserialize)]
pub struct TextDto {
    pub title: String,
    pub content: String,
    pub user_id: i64,
}

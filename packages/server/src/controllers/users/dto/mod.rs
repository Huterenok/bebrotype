use serde::{Deserialize, Serialize};

use crate::entities::User;

#[derive(Serialize, Deserialize)]
pub struct UserResponseDto {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub avatar: Option<String>,
    pub near_address: Option<String>,
    pub favourite_texts: Vec<Option<i64>>,
}

impl Into<UserResponseDto> for User {
    fn into(self) -> UserResponseDto {
        UserResponseDto {
            id: self.id,
            username: self.username,
            email: self.email,
            avatar: self.avatar,
            near_address: self.near_address,
            favourite_texts: self.favourite_texts,
        }
    }
}

#[derive(Deserialize)]
pub struct CreateUserDto {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct UpdateUserDto {
    pub near_address: Option<String>,
    pub avatar: Option<String>,
}

#[derive(Deserialize)]
pub struct AllUsersQuery {
    pub limit: Option<i64>,
}

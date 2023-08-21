use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::entities::User;

#[derive(Serialize, Deserialize)]
pub struct UserResponseDto {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub avatar: Option<String>,
    pub near_address: Option<String>
}

impl Into<UserResponseDto> for User {
    fn into(self) -> UserResponseDto {
        UserResponseDto {
            id: self.id,
            username: self.username,
            email: self.email,
            avatar: self.avatar,
            near_address: self.near_address
        }
    }
}

#[derive(Deserialize, Validate)]
pub struct CreateUserDto {
    #[validate(length(
        min = 6,
        max = 24,
        message = "Username must consist of 6 to 24 characters"
    ))]
    pub username: String,
    #[validate(email(message = "Email must be valid"))]
    pub email: String,
    #[validate(length(
        min = 6,
        max = 24,
        message = "Password must consist of 6 to 24 characters"
    ))]
    pub password: String,
}

#[derive(Deserialize)]
pub struct UpdateUserDto {
    pub near_address: Option<String>,
    pub avatar: Option<String>,
}

#[derive(Deserialize, Validate)]
pub struct AllUsersQuery {
    #[validate(range(min = 10, max = 50, message = "Limit must be in range from 10 to 50"))]
    pub limit: Option<i64>,
}

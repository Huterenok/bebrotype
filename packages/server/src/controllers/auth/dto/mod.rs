use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::controllers::users::dto::UserResponseDto;

#[derive(Deserialize, Validate)]
pub struct LoginDto {
    #[validate(email(message = "Email must be valid"))]
    pub email: String,
    #[validate(length(
        min = 6,
        max = 24,
        message = "Password must consist of 6 to 24 characters"
    ))]
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthPayload {
    pub token: String,
    pub user: UserResponseDto,
}

impl AuthPayload {
    pub fn new(token: String, user: UserResponseDto) -> Self {
        AuthPayload { token, user }
    }
}

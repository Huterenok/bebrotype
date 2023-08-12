use serde::{Serialize, Deserialize};

use crate::controllers::users::dto::UserResponseDto;

#[derive(Deserialize)]
pub struct LoginDto {
    pub email: String,
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

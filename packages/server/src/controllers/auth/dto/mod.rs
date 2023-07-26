use serde::{Serialize, Deserialize};

use crate::entities::user::FilteredUser;

#[derive(Deserialize)]
pub struct LoginDto {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthPayload {
    pub token: String,
    pub user: FilteredUser,
}

impl AuthPayload {
    pub fn new(token: String, user: FilteredUser) -> Self {
        AuthPayload { token, user }
    }
}

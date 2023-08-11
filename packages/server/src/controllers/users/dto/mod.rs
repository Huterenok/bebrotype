use crate::entities::schema::users;
use diesel::prelude::*;
use serde::Deserialize;

#[derive(Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct CreateUserDto {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Insertable, AsChangeset)]
#[diesel(table_name = users)]
pub struct UpdateUserDto {
    pub near_address: Option<String>,
    pub avatar: Option<String>,
}

#[derive(Deserialize)]
pub struct AllUsersQuery {
    pub limit: Option<usize>,
}

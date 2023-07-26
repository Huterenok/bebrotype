use crate::entities::schema::users;
use diesel::prelude::*;
use serde::Deserialize;

#[derive(Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct CreateUserDto {
    pub username: String,
    pub email: String,
    pub password: String,
    pub near_address: Option<String>,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct AddNearAddressDto {
    near_address: String,
}

#[derive(Deserialize)]
pub struct AllUsersQuery {
		pub limit: Option<usize>
}
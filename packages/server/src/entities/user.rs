use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Identifiable, Clone, Debug)]
#[diesel(table_name = super::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub avatar: Option<String>,
    pub near_address: Option<String>,
    pub favourite_texts: Vec<Option<i32>>,
}

#[derive(Serialize, Deserialize)]
pub struct FilteredUser {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub avatar: Option<String>,
    pub near_address: Option<String>,
    pub favourite_texts: Vec<Option<i32>>,
}

impl Into<FilteredUser> for User {
    fn into(self) -> FilteredUser {
        FilteredUser {
            id: self.id,
            username: self.username,
            email: self.email,
            avatar: self.avatar,
            near_address: self.near_address,
            favourite_texts: self.favourite_texts,
        }
    }
}

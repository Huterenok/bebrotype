use diesel::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Selectable, Clone, Debug)]
#[diesel(table_name = super::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub near_address: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct FilteredUser {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub near_address: Option<String>,
}

impl User {
    pub fn new(
        id: i32,
        username: String,
        email: String,
        pwd: String,
        near_address: Option<String>,
    ) -> Self {
        User {
            id,
            username,
            email,
            password: pwd,
            near_address,
        }
    }
}

impl Into<FilteredUser> for User {
    fn into(self) -> FilteredUser {
        FilteredUser {
            id: self.id,
            username: self.username,
            email: self.email,
            near_address: self.near_address,
        }
    }
}

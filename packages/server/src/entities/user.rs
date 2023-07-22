use serde::{Deserialize, Serialize};
use diesel::prelude::*;


#[derive(Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = super::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
		pub id: i32, 
    pub username: String,
    pub email: String,
}

impl User {
    pub fn new(id: i32, username: String, email: String) -> Self {
        User { id, username, email }
    }
}

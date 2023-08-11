use diesel::{Associations, Identifiable, Queryable, Selectable};
use serde::Serialize;

use crate::entities::User;

#[derive(Queryable, Selectable, Identifiable, Associations, Serialize, Clone)]
#[diesel(belongs_to(User))]
#[diesel(table_name = super::schema::texts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Text {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub user_id: i32,
		pub likes: i32,
}

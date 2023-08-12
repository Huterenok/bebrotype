use diesel::{Associations, Identifiable, Queryable, Selectable};
use serde::Serialize;

use crate::entities::User;

#[derive(Queryable, Selectable, Identifiable, Associations, Serialize, Clone)]
#[diesel(belongs_to(User))]
#[diesel(table_name = super::schema::texts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Text {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub user_id: i64,
    pub likes: i32,
}

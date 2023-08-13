use diesel::{Associations, Queryable, Selectable, Identifiable};
use serde::Serialize;

use super::{Text, User};

#[derive(Queryable, Selectable, Identifiable,Associations, Serialize, Clone)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Text))]
#[diesel(primary_key(user_id, text_id))]
#[diesel(table_name = super::schema::liked_texts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct LikedText {
    pub user_id: i64,
    pub text_id: i64,
}

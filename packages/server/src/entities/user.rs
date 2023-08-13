use diesel::prelude::*;

#[derive(Queryable, Selectable, Identifiable, Clone, Debug)]
#[diesel(table_name = super::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub password: String,
    pub avatar: Option<String>,
    pub near_address: Option<String>
}

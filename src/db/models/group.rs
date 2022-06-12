use crate::db::{models::user::User, schema::groups};

#[derive(Queryable, Identifiable, Associations)]
#[belongs_to(User, foreign_key="owner_id")]
pub struct Group {
    pub id: i32,

    pub name: String,
    pub description: String,

    pub owner_id: i32,
}

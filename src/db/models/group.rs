use crate::db::schema::groups;

#[derive(Queryable, Identifiable)]
pub struct Group {
    pub id: i32,

    pub name: String,
    pub description: String,
}

use diesel::Queryable;
use diesel_geometry::data_types::PgPoint;
use serde::Serialize;
use super::schema::spots;

#[derive(Queryable, Serialize)]
pub struct Spot {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub location: PgPoint,
}

#[derive(Insertable)]
#[table_name="spots"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub description: &'a str,
    pub location: &'a PgPoint,
}

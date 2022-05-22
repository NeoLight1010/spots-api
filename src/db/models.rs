use diesel::Queryable;
use diesel_geometry::data_types::PgPoint;
use super::schema::spots;

#[derive(Queryable)]
pub struct Spot {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub location: (f64, f64),
}

#[derive(Insertable)]
#[table_name="spots"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub description: &'a str,
    pub location: &'a PgPoint,
}

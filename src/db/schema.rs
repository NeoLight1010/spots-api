table! {
    use diesel::sql_types::*;
    use diesel_geometry::sql_types::*;

    spots (id) {
        id -> Int4,
        title -> Varchar,
        description -> Varchar,
        location -> Point,
    }
}

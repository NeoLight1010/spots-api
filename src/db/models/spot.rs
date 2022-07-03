use crate::db::{schema::spots, models::group::Group};
use diesel::{Queryable, Insertable, Identifiable, Associations};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Identifiable, Associations)]
#[belongs_to(Group)]
pub struct Spot {
    pub id: i32,
    pub title: String,
    pub description: String,

    pub latitude: f64,
    pub longitude: f64,

    pub group_id: i32,
}

#[derive(Insertable, Deserialize)]
#[table_name = "spots"]
pub struct NewSpot {
    pub title: String,
    pub description: String,

    pub latitude: f64,
    pub longitude: f64,
}

#[cfg(test)]
mod test {
    use diesel::{pg::PgConnection, Connection, prelude::*};
    use crate::db::{setup::get_database_url, schema::spots, connection::run_test_transaction};

    use super::{NewSpot, Spot};

    #[test]
    fn test_insert_new_spot() -> Result<(), diesel::result::Error> {
        run_test_transaction(|conn| {
            let new_spot = NewSpot {
                title: "My spot title".to_owned(),
                description: "My spot description".to_owned(),
                latitude: 0.0,
                longitude: 0.0,
            };

            let _: Spot = diesel::insert_into(spots::table)
                .values(&new_spot)
                .get_result(&conn)?;

            Ok(())
        })
    }
}

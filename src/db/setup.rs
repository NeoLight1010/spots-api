use std::env;

use diesel::{
    r2d2::{self, ConnectionManager},
    PgConnection,
};

pub type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn init_db_pool() -> DBPool {
    let manager = ConnectionManager::<PgConnection>::new(get_database_url());

    DBPool::new(manager).expect("Error creating DB pool.")
}

pub fn get_database_url() -> String {
    env::var("DATABASE_URL").expect("DATABASE_URL must be set.")
}

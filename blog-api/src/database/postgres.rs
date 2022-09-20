use diesel::{prelude::*, r2d2::ConnectionManager};
use dotenv::dotenv;
use std::env;

pub type Pool = diesel::r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> Pool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    diesel::r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}
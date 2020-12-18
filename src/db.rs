use diesel::{PgConnection, Connection};
use std::env;
use dotenv::dotenv;

pub fn establish_conn() -> PgConnection {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&db_url)
        .expect(&format!("Error connecting to {}", db_url))
}
#[macro_use]
extern crate diesel;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub mod models;
pub mod schema;
pub mod routing;
pub mod handlers;

#[cfg(not(test))]
pub fn establish_db_connection() -> PgConnection {
    println!("dev connection");
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[cfg(test)]
pub fn establish_db_connection() -> PgConnection {
    println!("test connection");
    dotenv().ok();

    let database_url = env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub mod schema;
pub mod models;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}


use self::models::{Link, NewLink};

pub fn create_link<'a>(conn: &PgConnection, src: &'a str, dst: &'a str) -> Link {
    use schema::links;

    let new_link = NewLink {
        src: src,
        dst: dst,
    };

    diesel::insert_into(links::table)
        .values(&new_link)
        .get_result(conn)
        .expect("Error saving new post")
}

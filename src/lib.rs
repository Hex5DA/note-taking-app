pub mod models;
pub mod schema;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub use schema::note;
// use models::Note;

pub fn get_db_connection() -> SqliteConnection {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("You must set a database url, see README.md for examples of .env files.");
    SqliteConnection::establish(&db_url).expect(&format!("There was an error while trying to connect to {}", db_url))
}

// pub fn create_note(note: Note) {
//     use schema::note;
//     diesel::insert_into()
// }
// use actix_web::{get, post, HttpResponse, Responder, web, Result};
use actix_web::{post, web, Result};

use notes::get_db_connection;
use notes::models::*;

use notes::models::NewNote;

use diesel::{self, prelude::*};

#[post("/notes/create")]
async fn create(data: web::Json<Note>) -> Result<String> {
    println!("Creating note {} (with id {})!", data.title, data.id);
    let conn = get_db_connection();

    let new = NewNote {
        title: &data.title,
        content: &data.content
    };

    diesel::insert_into(notes::note::table)
        .values(&new)
        .execute(&conn)
        .expect("Error saving new post");

    Ok("Post created succesfully!".to_string())
}
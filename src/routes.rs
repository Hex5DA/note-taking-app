use actix_web::{get, post, Responder, web, Result};

use notes::get_db_connection;
use notes::models::*;

use diesel::{self, prelude::*};

#[post("/notes/create")]
async fn create_note(data: web::Json<JsonNote>) -> Result<String> {
    println!("Creating note \"{}\"!", data.title);
    let conn = get_db_connection();

    let new = NewNote {
        title: &data.title,
        content: &data.content,
        starred: &data.starred
    };

    diesel::insert_into(notes::note::table)
        .values(&new)
        .execute(&conn)
        .expect("Error saving new post");

    Ok("Post created succesfully!".to_string())
}

#[get("/notes")]
async fn get_notes() -> Result<impl Responder> {
    println!("Retreiving notes!");

    let conn = get_db_connection();
    let data = notes::note::table
        .load::<Note>(&conn).expect("Error while getting data.");

    Ok(web::Json(data))
}
use actix_web::{get, patch, post, delete, web, HttpResponse, Responder, Result};

use notes::get_db_connection;
use notes::models::*;

use diesel::{self, prelude::*};

#[post("/notes/create")]
async fn create_note(data: web::Json<JsonNote>) -> Result<String> {
    let conn = get_db_connection();

    // Having JsonNote and NewNote is kinda messy but idk how to merge them
    let new = NewNote {
        title: &data.title,
        content: &data.content,
        starred: &data.starred,
    };

    diesel::insert_into(notes::note::table)
        .values(&new)
        .execute(&conn)
        .expect("Error saving new post");

    Ok("Post created succesfully!".to_string())
}

#[get("/notes")]
async fn get_notes() -> impl Responder {
    let conn = get_db_connection();
    let data = notes::note::table
        .load::<Note>(&conn)
        .expect("Error while getting data.");

    web::Json(data)
}

#[get("/notes/{id}")]
async fn get_note(path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();
    let conn = get_db_connection();

    match notes::note::table
        .find(id)
        .load::<Note>(&conn)
        .expect("Error retrieving from the database.")
        .get(0)
    {
        None => HttpResponse::NotFound().finish(),
        Some(data) => HttpResponse::Ok().json(data),
    }
}

#[patch("/notes/{id}")]
async fn update_note(path: web::Path<i32>, data: web::Json<OptionalNote>) -> impl Responder {
    let id = path.into_inner();
    let conn = get_db_connection();
    let old = match notes::note::table
        .find(id)
        .load::<Note>(&conn)
        .expect("Error retrieving from the database.")
        .get(0)
    {
        None => return HttpResponse::NotFound().finish(),
        Some(data) => data.clone(),
    };

    // Not the worlds prettiest code, but I dont know how to make this any cleaner without an `into_array()`
    // method which honestly feels worse so its here to stay I guess :)
    let note = Note {
        id,
        title: if let Some(title) = data.title.clone() { title.to_string() } else { old.title },
        content: if let Some(content) = data.content.clone() {content} else { old.content },
        starred: if let Some(starred) = data.starred {starred} else { old.starred },
    };

    diesel::update(notes::note::table.find(id))
        .set(&note)
        .execute(&conn)
        .expect("Error while updating the database.");

    HttpResponse::Ok().json(note)
}

#[delete("/notes/{id}")]
async fn delete_note(path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();
    let conn = get_db_connection();

    match diesel::delete(notes::note::table.find(id)).execute(&conn) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(msg) => {
            println!("Error: {}", msg);
            HttpResponse::NotFound().finish()
        }
    }
}

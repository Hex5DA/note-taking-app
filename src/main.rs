pub mod routes;

use actix_web::{web, App, HttpServer};
use actix_cors::Cors;
use routes::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::permissive()
            )
            .service(
                web::scope("/api")
                    .service(create_note)
                    .service(get_notes)
                    .service(get_note)
                    .service(update_note)
                    .service(delete_note)
            )
            .service(
                actix_files::Files::new("/", "./frontend/dist")
                    .index_file("index.html")
                    .default_handler(
                        actix_files::NamedFile::open("./frontend/dist/index.html")
                            .expect("Index file does not exist.")
                    )
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
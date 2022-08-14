pub mod routes;

use actix_web::{web, App, HttpServer};
use actix_cors::Cors;
use routes::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::permissive() // <- Dont use this in production! (apparently)
            )
            .service(
                web::scope("/api")
                    .service(create_note)
                    .service(get_notes)
                    .service(get_note)
                    .service(update_note)
                    .service(delete_note)
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
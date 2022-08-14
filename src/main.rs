pub mod routes;

use actix_web::{web, App, HttpServer};
use routes::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/api")
                    .service(create_note)
                    .service(get_notes)
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
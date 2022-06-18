mod api;
mod common;
mod domain;
mod service;

use actix_web::{web, App, HttpServer};
use api::nucleic_api;

#[actix_web::main] 
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/test", web::get().to(|| async { "server start success!" }))
            .service(nucleic_api::save)
            .service(nucleic_api::query)
    })
    .bind(("127.0.0.1", 8085))?
    .run()
    .await
}
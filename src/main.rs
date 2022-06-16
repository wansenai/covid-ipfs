mod api;
mod common;
mod domain;
mod service;

use actix_web::{get, web, App, HttpServer, Responder};

#[tokio::main] 
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/test", web::get().to(|| async { "server start success!" }))
    })
    .bind(("127.0.0.1", 8085))?
    .run()
    .await
}
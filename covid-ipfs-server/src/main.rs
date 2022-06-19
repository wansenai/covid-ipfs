mod api;
mod common;
mod domain;
mod service;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer, http::header};
use api::nucleic_api;

#[actix_web::main] 
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Cors::default()
                .allowed_origin("http://localhost:3000")
                .allowed_origin("http://101.43.52.162:3000/")
                .allowed_origin("http://47.243.23.220:3000/")
                .allowed_methods(vec!["GET", "POST"])
                .allowed_header(header::CONTENT_TYPE)
                .max_age(3600)
            )
            .route("/test", web::get().to(|| async { "server start success!" }))
            .service(nucleic_api::save)
            .service(nucleic_api::query)
    })
    .bind(("127.0.0.1", 8085))?
    .run()
    .await
}
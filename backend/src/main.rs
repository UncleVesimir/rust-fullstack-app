use std::io::{Result};
use actix_web::{get, web::scope, web, App, HttpServer, Responder, middleware::Logger};
use actix_web_lab::web::spa;
mod api;
mod repository;

use api::log::{create_request};


#[actix_web::main]
async fn main() -> Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(|| {

        let logger = Logger::default();

        App::new()
        .wrap(logger)
        .service(
            scope("/api")
                .service(create_request)
        )
        .service(
            spa()
            .index_file("./dist/index.html")
            .static_resources_mount("/")
            .static_resources_location("./dist")
            .finish()
        )

    })
    .bind(("0.0.0.0", 8079))?
    .run()
    .await
}

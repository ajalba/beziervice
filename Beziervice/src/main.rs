#![allow(unused_imports)]
mod bezier_curves;
use actix_web::web::Json;
use actix_web::{middleware::Logger, web, App, HttpServer};
use std;

async fn status() -> Json<String> {
    return Json("hello_world".to_string())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new().wrap(logger).route("/", web::get().to(status))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}

#![allow(unused_imports)]
mod bezier_curves;
use actix_web::web::Json;
use actix_web::{middleware::Logger, web, App, HttpServer};
use api::handlers::*;
use bezier_curves::curves::BezierCurve;
use std;
mod api;
use crate::api::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new().wrap(logger)
        .service(
        web::resource("/").route(web::get().to(get_simple_curve)),
        
        ).route("/evaluate_curve", web::post().to(evaluate_simple_curve))
        .route("/create_curve", web::post().to(create_simple_curve))
        .service(
            web::resource("/interpolate/{degree}").route(web::get().to(get_simple_curve)),
        )
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}

#![allow(unused_imports)]
mod bezier_curves;
use actix_web::web::Json;
use actix_web::{middleware::Logger, web, App, HttpServer};
use api::handlers::*;
use bezier_curves::curves::BezierCurve;
use std;
use simple_logger::SimpleLogger;
mod api;
use crate::api::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    SimpleLogger::new().init().unwrap();

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new().wrap(logger)
        .service(
        web::resource("/").route(web::get().to(get_simple_curve)),
        
        ).route("/evaluate_curve", web::post().to(evaluate_simple_curve))
        .route("/create_curve", web::post().to(create_simple_curve))
        .service(
            interpolate_function,
        )
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}

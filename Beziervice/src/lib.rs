pub mod models;
pub mod schema;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use models::{NewCurve, Curve};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_curve(
        conn: &mut PgConnection,
        curve_name: &str,
        expression_x: &str,
        expression_y: &str,
        expression_z: &str,
        points_x: Vec<Option<f64>>,
        points_y: Vec<Option<f64>>,
        points_z: Vec<Option<f64>>
) -> Curve {
    use crate::schema::curves;
    let new_curve = NewCurve {
        curve_name,
        expression_x,
        expression_y,
        expression_z,
        points_x,
        points_y,
        points_z,
    };

    diesel::insert_into(curves::table)
        .values(&new_curve)
        .get_result(conn)
        .expect("Error saving new curve")
}
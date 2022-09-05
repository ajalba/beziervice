
use crate::bezier_curves::curves::BezierCurve;
use actix_web::{web, HttpResponse, get, Responder};
use serde::{ Deserialize, Serialize};
use serde_json::json;

#[derive(Clone)]
#[derive(Deserialize, Serialize)]
pub struct CurveBase {
    pub points_x: Vec<f64>,
    pub points_y: Vec<f64>,
    pub points_z: Vec<f64>
}
#[derive(Serialize)]
pub struct Vector {
    pub points: Vec<f64>
}

pub async fn get_simple_curve() -> HttpResponse {
    return HttpResponse::Ok().json(
        BezierCurve::new(
            vec![0.1, 1.0, 2.0],
            vec![0.2, 2.0, 2.0],
            vec![0.3, 3.0, 2.0]
        )
    )
}

pub async fn create_simple_curve(request: web::Json<CurveBase>) -> HttpResponse {
    let curve_base =  request.to_owned();
    return HttpResponse::Ok().json(
        BezierCurve::new(
            curve_base.points_x,
            curve_base.points_y,
            curve_base.points_z,
        )
    )
}
pub async fn evaluate_simple_curve(request: web::Json<CurveBase>) -> HttpResponse {
    let curve_base =  request.to_owned();
    let curve = BezierCurve::new(
        curve_base.points_x,
        curve_base.points_y,
        curve_base.points_z,
    );
    let all_points = BezierCurve::evaluate_curve_all_axis(curve);
    return HttpResponse::Ok().json(
        CurveBase {
            points_x: all_points.0,
            points_y: all_points.1,
            points_z: all_points.2
        }
    )
}
#[get("/interpolate_function/{degree}")]
pub async fn interpolate_function(_degree: web::Path<(i32,)>) -> impl Responder {
    let degree = _degree.into_inner().0;
    let interpolated_points = match degree {
        2 => BezierCurve::cuadratic_c1_interpolant(0.0, 1.0, 0.05, BezierCurve::fun_sin),
        3 => BezierCurve::cubic_c2_interpolant(0.0, 1.0, 0.05, BezierCurve::fun_sin),
        _ => return HttpResponse::BadRequest().json(json!({ "error": "degree must be 2 or 3" }))
    };
    return HttpResponse::Ok().json(Vector {points: interpolated_points});
}
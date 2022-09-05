
use crate::bezier_curves::{curves::BezierCurve, point};
use actix_web::{web, HttpResponse, get, Responder};
use serde::{ Deserialize, Serialize};
use serde_json::json;
use beziervice::models::Curve;
use diesel::prelude::*;
use beziervice::*;



#[derive(Clone)]
#[derive(Deserialize, Serialize)]
pub struct CurveBase {
    pub name: String,
    pub points_x: Vec<f64>,
    pub points_y: Vec<f64>,
    pub points_z: Vec<f64>
}
#[derive(Clone)]
#[derive(Deserialize, Serialize)]
pub struct CurveBaseId {
    id: i32,
    pub name: String,
    pub points_x: Vec<f64>,
    pub points_y: Vec<f64>,
    pub points_z: Vec<f64>
}

#[derive(Clone)]
#[derive(Deserialize, Serialize)]
pub struct CurveId {
    id: i32,
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
    let _connection = &mut establish_connection();
    let curve_bezier = BezierCurve::new(
        curve_base.points_x.clone(),
        curve_base.points_y.clone(),
        curve_base.points_z.clone(),
    );
    let x_points = curve_base.points_x.clone().iter().map(|&x| Some(x)).collect::<Vec::<Option<f64>>>();
    let y_points = curve_base.points_y.iter().map(|&x| Some(x)).collect::<Vec::<Option<f64>>>();
    let z_points = curve_base.points_z.iter().map(|&x| Some(x)).collect::<Vec::<Option<f64>>>();
    let curve = create_curve(_connection,
        &curve_base.name,
        &curve_bezier.expression_x,
        &curve_bezier.expression_y,
        &curve_bezier.expression_z,
        x_points,
        y_points,
        z_points,
    );
    return HttpResponse::Ok().json(
        curve
    )
}

pub async fn edit_curve(request: web::Json<CurveBaseId>) -> HttpResponse {
    
    use beziervice::schema::curves::dsl::*;
    let _connection = &mut establish_connection();
    let curve_base =  request.to_owned();
    let curve_bezier = BezierCurve::new(
        curve_base.points_x.clone(),
        curve_base.points_y.clone(),
        curve_base.points_z.clone(),
    );
    let x_points = curve_base.points_x.clone().iter().map(|&x| Some(x)).collect::<Vec::<Option<f64>>>();
    let y_points = curve_base.points_y.iter().map(|&x| Some(x)).collect::<Vec::<Option<f64>>>();
    let z_points = curve_base.points_z.iter().map(|&x| Some(x)).collect::<Vec::<Option<f64>>>();
    let result: Curve = diesel::update(curves.find(curve_base.id))
        .set(
            (
                curve_name.eq(curve_base.name),
                expression_x.eq(curve_bezier.expression_x),
                expression_y.eq(curve_bezier.expression_y),
                expression_z.eq(curve_bezier.expression_z),
                points_x.eq(x_points),
                points_y.eq(y_points),
                points_z.eq(z_points)
            )
        )
        .get_result(_connection)
        .unwrap();
    return HttpResponse::Ok().json(result)
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
            name: "generic".to_string(),
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

#[get("/get_function/{id}")]
pub async fn get_function(_id: web::Path<(i32,)>) -> impl Responder {
    let _connection = &mut establish_connection();
    
    use beziervice::schema::curves::dsl::*;
    
    let result = curves
    .filter(id.eq(_id.into_inner().0))
    .load::<Curve>(_connection)
    .expect("Error loading curves");

    return HttpResponse::Ok().json(result);
}

pub async fn delete_curve(request: web::Json<CurveId>) -> impl Responder {
    let _connection = &mut establish_connection();
    use beziervice::schema::curves::dsl::*;
    let curve_id =  request.to_owned();
    
    let result: Curve = diesel::delete(curves.filter(id.eq(curve_id.id)))
    .get_result(_connection)
    .unwrap();

    return HttpResponse::Ok().json(result);
}

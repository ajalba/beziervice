use diesel::prelude::*;
use serde::{Serialize};

#[derive(Debug, Queryable, Serialize)]
pub struct Curve {
    pub id: i32,
    pub curve_name: String,
    pub expression_x: String,
    pub expression_y: String,
    pub expression_z: String,
    pub points_x: Vec<Option<f64>>,
    pub points_y: Vec<Option<f64>>,
    pub points_z: Vec<Option<f64>>,
}

use crate::schema::curves;

#[derive(Insertable)]
#[diesel(table_name = curves)]
pub struct NewCurve<'a> {
    pub curve_name: &'a str,
    pub expression_x: &'a str,
    pub expression_y: &'a str,
    pub expression_z: &'a str,
    pub points_x: Vec<Option<f64>>,
    pub points_y: Vec<Option<f64>>,
    pub points_z: Vec<Option<f64>>
}
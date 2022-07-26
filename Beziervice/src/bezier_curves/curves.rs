use mathru::algebra::abstr::Polynomial;

use crate::bezier_curves::point::Point;
#[derive(Clone)]
pub struct BezierCurve {
    points: Vec<Point>,
    grade: i8
}

impl BezierCurve {
    pub fn base_n_grade(grade: i32) -> String {
        todo!()
    }
    pub fn generate_curve(points: Vec<Polynomial<f64>>) -> Polynomial<f64> {
        let mut aux_points = points.clone();
        
        let n_puntos = points.len();
        for j in 1..n_puntos {
            let mut next_iter = vec![];
            for i in 1..n_puntos-j+1 {
                let x: Polynomial<f64> = Polynomial::from_coef(vec![1.0, 0.0]);
                let one_minus_x: Polynomial<f64> = Polynomial::from_coef(vec![1.0, -1.0]);
                let pol_one_minus_x = aux_points[i-1].clone();
                let pol_x = aux_points[i].clone();
                next_iter.push(pol_one_minus_x * one_minus_x + pol_x * x)
            }
            aux_points = next_iter
        }
        return aux_points[0].clone();
    }
    fn evaluate(_curve: BezierCurve, _point: Point) -> Point {
        todo!()
    }
    fn generate_curves(_points: Vec<Point>) -> Vec<BezierCurve> {
        todo!()
    }
    fn aprox_function(_function: String, _grade: i32) -> Vec<Point> {
        todo!()
    }

}
use mathru::algebra::abstr::Polynomial;

use crate::bezier_curves::point::Point;
#[derive(Clone)]
pub struct BezierCurve {
    points: Vec<Point>,
    grade: i8,
    expression: Polynomial<f64>
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
                let one_minus_x: Polynomial<f64> = Polynomial::from_coef(vec![-1.0, 1.0]);
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
#[cfg(test)]
mod tests {
    use mathru::algebra::abstr::Polynomial;
    use crate::bezier_curves::curves::BezierCurve;
    use pretty_assertions::{assert_eq, assert_ne};
    #[test]
    fn add_polynomial() {
        let pol_one = Polynomial::from_coef(vec![1.0, 0.0]);
        let pol_two = Polynomial::from_coef(vec![0.0, 1.0]);
        assert_eq!(pol_one + pol_two, Polynomial::from_coef(vec![1.0, 1.0]));
    }
    #[test]
    fn simple_curve() {
        let pol_one = Polynomial::from_coef(vec![2.0]);
        let pol_two = Polynomial::from_coef(vec![1.0]);
        let _vector_pol = vec![pol_one, pol_two];
        let decas_pol = BezierCurve::generate_curve(_vector_pol);
        assert_eq!(1, decas_pol.degree());
        assert_eq!(decas_pol, Polynomial::from_coef(vec![-1.0, 2.0]));
    }
    #[test]
    fn second_degree_curve() {
        let pol_one = Polynomial::from_coef(vec![1.0]);
        let pol_two = Polynomial::from_coef(vec![2.0]);
        let pol_three = Polynomial::from_coef(vec![1.0]);
        let vector_pol = vec![pol_one, pol_two, pol_three];
        let decas_pol = BezierCurve::generate_curve(vector_pol);
        assert_eq!(decas_pol, Polynomial::from_coef(vec![-2.0, 2.0, 1.0]));
        assert_eq!(2, decas_pol.degree());
    }
}
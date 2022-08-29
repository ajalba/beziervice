use mathru::algebra::abstr::Polynomial;
use mathru::elementary::trigonometry::Trigonometry;

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
    pub fn generate_curve(_points: Vec<Polynomial<f64>>) -> Polynomial<f64> {
        let mut aux_points = _points.clone();
        
        let n_puntos = _points.len();
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
    fn evaluate_point(_curve: BezierCurve, _point: f64) -> f64 {
        _curve.expression.eval(_point)
    }
    fn evaluate_curve(_curve: BezierCurve, n_points: Option<i32>) -> Vec<f64> {
        let len_vector = n_points.unwrap_or(1000);
        let mut points = vec![];
        for i in 0..len_vector {
            points.push(_curve.expression.eval((i/1000).into()));
        }
        return points;
    }
    fn generate_curves(_points: Vec<Polynomial<f64>>, _grade: i32) -> Vec<Polynomial<f64>> {
        let mut curves = Vec::<Polynomial<f64>>::new();
        let n_curves = _points.len() / _grade as usize;
        for i in 0..n_curves {
            let subvector: Vec<Polynomial<f64>>  = _points[i*_grade as usize..1+(i+1)*_grade as usize].to_vec();
            curves.push(BezierCurve::generate_curve(subvector));
        }
        return curves;
    }
    fn aprox_function(_function: String, _grade: i32) -> Vec<Point> {
        todo!()
    }
    fn must_split_curve(_curve: BezierCurve) -> bool {
        _curve.points.len() > 10
    }

    pub fn fun_sin(_x: f64) -> f64 {
        return Trigonometry::sin(_x);
    }

    pub fn cuadratic_c1_interpolant(_a: f64, _b:f64, _step: f64, f: fn(f64) -> f64) -> Vec<f64>{
        let first_base_pol = Polynomial::from_coef(vec![1.0, -2.0, 1.0]);
        let second_base_pol = Polynomial::from_coef(vec![-2.0, 2.0, 0.0]);
        let third_base_pol = Polynomial::from_coef(vec![1.0, 0.0, 0.0]);

        let mut i: f64 = 0.0;
        let mut left_limit = _a.clone();
        let mut values = Vec::<f64>::new();
        let mut vi: f64 = 0.0;
        let mut wi: f64 = 0.0;
        let mut vI: f64 = 0.0;
        let mut cvi: f64 = 0.0;
        let mut cwi: f64 = 0.0;
        let mut cvI: f64 = 0.0;
        let mut pol = Polynomial::from_coef(vec![0.0]);
        let num_evals: i32 = 10;

        while left_limit < _b {
            if left_limit + _step < _b {
                vi = left_limit;
                wi = left_limit + _step*0.5;
                vI = left_limit + _step;
            } else {
                vi = left_limit;
                wi = _b;
                vI = left_limit + (_b + left_limit) * 0.5;
            }
            cvi = -0.25 * f((i-1.0)*_step) + f((i-0.5)*_step) - 0.5*f(i*_step) +
                f((i+0.5)*_step) -0.25 * f((i+1.0)*_step);
            cwi = -0.5 * f(i*_step) + 2.0*f((i+0.5)*_step) -0.5*f((i+1.0)*_step);
            cvI = -0.25 * f(i*_step) + f((i+0.5)*_step) - 0.5*f((i+1.0)*_step) +
                f((i+1.5)*_step) - 0.25*f((i+2.0)*_step);
            pol = Polynomial::from_coef(vec![cvi]) * first_base_pol.clone() +
                Polynomial::from_coef(vec![cwi]) * second_base_pol.clone() +
                Polynomial::from_coef(vec![cvI]) * third_base_pol.clone();

            for j in 0..num_evals.into() {
                values.push(pol.eval(
                    (
                        (((j as f64)/num_evals as f64)*(cvI-cvi) as f64)-(i*_step)
                    ) / _step
                ));
            }

            i = i + 1.0;
            left_limit = left_limit + _step

        }
        return values;
    }

}
#[cfg(test)]
mod tests {
    use std::vec;

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
    #[test]
    fn joins_two_curves() {
        let first_curve = BezierCurve::generate_curve(vec![
            Polynomial::from_coef(vec![1.0]),
            Polynomial::from_coef(vec![2.0]),
            Polynomial::from_coef(vec![1.0])]);
        let second_curve = BezierCurve::generate_curve(vec![
            Polynomial::from_coef(vec![1.0]),
            Polynomial::from_coef(vec![2.0]),
            Polynomial::from_coef(vec![1.0])]);
        let two_curves = vec![first_curve, second_curve];
        let vector_points = vec![
            Polynomial::from_coef(vec![1.0]),
            Polynomial::from_coef(vec![2.0]),
            Polynomial::from_coef(vec![1.0]),
            Polynomial::from_coef(vec![2.0]),
            Polynomial::from_coef(vec![1.0])];
        assert_eq!(BezierCurve::generate_curves(vector_points, 2), two_curves);
    }
    #[test]
    fn cuaratic_c1_interpolant_works() {
        let cal_values = vec![0.0, 0.0049986452042909075, 0.009996665929408238, 0.014994062175351985, 0.019990833942122158, 0.024986981229718753, 0.029982504038141768, 0.0349774023673912, 0.03997167621746707, 0.04496532558836934, -6.242188680533056e-5, 0.004935557484945376, 0.009931674325787232, 0.014925928635720238, 0.019918320414744402, 0.024908849662859712, 0.029897516380066172, 0.034884320566363786, 0.03986926222175255, 0.04485234134623246];
        let fun_values = BezierCurve::cuadratic_c1_interpolant(0.0, 0.1, 0.05, BezierCurve::fun_sin);
        assert_eq!(cal_values, fun_values);
    }
}
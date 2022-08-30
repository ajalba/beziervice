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
                vI = left_limit + _step;
            } else {
                vi = left_limit;
                vI = _b
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
                        (((j as f64)/num_evals as f64)*(vI-vi) as f64)-(i*_step)
                    ) / _step
                ));
            }

            i = i + 1.0;
            left_limit = left_limit + _step

        }
        return values;
    }
    pub fn cubic_c2_interpolant(_a: f64, _b:f64, _step: f64, f: fn(f64) -> f64) -> Vec<f64> {
        let first_base_pol = Polynomial::from_coef(vec![1.0, -3.0, 3.0, -1.0]);
        let second_base_pol = Polynomial::from_coef(vec![3.0, -6.0, 3.0, 0.0]);
        let third_base_pol = Polynomial::from_coef(vec![-3.0, 3.0, 0.0, 0.0]);
        let fourth_base_pol = Polynomial::from_coef(vec![1.0, 0.0, 0.0, 0.0]);
        let mut i: f64 = 0.0;
        let mut left_limit = _a.clone();
        let mut values = Vec::<f64>::new();
        let mut ui: f64 = 0.0;
        let mut vi: f64 = 0.0;
        let mut wi: f64 = 0.0;
        let mut vI: f64 = 0.0;
        let mut cvi: f64 = 0.0;
        let mut cwi: f64 = 0.0;
        let mut cvI: f64 = 0.0;
        let mut cui: f64 = 0.0;
        let mut pol = Polynomial::from_coef(vec![0.0]);
        let num_evals: i32 = 10;


        while left_limit < _b {
            if left_limit + _step < _b {
                vi = left_limit;
                vI = left_limit + _step;
            } else {
                vi = left_limit;
                vI = _b
            }
            cvi = -(1.0/36.0) * f((i-2.0)*_step) + (1.0/9.0) * f((i-1.0)*_step) + (6.0/5.0)*f(i*_step) +
                (1.0/9.0) * f((i+1.0)*_step) - (1.0/36.0) * f((i+2.0)*_step);

            cwi = -(1.0/9.0) * f((i-1.0)*_step) + (6.0/5.0)*f(i*_step) +
            (1.0/3.0) * f((i+1.0)*_step) - (1.0/18.0) * f((i+2.0)*_step);

            cui = -(1.0/18.0) * f((i-1.0)*_step) + (1.0/3.0) * f((i)*_step) + (6.0/5.0)*f((i+1.0)*_step)
            -(1.0/9.0) * f((i+2.0)*_step);
            cvI = -(1.0/36.0) * f((i-1.0)*_step) + (1.0/9.0) * f((i)*_step) + (6.0/5.0)*f((i+1.0)*_step) +
            (1.0/9.0) * f((i+2.0)*_step) - (1.0/36.0) * f((i+3.0)*_step);
            pol = Polynomial::from_coef(vec![cvi]) * first_base_pol.clone() +
                Polynomial::from_coef(vec![cwi]) * second_base_pol.clone() +
                Polynomial::from_coef(vec![cui]) * third_base_pol.clone() +
                Polynomial::from_coef(vec![cvI]) * fourth_base_pol.clone();

            for j in 0..num_evals.into() {
                values.push(pol.eval(
                    (
                        (((j as f64)/num_evals as f64)*(vI-vi) as f64)-(i*_step)
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
        let cal_values = vec![0.0, 0.005000728938839476, 0.0100008328777237, 0.015000311816652673, 0.0199991657556264, 0.024997394694644872, 0.029994998633708095, 0.03499197757281606, 0.0399883315119688, 0.04498406045116626, -6.242188680533056e-5, 0.0049501672105271635, 0.009960882870168296, 0.01496972509211806, 0.019976693876376485, 0.024981789222943546, 0.029985011131819245, 0.03498635960300359, 0.03998583463649659, 0.04498343623229822];
        let fun_values = BezierCurve::cuadratic_c1_interpolant(0.0, 0.1, 0.05, BezierCurve::fun_sin);
        assert_eq!(cal_values, fun_values);
    }
    #[test]
    fn cubic_c2_interpolant_works () {
        let cal_values = vec![0.0, 0.005513097600833195, 0.011905703606895853, 0.01895778469946251, 0.026449307559807726, 0.03416023886920601, 0.041870545308931924, 0.049360193560259995, 0.056409150304464806, 0.06279738222282083, -0.9831525990850775, -0.8426081639259776, -0.7145663420809947, -0.5984269586281894, -0.4935898386456215, -0.3994548072113524, -0.3154216894034423, -0.24089031029995167, -0.17526049497894103, -0.11793206851847124];
        let fun_values = BezierCurve::cubic_c2_interpolant(0.0, 0.1, 0.05, BezierCurve::fun_sin);
        assert_eq!(cal_values, fun_values);
    }
}
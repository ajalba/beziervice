use crate::bezier_curves::curves::BezierCurve;
use crate::bezier_curves::point::Point;
#[derive(Clone)]

pub struct Surface {
    first_curve: BezierCurve,
    second_curve: BezierCurve
}

impl Surface {
    fn generate_surface_expression(surface: Surface) -> Surface{
        todo!()
    }
    fn evaluate_surface(surface: Surface) -> Vec<Point> {
        todo!()
    }
}
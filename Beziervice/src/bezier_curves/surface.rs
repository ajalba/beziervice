use crate::bezier_curves::curves::BezierCurve;
use crate::bezier_curves::point::Point;
#[derive(Clone)]

pub struct Surface {
    first_curve: BezierCurve,
    second_curve: BezierCurve,
}

impl Surface {
    fn generate_surface_expression(_surface: Surface) -> Surface {
        todo!()
    }
    fn evaluate_surface(_surface: Surface) -> Vec<Point> {
        todo!()
    }
}

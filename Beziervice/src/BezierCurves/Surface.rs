mod BezierCurve;
pub struct Surface {
    first_curve: BezierCurve,
    second_curve: BezierCurve
}

impl Surface {
    fn generate_surface_expression(surface: Surface) -> Surface{
        !todo()
    }
    fn evaluate_surface(surface: Surface) -> Vec<Point> {
        !todo()
    }
}
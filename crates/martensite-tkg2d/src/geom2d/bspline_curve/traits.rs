//! The virtual members of Geom2dGeometry, Geom2dCurve and Geom2dBoundedCurve.
use super::Geom2dBSplineCurve;
use crate::geom2d::{
    Geom2dBoundedCurve, Geom2dCurve, Geom2dCurveD1, Geom2dCurveD2, Geom2dCurveD3, Geom2dGeometry,
    dump_bounded_curve,
};
use martensite_tkernel::precision;
use martensite_tkernel::standard::Failure;
use martensite_tkmath::geom_abs::Shape;
use martensite_tkmath::gp::{Point2, Transform2, Vector2};

impl Geom2dGeometry for Geom2dBSplineCurve {
    crate::spline_geometry_members!(
        Transform2,
        dyn Geom2dGeometry,
        "Geom2d_BSplineCurve",
        dump_bounded_curve
    );
}
impl Geom2dCurve for Geom2dBSplineCurve {
    fn reverse(&self) -> Result<(), Failure> {
        self.state.write().reverse()
    }
    fn reversed_parameter(&self, u: f64) -> Result<f64, Failure> {
        Ok(self.first_parameter() + self.last_parameter() - u)
    }
    fn first_parameter(&self) -> f64 {
        self.state.read().first_parameter()
    }
    fn last_parameter(&self) -> f64 {
        self.state.read().last_parameter()
    }
    fn is_closed(&self) -> Result<bool, Failure> {
        let state = self.state.read();
        let (start, end) = (state.start_point()?, state.end_point()?);
        Ok(start.square_distance(end) <= precision::computational())
    }
    fn is_periodic(&self) -> bool {
        self.state.read().periodic
    }
    fn continuity(&self) -> Shape {
        self.state.read().smooth
    }
    fn is_cn(&self, n: i32) -> Result<bool, Failure> {
        self.state.read().is_cn(n)
    }
    crate::spline_state_evaluators!(Point2, Vector2, Geom2dCurveD1, Geom2dCurveD2, Geom2dCurveD3);
}
impl Geom2dBoundedCurve for Geom2dBSplineCurve {
    fn end_point(&self) -> Result<Point2, Failure> {
        self.state.read().end_point()
    }
    fn start_point(&self) -> Result<Point2, Failure> {
        self.state.read().start_point()
    }
}

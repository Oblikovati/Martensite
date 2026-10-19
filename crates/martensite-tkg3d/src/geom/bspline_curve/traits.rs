//! The virtual members of GeomGeometry, GeomCurve and GeomBoundedCurve.
use super::GeomBSplineCurve;
use crate::geom::{
    GeomBoundedCurve, GeomCurve, GeomCurveD1, GeomCurveD2, GeomCurveD3, GeomGeometry,
    dump_bounded_curve,
};
use martensite_tkernel::precision;
use martensite_tkernel::standard::Failure;
use martensite_tkmath::geom_abs::Shape;
use martensite_tkmath::gp::{Point3, Transform3, Vector3};

impl GeomGeometry for GeomBSplineCurve {
    martensite_tkg2d::spline_geometry_members!(
        Transform3,
        dyn GeomGeometry,
        "Geom_BSplineCurve",
        dump_bounded_curve
    );
}
impl GeomCurve for GeomBSplineCurve {
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
        Ok(start.square_distance(&end) <= precision::computational())
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
    martensite_tkg2d::spline_state_evaluators!(
        Point3,
        Vector3,
        GeomCurveD1,
        GeomCurveD2,
        GeomCurveD3
    );
}
impl GeomBoundedCurve for GeomBSplineCurve {
    fn end_point(&self) -> Result<Point3, Failure> {
        self.state.read().end_point()
    }
    fn start_point(&self) -> Result<Point3, Failure> {
        self.state.read().start_point()
    }
}

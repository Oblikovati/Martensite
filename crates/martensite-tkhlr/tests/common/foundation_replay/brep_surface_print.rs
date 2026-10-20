//! One ordered 3D intersection point report for owned tokens and legacy stdout.
use super::common::{Sink, int, pnt, real};
use martensite_tkgeomalgo::int_curve_surface::IntersectionPoint;
use martensite_tkmath::gp::Point3;

pub(crate) trait SurfacePointEmitter {
    fn pnt(&mut self, point: Point3);
    fn real(&mut self, value: f64);
    fn int(&mut self, value: i32);
}
impl SurfacePointEmitter for Sink {
    fn pnt(&mut self, point: Point3) {
        pnt(self, point);
    }
    fn real(&mut self, value: f64) {
        real(self, value);
    }
    fn int(&mut self, value: i32) {
        int(self, value);
    }
}

/// Emit point, surface/curve parameters and transition; `ics_point(emit, &point)`.
pub fn ics_point(emit: &mut impl SurfacePointEmitter, point: &IntersectionPoint) {
    emit.pnt(point.pnt());
    emit.real(point.u());
    emit.real(point.v());
    emit.real(point.w());
    emit.int(point.transition() as i32);
}

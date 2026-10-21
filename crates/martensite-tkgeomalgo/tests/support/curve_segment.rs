//! The production line/trimmed-curve publication shared by the spatial commands.
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::geom::{GeomCurve, GeomLine, GeomTrimmedCurve};
use martensite_tkmath::gp::{Direction3, Point3};

/// Construct the segment that a spatial command publishes between its endpoints.
/// Example: `extrema_segment(Point3::default(), Point3::new(1.0, 0.0, 0.0))`.
pub(crate) fn extrema_segment(a: Point3, b: Point3) -> Handle<dyn GeomCurve> {
    let direction = Direction3::new(b.x() - a.x(), b.y() - a.y(), b.z() - a.z()).unwrap();
    let line: Handle<dyn GeomCurve> = Handle::new(GeomLine::from_point(a, direction)).cast();
    Handle::new(GeomTrimmedCurve::new(&line, 0.0, a.distance(&b), true, true).unwrap()).cast()
}

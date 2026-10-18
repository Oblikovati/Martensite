//! `GeomBoundedCurve`: curves limited by two finite parameters.
use super::curve::{GeomCurve, dump_curve};
use martensite_tkernel::impl_transient_interface;
use martensite_tkernel::standard::Failure;
use martensite_tkernel::standard::dump::DumpStream;
use martensite_tkg2d::geom2d::{dump_base, dump_class_name};
use martensite_tkmath::gp::Point3;
use std::io;

/// A curve with a start point and an end point; `curve.start_point()?`.
pub trait GeomBoundedCurve: GeomCurve {
    /// Point of the last parameter (`EndPoint`); `curve.end_point()?`.
    fn end_point(&self) -> Result<Point3, Failure>;
    /// Point of the first parameter (`StartPoint`); `curve.start_point()?`.
    fn start_point(&self) -> Result<Point3, Failure>;
}
impl_transient_interface!(dyn GeomBoundedCurve, "Geom_BoundedCurve", dyn GeomCurve);

/// `GeomBoundedCurve::DumpJson`; `dump_bounded_curve(&mut stream, -1)?`.
pub fn dump_bounded_curve(stream: &mut DumpStream, depth: i32) -> io::Result<()> {
    dump_class_name(stream, "Geom_BoundedCurve")?;
    dump_base(stream, depth, dump_curve)
}

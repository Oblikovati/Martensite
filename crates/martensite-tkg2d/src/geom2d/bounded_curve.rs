//! `Geom2dBoundedCurve`: curves limited by two finite parameters.
use super::curve::{Geom2dCurve, dump_curve};
use super::geometry::{dump_base, dump_class_name};
use martensite_tkernel::impl_transient_interface;
use martensite_tkernel::standard::Failure;
use martensite_tkernel::standard::dump::DumpStream;
use martensite_tkmath::gp::Point2;
use std::io;

/// A curve with a start point and an end point; `curve.start_point()?`.
pub trait Geom2dBoundedCurve: Geom2dCurve {
    /// Point of the last parameter (`EndPoint`); `curve.end_point()?`.
    fn end_point(&self) -> Result<Point2, Failure>;
    /// Point of the first parameter (`StartPoint`); `curve.start_point()?`.
    fn start_point(&self) -> Result<Point2, Failure>;
}
impl_transient_interface!(
    dyn Geom2dBoundedCurve,
    "Geom2d_BoundedCurve",
    dyn Geom2dCurve
);

/// `Geom2dBoundedCurve::DumpJson`; `dump_bounded_curve(&mut stream, -1)?`.
pub fn dump_bounded_curve(stream: &mut DumpStream, depth: i32) -> io::Result<()> {
    dump_class_name(stream, "Geom2d_BoundedCurve")?;
    dump_base(stream, depth, dump_curve)
}

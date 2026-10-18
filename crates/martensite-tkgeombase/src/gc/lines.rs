//! MakeLine and MakeSegment: Line and trimmed Geom lines.
use crate::gce::{self, ErrorType};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::geom::{GeomCurve, GeomLine, GeomTrimmedCurve};
use martensite_tkmath::elclib;
use martensite_tkmath::gp::{Axis1, Direction3, Line3, Point3};

/// MakeLine: builds a Line from a point and a direction, an axis, a gp
/// line, two points, or a parallel line.
///
/// `let line = MakeLine::from_axis(Axis1::default()).value()?;`
#[derive(Clone, Default)]
pub struct MakeLine {
    the_result: Handle<GeomLine>,
    the_error: ErrorType,
}
gc_result!(MakeLine, GeomLine, "GC_MakeLine");
gc_from_gce!(MakeLine, gce::MakeLin, GeomLine::from_lin);

impl MakeLine {
    /// MakeLine(P, V): the line through `point` of direction `direction`.
    /// `MakeLine::new(p, d)`.
    pub fn new(point: Point3, direction: Direction3) -> Self {
        Self::built(GeomLine::from_point(point, direction))
    }

    /// MakeLine(A1): the line of axis `axis`. `MakeLine::from_axis(axis)`.
    pub fn from_axis(axis: Axis1) -> Self {
        Self::built(GeomLine::new(axis))
    }

    /// MakeLine(L): the Geom line of `line`. `MakeLine::from_lin(line)`.
    pub fn from_lin(line: Line3) -> Self {
        Self::built(GeomLine::from_lin(line))
    }

    /// MakeLine(P1, P2): MakeLin(P1, P2). `MakeLine::from_points(a, b)?`.
    pub fn from_points(p1: Point3, p2: Point3) -> Result<Self, Failure> {
        Self::from_gce(gce::MakeLin::from_points(p1, p2)?)
    }

    /// MakeLine(Lin, Point): MakeLin(Lin, Point).
    /// `MakeLine::from_line_point(line, p)?`.
    pub fn from_line_point(line: Line3, point: Point3) -> Result<Self, Failure> {
        Self::from_gce(gce::MakeLin::from_line_point(line, point))
    }
}

/// MakeSegment: builds a trimmed Line between two points, or on a line
/// between points or parameters.
///
/// `let segment = MakeSegment::from_parameters(line, 0.0, 1.0)?.value()?;`
#[derive(Clone, Default)]
pub struct MakeSegment {
    the_result: Handle<GeomTrimmedCurve>,
    the_error: ErrorType,
}
gc_result!(MakeSegment, GeomTrimmedCurve, "GC_MakeSegment");

impl MakeSegment {
    /// `TrimmedCurve(L, U1, U2, true)`: the segment of `line` between two
    /// parameters.
    fn trimmed(line: Handle<GeomLine>, u1: f64, u2: f64) -> Result<Self, Failure> {
        let basis: Handle<dyn GeomCurve> = line.cast();
        Ok(Self::built(GeomTrimmedCurve::new(
            &basis, u1, u2, true, true,
        )?))
    }

    /// MakeSegment(P1, P2): the line of MakeLine(P1, P2) (its conversion
    /// raises not-done failure for confused points) trimmed to `[0, |P1 P2|]`.
    /// `MakeSegment::from_points(a, b)?`.
    pub fn from_points(p1: Point3, p2: Point3) -> Result<Self, Failure> {
        let dist: f64 = p1.distance(&p2);
        let line: Handle<GeomLine> = Handle::try_from(&MakeLine::from_points(p1, p2)?)?;
        Self::trimmed(line, 0.0, dist)
    }

    /// MakeSegment(Line, Point, U): the segment from the parameter of `point`
    /// to `u`. `MakeSegment::from_point_parameter(line, p, 1.0)?`.
    pub fn from_point_parameter(line: Line3, point: Point3, u: f64) -> Result<Self, Failure> {
        let u_first: f64 = elclib::parameter(&line, point)?;
        Self::trimmed(Handle::new(GeomLine::from_lin(line)), u_first, u)
    }

    /// MakeSegment(Line, P1, P2): the segment between the parameters of `p1`
    /// and `p2`. `MakeSegment::from_line_points(line, a, b)?`.
    pub fn from_line_points(line: Line3, p1: Point3, p2: Point3) -> Result<Self, Failure> {
        let u_first: f64 = elclib::parameter(&line, p1)?;
        let u_last: f64 = elclib::parameter(&line, p2)?;
        Self::trimmed(Handle::new(GeomLine::from_lin(line)), u_first, u_last)
    }

    /// MakeSegment(Line, U1, U2): the segment between two parameters.
    /// `MakeSegment::from_parameters(line, 0.0, 1.0)?`.
    pub fn from_parameters(line: Line3, u1: f64, u2: f64) -> Result<Self, Failure> {
        Self::trimmed(Handle::new(GeomLine::from_lin(line)), u1, u2)
    }
}

//! MakeCircle and MakeCircle2d: Circle and Circle.
use crate::gce::{self, ErrorType};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg2d::geom2d::Geom2dCircle;
use martensite_tkg3d::geom::GeomCircle;
use martensite_tkmath::gp::{
    Axis1, Axis2, Axis2d, Axis22d, Circle2, Circle3, Direction3, Point2, Point3,
};

/// MakeCircle: builds a Circle from a gp circle, a frame, a parallel
/// circle, three points, a center and a normal, or an axis.
///
/// `let circle = MakeCircle::new(Axis2::default(), 1.0)?.value()?;`
#[derive(Clone, Default)]
pub struct MakeCircle {
    the_result: Handle<GeomCircle>,
    the_error: ErrorType,
}
gc_result!(MakeCircle, GeomCircle, "GC_MakeCircle");
gc_from_gce!(MakeCircle, gce::MakeCirc, GeomCircle::from_circ);

impl MakeCircle {
    /// MakeCircle(C): the Geom circle of `circle`. `MakeCircle::from_circ(c)`.
    pub fn from_circ(circle: Circle3) -> Self {
        Self::built(GeomCircle::from_circ(circle))
    }

    /// MakeCircle(A2, Radius): NegativeRadius when `radius < 0`.
    /// `MakeCircle::new(frame, 1.0)?`.
    pub fn new(frame: Axis2, radius: f64) -> Result<Self, Failure> {
        if radius < 0.0 {
            return Ok(Self::failed(ErrorType::NegativeRadius));
        }
        Ok(Self::built(GeomCircle::from_circ(Circle3::new(
            frame, radius,
        )?)))
    }

    /// MakeCircle(Circ, Point): the conversion of MakeCirc(Circ, Point),
    /// always done. `MakeCircle::from_circ_point(c, p)?`.
    pub fn from_circ_point(circle: Circle3, point: Point3) -> Result<Self, Failure> {
        let circ: Circle3 = gce::MakeCirc::from_circ_point(circle, point)?.operator()?;
        Ok(Self::built(GeomCircle::from_circ(circ)))
    }

    /// MakeCircle(Circ, Dist): MakeCirc(Circ, Dist).
    /// `MakeCircle::from_circ_distance(c, 1.0)?`.
    pub fn from_circ_distance(circle: Circle3, dist: f64) -> Result<Self, Failure> {
        Self::from_gce(gce::MakeCirc::from_circ_distance(circle, dist)?)
    }

    /// MakeCircle(P1, P2, P3): MakeCirc(P1, P2, P3).
    /// `MakeCircle::through_points(a, b, c)?`.
    pub fn through_points(p1: Point3, p2: Point3, p3: Point3) -> Result<Self, Failure> {
        Self::from_gce(gce::MakeCirc::through_points(p1, p2, p3)?)
    }

    /// MakeCircle(Point, Norm, Radius): MakeCirc(Point, Norm, Radius).
    /// `MakeCircle::from_center_normal(p, n, 1.0)?`.
    pub fn from_center_normal(
        center: Point3,
        normal: Direction3,
        radius: f64,
    ) -> Result<Self, Failure> {
        Self::from_gce(gce::MakeCirc::from_center_normal(center, normal, radius)?)
    }

    /// MakeCircle(Point, PtAxis, Radius): MakeCirc(Point, PtAxis, Radius).
    /// `MakeCircle::from_center_axis_point(c, p, 1.0)?`.
    pub fn from_center_axis_point(
        center: Point3,
        axis_point: Point3,
        radius: f64,
    ) -> Result<Self, Failure> {
        Self::from_gce(gce::MakeCirc::from_center_axis_point(
            center, axis_point, radius,
        )?)
    }

    /// MakeCircle(Axis, Radius): MakeCirc(Axis, Radius).
    /// `MakeCircle::from_axis(axis, 1.0)?`.
    pub fn from_axis(axis: Axis1, radius: f64) -> Result<Self, Failure> {
        Self::from_gce(gce::MakeCirc::from_axis(axis, radius)?)
    }
}

/// MakeCircle2d: builds a Circle from a gp circle, an axis, a frame, a
/// parallel circle, three points or a center.
///
/// `let circle = MakeCircle2d::from_frame(Axis22d::default(), 1.0)?.value()?;`
#[derive(Clone, Default)]
pub struct MakeCircle2d {
    the_result: Handle<Geom2dCircle>,
    the_error: ErrorType,
}
gc_result!(MakeCircle2d, Geom2dCircle, "GC_MakeCircle2d");
gc_from_gce!(MakeCircle2d, gce::MakeCirc2d, Geom2dCircle::from_circ2d);

impl MakeCircle2d {
    /// MakeCircle2d(C): the Geom2d circle of `circle`.
    /// `MakeCircle2d::from_circ(c)`.
    pub fn from_circ(circle: Circle2) -> Self {
        Self::built(Geom2dCircle::from_circ2d(circle))
    }

    /// MakeCircle2d(A, Radius, Sense): NegativeRadius when `radius < 0`.
    /// `MakeCircle2d::new(axis, 1.0, true)?`.
    pub fn new(axis: Axis2d, radius: f64, sense: bool) -> Result<Self, Failure> {
        if radius < 0.0 {
            return Ok(Self::failed(ErrorType::NegativeRadius));
        }
        Ok(Self::built(Geom2dCircle::from_axis(axis, radius, sense)?))
    }

    /// MakeCircle2d(A, Radius) with a Ax22d frame: NegativeRadius when
    /// `radius < 0`. `MakeCircle2d::from_frame(frame, 1.0)?`.
    pub fn from_frame(frame: Axis22d, radius: f64) -> Result<Self, Failure> {
        if radius < 0.0 {
            return Ok(Self::failed(ErrorType::NegativeRadius));
        }
        Ok(Self::built(Geom2dCircle::new(frame, radius)?))
    }

    /// MakeCircle2d(Circ, Point): MakeCirc2d(Circ, Point).
    /// `MakeCircle2d::from_circ_point(c, p)?`.
    pub fn from_circ_point(circle: Circle2, point: Point2) -> Result<Self, Failure> {
        Self::from_gce(gce::MakeCirc2d::from_circ_point(circle, point)?)
    }

    /// MakeCircle2d(Circ, Dist): MakeCirc2d(Circ, Dist).
    /// `MakeCircle2d::from_circ_distance(c, 1.0)?`.
    pub fn from_circ_distance(circle: Circle2, dist: f64) -> Result<Self, Failure> {
        Self::from_gce(gce::MakeCirc2d::from_circ_distance(circle, dist)?)
    }

    /// MakeCircle2d(P1, P2, P3): MakeCirc2d(P1, P2, P3).
    /// `MakeCircle2d::through_points(a, b, c)?`.
    pub fn through_points(p1: Point2, p2: Point2, p3: Point2) -> Result<Self, Failure> {
        Self::from_gce(gce::MakeCirc2d::through_points(p1, p2, p3)?)
    }

    /// MakeCircle2d(Point, Radius, Sense): MakeCirc2d(Point, Radius, Sense).
    /// `MakeCircle2d::from_center(p, 1.0, true)?`.
    pub fn from_center(center: Point2, radius: f64, sense: bool) -> Result<Self, Failure> {
        Self::from_gce(gce::MakeCirc2d::from_center(center, radius, sense)?)
    }

    /// MakeCircle2d(Center, Point, Sense): MakeCirc2d(Center, Point, Sense).
    /// `MakeCircle2d::from_center_point(c, p, true)?`.
    pub fn from_center_point(center: Point2, point: Point2, sense: bool) -> Result<Self, Failure> {
        Self::from_gce(gce::MakeCirc2d::from_center_point(center, point, sense)?)
    }
}

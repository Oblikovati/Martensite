//! Points of elementary surfaces; each `*D0` repeats its `*Value` arithmetic exactly.
use super::frame::{Frame, flush, torus_tolerance};
use crate::gp::{Axis3, Point3};

/// Plane point `u·X + v·Y + origin`; `plane_value(1.0,2.0,frame)`.
pub fn plane_value(u: f64, v: f64, position: Axis3) -> Point3 {
    let frame = Frame::of(position);
    frame.point(frame.pair(u, v))
}
/// Cone point on the circle of radius `R + v·sin(a)` at height `v·cos(a)`;
/// `cone_value(0.5,1.0,frame,2.0,0.3)`.
pub fn cone_value(u: f64, v: f64, position: Axis3, radius: f64, semi_angle: f64) -> Point3 {
    let frame = Frame::of(position);
    let ring = radius + v * martensite_cmath::sin(semi_angle);
    let height = v * martensite_cmath::cos(semi_angle);
    frame.point(frame.triple(
        ring * martensite_cmath::cos(u),
        ring * martensite_cmath::sin(u),
        height,
    ))
}
/// Cylinder point `R·cos(u)·X + R·sin(u)·Y + v·Z + origin`; `cylinder_value(0.5,1.0,frame,2.0)`.
pub fn cylinder_value(u: f64, v: f64, position: Axis3, radius: f64) -> Point3 {
    let frame = Frame::of(position);
    frame.point(frame.triple(
        radius * martensite_cmath::cos(u),
        radius * martensite_cmath::sin(u),
        v,
    ))
}
/// Sphere point with longitude `u` and latitude `v`; `sphere_value(0.5,0.2,frame,2.0)`.
pub fn sphere_value(u: f64, v: f64, position: Axis3, radius: f64) -> Point3 {
    let frame = Frame::of(position);
    let ring = radius * martensite_cmath::cos(v);
    let height = radius * martensite_cmath::sin(v);
    frame.point(frame.triple(
        ring * martensite_cmath::cos(u),
        ring * martensite_cmath::sin(u),
        height,
    ))
}
/// Torus point with terms within `10·(r+R)·ε` flushed to zero; `torus_value(0.5,0.2,frame,5.0,1.0)`.
pub fn torus_value(u: f64, v: f64, position: Axis3, major: f64, minor: f64) -> Point3 {
    let frame = Frame::of(position);
    let ring = major + minor * martensite_cmath::cos(v);
    let height = minor * martensite_cmath::sin(v);
    let tolerance = torus_tolerance(major, minor);
    let (a1, a2) = (
        ring * martensite_cmath::cos(u),
        ring * martensite_cmath::sin(u),
    );
    frame.point(frame.triple(
        flush(a1, tolerance),
        flush(a2, tolerance),
        flush(height, tolerance),
    ))
}

/// elslib::plane_d0, identical to [`plane_value`]; `plane_d0(1.0,2.0,frame)`.
pub fn plane_d0(u: f64, v: f64, position: Axis3) -> Point3 {
    plane_value(u, v, position)
}
/// elslib::cone_d0, identical to [`cone_value`]; `cone_d0(0.5,1.0,frame,2.0,0.3)`.
pub fn cone_d0(u: f64, v: f64, position: Axis3, radius: f64, semi_angle: f64) -> Point3 {
    cone_value(u, v, position, radius, semi_angle)
}
/// elslib::cylinder_d0, identical to [`cylinder_value`]; `cylinder_d0(0.5,1.0,frame,2.0)`.
pub fn cylinder_d0(u: f64, v: f64, position: Axis3, radius: f64) -> Point3 {
    cylinder_value(u, v, position, radius)
}
/// elslib::sphere_d0, identical to [`sphere_value`]; `sphere_d0(0.5,0.2,frame,2.0)`.
pub fn sphere_d0(u: f64, v: f64, position: Axis3, radius: f64) -> Point3 {
    sphere_value(u, v, position, radius)
}
/// elslib::torus_d0, identical to [`torus_value`]; `torus_d0(0.5,0.2,frame,5.0,1.0)`.
pub fn torus_d0(u: f64, v: f64, position: Axis3, major: f64, minor: f64) -> Point3 {
    torus_value(u, v, position, major, minor)
}

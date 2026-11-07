//! Value-only frame arithmetic; construction and admission use the same coordinates.
use super::request::CylinderSupport;
use martensite_cmath::{cos, sin};

pub(super) fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a.into_iter().zip(b).map(|(x, y)| x * y).sum()
}
pub(super) fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}
pub(super) fn point(support: &CylinderSupport, angle: f64, radius: f64, axial: f64) -> [f64; 3] {
    let tangent = cross(support.axis, support.radial);
    std::array::from_fn(|i| {
        support.origin[i]
            + radius * (cos(angle) * support.radial[i] + sin(angle) * tangent[i])
            + axial * support.axis[i]
    })
}

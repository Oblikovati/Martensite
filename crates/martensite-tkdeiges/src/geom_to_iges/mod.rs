//! Translation of geometric points, curves and surfaces into IGES entities.
mod curve;
mod curve_bspline;
mod curve_conics;
mod entity;
mod point;
mod surface;
mod surface_bspline;
mod surface_quadrics;
mod surface_solids;
mod surface_swept;

pub use curve::GeomToIgesGeomCurve;
pub use entity::GeomToIgesGeomEntity;
pub use point::GeomToIgesGeomPoint;
pub use surface::GeomToIgesGeomSurface;

use martensite_tkernel::precision;

/// Replaces the bounds beyond the infinite limit by that limit.
fn bounded_range(first: f64, last: f64) -> (f64, f64) {
    let lower = if precision::is_negative_infinite(first) {
        -precision::infinite()
    } else {
        first
    };
    let upper = if precision::is_positive_infinite(last) {
        precision::infinite()
    } else {
        last
    };
    (lower, upper)
}

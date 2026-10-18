//! Read access: local evaluation, knots, multiplicities, poles and weights, through
//! the accessors shared with the other B-spline curve class.
use super::Geom2dBSplineCurve;
use martensite_tkernel::ncollection::Array1;
use martensite_tkernel::standard::Failure;
use martensite_tkmath::gp::{Point2, Vector2};

impl Geom2dBSplineCurve {
    crate::bspline_curve_accessors!(Point2, Vector2);
}

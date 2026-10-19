//! Read access: local evaluation, knots, multiplicities, poles and weights, through
//! the accessors shared with the other B-spline curve class.
use super::GeomBSplineCurve;
use martensite_tkernel::ncollection::Array1;
use martensite_tkernel::standard::Failure;
use martensite_tkmath::gp::{Point3, Vector3};

impl GeomBSplineCurve {
    martensite_tkg2d::bspline_curve_accessors!(Point3, Vector3);
}

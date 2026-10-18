//! Geometry regression fixtures separated by their actual algorithm dependencies.
//! Original inputs/assertions at .
#![allow(non_snake_case)]
mod common;
use common::EXPECT_NEAR as legacy_near;
use martensite_tkernel::standard::Handle;
use martensite_tkg2d::{
    adaptor2d::Adaptor2dCurve2d,
    geom2d::{Geom2dCircle, Geom2dCurve, Geom2dOffsetCurve},
    geom2d_adaptor::Geom2dAdaptorCurve,
};
use martensite_tkgeombase::{
    gcpnts::AbscissaPoint,
    geom2d_convert::{self, ApproxCurve},
};
use martensite_tkmath::{
    convert::ParameterisationType,
    geom_abs::Shape,
    gp::{Axis2d, Direction2, Point2},
};

#[test]
// Keep the reference length from fclasses/bug31381 verbatim.
#[allow(clippy::excessive_precision)]
fn geom2d_offset_curve_additional_rational_b_spline_approximation() {
    let circle: Handle<dyn Geom2dCurve> = Handle::new(
        Geom2dCircle::from_axis(Axis2d::new(Point2::default(), Direction2::X), 1.0, true).unwrap(),
    )
    .cast();
    let converted =
        geom2d_convert::curve_to_bspline_curve(&circle, ParameterisationType::TgtThetaOver2)
            .unwrap();
    assert!(!converted.is_null());
    let offset: Handle<dyn Geom2dCurve> =
        Handle::new(Geom2dOffsetCurve::new(&converted.cast(), -0.5, false).unwrap()).cast();
    let approximation = ApproxCurve::from_curve(&offset, 1e-4, Shape::C2, 25, 5).unwrap();
    assert!(approximation.has_result());
    let curve = approximation.curve();
    assert!(!curve.is_null());
    let adaptor = Geom2dAdaptorCurve::with_curve(&curve.cast()).unwrap();
    let length = AbscissaPoint::length(&adaptor as &dyn Adaptor2dCurve2d).unwrap();
    legacy_near(length, 3.1415850499856011, 1e-6);
}

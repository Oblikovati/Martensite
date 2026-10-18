//! Gcpnts abscissa point contracts.
#![allow(non_snake_case)]
mod common;
use common::prelude::*;
use martensite_tkgeombase::gcpnts::AbscissaPoint as GCPnts_AbscissaPoint;

// `GeomAdaptor_Curve(aLine, 0.0, 10.0)` on the X axis.
fn lineAdaptor() -> GeomAdaptorCurve {
    let aLine: Handle<GeomLine> = Handle::new(GeomLine::from_point(
        Point3::new(0.0, 0.0, 0.0),
        dir3(1.0, 0.0, 0.0),
    ));
    GeomAdaptorCurve::with_range(&aLine.cast(), 0.0, 10.0).unwrap()
}

// `GeomAdaptor_Curve(aCircle, 0.0, theLast)` on a circle of the XY plane.
fn circleAdaptor(theRadius: f64, theLast: f64) -> GeomAdaptorCurve {
    let aCircle: Handle<GeomCircle> = Handle::new(
        GeomCircle::new(
            Axis2::new(Point3::new(0.0, 0.0, 0.0), dir3(0.0, 0.0, 1.0)).unwrap(),
            theRadius,
        )
        .unwrap(),
    );
    GeomAdaptorCurve::with_range(&aCircle.cast(), 0.0, theLast).unwrap()
}

#[test]
fn pnts_abscissa_point_line_length() {
    let anAdaptor = lineAdaptor();

    let aLength: f64 = GCPnts_AbscissaPoint::length(&anAdaptor).unwrap();

    EXPECT_NEAR(aLength, 10.0, precision::confusion());
}

#[test]
fn pnts_abscissa_point_circle_arc_length() {
    let anAdaptor = circleAdaptor(1.0, PI);

    let aLength: f64 = GCPnts_AbscissaPoint::length(&anAdaptor).unwrap();

    EXPECT_NEAR(aLength, PI, precision::confusion());
}

#[test]
fn pnts_abscissa_point_full_circle_length() {
    let anAdaptor = circleAdaptor(5.0, 2.0 * PI);

    let aLength: f64 = GCPnts_AbscissaPoint::length(&anAdaptor).unwrap();

    EXPECT_NEAR(aLength, 2.0 * PI * 5.0, precision::confusion());
}

#[test]
fn pnts_abscissa_point_parameter_at_abscissa_line() {
    let anAdaptor = lineAdaptor();

    let anAbscissa = GCPnts_AbscissaPoint::from_abscissa(&anAdaptor, 5.0, 0.0).unwrap();

    assert!(anAbscissa.is_done());
    let aParam: f64 = anAbscissa.parameter().unwrap();
    EXPECT_NEAR(aParam, 5.0, precision::confusion());
}

#[test]
fn pnts_abscissa_point_parameter_at_abscissa_circle() {
    let anAdaptor = circleAdaptor(1.0, 2.0 * PI);

    let anAbscissa = GCPnts_AbscissaPoint::from_abscissa(&anAdaptor, PI / 2.0, 0.0).unwrap();

    assert!(anAbscissa.is_done());
    let aParam: f64 = anAbscissa.parameter().unwrap();
    EXPECT_NEAR(aParam, PI / 2.0, precision::confusion());
}

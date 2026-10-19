//! Geom convert contracts.
#![allow(non_snake_case)]
#[path = "support/geometric_test_support/mod.rs"]
mod common;
use common::EXPECT_NEAR;
use martensite_tkernel::precision;
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::geom::{
    GeomBSplineCurve, GeomBSplineSurface, GeomCircle, GeomCurve, GeomLine, GeomPlane,
    GeomRectangularTrimmedSurface, GeomSurface, GeomTrimmedCurve,
};
use martensite_tkgeombase::geom_convert as GeomConvert;
use martensite_tkmath::convert::ParameterisationType;
use martensite_tkmath::gp::{self, Axis2, Direction3, Point3};

#[test]
fn circle_to_b_spline() {
    let aCircle: Handle<GeomCircle> = Handle::new(
        GeomCircle::new(
            Axis2::new(
                Point3::new(0.0, 0.0, 0.0),
                Direction3::new(0.0, 0.0, 1.0).unwrap(),
            )
            .unwrap(),
            5.0,
        )
        .unwrap(),
    );

    let aBSpline: Handle<GeomBSplineCurve> =
        GeomConvert::curve_to_bspline_curve(&aCircle.cast(), ParameterisationType::TgtThetaOver2)
            .unwrap();

    assert!(!aBSpline.is_null());

    let aOrigPnt: Point3 = aCircle.d0(0.0).unwrap();
    let aBSplinePnt: Point3 = aBSpline.d0(0.0).unwrap();

    EXPECT_NEAR(aOrigPnt.distance(&aBSplinePnt), 0.0, precision::confusion());
}

#[test]
fn line_to_b_spline() {
    let aLine: Handle<GeomLine> = Handle::new(GeomLine::from_point(
        Point3::new(0.0, 0.0, 0.0),
        Direction3::new(1.0, 0.0, 0.0).unwrap(),
    ));
    let aTrimmed: Handle<GeomTrimmedCurve> =
        Handle::new(GeomTrimmedCurve::new(&aLine.cast(), 0.0, 10.0, true, true).unwrap());

    let aBSpline: Handle<GeomBSplineCurve> = GeomConvert::curve_to_bspline_curve(
        &aTrimmed.cast::<dyn GeomCurve>(),
        ParameterisationType::TgtThetaOver2,
    )
    .unwrap();

    assert!(!aBSpline.is_null());

    let aPnt: Point3 = aBSpline.d0(5.0).unwrap();
    let anExpectedPnt = Point3::new(5.0, 0.0, 0.0);

    EXPECT_NEAR(aPnt.distance(&anExpectedPnt), 0.0, precision::confusion());
}

#[test]
fn plane_to_b_spline_surface() {
    let aPlane: Handle<GeomPlane> = Handle::new(GeomPlane::new(gp::Axis3::from_axis2(gp::xoy())));
    let aTrimmed: Handle<GeomRectangularTrimmedSurface> = Handle::new(
        GeomRectangularTrimmedSurface::new(&aPlane.cast(), (0.0, 10.0), (0.0, 10.0), (true, true))
            .unwrap(),
    );

    let aBSurf: Handle<GeomBSplineSurface> =
        GeomConvert::surface_to_bspline_surface(&aTrimmed.cast::<dyn GeomSurface>()).unwrap();

    assert!(!aBSurf.is_null());

    let aPnt: Point3 = aBSurf.d0(5.0, 5.0).unwrap();
    let anExpectedPnt = Point3::new(5.0, 5.0, 0.0);

    EXPECT_NEAR(aPnt.distance(&anExpectedPnt), 0.0, precision::confusion());
}

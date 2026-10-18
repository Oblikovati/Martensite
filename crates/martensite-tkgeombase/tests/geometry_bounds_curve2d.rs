//! Geom bnd lib curve2d contracts.
//! The suites are split across `geometry_bounds_curve2d/suites_N.rs` in the defined order.
#![allow(non_snake_case)]
mod bnd_support;
#[path = "support/geometric_test_support/mod.rs"]
mod common;
use bnd_support::{array1, curve2d, dir2d, get2, pnt2d};
use common::EXPECT_NEAR;
use martensite_tkernel::precision;
use martensite_tkernel::standard::Handle;
use martensite_tkg2d::adaptor2d::Adaptor2dCurve2d;
use martensite_tkg2d::geom2d::{
    Geom2dBSplineCurve, Geom2dBezierCurve, Geom2dCircle, Geom2dCurve, Geom2dEllipse,
    Geom2dHyperbola, Geom2dLine, Geom2dParabola, Geom2dTrimmedCurve,
};
use martensite_tkg2d::geom2d_adaptor::Geom2dAdaptorCurve;
use martensite_tkgeombase::bnd_lib::Add2dCurve;
use martensite_tkgeombase::geom_bnd_lib::{Curve2d, sampling};
use martensite_tkmath::bnd::BoundingBox2;
use martensite_tkmath::geom_abs::CurveType;
use martensite_tkmath::gp::{Axis2d, Axis22d, Point2};
use std::f64::consts::PI;

// Helper: compare two BoundingBox2 results within tolerance.
fn CompareBoxes2d(theNew: &BoundingBox2, theOld: &BoundingBox2, theTol: f64) {
    for (aN, aO) in get2(theNew).iter().zip(get2(theOld)) {
        EXPECT_NEAR(*aN, aO, theTol);
    }
}

// Helper: verify that the new box conservatively encloses the old box.
fn ExpectConservativeEnvelope2d(theNew: &BoundingBox2, theOld: &BoundingBox2, theTol: f64) {
    let [aXminN, aXmaxN, aYminN, aYmaxN] = get2(theNew);
    let [aXminO, aXmaxO, aYminO, aYmaxO] = get2(theOld);
    assert!(aXminN <= aXminO + theTol);
    assert!(aXmaxN >= aXmaxO - theTol);
    assert!(aYminN <= aYminO + theTol);
    assert!(aYmaxN >= aYmaxO - theTol);
}

fn adaptor(theCurve: &Handle<dyn Geom2dCurve>) -> Geom2dAdaptorCurve {
    Geom2dAdaptorCurve::with_curve(theCurve).unwrap()
}

// `new Circle(Ax2d(Point2(0.0, 0.0), Dir2d(1.0, 0.0)), theRadius)`.
fn circle(theRadius: f64) -> Handle<dyn Geom2dCurve> {
    let anAxis = Axis2d::new(pnt2d(0.0, 0.0), dir2d(1.0, 0.0));
    curve2d(Geom2dCircle::from_axis(anAxis, theRadius, true).unwrap())
}

// `Ax22d(Point2(0.0, 0.0), Dir2d(1.0, 0.0), Dir2d(0.0, 1.0))`.
fn frame() -> Axis22d {
    Axis22d::with_directions(pnt2d(0.0, 0.0), dir2d(1.0, 0.0), dir2d(0.0, 1.0)).unwrap()
}

fn new_box(theCurve: &Handle<dyn Geom2dCurve>, theTol: f64) -> BoundingBox2 {
    let mut aBox = BoundingBox2::default();
    Curve2d::from_curve(theCurve)
        .unwrap()
        .add(theTol, &mut aBox)
        .unwrap();
    aBox
}

fn new_box_range(
    theCurve: &Handle<dyn Geom2dCurve>,
    theU1: f64,
    theU2: f64,
    theTol: f64,
) -> BoundingBox2 {
    let mut aBox = BoundingBox2::default();
    Curve2d::from_curve(theCurve)
        .unwrap()
        .add_range(theU1, theU2, theTol, &mut aBox)
        .unwrap();
    aBox
}

fn old_box(theAdaptor: &dyn Adaptor2dCurve2d, theTol: f64) -> BoundingBox2 {
    let mut aBox = BoundingBox2::default();
    Add2dCurve::add_adaptor(theAdaptor, theTol, &mut aBox).unwrap();
    aBox
}

fn old_box_range(
    theCurve: &Handle<dyn Geom2dCurve>,
    theU1: f64,
    theU2: f64,
    theTol: f64,
) -> BoundingBox2 {
    let mut aBox = BoundingBox2::default();
    Add2dCurve::add_adaptor_range(&adaptor(theCurve), theU1, theU2, theTol, &mut aBox).unwrap();
    aBox
}

include!("geometry_bounds_curve2d/suites_0.rs");
include!("geometry_bounds_curve2d/suites_1.rs");

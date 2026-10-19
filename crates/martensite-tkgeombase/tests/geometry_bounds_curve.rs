//! Geom bnd lib curve contracts.
//! The suites are split across `geometry_bounds_curve/suites_N.rs` in the defined order.
#![allow(non_snake_case)]
mod bnd_support;
#[path = "support/geometric_test_support/mod.rs"]
mod common;
use bnd_support::{array1, curve, dir, get3, pnt};
use common::EXPECT_NEAR;
use martensite_tkernel::precision;
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::adaptor3d::Adaptor3dCurve;
use martensite_tkg3d::geom::{
    GeomBSplineCurve, GeomBezierCurve, GeomCircle, GeomCurve, GeomEllipse, GeomHyperbola, GeomLine,
    GeomParabola, GeomTrimmedCurve,
};
use martensite_tkg3d::geom_adaptor::GeomAdaptorCurve;
use martensite_tkgeombase::bnd_lib::Add3dCurve;
use martensite_tkgeombase::geom_bnd_lib::{Curve, sampling};
use martensite_tkmath::bnd::BoundingBox3;
use martensite_tkmath::geom_abs::CurveType;
use martensite_tkmath::gp;
use std::f64::consts::PI;

// Helper: compare two BoundingBox3 results within tolerance.
fn CompareBoxes(theNew: &BoundingBox3, theOld: &BoundingBox3, theTol: f64) {
    let aNew = get3(theNew);
    let aOld = get3(theOld);
    for (aN, aO) in aNew.iter().zip(aOld) {
        EXPECT_NEAR(*aN, aO, theTol);
    }
}

// Helper: verify that the new box conservatively encloses the old box.
fn ExpectConservativeEnvelope(theNew: &BoundingBox3, theOld: &BoundingBox3, theTol: f64) {
    let [aXminN, aXmaxN, aYminN, aYmaxN, aZminN, aZmaxN] = get3(theNew);
    let [aXminO, aXmaxO, aYminO, aYmaxO, aZminO, aZmaxO] = get3(theOld);
    assert!(aXminN <= aXminO + theTol);
    assert!(aXmaxN >= aXmaxO - theTol);
    assert!(aYminN <= aYminO + theTol);
    assert!(aYmaxN >= aYmaxO - theTol);
    assert!(aZminN <= aZminO + theTol);
    assert!(aZmaxN >= aZmaxO - theTol);
}

fn adaptor(theCurve: &Handle<dyn GeomCurve>) -> GeomAdaptorCurve {
    GeomAdaptorCurve::with_curve(theCurve).unwrap()
}

fn circle(theRadius: f64) -> Handle<dyn GeomCurve> {
    curve(GeomCircle::new(gp::xoy(), theRadius).unwrap())
}

fn ellipse(theMajR: f64, theMinR: f64) -> Handle<dyn GeomCurve> {
    curve(GeomEllipse::new(gp::xoy(), theMajR, theMinR).unwrap())
}

// Add3dCurve::Add(adaptor, U1, U2, Tol, B) on a fresh box.
fn old_box_range(
    theCurve: &Handle<dyn GeomCurve>,
    theU1: f64,
    theU2: f64,
    theTol: f64,
) -> BoundingBox3 {
    let mut aBox = BoundingBox3::default();
    Add3dCurve::add_range(&adaptor(theCurve), theU1, theU2, theTol, &mut aBox).unwrap();
    aBox
}

// Add3dCurve::Add(adaptor, Tol, B) on a fresh box.
fn old_box(theAdaptor: &dyn Adaptor3dCurve, theTol: f64) -> BoundingBox3 {
    let mut aBox = BoundingBox3::default();
    Add3dCurve::add(theAdaptor, theTol, &mut aBox).unwrap();
    aBox
}

// Curve(handle).Add(Tol, B) on a fresh box.
fn new_box(theCurve: &Handle<dyn GeomCurve>, theTol: f64) -> BoundingBox3 {
    let mut aBox = BoundingBox3::default();
    Curve::from_curve(theCurve)
        .unwrap()
        .add(theTol, &mut aBox)
        .unwrap();
    aBox
}

// Curve(handle).Add(U1, U2, Tol, B) on a fresh box.
fn new_box_range(
    theCurve: &Handle<dyn GeomCurve>,
    theU1: f64,
    theU2: f64,
    theTol: f64,
) -> BoundingBox3 {
    let mut aBox = BoundingBox3::default();
    Curve::from_curve(theCurve)
        .unwrap()
        .add_range(theU1, theU2, theTol, &mut aBox)
        .unwrap();
    aBox
}

include!("geometry_bounds_curve/suites_0.rs");
include!("geometry_bounds_curve/suites_1.rs");

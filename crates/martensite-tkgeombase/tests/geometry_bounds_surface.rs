//! Geom bnd lib surface contracts.
//! The suites are split across `geometry_bounds_surface/suites_N.rs` in the defined order.
#![allow(non_snake_case)]
mod bnd_support;
#[path = "support/geometric_test_support/mod.rs"]
mod common;
use bnd_support::{array1, array2, dir, get3, pnt, surface};
use common::EXPECT_NEAR;
use martensite_tkernel::precision;
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::adaptor3d::Adaptor3dSurface;
use martensite_tkg3d::geom::{
    GeomBSplineSurface, GeomBezierSurface, GeomConicalSurface, GeomCylindricalSurface, GeomPlane,
    GeomRectangularTrimmedSurface, GeomSphericalSurface, GeomSurface, GeomToroidalSurface,
};
use martensite_tkg3d::geom_adaptor::GeomAdaptorSurface;
use martensite_tkgeombase::bnd_lib::AddSurface;
use martensite_tkgeombase::geom_bnd_lib::Surface;
use martensite_tkmath::bnd::BoundingBox3;
use martensite_tkmath::geom_abs::SurfaceType;
use martensite_tkmath::gp::{self, Axis3, Cone3};
use std::f64::consts::PI;

// Helper: compare two BoundingBox3 results within tolerance.
fn CompareBoxes(theNew: &BoundingBox3, theOld: &BoundingBox3, theTol: f64) {
    for (aN, aO) in get3(theNew).iter().zip(get3(theOld)) {
        EXPECT_NEAR(*aN, aO, theTol);
    }
}

// Check that every side of `theNew` lies inside `theOld`.
fn expect_no_larger(theNew: &BoundingBox3, theOld: &BoundingBox3) {
    let [aXminN, aXmaxN, aYminN, aYmaxN, aZminN, aZmaxN] = get3(theNew);
    let [aXminO, aXmaxO, aYminO, aYmaxO, aZminO, aZmaxO] = get3(theOld);
    let aTol = precision::confusion();
    assert!(aXminN >= aXminO - aTol);
    assert!(aXmaxN <= aXmaxO + aTol);
    assert!(aYminN >= aYminO - aTol);
    assert!(aYmaxN <= aYmaxO + aTol);
    assert!(aZminN >= aZminO - aTol);
    assert!(aZmaxN <= aZmaxO + aTol);
}

fn xoy() -> Axis3 {
    gp::xoy().axis3()
}

fn adaptor(theSurf: &Handle<dyn GeomSurface>) -> GeomAdaptorSurface {
    GeomAdaptorSurface::with_surface(theSurf).unwrap()
}

// Surface(handle).Add over the patch, or the natural bounds when None.
fn new_box(theSurf: &Handle<dyn GeomSurface>, thePatch: Option<[f64; 4]>) -> BoundingBox3 {
    let anEval = Surface::from_surface(theSurf).unwrap();
    let mut aBox = BoundingBox3::default();
    match thePatch {
        Some(aPatch) => anEval
            .add_range(aPatch, precision::confusion(), &mut aBox)
            .unwrap(),
        None => anEval.add(precision::confusion(), &mut aBox).unwrap(),
    }
    aBox
}

// AddSurface::Add on a GeomAdaptorSurface over the patch or the adaptor bounds.
fn old_box(theAdaptor: &dyn Adaptor3dSurface, thePatch: Option<[f64; 4]>) -> BoundingBox3 {
    let mut aBox = BoundingBox3::default();
    match thePatch {
        Some(aPatch) => {
            AddSurface::add_range(theAdaptor, aPatch, precision::confusion(), &mut aBox).unwrap()
        }
        None => AddSurface::add(theAdaptor, precision::confusion(), &mut aBox).unwrap(),
    }
    aBox
}

fn sphere(theRadius: f64) -> Handle<dyn GeomSurface> {
    surface(GeomSphericalSurface::new(xoy(), theRadius).unwrap())
}

fn torus() -> Handle<dyn GeomSurface> {
    surface(GeomToroidalSurface::new(xoy(), 10.0, 3.0).unwrap())
}

include!("geometry_bounds_surface/suites_0.rs");
include!("geometry_bounds_surface/suites_1.rs");

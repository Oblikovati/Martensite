//! Bnd lib contracts.
//! The suites are split across `analytic_bounds/suites_N.rs` in the defined order.
#![allow(non_snake_case)]
mod bnd_support;
#[path = "support/geometric_test_support/mod.rs"]
mod common;
use bnd_support::{array1, array2, curve, curve2d, dir, dir2d, pnt, pnt2d, surface};
use common::EXPECT_NEAR;
use martensite_tkernel::precision;
use martensite_tkg2d::geom2d::{
    Geom2dBezierCurve, Geom2dCircle, Geom2dEllipse, Geom2dLine, Geom2dTrimmedCurve,
};
use martensite_tkg2d::geom2d_adaptor::Geom2dAdaptorCurve;
use martensite_tkg3d::geom::{
    GeomBSplineCurve, GeomBezierCurve, GeomBezierSurface, GeomCircle, GeomConicalSurface,
    GeomCylindricalSurface, GeomEllipse, GeomLine, GeomPlane, GeomSphericalSurface,
    GeomToroidalSurface, GeomTrimmedCurve,
};
use martensite_tkg3d::geom_adaptor::{GeomAdaptorCurve, GeomAdaptorSurface};
use martensite_tkgeombase::bnd_lib::{self, Add2dCurve, Add3dCurve, AddSurface};
use martensite_tkmath::bnd::{BoundingBox2, BoundingBox3};
use martensite_tkmath::gp::{
    Axis2, Axis2d, Axis3, Circle2, Circle3, Cone3, Cylinder3, Direction3, Ellipse2, Ellipse3,
    Hyperbola2, Hyperbola3, Line2, Line3, Parabola2, Parabola3, Point3, Sphere3, Torus3,
};
use std::f64::consts::PI;

// `Axis2(P, V)`.
fn ax2(theP: Point3, theV: Direction3) -> Axis2 {
    Axis2::new(theP, theV).unwrap()
}

// `Axis3(P, V)`.
fn ax3(theP: Point3, theV: Direction3) -> Axis3 {
    Axis3::from_normal(theP, theV).unwrap()
}

// `Ax2d(P, V)`.
fn ax2d(theX: f64, theY: f64, theDx: f64, theDy: f64) -> Axis2d {
    Axis2d::new(pnt2d(theX, theY), dir2d(theDx, theDy))
}

// The retained `Get(Xmin, Ymin, Zmin, Xmax, Ymax, Zmax)`.
fn get(theBox: &BoundingBox3) -> [f64; 6] {
    theBox.get().unwrap()
}

// The retained `Get(Xmin, Ymin, Xmax, Ymax)`.
fn get2d(theBox: &BoundingBox2) -> [f64; 4] {
    theBox.get().unwrap()
}

include!("analytic_bounds/suites_0.rs");
include!("analytic_bounds/suites_1.rs");
include!("analytic_bounds/suites_2.rs");
include!("analytic_bounds/suites_3.rs");

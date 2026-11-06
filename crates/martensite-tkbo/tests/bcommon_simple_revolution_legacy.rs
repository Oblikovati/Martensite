//! Regression of a full-revolution and box common regression.
#![allow(non_snake_case)]

#[path = "bop_test_utilities/mod.rs"]
mod bop_test_utilities;

use bop_test_utilities::*;
use martensite_tkmath::gp::{Axis1, Direction3, Point3};

#[test]
fn BCommonSimpleTest__RevolvedProfileBox_I1() {
    let box_shape = CreateBox(Point3::new(-1.0, -1.0, 0.0), 2.5, 3.0, 1.0);
    let points = [
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(1.0, 0.0, 0.0),
        Point3::new(1.0, 0.0, 2.0),
        Point3::new(0.0, 0.0, 2.0),
    ];
    let profile = CreateFaceFromWire(&CreatePolygonWire(&points, true));
    let axis = Axis1::new(
        Point3::new(0.0, 0.0, 0.0),
        Direction3::new(0.0, 0.0, 1.0).unwrap(),
    );
    let revolution = CreateRevolution(&profile, axis, std::f64::consts::TAU);
    ValidateResult(
        &PerformCommon(&box_shape, &revolution),
        12.5664,
        -1.0,
        false,
    );
}

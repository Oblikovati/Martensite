//! Regressions of box and half-space common regressions.
#![allow(non_snake_case)]

#[path = "bop_test_utilities/mod.rs"]
mod bop_test_utilities;

use bop_test_utilities::*;
use martensite_tkbrep::topods::Shape;
use martensite_tkmath::gp::Point3;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeHalfSpace;

fn xy_half_space(reference_z: f64) -> Shape {
    let face = CreatePolygonFace(
        Point3::new(250.0, 250.0, 0.0),
        Point3::new(-250.0, 250.0, 0.0),
        Point3::new(-250.0, -250.0, 0.0),
        Point3::new(250.0, -250.0, 0.0),
    );
    let face = martensite_tkbrep::topods::face(&face).unwrap();
    BRepPrimAPIMakeHalfSpace::with_face(&face, Point3::new(0.0, 0.0, reference_z))
        .unwrap()
        .solid()
        .unwrap()
        .clone()
        .into_shape()
}

fn common_box_and_half_space(box_z: f64, reference_z: f64, expected_area: f64) {
    let box_shape = CreateBox(Point3::new(0.0, 0.0, box_z), 150.0, 200.0, 200.0);
    let result = PerformCommon(&box_shape, &xy_half_space(reference_z));
    ValidateResult(&result, expected_area, -1.0, false);
}

#[test]
fn BOPCommonSimpleTest__BoxHalfSpace_ZP3() {
    common_box_and_half_space(-30.0, -100.0, 81000.0);
}

#[test]
fn BOPCommonSimpleTest__BoxHalfSpace_ZP4() {
    common_box_and_half_space(-30.0, 100.0, 179000.0);
}

#[test]
fn BOPCommonSimpleTest__BoxHalfSpace_ZP5() {
    common_box_and_half_space(-80.0, -100.0, 116000.0);
}

#[test]
fn BOPCommonSimpleTest__BoxHalfSpace_ZP6() {
    common_box_and_half_space(-80.0, 100.0, 144000.0);
}

//! Regressions of cylinder overlap and disjoint common regressions.
#![allow(non_snake_case)]

#[path = "bop_test_utilities/mod.rs"]
mod bop_test_utilities;

use bop_test_utilities::*;

fn cylinder_common(
    radius: f64,
    height: f64,
    other_radius: f64,
    other_height: f64,
    z: f64,
    area: f64,
    empty: bool,
) {
    let first = CreateCylinder(radius, height);
    let second = Translate(&CreateCylinder(other_radius, other_height), 0.0, 0.0, z);
    ValidateResult(&PerformCommon(&first, &second), area, -1.0, empty);
}

#[test]
fn BCommonSimpleTest__DisjointCylinders_I2() {
    cylinder_common(1.0, 2.0, 0.5, 1.0, 2.0, -1.0, true);
}
#[test]
fn BCommonSimpleTest__UpperCylinderOverlap_I3() {
    cylinder_common(1.0, 2.0, 0.5, 1.0, 1.0, 4.71239, false);
}
#[test]
fn BCommonSimpleTest__MiddleCylinderOverlap_I4() {
    cylinder_common(1.0, 2.0, 0.5, 1.0, 0.5, 4.71239, false);
}
#[test]
fn BCommonSimpleTest__TouchingBoxes_I5() {
    let first = CreateBox(
        martensite_tkmath::gp::Point3::new(0.0, 0.0, 0.0),
        3.0,
        3.0,
        3.0,
    );
    let second = CreateBox(
        martensite_tkmath::gp::Point3::new(0.0, 3.0, 0.0),
        1.0,
        1.0,
        1.0,
    );
    ValidateResult(&PerformCommon(&first, &second), -1.0, -1.0, true);
}
#[test]
fn BCommonSimpleTest__OverlappingCylinders_J1() {
    cylinder_common(20.0, 100.0, 20.0, 100.0, 50.0, 8796.46, false);
}

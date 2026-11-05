//! Regressions of doubly rotated and translated cylinder common regressions.
#![allow(non_snake_case)]
#[path = "bop_test_utilities/mod.rs"]
mod bop_test_utilities;
use bop_test_utilities::*;
use martensite_tkmath::gp::Point3;
fn common_cylinders(second_angle: f64, expected_area: f64) {
    let first = CreateCylinder(1.0, 4.0);
    let mut second = CreateCylinder(0.5, 4.0);
    let pivot = Point3::new(0.0, 0.0, 2.0);
    RotateAroundAxis(&mut second, pivot, [1.0, 0.0, 0.0], 90.0);
    RotateAroundAxis(&mut second, pivot, [0.0, 1.0, 0.0], second_angle);
    let second = Translate(&second, 0.5, 0.0, 0.0);
    ValidateResult(&PerformCommon(&first, &second), expected_area, -1.0, false);
}
#[test]
fn BOPCommonSimpleTest__DoubleRotatedTranslatedCylinders_ZE8() {
    common_cylinders(90.0, 6.72254);
}
#[test]
fn BOPCommonSimpleTest__DoubleRotatedTranslatedCylinders_ZE9() {
    common_cylinders(180.0, 6.72239);
}
#[test]
fn BOPCommonSimpleTest__DoubleRotatedTranslatedCylinders_ZF1() {
    common_cylinders(270.0, 6.72254);
}

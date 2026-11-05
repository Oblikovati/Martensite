//! Regressions of single-rotation cylinder common regressions.
#![allow(non_snake_case)]
#[path = "bop_test_utilities/mod.rs"]
mod bop_test_utilities;
use bop_test_utilities::*;
use martensite_tkmath::gp::Point3;
fn common(radius: f64, area: f64) {
    let first = CreateCylinder(1., 4.);
    let mut second = CreateCylinder(radius, 4.);
    RotateAroundAxis(&mut second, Point3::new(0., 0., 2.), [1., 0., 0.], 90.);
    ValidateResult(&PerformCommon(&first, &second), area, -1., false);
}
#[test]
fn BOPCommonSimpleTest__RotatedCylinders_ZD8() {
    common(1., 16.);
}
#[test]
fn BOPCommonSimpleTest__RotatedCylinders_ZE3() {
    common(0.5, 7.49478);
}

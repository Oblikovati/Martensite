//! Regressions of transformed box-cylinder common regressions.
#![allow(non_snake_case)]
#[path = "bop_test_utilities/mod.rs"]
mod bop_test_utilities;
use bop_test_utilities::*;
use martensite_tkmath::gp::Point3;
fn common(angle: f64) {
    let box_shape = CreateBox(Point3::new(0., 0., 0.), 4., 4., 4.);
    let mut cylinder = Translate(&CreateCylinder(4., 4.), -2., 2., 0.);
    RotateAroundAxis(&mut cylinder, Point3::new(-2., 2., 0.), [0., 0., 1.], angle);
    ValidateResult(&PerformCommon(&box_shape, &cylinder), 59.0795, -1., false);
}
#[test]
fn BOPCommonSimpleTest__TransformedBoxCylinder_ZO7() {
    common(30.);
}
#[test]
fn BOPCommonSimpleTest__TransformedBoxCylinder_ZO8() {
    common(90.);
}

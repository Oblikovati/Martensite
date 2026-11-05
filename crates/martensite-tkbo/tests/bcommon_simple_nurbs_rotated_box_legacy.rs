//! Regressions of rotated NURBS-box common regressions.
#![allow(non_snake_case)]
#[path = "bop_test_utilities/mod.rs"]
mod bop_test_utilities;
use bop_test_utilities::*;
use martensite_tkmath::gp::Point3;
fn common(size: [f64; 3], angle: f64, area: f64) {
    let first = ConvertToNurbs(&CreateBox(Point3::new(0., 0., 0.), 1., 1., 1.));
    let mut second = CreateBox(Point3::new(0., 0., 0.), size[0], size[1], size[2]);
    RotateAroundAxis(&mut second, Point3::new(0., 0., 0.), [0., 0., 1.], angle);
    ValidateResult(&PerformCommon(&first, &second), area, -1., false);
}
#[test]
fn BCommonSimpleTest__NurbsRotatedBox_C8() {
    common([2f64.sqrt(), 2f64.sqrt() / 2., 1.], 45., 4.41421);
}
#[test]
fn BCommonSimpleTest__NurbsRotatedBox_C9() {
    common([2f64.sqrt() / 2., 2f64.sqrt() / 2., 1.], 45., 2.91421);
}
#[test]
fn BCommonSimpleTest__NurbsRotatedBox_D1() {
    common([2f64.sqrt(), 0.25, 1.], 45., 3.61764);
}
#[test]
fn BCommonSimpleTest__NurbsRotatedBox_D2() {
    common([31f64.sqrt() / 4., 0.25, 1.], 34.73, 3.65032);
}

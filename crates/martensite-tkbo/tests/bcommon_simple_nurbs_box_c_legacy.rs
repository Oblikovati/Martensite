//! Regressions of C-series NURBS-box common regressions.
#![allow(non_snake_case)]
#[path = "bop_test_utilities/mod.rs"]
mod bop_test_utilities;
use bop_test_utilities::*;
use martensite_tkmath::gp::Point3;
fn common(second: [f64; 6], area: f64, empty: bool) {
    let first = ConvertToNurbs(&CreateBox(Point3::new(0., 0., 0.), 1., 1., 1.));
    let second = CreateBox(
        Point3::new(second[0], second[1], second[2]),
        second[3],
        second[4],
        second[5],
    );
    ValidateResult(&PerformCommon(&first, &second), area, -1., empty);
}
#[test]
fn BCommonSimpleTest__NurbsBox_C1() {
    common([0.0, 0.0, 0.0, 1.5, 0.5, 0.5], 2.5, false);
}
#[test]
fn BCommonSimpleTest__NurbsBox_C2() {
    common([0.0, -0.5, 0.0, 1.5, 0.5, 0.5], -1., true);
}
#[test]
fn BCommonSimpleTest__NurbsBox_C3() {
    common([0.25, 0.0, 0.0, 0.5, 0.5, 1.0], 2.5, false);
}
#[test]
fn BCommonSimpleTest__NurbsBox_C4() {
    common([0.25, -0.5, 0.0, 0.5, 0.5, 1.0], -1., true);
}
#[test]
fn BCommonSimpleTest__NurbsBox_C5() {
    common([0.25, 0.0, 0.0, 0.5, 1.5, 1.0], 4., false);
}
#[test]
fn BCommonSimpleTest__NurbsBox_C6() {
    common([0.5, 0.0, 0.0, 1.0, 1.0, 0.5], 2.5, false);
}
#[test]
fn BCommonSimpleTest__NurbsBox_C7() {
    common([0.5, 0.0, -0.5, 1.0, 1.0, 0.5], -1., true);
}

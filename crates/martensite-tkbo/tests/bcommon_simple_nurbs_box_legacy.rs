//! Regressions of NURBS-box BRepAlgoAPI common regressions.
#![allow(non_snake_case)]
#[path = "bop_test_utilities/mod.rs"]
mod bop_test_utilities;
use bop_test_utilities::*;
use martensite_tkmath::gp::Point3;
fn common_nurbs_box(second: [f64; 6], area: f64, empty: bool) {
    let first = ConvertToNurbs(&CreateBox(Point3::new(0.0, 0.0, 0.0), 1.0, 1.0, 1.0));
    let second = CreateBox(
        Point3::new(second[0], second[1], second[2]),
        second[3],
        second[4],
        second[5],
    );
    ValidateResult(&PerformCommon(&first, &second), area, -1.0, empty);
}
#[test]
fn BCommonSimpleTest__NurbsBox_A6() {
    common_nurbs_box([0., 0., 0., 1., 1., 1.], 6., false);
}
#[test]
fn BCommonSimpleTest__NurbsBox_A7() {
    common_nurbs_box([0., 0., 0., 1., 1.5, 1.], 6., false);
}
#[test]
fn BCommonSimpleTest__NurbsBox_A8() {
    common_nurbs_box([0., 1., 0., 1., 0.5, 1.], -1., true);
}
#[test]
fn BCommonSimpleTest__NurbsBox_A9() {
    common_nurbs_box([1., 1., 0., 1., 1., 1.], -1., true);
}

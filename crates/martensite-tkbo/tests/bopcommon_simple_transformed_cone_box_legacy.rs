//! Regressions of transformed cone-box common regressions.
#![allow(non_snake_case)]
#[path = "bop_test_utilities/mod.rs"]
mod bop_test_utilities;
use bop_test_utilities::*;
use martensite_tkmath::gp::Point3;
fn common_cone_box(cone: [f64; 3], translation: [f64; 3], area: f64) {
    let box_shape = CreateBox(Point3::new(0.0, 0.0, 0.0), 4.0, 4.0, 4.0);
    let mut cone = Translate(
        &CreateCone(cone[0], cone[1], cone[2]),
        translation[0],
        translation[1],
        translation[2],
    );
    RotateAroundAxis(
        &mut cone,
        Point3::new(translation[0], translation[1], translation[2]),
        [0.0, 0.0, 1.0],
        30.0,
    );
    ValidateResult(&PerformCommon(&box_shape, &cone), area, -1.0, false);
}
#[test]
fn BOPCommonSimpleTest__TransformedConeBox_ZG4() {
    common_cone_box([3.0, 2.0, 4.0], [2.0, 0.0, 0.0], 64.3685);
}
#[test]
fn BOPCommonSimpleTest__TransformedConeBox_ZH3() {
    common_cone_box([5.0, 4.0, 4.0], [-2.0, 2.0, 0.0], 68.087);
}
#[test]
fn BOPCommonSimpleTest__TransformedConeBox_ZH4() {
    common_cone_box([5.0, 3.5, 4.0], [-2.0, 2.0, 0.0], 64.4112);
}

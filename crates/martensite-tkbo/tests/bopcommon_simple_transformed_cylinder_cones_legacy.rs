//! Regressions of transformed cylinder-cone common regressions.
#![allow(non_snake_case)]
#[path = "bop_test_utilities/mod.rs"]
mod bop_test_utilities;
use bop_test_utilities::*;
use martensite_tkmath::gp::Point3;
fn common(cone: [f64; 3], axis: [f64; 3], angle: f64, area: f64) {
    let cylinder = CreateCylinder(4., 8.);
    let mut cone = Translate(&CreateCone(cone[0], cone[1], cone[2]), 0., 0., -1.);
    RotateAroundAxis(&mut cone, Point3::new(0., 0., 4.), axis, angle);
    ValidateResult(&PerformCommon(&cylinder, &cone), area, -1., false);
}
#[test]
fn BOPCommonSimpleTest__TransformedCylinderCone_ZK5() {
    common([2., 1., 10.], [0., 1., 0.], 90., 87.8407);
}
#[test]
fn BOPCommonSimpleTest__TransformedCylinderCone_ZK6() {
    common([2., 1., 10.], [1., 0., 0.], 90., 87.8112);
}
#[test]
fn BOPCommonSimpleTest__TransformedCylinderCone_ZK7() {
    common([6., 1., 10.], [0., 1., 0.], 90., 216.554);
}
#[test]
fn BOPCommonSimpleTest__TransformedCylinderCone_ZK8() {
    common([6., 1., 10.], [1., 0., 0.], 90., 216.563);
}
#[test]
fn BOPCommonSimpleTest__TransformedCylinderCone_ZK9() {
    common([1., 6., 8.], [0., 1., 0.], 90., 228.558);
}
#[test]
fn BOPCommonSimpleTest__TransformedCylinderCone_ZL1() {
    common([1., 6., 8.], [0., 1., 0.], -90., 228.47);
}

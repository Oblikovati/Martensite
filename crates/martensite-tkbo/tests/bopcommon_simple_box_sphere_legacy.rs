//! Regression of the oriented sphere and box common regression.
#![allow(non_snake_case)]

#[path = "bop_test_utilities/mod.rs"]
mod bop_test_utilities;

use bop_test_utilities::*;
use martensite_tkmath::gp::{Axis2, Direction3, Point3};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeSphere;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;

#[test]
fn BOPCommonSimpleTest__OrientedSphereBox_ZP9() {
    let box_shape = CreateBox(Point3::new(0.0, 0.0, 0.0), 100.0, 100.0, 100.0);
    let axis = Axis2::new(
        Point3::new(98.946735014, 46.491265177, 17.092869659),
        Direction3::new(0.0, 0.0, 1.0).unwrap(),
    )
    .unwrap();
    let sphere = BRepPrimAPIMakeSphere::with_axis(axis, 7.5)
        .unwrap()
        .shape()
        .unwrap()
        .clone();
    let result = PerformCommon(&box_shape, &sphere);
    ValidateResult(&result, 576.293, -1.0, false);
}

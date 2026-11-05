//! Regressions of rectangular-profile cylinder common regressions.
#![allow(non_snake_case)]

#[path = "bop_test_utilities/mod.rs"]
mod bop_test_utilities;

use bop_test_utilities::*;
use martensite_tkmath::gp::Point3;

fn rectangle_cylinder_common(corner: Point3, x: f64, y: f64, z: f64, angle: f64, area: f64) {
    let mut prism = CreateBox(corner, x, y, z);
    RotateAroundAxis(
        &mut prism,
        Point3::new(0.0, 0.0, 0.0),
        [0.0, 0.0, 1.0],
        angle,
    );
    let result = PerformCommon(&prism, &CreateCylinder(1.0, 2.0));
    ValidateResult(&result, area, -1.0, false);
}

#[test]
fn BCommonSimpleTest__SquareCylinder_H6() {
    rectangle_cylinder_common(Point3::new(-1.0, -1.0, 0.0), 2.0, 2.0, 2.0, 0.0, 18.8496);
}
#[test]
fn BCommonSimpleTest__ShortSquareCylinder_H7() {
    rectangle_cylinder_common(Point3::new(-1.0, -1.0, 0.0), 2.0, 2.0, 1.0, 0.0, 12.5664);
}
#[test]
fn BCommonSimpleTest__HalfCylinder_H8() {
    rectangle_cylinder_common(Point3::new(-1.0, -1.0, 0.0), 3.0, 1.0, 1.0, 0.0, 8.28319);
}

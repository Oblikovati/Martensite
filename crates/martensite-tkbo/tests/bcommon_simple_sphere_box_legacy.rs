//! Regressions of sphere-box BRepAlgoAPI common regressions.
#![allow(non_snake_case)]
#[path = "bop_test_utilities/mod.rs"]
mod bop_test_utilities;
use bop_test_utilities::*;
use martensite_tkmath::gp::Point3;

fn sphere_box_common(rotate_sphere: bool, rotate_box: bool, box_first: bool) {
    let mut sphere = CreateSphere(Point3::new(0.0, 0.0, 0.0), 1.0);
    let mut box_shape = CreateBox(Point3::new(0.0, 0.0, 0.0), 1.0, 1.0, 1.0);
    if rotate_sphere {
        RotateAroundAxis(
            &mut sphere,
            Point3::new(0.0, 0.0, 0.0),
            [0.0, 0.0, 1.0],
            -90.0,
        );
        RotateAroundAxis(
            &mut sphere,
            Point3::new(0.0, 0.0, 0.0),
            [0.0, 1.0, 0.0],
            -45.0,
        );
    }
    if rotate_box {
        RotateAroundAxis(
            &mut box_shape,
            Point3::new(0.0, 0.0, 1.0),
            [0.0, 1.0, 0.0],
            90.0,
        );
    }
    let result = if box_first {
        PerformCommon(&box_shape, &sphere)
    } else {
        PerformCommon(&sphere, &box_shape)
    };
    ValidateResult(&result, 3.92699, -1.0, false);
}
#[test]
fn BCommonSimpleTest__SphereBox_A1() {
    sphere_box_common(false, false, false);
}
#[test]
fn BCommonSimpleTest__RotatedSphereBox_A2() {
    sphere_box_common(true, false, false);
}
#[test]
fn BCommonSimpleTest__BoxRotatedSphere_A3() {
    sphere_box_common(true, false, true);
}
#[test]
fn BCommonSimpleTest__SphereRotatedBox_A4() {
    sphere_box_common(false, true, false);
}
#[test]
fn BCommonSimpleTest__RotatedBoxSphere_A5() {
    sphere_box_common(false, true, true);
}

#[test]
fn BCommonSimpleTest__SphereBox_D3() {
    sphere_box_common(false, false, false);
}
#[test]
fn BCommonSimpleTest__BoxSphere_D4() {
    sphere_box_common(false, false, true);
}
#[test]
fn BCommonSimpleTest__RotatedSphereBox_D5() {
    sphere_box_common(true, false, false);
}
#[test]
fn BCommonSimpleTest__BoxRotatedSphere_D6() {
    sphere_box_common(true, false, true);
}
#[test]
fn BCommonSimpleTest__SphereRotatedBox_D7() {
    sphere_box_common(false, true, false);
}
#[test]
fn BCommonSimpleTest__RotatedBoxSphere_D8() {
    sphere_box_common(false, true, true);
}

//! Regressions of later cylinder and box cut regressions.
#![allow(non_snake_case)]
#[path = "bop_test_utilities/mod.rs"]
mod bop_test_utilities;
use bop_test_utilities::*;
use martensite_tkbo::bop_algo::Operation;
use martensite_tkmath::gp::Point3;
#[test]
fn BOPCutSimpleTest__CylinderSphere_ZH5() {
    let first = CreateCylinder(4.0, 8.0);
    let second = Translate(
        &CreateSphere(Point3::new(0.0, 0.0, 0.0), 4.0),
        0.0,
        0.0,
        8.0,
    );
    ValidateResult(
        &PerformDirectBOP(&first, &second, Operation::Cut),
        351.858,
        -1.0,
        false,
    );
}
#[test]
fn BOPCutSimpleTest__CylinderSphere_ZH6() {
    let first = CreateCylinder(4.0, 8.0);
    let mut second = Translate(
        &CreateSphere(Point3::new(0.0, 0.0, 0.0), 4.0),
        0.0,
        0.0,
        8.0,
    );
    RotateAroundAxis(
        &mut second,
        Point3::new(0.0, 0.0, 8.0),
        [0.0, 0.0, 1.0],
        90.0,
    );
    ValidateResult(
        &PerformDirectBOP(&first, &second, Operation::Cut),
        351.858,
        -1.0,
        false,
    );
}
#[test]
fn BOPCutSimpleTest__CylinderSphere_ZH7() {
    let first = CreateCylinder(4.0, 8.0);
    let mut second = Translate(
        &CreateSphere(Point3::new(0.0, 0.0, 0.0), 4.0),
        0.0,
        0.0,
        8.0,
    );
    RotateAroundAxis(
        &mut second,
        Point3::new(0.0, 0.0, 8.0),
        [0.0, 0.0, 1.0],
        180.0,
    );
    ValidateResult(
        &PerformDirectBOP(&first, &second, Operation::Cut),
        351.858,
        -1.0,
        false,
    );
}
#[test]
fn BOPCutSimpleTest__CylinderSphere_ZH8() {
    let first = CreateCylinder(4.0, 8.0);
    let mut second = Translate(
        &CreateSphere(Point3::new(0.0, 0.0, 0.0), 4.0),
        0.0,
        0.0,
        8.0,
    );
    RotateAroundAxis(
        &mut second,
        Point3::new(0.0, 0.0, 8.0),
        [0.0, 0.0, 1.0],
        270.0,
    );
    ValidateResult(
        &PerformDirectBOP(&first, &second, Operation::Cut),
        351.858,
        -1.0,
        false,
    );
}
#[test]
fn BOPCutSimpleTest__CylinderSphere_ZH9() {
    let first = CreateCylinder(4.0, 8.0);
    let mut second = Translate(
        &CreateSphere(Point3::new(0.0, 0.0, 0.0), 4.0),
        0.0,
        0.0,
        8.0,
    );
    RotateAroundAxis(
        &mut second,
        Point3::new(0.0, 0.0, 8.0),
        [1.0, 0.0, 0.0],
        90.0,
    );
    ValidateResult(
        &PerformDirectBOP(&first, &second, Operation::Cut),
        351.858,
        -1.0,
        false,
    );
}
#[test]
fn BOPCutSimpleTest__CylinderSphere_ZI1() {
    let first = CreateCylinder(4.0, 8.0);
    let mut second = Translate(
        &CreateSphere(Point3::new(0.0, 0.0, 0.0), 4.0),
        0.0,
        0.0,
        8.0,
    );
    RotateAroundAxis(
        &mut second,
        Point3::new(0.0, 0.0, 8.0),
        [1.0, 0.0, 0.0],
        90.0,
    );
    RotateAroundAxis(
        &mut second,
        Point3::new(0.0, 0.0, 8.0),
        [0.0, 0.0, 1.0],
        90.0,
    );
    ValidateResult(
        &PerformDirectBOP(&first, &second, Operation::Cut),
        351.858,
        -1.0,
        false,
    );
}
#[test]
fn BOPCutSimpleTest__CylinderSphere_ZI2() {
    let first = CreateCylinder(4.0, 8.0);
    let mut second = Translate(
        &CreateSphere(Point3::new(0.0, 0.0, 0.0), 4.0),
        0.0,
        0.0,
        8.0,
    );
    RotateAroundAxis(
        &mut second,
        Point3::new(0.0, 0.0, 8.0),
        [1.0, 0.0, 0.0],
        90.0,
    );
    RotateAroundAxis(
        &mut second,
        Point3::new(0.0, 0.0, 8.0),
        [0.0, 0.0, 1.0],
        180.0,
    );
    ValidateResult(
        &PerformDirectBOP(&first, &second, Operation::Cut),
        351.858,
        -1.0,
        false,
    );
}

fn cylinder_sphere_cut(radius: f64, rotations: &[([f64; 3], f64)], area: f64) {
    let first = CreateCylinder(4.0, 8.0);
    let mut second = Translate(
        &CreateSphere(Point3::new(0.0, 0.0, 0.0), radius),
        0.0,
        0.0,
        8.0,
    );
    for (axis, angle) in rotations {
        RotateAroundAxis(&mut second, Point3::new(0.0, 0.0, 8.0), *axis, *angle);
    }
    ValidateResult(
        &PerformDirectBOP(&first, &second, Operation::Cut),
        area,
        -1.0,
        false,
    );
}

#[test]
fn BOPCutSimpleTest__CylinderSphere_ZI3() {
    cylinder_sphere_cut(
        4.0,
        &[([1.0, 0.0, 0.0], 90.0), ([0.0, 0.0, 1.0], 270.0)],
        351.858,
    );
}
#[test]
fn BOPCutSimpleTest__CylinderSphere_ZI4() {
    cylinder_sphere_cut(2.0, &[([1.0, 0.0, 0.0], 90.0)], 314.159);
}
#[test]
fn BOPCutSimpleTest__CylinderSphere_ZI5() {
    cylinder_sphere_cut(
        2.0,
        &[([1.0, 0.0, 0.0], 90.0), ([0.0, 1.0, 0.0], 90.0)],
        314.159,
    );
}
#[test]
fn BOPCutSimpleTest__CylinderSphere_ZI6() {
    cylinder_sphere_cut(
        2.0,
        &[([1.0, 0.0, 0.0], 90.0), ([0.0, 1.0, 0.0], 180.0)],
        314.159,
    );
}
#[test]
fn BOPCutSimpleTest__CylinderSphere_ZI7() {
    cylinder_sphere_cut(
        2.0,
        &[([1.0, 0.0, 0.0], 90.0), ([0.0, 1.0, 0.0], 270.0)],
        314.159,
    );
}
#[test]
fn BOPCutSimpleTest__CylinderSphere_ZI8() {
    cylinder_sphere_cut(6.0, &[], 196.529);
}
#[test]
fn BOPCutSimpleTest__CylinderSphere_ZI9() {
    cylinder_sphere_cut(6.0, &[([0.0, 0.0, 1.0], 90.0)], 196.529);
}
#[test]
fn BOPCutSimpleTest__CylinderSphere_ZJ1() {
    cylinder_sphere_cut(6.0, &[([0.0, 0.0, 1.0], 180.0)], 196.529);
}
#[test]
fn BOPCutSimpleTest__CylinderSphere_ZJ2() {
    cylinder_sphere_cut(6.0, &[([0.0, 0.0, 1.0], 270.0)], 196.529);
}
#[test]
fn BOPCutSimpleTest__CylinderSphere_ZJ3() {
    cylinder_sphere_cut(6.0, &[([0.0, 1.0, 0.0], 90.0)], 196.529);
}

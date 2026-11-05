//! Regressions of remaining transformed primitive common regressions.
#![allow(non_snake_case)]

#[path = "bop_test_utilities/mod.rs"]
mod bop_test_utilities;

use bop_test_utilities::*;
use martensite_tkbrep::topods::Shape;
use martensite_tkmath::gp::Point3;

fn rotated_common(
    first: Shape,
    mut second: Shape,
    rotations: &[(Point3, [f64; 3], f64)],
    translation: [f64; 3],
    expected_area: f64,
) {
    for (pivot, axis, angle) in rotations {
        RotateAroundAxis(&mut second, *pivot, *axis, *angle);
    }
    let translated = Translate(&second, translation[0], translation[1], translation[2]);
    let result = PerformCommon(&first, &translated);
    ValidateResult(&result, expected_area, -1.0, false);
}

#[test]
fn BOPCommonSimpleTest__RotatedOffsetBoxes_Q7() {
    let first = CreateBox(Point3::new(0.0, 0.0, 0.0), 1.0, 1.0, 1.0);
    let second = CreateBox(Point3::new(-0.25, -0.25, 0.25), 0.25, 1.0, 0.5);
    rotated_common(
        first,
        second,
        &[(Point3::new(0.0, 0.0, 0.0), [0.0, 0.0, 1.0], -30.0)],
        [0.0; 3],
        1.17524,
    );
}

#[test]
fn BOPCommonSimpleTest__RotatedTranslatedBoxes_Q9() {
    let first = CreateBox(Point3::new(0.0, 0.0, 0.0), 3.0, 3.0, 3.0);
    let second = CreateBox(Point3::new(0.0, 0.0, 0.0), 1.0, 1.0, 1.0);
    rotated_common(
        first,
        second,
        &[(Point3::new(0.0, 0.0, 0.0), [1.0, 0.0, 0.0], -30.0)],
        [1.0, 1.0, 3.0],
        3.3094,
    );
}

#[test]
fn BOPCommonSimpleTest__RotatedCylinderBox_ZC5() {
    let first = CreateCylinder(1.0, 2.0);
    let second = CreateBox(Point3::new(0.0, 0.0, -0.2), 1.4, 1.4, 1.4);
    rotated_common(
        first,
        second,
        &[(Point3::new(0.0, 0.0, -0.2), [-1.0, 1.0, 0.0], -45.0)],
        [0.0; 3],
        10.8874,
    );
}

#[test]
fn BOPCommonSimpleTest__NestedCylinders_ZD2() {
    let result = PerformCommon(&CreateCylinder(1.0, 2.0), &CreateCylinder(0.5, 2.0));
    ValidateResult(&result, 7.85398, -1.0, false);
}

#[test]
fn BOPCommonSimpleTest__TripleRotatedCylinders_ZE2() {
    let first = CreateCylinder(1.0, 4.0);
    let second = CreateCylinder(1.0, 4.0);
    let rotations = [
        (Point3::new(0.0, 0.0, 2.0), [1.0, 0.0, 0.0], 90.0),
        (Point3::new(0.0, 0.0, 2.0), [0.0, 1.0, 0.0], 60.0),
        (Point3::new(0.0, 0.0, 2.0), [0.0, 0.0, 1.0], 120.0),
    ];
    rotated_common(first, second, &rotations, [0.0; 3], 16.0);
}

#[test]
fn BOPCommonSimpleTest__RotatedTranslatedCylinders_ZE7() {
    let first = CreateCylinder(1.0, 4.0);
    let second = CreateCylinder(0.5, 4.0);
    rotated_common(
        first,
        second,
        &[(Point3::new(0.0, 0.0, 2.0), [1.0, 0.0, 0.0], 90.0)],
        [0.5, 0.0, 0.0],
        6.72264,
    );
}

fn double_rotated_cylinder(angle: f64) -> Shape {
    let mut cylinder = Translate(&CreateCylinder(1.0, 4.0), 1.0, 0.0, 0.0);
    RotateAroundAxis(
        &mut cylinder,
        Point3::new(1.0, 0.0, 0.0),
        [0.0, 0.0, 1.0],
        angle,
    );
    RotateAroundAxis(
        &mut cylinder,
        Point3::new(0.0, 0.0, 0.0),
        [0.0, 0.0, 1.0],
        60.0,
    );
    cylinder
}

#[test]
fn BOPCommonSimpleTest__DoubleRotatedCylinders_ZF2() {
    let result = PerformCommon(&CreateCylinder(1.0, 4.0), &double_rotated_cylinder(-120.0));
    ValidateResult(&result, 19.2119, -1.0, false);
}

#[test]
fn BOPCommonSimpleTest__DoubleRotatedCylinders_ZF3() {
    let result = PerformCommon(&CreateCylinder(1.0, 4.0), &double_rotated_cylinder(120.0));
    ValidateResult(&result, 19.2119, -1.0, false);
}

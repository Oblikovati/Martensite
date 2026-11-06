//! Regressions of later cylinder and box cut regressions.
#![allow(non_snake_case)]
#[path = "bop_test_utilities/mod.rs"]
mod bop_test_utilities;
use bop_test_utilities::*;
use martensite_tkbo::bop_algo::Operation;
use martensite_tkmath::gp::Point3;
fn cut_box(second: [f64; 6], area: f64) {
    let first = CreateCylinder(1.0, 2.0);
    let second = CreateBox(
        Point3::new(second[0], second[1], second[2]),
        second[3],
        second[4],
        second[5],
    );
    ValidateResult(
        &PerformDirectBOP(&first, &second, Operation::Cut),
        area,
        -1.0,
        false,
    );
}
fn cut_rotated(second: [f64; 6], angle: f64, area: f64) {
    let first = CreateCylinder(1.0, 2.0);
    let mut second = CreateBox(
        Point3::new(second[0], second[1], second[2]),
        second[3],
        second[4],
        second[5],
    );
    RotateAroundAxis(
        &mut second,
        Point3::new(0.0, 0.0, 0.0),
        [0.0, 0.0, 1.0],
        angle,
    );
    ValidateResult(
        &PerformDirectBOP(&first, &second, Operation::Cut),
        area,
        -1.0,
        false,
    );
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_Z7() {
    let r = 3_f64.sqrt() / 2.0;
    cut_rotated([-0.5, -r, 0.0, 1.4, 2.0 * r, 1.0], 60.0, 22.261);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_Z8() {
    let r = 2_f64.sqrt() / 2.0;
    cut_box([-r, -r, 0.0, 1.6, 2.0 * r, 1.0], 23.3561);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_ZA1() {
    let r = 2_f64.sqrt() / 2.0;
    cut_box([-r, -r, 0.0, 1.6, 1.6, 2.0], 20.6804);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_ZA2() {
    let r = 3_f64.sqrt() / 2.0;
    cut_rotated([-0.5, -r, 0.0, 1.45, 1.8, 2.0], 60.0, 18.6723);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_ZA3() {
    let r = 2_f64.sqrt() / 2.0;
    cut_rotated([-r, -r, 0.0, 1.6, 1.6, 2.0], 135.0, 20.6804);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_ZA4() {
    let r = 3_f64.sqrt() / 2.0;
    cut_rotated([-r, -r, 0.0, 2.0 * r, 2.0 * r, 2.0], 60.0, 17.1023);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_ZA5() {
    cut_box([-0.8, -0.8, 0.0, 1.6, 1.6, 2.0], 21.204);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_ZA6() {
    cut_box([0.0, -0.5, 0.0, 2.0, 1.0, 2.0], 20.306);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_ZA7() {
    cut_rotated([0.0, -0.5, 0.0, 2.0, 1.0, 2.0], 30.0, 20.306);
}

#[test]
fn BOPCutSimpleTest__CylinderBoxes_Z9() {
    let first = CreateBox(Point3::new(0.0, 0.0, 0.0), 4.0, 4.0, 4.0);
    let second = Translate(&CreateCylinder(2.0, 6.0), 0.0, 0.0, -2.0);
    ValidateResult(
        &PerformDirectBOP(&first, &second, Operation::Cut),
        86.2832,
        -1.0,
        false,
    );
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_ZA8() {
    cut_box([0.0, 0.0, 0.0, 2.0, 2.0, 2.0], 18.1372);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_ZA9() {
    cut_box([-2.0, -2.0, 0.0, 2.0, 2.0, 2.0], 18.1372);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_ZB1() {
    let r = 3_f64.sqrt() / 2.0;
    cut_box([0.0, 0.0, 0.0, r, r, 2.0], 21.8768);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_ZB3() {
    let r = 2_f64.sqrt() / 2.0;
    cut_box([-r, -r, 0.0, 1.6, 1.6, 1.0], 22.2057);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_ZB7() {
    cut_box([-0.8, -0.8, 0.0, 1.6, 1.6, 1.0], 22.5144);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_ZB8() {
    cut_box([0.0, -0.5, 0.0, 2.0, 1.0, 1.0], 20.5344);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_ZB2() {
    let r = 3_f64.sqrt() / 2.0;
    cut_rotated([0.0, 0.0, 0.0, r, r, 2.0], -30.0, 21.8768);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_ZB4() {
    let r = 3_f64.sqrt() / 2.0;
    cut_rotated([-0.5, -r, 0.0, 1.45, 1.8, 1.0], 60.0, 21.1451);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_ZB5() {
    let r = 2_f64.sqrt() / 2.0;
    cut_rotated([-r, -r, 0.0, 1.6, 1.6, 1.0], 135.0, 22.2057);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_ZB6() {
    let r = 3_f64.sqrt() / 2.0;
    cut_rotated([-r, -r, 0.0, 2.0 * r, 2.0 * r, 1.0], 60.0, 20.7552);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_ZB9() {
    cut_rotated([0.0, -0.5, 0.0, 2.0, 1.0, 1.0], 30.0, 20.5344);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_ZC1() {
    cut_box([0.0, 0.0, 0.0, 2.0, 2.0, 1.0], 19.2788);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_ZC2() {
    cut_box([-2.0, -2.0, 0.0, 2.0, 2.0, 1.0], 19.2788);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_ZC3() {
    let r = 3_f64.sqrt() / 2.0;
    cut_box([0.0, 0.0, 0.0, r, r, 1.0], 21.058);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_ZC4() {
    let r = 3_f64.sqrt() / 2.0;
    cut_rotated([0.0, 0.0, 0.0, r, r, 1.0], -30.0, 21.058);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_ZC6() {
    cut_box([-1.0, -2.0, 0.5, 2.0, 2.0, 1.0], 20.8496);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_ZC7() {
    let first = CreateBox(Point3::new(-1.0, 0.0, 1.0), 2.0, 2.0, 2.0);
    let second = CreateCylinder(1.0, 4.0);
    ValidateResult(
        &PerformDirectBOP(&first, &second, Operation::Cut),
        23.1416,
        -1.0,
        false,
    );
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_ZC8() {
    let second = Translate(&CreateCylinder(0.5, 1.0), 0.0, 0.0, 2.0);
    ValidateResult(
        &PerformDirectBOP(&CreateCylinder(1.0, 2.0), &second, Operation::Cut),
        18.8496,
        -1.0,
        false,
    );
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_ZC9() {
    let second = Translate(&CreateCylinder(0.5, 1.0), 0.0, 0.0, 1.0);
    ValidateResult(
        &PerformDirectBOP(&CreateCylinder(1.0, 2.0), &second, Operation::Cut),
        21.9911,
        -1.0,
        false,
    );
}

#[test]
fn BOPCutSimpleTest__CylinderBoxes_ZC5() {
    let first = CreateCylinder(1.0, 2.0);
    let mut second = CreateBox(Point3::new(0.0, 0.0, -0.2), 1.4, 1.4, 1.4);
    RotateAroundAxis(
        &mut second,
        Point3::new(0.0, 0.0, -0.2),
        [-1.0, 1.0, 0.0],
        -45.0,
    );
    ValidateResult(
        &PerformDirectBOP(&first, &second, Operation::Cut),
        27.1316,
        -1.0,
        false,
    );
}
fn cylinder_cut(first: [f64; 2], second: [f64; 2], z: f64, area: f64) {
    let second = Translate(&CreateCylinder(second[0], second[1]), 0.0, 0.0, z);
    ValidateResult(
        &PerformDirectBOP(&CreateCylinder(first[0], first[1]), &second, Operation::Cut),
        area,
        -1.0,
        false,
    );
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_ZD1() {
    cylinder_cut([1.0, 2.0], [0.5, 1.0], 0.5, 23.5619);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_ZD2() {
    cylinder_cut([1.0, 2.0], [0.5, 2.0], 0.0, 23.5619);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_ZD3() {
    cylinder_cut([1.0, 2.0], [0.5, 3.0], -1.0, 23.5619);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_ZD4() {
    cylinder_cut([1.0, 2.0], [1.0, 2.0], 2.0, 18.8496);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_ZD5() {
    cylinder_cut([1.0, 2.0], [1.0, 2.0], 2.0, 18.8496);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_ZD6() {
    cylinder_cut([1.0, 2.0], [1.0, 2.0], 2.0, 18.8496);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_ZD7() {
    cylinder_cut([1.0, 2.0], [1.0, 2.0], 2.0, 18.8496);
}
fn crossed(angle: f64, area: f64) {
    let first = CreateCylinder(1.0, 4.0);
    let mut second = CreateCylinder(1.0, 4.0);
    RotateAroundAxis(
        &mut second,
        Point3::new(0.0, 0.0, 2.0),
        [1.0, 0.0, 0.0],
        90.0,
    );
    RotateAroundAxis(
        &mut second,
        Point3::new(0.0, 0.0, 2.0),
        [0.0, 1.0, 0.0],
        angle,
    );
    ValidateResult(
        &PerformDirectBOP(&first, &second, Operation::Cut),
        area,
        -1.0,
        false,
    );
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_ZD8() {
    crossed(0.0, 31.4159);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_ZD9() {
    crossed(60.0, 31.4159);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_ZE1() {
    crossed(180.0, 31.4159);
}
fn crossed_small(second_y_angle: f64, shift: f64, area: f64) {
    let first = CreateCylinder(1.0, 4.0);
    let mut second = CreateCylinder(0.5, 4.0);
    RotateAroundAxis(
        &mut second,
        Point3::new(0.0, 0.0, 2.0),
        [1.0, 0.0, 0.0],
        90.0,
    );
    RotateAroundAxis(
        &mut second,
        Point3::new(0.0, 0.0, 2.0),
        [0.0, 1.0, 0.0],
        second_y_angle,
    );
    let second = Translate(&second, shift, 0.0, 0.0);
    ValidateResult(
        &PerformDirectBOP(&first, &second, Operation::Cut),
        area,
        -1.0,
        false,
    );
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_ZE3() {
    crossed_small(0.0, 0.0, 35.6608);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_ZE4() {
    crossed_small(90.0, 0.0, 35.6613);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_ZE5() {
    crossed_small(180.0, 0.0, 35.6602);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_ZE6() {
    crossed_small(270.0, 0.0, 35.6601);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_ZE7() {
    crossed_small(0.0, 0.5, 33.8759);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_ZE8() {
    crossed_small(90.0, 0.5, 33.8756);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_ZE9() {
    crossed_small(180.0, 0.5, 33.8757);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_ZF1() {
    crossed_small(270.0, 0.5, 33.8758);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_ZE2() {
    let first = CreateCylinder(1.0, 4.0);
    let mut second = CreateCylinder(1.0, 4.0);
    for (axis, angle) in [
        ([1.0, 0.0, 0.0], 90.0),
        ([0.0, 1.0, 0.0], 60.0),
        ([0.0, 0.0, 1.0], 120.0),
    ] {
        RotateAroundAxis(&mut second, Point3::new(0.0, 0.0, 2.0), axis, angle);
    }
    ValidateResult(
        &PerformDirectBOP(&first, &second, Operation::Cut),
        31.4159,
        -1.0,
        false,
    );
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_ZF2() {
    let first = CreateCylinder(1.0, 4.0);
    let second = Translate(&CreateCylinder(1.0, 4.0), 1.0, 0.0, 0.0);
    let mut second = second;
    RotateAroundAxis(
        &mut second,
        Point3::new(1.0, 0.0, 0.0),
        [0.0, 0.0, 1.0],
        -120.0,
    );
    RotateAroundAxis(
        &mut second,
        Point3::new(0.0, 0.0, 0.0),
        [0.0, 0.0, 1.0],
        60.0,
    );
    ValidateResult(
        &PerformDirectBOP(&first, &second, Operation::Cut),
        28.9592,
        -1.0,
        false,
    );
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_ZF3() {
    let first = CreateCylinder(1.0, 4.0);
    let mut second = Translate(&CreateCylinder(1.0, 4.0), 1.0, 0.0, 0.0);
    RotateAroundAxis(
        &mut second,
        Point3::new(1.0, 0.0, 0.0),
        [0.0, 0.0, 1.0],
        120.0,
    );
    RotateAroundAxis(
        &mut second,
        Point3::new(0.0, 0.0, 0.0),
        [0.0, 0.0, 1.0],
        60.0,
    );
    ValidateResult(
        &PerformDirectBOP(&first, &second, Operation::Cut),
        28.9592,
        -1.0,
        false,
    );
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_ZF4() {
    let second = Translate(&CreateCylinder(1.0, 4.0), 1.0, 0.0, 0.0);
    ValidateResult(
        &PerformDirectBOP(&CreateCylinder(1.0, 4.0), &second, Operation::Cut),
        28.9592,
        -1.0,
        false,
    );
}

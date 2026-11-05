//! Regressions of cylinder and box cut regressions.
#![allow(non_snake_case)]

#[path = "bop_test_utilities/mod.rs"]
mod bop_test_utilities;

use bop_test_utilities::*;
use martensite_tkbo::bop_algo::Operation;
use martensite_tkmath::gp::Point3;

fn cylinder_cut(second: [f64; 6], area: f64, empty: bool) {
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
        empty,
    );
}

#[test]
fn BOPCutSimpleTest__CylinderBoxes_R1() {
    cylinder_cut([-1.0, -1.0, 0.0, 2.0, 2.0, 2.0], -1.0, true);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_R3() {
    cylinder_cut([-1.0, -1.0, 0.0, 2.0, 1.5, 2.0], 8.88126, false);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_R5() {
    cylinder_cut([-1.0, -1.0, 0.0, 1.5, 2.0, 2.0], 8.88126, false);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_R6() {
    cylinder_cut([-1.0, -1.0, 0.0, 2.0, 2.5, 2.0], -1.0, true);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_R7() {
    cylinder_cut([-1.0, -1.0, 0.0, 2.5, 2.0, 2.0], -1.0, true);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_R8() {
    cylinder_cut([-1.0, -1.0, 0.0, 1.0, 2.0, 2.0], 13.4248, false);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_R9() {
    cylinder_cut([0.0, -1.0, 0.0, 1.0, 2.0, 2.0], 13.4248, false);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_S1() {
    cylinder_cut([-1.0, -1.0, 0.0, 2.0, 1.0, 2.0], 13.4248, false);
}

#[test]
fn BOPCutSimpleTest__CylinderBoxes_R2() {
    let first = CreateCylinder(1.0, 2.0);
    let mut second = CreateBox(Point3::new(-1.0, -1.0, 0.0), 2.0, 2.0, 2.0);
    RotateAroundAxis(
        &mut second,
        Point3::new(0.0, 0.0, 0.0),
        [0.0, 0.0, 1.0],
        45.0,
    );
    ValidateResult(
        &PerformDirectBOP(&first, &second, Operation::Cut),
        -1.0,
        -1.0,
        true,
    );
}

#[test]
fn BOPCutSimpleTest__CylinderBoxes_R4() {
    let first = CreateCylinder(1.0, 2.0);
    let mut second = CreateBox(
        Point3::new(-1.0, -1.0, 0.0),
        1.0 + 3_f64.sqrt() / 2.0,
        2.0,
        2.0,
    );
    RotateAroundAxis(
        &mut second,
        Point3::new(0.0, 0.0, 0.0),
        [0.0, 0.0, 1.0],
        60.0,
    );
    ValidateResult(
        &PerformDirectBOP(&first, &second, Operation::Cut),
        4.27557,
        -1.0,
        false,
    );
}

#[test]
fn BOPCutSimpleTest__CylinderBoxes_S2() {
    let first = CreateCylinder(1.0, 2.0);
    let side = 1.0 + 2_f64.sqrt() / 2.0;
    let mut second = CreateBox(Point3::new(-1.0, -1.0, 0.0), side, side, 2.0);
    RotateAroundAxis(
        &mut second,
        Point3::new(0.0, 0.0, 0.0),
        [0.0, 0.0, 1.0],
        45.0,
    );
    ValidateResult(
        &PerformDirectBOP(&first, &second, Operation::Cut),
        13.0816,
        -1.0,
        false,
    );
}

#[test]
fn BOPCutSimpleTest__CylinderBoxes_S3() {
    let first = CreateCylinder(1.0, 2.0);
    let side = 1.0 + 2_f64.sqrt() / 2.0;
    let mut second = CreateBox(Point3::new(-1.0, -1.0, 0.0), side, side, 2.0);
    RotateAroundAxis(
        &mut second,
        Point3::new(0.0, 0.0, 0.0),
        [0.0, 0.0, 1.0],
        -45.0,
    );
    ValidateResult(
        &PerformDirectBOP(&first, &second, Operation::Cut),
        13.0816,
        -1.0,
        false,
    );
}

#[test]
fn BOPCutSimpleTest__CylinderBoxes_S4() {
    cylinder_cut(
        [
            -1.0,
            -1.0,
            0.0,
            1.0 + 2_f64.sqrt() / 2.0,
            1.0 + 2_f64.sqrt() / 2.0,
            2.0,
        ],
        13.0816,
        false,
    );
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_S5() {
    let r = 2_f64.sqrt() / 2.0;
    cylinder_cut([-r, -r, 0.0, 1.0 + r, 1.0 + r, 2.0], 13.0816, false);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_S6() {
    cylinder_cut([-0.8, -0.8, 0.0, 1.8, 1.8, 2.0], 10.602, false);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_S7() {
    cylinder_cut([-1.0, -0.8, 0.0, 1.8, 1.8, 2.0], 10.602, false);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_S9() {
    cylinder_cut([-1.0, -0.8, 0.0, 2.0, 1.6, 2.0], 10.602, false);
}

#[test]
fn BOPCutSimpleTest__CylinderBoxes_S8() {
    let first = CreateCylinder(1.0, 2.0);
    let side = 1.0 + 3_f64.sqrt() / 2.0;
    let mut second = CreateBox(Point3::new(-1.0, -3_f64.sqrt() / 2.0, 0.0), side, side, 2.0);
    RotateAroundAxis(
        &mut second,
        Point3::new(0.0, 0.0, 0.0),
        [0.0, 0.0, 1.0],
        30.0,
    );
    ValidateResult(
        &PerformDirectBOP(&first, &second, Operation::Cut),
        8.55113,
        -1.0,
        false,
    );
}

#[test]
fn BOPCutSimpleTest__CylinderBoxes_T1() {
    cylinder_cut([-0.8, -1.0, 0.0, 1.6, 2.0, 2.0], 10.602, false);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_T3() {
    cylinder_cut([-0.5, -1.0, 0.0, 1.5, 1.5, 2.0], 15.0937, false);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_T5() {
    cylinder_cut([-1.0, -1.0, 0.0, 1.5, 1.5, 2.0], 15.0937, false);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_T6() {
    let r = 2_f64.sqrt() / 2.0;
    cylinder_cut([-r, -r, 0.0, 1.0 + r, 2.0 * r, 2.0], 19.6224, false);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_T9() {
    let r = 2_f64.sqrt() / 2.0;
    cylinder_cut([-r, -r, 0.0, 2.0 * r, r + 1.0, 2.0], 19.6224, false);
}

fn rotated_cylinder_cut(second: [f64; 6], angle: f64, area: f64) {
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
fn BOPCutSimpleTest__CylinderBoxes_T2() {
    let r = 3_f64.sqrt() / 2.0;
    rotated_cylinder_cut([-r, -1.0, 0.0, 2.0 * r, 2.0, 2.0], 30.0, 8.55113);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_T4() {
    rotated_cylinder_cut([-0.5, -1.0, 0.0, 1.5, 1.5, 2.0], 30.0, 15.0937);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_T7() {
    let r = 2_f64.sqrt() / 2.0;
    rotated_cylinder_cut([-r, -r, 0.0, 1.0 + r, 2.0 * r, 2.0], 135.0, 19.6224);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_T8() {
    let r = 3_f64.sqrt() / 2.0;
    rotated_cylinder_cut([-0.5, -r, 0.0, 1.5, 2.0 * r, 2.0], 60.0, 17.4324);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_U1() {
    let r = 3_f64.sqrt() / 2.0;
    rotated_cylinder_cut([-r, -1.0, 0.0, 2.0 * r, 1.0 + r, 2.0], 30.0, 12.8267);
}

#[test]
fn BOPCutSimpleTest__CylinderBoxes_U3() {
    cylinder_cut([-0.8, -1.0, 0.0, 1.6, 1.8, 2.0], 15.903, false);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_U4() {
    cylinder_cut([-1.0, -1.1, 0.0, 1.0, 2.2, 2.0], 13.4248, false);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_U5() {
    cylinder_cut([0.0, -1.1, 0.0, 1.0, 2.2, 2.0], 13.4248, false);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_U6() {
    cylinder_cut([-1.1, -1.0, 0.0, 2.2, 1.0, 2.0], 13.4248, false);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_U7() {
    cylinder_cut([-1.0, 1.0, 0.0, 2.0, 2.0, 2.0], 18.8496, false);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_U8() {
    cylinder_cut([1.0, -1.0, 0.0, 2.0, 2.0, 2.0], 18.8496, false);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_V1() {
    let r = 2_f64.sqrt() / 2.0;
    cylinder_cut([-r, -r, 0.0, 2.0 * r, 2.0 * r, 2.0], 26.1633, false);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_U2() {
    rotated_cylinder_cut([-0.8, -1.0, 0.0, 1.6, 1.8, 2.0], 90.0, 15.903);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_U9() {
    let r = 2_f64.sqrt() / 2.0;
    rotated_cylinder_cut([-r, -r, 0.0, 2.0 * r, 2.0 * r, 2.0], 135.0, 26.1633);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_V2() {
    let r = 2_f64.sqrt() / 2.0;
    rotated_cylinder_cut([-r, -r, 0.0, 1.6, 2.0 * r, 2.0], 135.0, 23.4218);
}

#[test]
fn BOPCutSimpleTest__CylinderBoxes_V4() {
    let r = 2_f64.sqrt() / 2.0;
    cylinder_cut([-r, -r, 0.0, 1.6, 2.0 * r, 2.0], 23.4218, false);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_V5() {
    cylinder_cut([-1.0, -1.0, 0.0, 2.0, 2.0, 1.0], 12.5664, false);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_V7() {
    cylinder_cut([-1.0, -1.0, 0.0, 2.0, 1.5, 1.0], 16.3928, false);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_V9() {
    cylinder_cut([-1.0, -1.0, 0.0, 1.5, 2.0, 1.0], 16.3928, false);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_W1() {
    cylinder_cut([-1.0, -1.0, 0.0, 2.0, 2.5, 1.0], 12.5664, false);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_W2() {
    cylinder_cut([-1.0, -1.0, 0.0, 2.5, 2.0, 1.0], 12.5664, false);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_W3() {
    cylinder_cut([-1.0, -1.0, 0.0, 1.0, 2.0, 1.0], 17.708, false);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_V3() {
    let r = 3_f64.sqrt() / 2.0;
    rotated_cylinder_cut([-0.5, -r, 0.0, 1.4, 2.0 * r, 2.0], 60.0, 21.0975);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_V6() {
    rotated_cylinder_cut([-1.0, -1.0, 0.0, 2.0, 2.0, 1.0], 45.0, 12.5664);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_V8() {
    let r = 3_f64.sqrt() / 2.0;
    rotated_cylinder_cut([-1.0, -1.0, 0.0, 1.0 + r, 2.0, 1.0], 60.0, 14.6136);
}

#[test]
fn BOPCutSimpleTest__CylinderBoxes_W4() {
    cylinder_cut([0.0, -1.0, 0.0, 1.0, 2.0, 1.0], 17.708, false);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_W5() {
    cylinder_cut([-1.0, -1.0, 0.0, 2.0, 1.0, 1.0], 17.708, false);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_W8() {
    let r = 2_f64.sqrt() / 2.0;
    cylinder_cut([-1.0, -1.0, 0.0, 1.0 + r, 1.0 + r, 1.0], 18.5364, false);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_W9() {
    let r = 2_f64.sqrt() / 2.0;
    cylinder_cut([-r, -r, 0.0, 1.0 + r, 1.0 + r, 1.0], 18.5364, false);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_X1() {
    cylinder_cut([-0.8, -0.8, 0.0, 1.8, 1.8, 1.0], 17.5404, false);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_X2() {
    cylinder_cut([-1.0, -0.8, 0.0, 1.8, 1.8, 1.0], 17.5404, false);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_X4() {
    cylinder_cut([-1.0, -0.8, 0.0, 2.0, 1.6, 1.0], 17.5404, false);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_W6() {
    let r = 2_f64.sqrt() / 2.0;
    rotated_cylinder_cut([-1.0, -1.0, 0.0, 1.0 + r, 1.0 + r, 1.0], 45.0, 18.5364);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_W7() {
    let r = 2_f64.sqrt() / 2.0;
    rotated_cylinder_cut([-1.0, -1.0, 0.0, 1.0 + r, 1.0 + r, 1.0], -45.0, 18.5364);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_X3() {
    let r = 3_f64.sqrt() / 2.0;
    rotated_cylinder_cut([-1.0, -r, 0.0, 1.0 + r, 1.0 + r, 1.0], 30.0, 16.6608);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_X5() {
    cylinder_cut([-0.8, -1.0, 0.0, 1.6, 2.0, 1.0], 17.5404, false);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_X7() {
    cylinder_cut([-0.5, -1.0, 0.0, 1.5, 1.5, 1.0], 18.9636, false);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_X9() {
    cylinder_cut([-1.0, -1.0, 0.0, 1.5, 1.5, 1.0], 18.9636, false);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_Y1() {
    let r = 2_f64.sqrt() / 2.0;
    cylinder_cut([-r, -r, 0.0, 1.0 + r, 2.0 * r, 1.0], 21.5214, false);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_Y4() {
    let r = 2_f64.sqrt() / 2.0;
    cylinder_cut([-r, -r, 0.0, 2.0 * r, r + 1.0, 1.0], 21.5214, false);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_X6() {
    let r = 3_f64.sqrt() / 2.0;
    rotated_cylinder_cut([-r, -1.0, 0.0, 2.0 * r, 2.0, 1.0], 30.0, 16.6608);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_X8() {
    rotated_cylinder_cut([-0.5, -1.0, 0.0, 1.5, 1.5, 1.0], 30.0, 18.9636);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_Y2() {
    let r = 2_f64.sqrt() / 2.0;
    rotated_cylinder_cut([-r, -r, 0.0, 1.0 + r, 2.0 * r, 1.0], 135.0, 21.5214);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_Y3() {
    let r = 3_f64.sqrt() / 2.0;
    rotated_cylinder_cut([-0.5, -r, 0.0, 1.5, 2.0 * r, 1.0], 60.0, 20.4872);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_Y5() {
    let r = 3_f64.sqrt() / 2.0;
    rotated_cylinder_cut([-r, -1.0, 0.0, 2.0 * r, 1.0 + r, 1.0], 30.0, 18.708);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_Y7() {
    cylinder_cut([-0.8, -1.0, 0.0, 1.6, 1.8, 1.0], 20.0274, false);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_Y8() {
    cylinder_cut([-1.0, -1.1, 0.0, 1.0, 2.2, 1.0], 17.708, false);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_Y9() {
    cylinder_cut([0.0, -1.1, 0.0, 1.0, 2.2, 1.0], 17.708, false);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_Z1() {
    cylinder_cut([-1.1, -1.0, 0.0, 2.2, 1.0, 1.0], 17.708, false);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_Z2() {
    cylinder_cut([-1.0, 1.0, 0.0, 2.0, 2.0, 1.0], 18.8496, false);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_Z3() {
    cylinder_cut([1.0, -1.0, 0.0, 2.0, 2.0, 1.0], 18.8496, false);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_Z5() {
    let r = 2_f64.sqrt() / 2.0;
    cylinder_cut([-r, -r, 0.0, 2.0 * r, 2.0 * r, 1.0], 24.5064, false);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_Y6() {
    rotated_cylinder_cut([-0.8, -1.0, 0.0, 1.6, 1.8, 1.0], 90.0, 20.0274);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_Z4() {
    let r = 2_f64.sqrt() / 2.0;
    rotated_cylinder_cut([-r, -r, 0.0, 2.0 * r, 2.0 * r, 1.0], 135.0, 24.5064);
}
#[test]
fn BOPCutSimpleTest__CylinderBoxes_Z6() {
    let r = 2_f64.sqrt() / 2.0;
    rotated_cylinder_cut([-r, -r, 0.0, 1.6, 2.0 * r, 1.0], 135.0, 23.3561);
}

//! Regressions of elevated direct box cut regressions.
#![allow(non_snake_case)]

#[path = "bop_test_utilities/mod.rs"]
mod bop_test_utilities;
#[path = "bopcut_simple_box_support.rs"]
mod bopcut_simple_box_support;

use bop_test_utilities::*;
use bopcut_simple_box_support::*;
use martensite_tkbo::bop_algo::Operation;
use martensite_tkmath::gp::Point3;

#[test]
fn BOPCutSimpleTest__ElevatedBoxes_P1() {
    cut_with_rotated_box(
        [0.0, 0.25, 0.5, 0.5, 0.5, 1.0],
        Point3::new(0.0, 0.25, 0.0),
        230.0,
        5.9205,
    );
}

#[test]
fn BOPCutSimpleTest__ElevatedBoxes_P2() {
    cut_with_rotated_box(
        [0.1, 0.4, 0.5, 0.5, 0.5, 1.0],
        Point3::new(0.1, 0.4, 0.0),
        245.0,
        6.14705,
    );
}

#[test]
fn BOPCutSimpleTest__ElevatedBoxes_P3() {
    cut_with_box([-0.25, -0.25, 0.5, 0.5, 0.5, 1.0], 6.0);
}

#[test]
fn BOPCutSimpleTest__ElevatedBoxes_P4() {
    cut_with_box([0.25, 0.5, 0.5, 0.5, 0.5, 1.0], 6.5);
}

#[test]
fn BOPCutSimpleTest__ElevatedBoxes_P5() {
    cut_with_rotated_box(
        [0.25, 0.5, 0.5, 0.5, 0.5, 1.0],
        Point3::new(0.25, 0.25, 0.0),
        45.0,
        6.10355,
    );
}

#[test]
fn BOPCutSimpleTest__ElevatedBoxes_P6() {
    cut_with_rotated_box(
        [0.25, 0.5, 0.5, 0.5, 0.5, 1.0],
        Point3::new(0.25, 0.25, 0.0),
        90.0,
        6.0,
    );
}

#[test]
fn BOPCutSimpleTest__ElevatedBoxes_P7() {
    cut_with_rotated_box(
        [0.25, 0.25, 0.5, 0.5, 0.5, 1.0],
        Point3::new(0.25, 0.25, 0.0),
        125.0,
        6.10448,
    );
}

#[test]
fn BOPCutSimpleTest__ElevatedBoxes_P8() {
    cut_with_box([0.25, -0.25, 0.5, 0.5, 1.5, 1.0], 6.5);
}

#[test]
fn BOPCutSimpleTest__ElevatedBoxes_P9() {
    cut_with_rotated_box(
        [0.9, -0.25, 0.5, 0.5, 1.7, 1.0],
        Point3::new(0.9, -0.25, 0.0),
        45.0,
        6.20711,
    );
}

#[test]
fn BOPCutSimpleTest__ElevatedBoxes_Q1() {
    cut_with_rotated_box(
        [1.25, 0.0, 0.5, 0.25, 1.7, 1.0],
        Point3::new(1.1, 0.0, 0.0),
        45.0,
        6.36924,
    );
}

#[test]
fn BOPCutSimpleTest__ElevatedBoxes_Q2() {
    cut_with_rotated_box(
        [-0.25, -0.25, 0.5, 0.5, 1.7, 1.0],
        Point3::new(0.0, 0.0, 0.0),
        -30.0,
        6.18301,
    );
}

#[test]
fn BOPCutSimpleTest__ElevatedBoxes_Q3() {
    cut_with_rotated_box(
        [-0.25, -0.25, 0.5, 0.5, 1.55, 1.0],
        Point3::new(0.0, 0.0, 0.0),
        -45.0,
        6.48284,
    );
}

#[test]
fn BOPCutSimpleTest__ElevatedBoxes_Q4() {
    cut_with_rotated_box(
        [0.5, -0.25, 0.5, 1.0, 1.0, 1.0],
        Point3::new(0.5, -0.25, 0.0),
        45.0,
        6.0,
    );
}

#[test]
fn BOPCutSimpleTest__ElevatedBoxes_Q5() {
    cut_with_rotated_box(
        [0.0, 0.15, 0.5, 0.8, 0.9, 1.0],
        Point3::new(0.0, 0.25, 0.0),
        -45.0,
        6.27929,
    );
}

#[test]
fn BOPCutSimpleTest__ElevatedBoxes_Q6() {
    cut_with_rotated_box(
        [0.5, -0.25, 0.5, 0.5, 1.0, 1.0],
        Point3::new(0.5, -0.25, 0.0),
        45.0,
        6.39645,
    );
}

#[test]
fn BOPCutSimpleTest__ElevatedBoxes_Q7() {
    cut_with_rotated_box(
        [-0.25, -0.25, 0.25, 0.25, 1.0, 0.5],
        Point3::new(0.0, 0.0, 0.0),
        -30.0,
        6.67524,
    );
}

#[test]
fn BOPCutSimpleTest__ElevatedBoxes_Q8() {
    cut_with_rotated_box(
        [0.0, -0.25, 0.25, 0.5, 1.7, 0.5],
        Point3::new(0.0, 0.0, 0.0),
        -35.0,
        7.35615,
    );
}

#[test]
fn BOPCutSimpleTest__ElevatedBoxes_Q9() {
    let first = CreateBox(Point3::new(0.0, 0.0, 0.0), 3.0, 3.0, 3.0);
    let mut second = CreateBox(Point3::new(0.0, 0.0, 0.0), 1.0, 1.0, 1.0);
    RotateAroundAxis(
        &mut second,
        Point3::new(0.0, 0.0, 0.0),
        [1.0, 0.0, 0.0],
        -30.0,
    );
    let second = Translate(&second, 1.0, 1.0, 3.0);
    ValidateResult(
        &PerformDirectBOP(&first, &second, Operation::Cut),
        55.0,
        -1.0,
        false,
    );
}

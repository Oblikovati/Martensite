//! Regressions of later pivoted direct box cut regressions.
#![allow(non_snake_case)]

#[path = "bopcut_simple_box_support.rs"]
mod bopcut_simple_box_support;

use bopcut_simple_box_support::*;
use martensite_tkmath::gp::Point3;

#[test]
fn BOPCutSimpleTest__RotatedBoxes_I4() {
    cut_with_rotated_box(
        [0.0, 0.25, 0.0, 0.5, 0.5, 1.0],
        Point3::new(0.0, 0.25, 0.0),
        230.0,
        5.76651,
    );
}

#[test]
fn BOPCutSimpleTest__RotatedBoxes_I5() {
    cut_with_rotated_box(
        [0.0, 0.25, 0.0, 0.5, 0.5, 1.0],
        Point3::new(0.0, 0.25, 0.0),
        250.0,
        5.74066,
    );
}

#[test]
fn BOPCutSimpleTest__RotatedBoxes_I6() {
    cut_with_rotated_box(
        [0.0, 0.5, 0.0, 1.5, 1.5, 1.0],
        Point3::new(0.0, 0.5, 0.0),
        -45.0,
        3.91421,
    );
}

#[test]
fn BOPCutSimpleTest__RotatedBoxes_I7() {
    cut_with_rotated_box(
        [0.0, 0.5, 0.0, 0.8, 0.8, 1.0],
        Point3::new(0.0, 0.5, 0.0),
        -45.0,
        6.70315,
    );
}

#[test]
fn BOPCutSimpleTest__RotatedBoxes_I8() {
    cut_with_rotated_box(
        [0.0, 0.25, 0.0, 0.8, 0.8, 1.0],
        Point3::new(0.0, 0.25, 0.0),
        -45.0,
        5.99525,
    );
}

#[test]
fn BOPCutSimpleTest__RotatedBoxes_I9() {
    cut_with_rotated_box(
        [0.0, 0.25, 0.0, 0.5, 0.5, 1.0],
        Point3::new(0.0, 0.25, 0.0),
        -45.0,
        7.02145,
    );
}

#[test]
fn BOPCutSimpleTest__RotatedBoxes_J1() {
    let side = 2_f64.sqrt() * 0.75;
    cut_with_rotated_box(
        [0.0, 0.625, 0.0, side, side, 1.0],
        Point3::new(0.0, 0.625, 0.0),
        -45.0,
        4.38787,
    );
}

#[test]
fn BOPCutSimpleTest__RotatedBoxes_J2() {
    cut_with_rotated_box(
        [-0.25, 0.0, 0.0, 0.5, 1.7, 1.0],
        Point3::new(0.0, 0.0, 0.0),
        -45.0,
        5.25,
    );
}

#[test]
fn BOPCutSimpleTest__RotatedBoxes_J3() {
    cut_with_rotated_box(
        [-0.25, 0.0, 0.0, 0.5, 1.7, 1.0],
        Point3::new(0.0, 0.0, 0.0),
        -30.0,
        5.35566,
    );
}

#[test]
fn BOPCutSimpleTest__RotatedBoxes_J4() {
    cut_with_rotated_box(
        [-0.25, -0.25, 0.0, 0.25, 2.0, 1.0],
        Point3::new(0.0, 0.0, 0.0),
        -30.0,
        6.51036,
    );
}

#[test]
fn BOPCutSimpleTest__RotatedBoxes_J5() {
    cut_with_rotated_box(
        [-0.25, 0.0, 0.0, 0.5, 1.3, 1.0],
        Point3::new(0.0, 0.0, 0.0),
        -45.0,
        5.82756,
    );
}

#[test]
fn BOPCutSimpleTest__RotatedBoxes_J6() {
    cut_with_rotated_box(
        [0.0, -0.25, 0.0, 0.5, 1.55, 1.0],
        Point3::new(0.0, 0.0, 0.0),
        -35.0,
        5.95381,
    );
}

#[test]
fn BOPCutSimpleTest__RotatedBoxes_J7() {
    cut_with_rotated_box(
        [0.1, 0.25, 0.0, 0.5, 0.5, 1.0],
        Point3::new(0.0, 0.25, 0.0),
        230.0,
        5.90701,
    );
}

#[test]
fn BOPCutSimpleTest__RotatedBoxes_J8() {
    cut_with_rotated_box(
        [0.1, 0.4, 0.0, 0.5, 0.5, 1.0],
        Point3::new(0.1, 0.4, 0.0),
        245.0,
        5.98538,
    );
}

#[test]
fn BOPCutSimpleTest__RotatedBoxes_J9() {
    cut_with_box([-0.25, -0.25, 0.0, 0.5, 0.5, 1.0], 5.875);
}

#[test]
fn BOPCutSimpleTest__RotatedBoxes_K1() {
    cut_with_box([0.25, 0.25, 0.0, 0.5, 0.5, 1.0], 7.5);
}

#[test]
fn BOPCutSimpleTest__RotatedBoxes_K2() {
    cut_with_rotated_box(
        [0.25, 0.25, 0.0, 0.5, 0.5, 1.0],
        Point3::new(0.25, 0.25, 0.0),
        45.0,
        7.02145,
    );
}

#[test]
fn BOPCutSimpleTest__RotatedBoxes_K3() {
    cut_with_rotated_box(
        [0.25, 0.25, 0.0, 0.5, 0.5, 1.0],
        Point3::new(0.25, 0.25, 0.0),
        90.0,
        6.25,
    );
}

#[test]
fn BOPCutSimpleTest__RotatedBoxes_K4() {
    cut_with_rotated_box(
        [0.25, 0.25, 0.0, 0.5, 0.5, 1.0],
        Point3::new(0.25, 0.25, 0.0),
        125.0,
        6.07594,
    );
}

#[test]
fn BOPCutSimpleTest__RotatedBoxes_K5() {
    cut_with_box([0.25, -0.25, 0.0, 0.5, 1.5, 1.0], 6.0);
}

#[test]
fn BOPCutSimpleTest__PivotedBoxes_K6() {
    cut_with_rotated_box(
        [0.9, -0.25, 0.0, 0.5, 1.7, 1.0],
        Point3::new(0.9, -0.25, 0.0),
        45.0,
        5.25003,
    );
}

#[test]
fn BOPCutSimpleTest__PivotedBoxes_K7() {
    cut_with_rotated_box(
        [1.25, 0.0, 0.0, 0.25, 1.7, 1.0],
        Point3::new(1.1, 0.0, 0.0),
        45.0,
        6.37708,
    );
}

#[test]
fn BOPCutSimpleTest__PivotedBoxes_K8() {
    cut_with_rotated_box(
        [-0.25, -0.25, 0.0, 0.5, 1.7, 1.0],
        Point3::new(0.0, 0.0, 0.0),
        -30.0,
        5.35566,
    );
}

#[test]
fn BOPCutSimpleTest__PivotedBoxes_K9() {
    cut_with_rotated_box(
        [-0.25, -0.25, 0.0, 0.5, 1.55, 1.0],
        Point3::new(0.0, 0.0, 0.0),
        -45.0,
        5.82756,
    );
}

#[test]
fn BOPCutSimpleTest__PivotedBoxes_L1() {
    cut_with_rotated_box(
        [0.5, -0.25, 0.0, 1.0, 1.0, 1.0],
        Point3::new(0.5, -0.25, 0.0),
        45.0,
        4.35051,
    );
}

#[test]
fn BOPCutSimpleTest__PivotedBoxes_L2() {
    cut_with_rotated_box(
        [0.0, 0.15, 0.0, 0.8, 0.9, 1.0],
        Point3::new(0.0, 0.25, 0.0),
        -45.0,
        5.4617,
    );
}

#[test]
fn BOPCutSimpleTest__PivotedBoxes_L3() {
    cut_with_rotated_box(
        [0.5, -0.25, 0.0, 0.5, 1.0, 1.0],
        Point3::new(0.5, -0.25, 0.0),
        45.0,
        6.00368,
    );
}

#[test]
fn BOPCutSimpleTest__PivotedBoxes_L4() {
    cut_with_rotated_box(
        [0.0, 0.0, 0.0, 2_f64.sqrt(), 2_f64.sqrt() / 2.0, 0.5],
        Point3::new(0.0, 0.0, 0.0),
        45.0,
        5.70711,
    );
}

#[test]
fn BOPCutSimpleTest__PivotedBoxes_L5() {
    let side = 2_f64.sqrt() / 2.0;
    cut_with_rotated_box(
        [0.0, 0.0, 0.0, side, side, 0.5],
        Point3::new(0.0, 0.0, 0.0),
        45.0,
        6.20711,
    );
}

#[test]
fn BOPCutSimpleTest__PivotedBoxes_L6() {
    cut_with_rotated_box(
        [0.0, 0.0, 0.0, 2_f64.sqrt(), 0.25, 0.5],
        Point3::new(0.0, 0.0, 0.0),
        45.0,
        6.81066,
    );
}

#[test]
fn BOPCutSimpleTest__PivotedBoxes_L7() {
    cut_with_rotated_box(
        [0.0, 0.0, 0.0, 31_f64.sqrt() / 4.0, 0.25, 0.5],
        Point3::new(0.0, 0.0, 0.0),
        34.73,
        6.91258,
    );
}

#[test]
fn BOPCutSimpleTest__PivotedBoxes_L8() {
    cut_with_rotated_box(
        [0.0, 0.0, 0.0, 1.5, 2_f64.sqrt() / 2.0, 0.5],
        Point3::new(0.0, 0.0, 0.0),
        45.0,
        5.70711,
    );
}

#[test]
fn BOPCutSimpleTest__PivotedBoxes_L9() {
    cut_with_rotated_box(
        [0.0, 0.5, 0.0, 5_f64.sqrt() / 2.0, 5_f64.sqrt().recip(), 0.5],
        Point3::new(0.0, 0.5, 0.0),
        martensite_cmath::atan2(1_f64, 2.0).to_degrees(),
        5.80902,
    );
}

#[test]
fn BOPCutSimpleTest__PivotedBoxes_M1() {
    cut_with_rotated_box(
        [0.0, 0.0, 0.0, 0.5, 0.5, 0.5],
        Point3::new(0.0, 0.0, 0.0),
        45.0,
        6.14645,
    );
}

#[test]
fn BOPCutSimpleTest__PivotedBoxes_M2() {
    cut_with_rotated_box(
        [0.0, 0.5, 0.0, 5_f64.sqrt().recip(), 5_f64.sqrt() / 2.0, 0.5],
        Point3::new(0.0, 0.5, 0.0),
        martensite_cmath::atan2(1_f64, 2.0).to_degrees() - 90.0,
        6.97984,
    );
}

#[test]
fn BOPCutSimpleTest__PivotedBoxes_M3() {
    cut_with_rotated_box(
        [-0.5, 0.0, 0.0, 0.5, 1.7, 0.5],
        Point3::new(0.0, 0.0, 0.0),
        -30.0,
        5.78868,
    );
}

#[test]
fn BOPCutSimpleTest__PivotedBoxes_M4() {
    cut_with_rotated_box(
        [0.0, 0.0, 0.0, 1.5, 0.25, 0.5],
        Point3::new(0.0, 0.0, 0.0),
        45.0,
        6.81066,
    );
}

#[test]
fn BOPCutSimpleTest__PivotedBoxes_M5() {
    cut_with_rotated_box(
        [0.0, 0.0, 0.0, 31_f64.sqrt() / 4.0, 0.5, 0.5],
        Point3::new(0.0, 0.0, 0.0),
        34.73,
        6.27696,
    );
}

#[test]
fn BOPCutSimpleTest__PivotedBoxes_M6() {
    cut_with_rotated_box(
        [0.0, 0.0, 0.0, 0.5, 0.5, 0.5],
        Point3::new(0.0, 0.0, 0.0),
        30.0,
        6.31699,
    );
}

#[test]
fn BOPCutSimpleTest__PivotedBoxes_M7() {
    cut_with_rotated_box(
        [0.0, 0.0, 0.0, 0.5, 0.5, 0.5],
        Point3::new(0.0, 0.0, 0.0),
        60.0,
        6.10566,
    );
}

#[test]
fn BOPCutSimpleTest__PivotedBoxes_M8() {
    cut_with_rotated_box(
        [1.0, 1.0, 0.0, 1.0, 1.0, 0.5],
        Point3::new(1.0, 1.0, 1.0),
        30.0,
        6.0,
    );
}

#[test]
fn BOPCutSimpleTest__PivotedBoxes_M9() {
    cut_with_rotated_box(
        [-0.25, 0.0, 0.0, 0.25, 1.7, 0.5],
        Point3::new(0.0, 0.0, 0.0),
        -30.0,
        6.47169,
    );
}

#[test]
fn BOPCutSimpleTest__PivotedBoxes_N1() {
    cut_with_rotated_box(
        [-0.5, 0.0, 0.0, 0.5, 1.7, 0.5],
        Point3::new(0.0, 0.0, 0.0),
        -50.0,
        6.223,
    );
}

#[test]
fn BOPCutSimpleTest__PivotedBoxes_N2() {
    cut_with_rotated_box(
        [0.0, 0.0, 0.0, 30_f64.sqrt() / 4.0, 0.25, 0.5],
        Point3::new(0.0, 0.0, 0.0),
        40.0,
        6.93117,
    );
}

fn downward_cut(pivot: Point3, angle: f64) {
    cut_with_rotated_box([0.0, 0.25, 0.0, 0.5, 0.5, -1.0], pivot, angle, 6.0);
}

#[test]
fn BOPCutSimpleTest__PivotedBoxes_N3() {
    downward_cut(Point3::new(0.25, 0.25, 0.0), 30.0);
}

#[test]
fn BOPCutSimpleTest__PivotedBoxes_N4() {
    downward_cut(Point3::new(0.25, 0.25, 0.0), 120.0);
}

#[test]
fn BOPCutSimpleTest__PivotedBoxes_N5() {
    downward_cut(Point3::new(0.0, 0.25, 0.0), 35.0);
}

#[test]
fn BOPCutSimpleTest__PivotedBoxes_N6() {
    downward_cut(Point3::new(0.0, 0.25, 0.0), 115.0);
}

#[test]
fn BOPCutSimpleTest__PivotedBoxes_N7() {
    downward_cut(Point3::new(0.0, 0.25, 0.0), 230.0);
}

#[test]
fn BOPCutSimpleTest__PivotedBoxes_N8() {
    downward_cut(Point3::new(0.0, 0.25, 0.0), 250.0);
}

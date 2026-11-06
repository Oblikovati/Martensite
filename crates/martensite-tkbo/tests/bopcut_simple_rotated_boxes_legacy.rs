//! Regressions of rotated and offset direct box cut regressions.
#![allow(non_snake_case)]

#[path = "bopcut_simple_box_support.rs"]
mod bopcut_simple_box_support;

use bopcut_simple_box_support::*;
use martensite_tkmath::gp::Point3;

#[test]
fn BOPCutSimpleTest__RotatedBoxes_D4() {
    cut_with_rotated_box(
        [0.0, 0.0, 0.0, 31_f64.sqrt() / 4.0, 0.5, 1.0],
        Point3::new(0.0, 0.0, 0.0),
        34.73,
        5.46836,
    );
}

#[test]
fn BOPCutSimpleTest__RotatedBoxes_D5() {
    cut_with_rotated_box(
        [0.0, 0.0, 0.0, 0.5, 0.5, 1.0],
        Point3::new(0.0, 0.0, 0.0),
        30.0,
        6.27831,
    );
}

#[test]
fn BOPCutSimpleTest__RotatedBoxes_D6() {
    cut_with_rotated_box(
        [0.0, 0.0, 0.0, 0.5, 0.5, 1.0],
        Point3::new(0.0, 0.0, 0.0),
        60.0,
        6.06699,
    );
}

#[test]
fn BOPCutSimpleTest__RotatedBoxes_D7() {
    cut_with_rotated_box(
        [1.0, 1.0, 0.0, 1.0, 1.0, 1.0],
        Point3::new(1.0, 1.0, 1.0),
        30.0,
        6.0,
    );
}

#[test]
fn BOPCutSimpleTest__RotatedBoxes_D8() {
    cut_with_rotated_box(
        [-0.25, 0.0, 0.0, 0.25, 1.7, 1.0],
        Point3::new(0.0, 0.0, 0.0),
        -30.0,
        6.51036,
    );
}

#[test]
fn BOPCutSimpleTest__RotatedBoxes_D9() {
    cut_with_rotated_box(
        [-0.5, 0.0, 0.0, 0.5, 1.7, 1.0],
        Point3::new(0.0, 0.0, 0.0),
        -50.0,
        5.42883,
    );
}

#[test]
fn BOPCutSimpleTest__RotatedBoxes_E1() {
    cut_with_rotated_box(
        [0.0, 0.0, 0.0, 30_f64.sqrt() / 4.0, 0.25, 1.0],
        Point3::new(0.0, 0.0, 0.0),
        40.0,
        7.24544,
    );
}

#[test]
fn BOPCutSimpleTest__OffsetBoxes_E2() {
    cut_with_box([0.25, 0.0, 0.0, 0.5, 0.5, 0.5], 6.5);
}

#[test]
fn BOPCutSimpleTest__OffsetBoxes_E3() {
    cut_with_box([0.25, -0.5, 0.0, 0.5, 0.5, 0.5], 6.0);
}

#[test]
fn BOPCutSimpleTest__OffsetBoxes_E4() {
    cut_with_box([0.25, -0.5, -0.5, 0.5, 0.5, 0.5], 6.0);
}

#[test]
fn BOPCutSimpleTest__OffsetBoxes_E5() {
    cut_with_box([0.25, 0.0, 0.0, 0.5, 1.5, 0.5], 6.5);
}

#[test]
fn BOPCutSimpleTest__OffsetBoxes_E6() {
    cut_with_box([0.25, 0.0, 0.0, 1.5, 0.5, 0.5], 6.0);
}

#[test]
fn BOPCutSimpleTest__OffsetBoxes_E7() {
    cut_with_box([0.25, -0.5, 0.0, 1.5, 0.5, 0.5], 6.0);
}

#[test]
fn BOPCutSimpleTest__OffsetBoxes_E8() {
    cut_with_box([0.25, -0.5, -0.5, 1.5, 0.5, 0.5], 6.0);
}

#[test]
fn BOPCutSimpleTest__OffsetBoxes_E9() {
    cut_with_box([0.25, 0.0, 0.0, 1.5, 1.5, 0.5], 5.25);
}

#[test]
fn BOPCutSimpleTest__RotatedBoxes_F7() {
    cut_with_rotated_box(
        [0.0, 0.5, 0.0, 1.25, 5_f64.sqrt().recip(), 1.0],
        Point3::new(0.0, 0.5, 0.0),
        martensite_cmath::atan2(1_f64, 2.0).to_degrees(),
        5.11803,
    );
}

#[test]
fn BOPCutSimpleTest__RotatedBoxes_F8() {
    cut_with_rotated_box(
        [-0.25, -0.25, 0.0, 2.0, 2_f64.sqrt() / 2.0, 1.0],
        Point3::new(-0.25, -0.25, 0.0),
        45.0,
        4.41421,
    );
}

#[test]
fn BOPCutSimpleTest__RotatedBoxes_F9() {
    cut_with_rotated_box(
        [0.0, 0.5, 0.0, 2_f64.sqrt() / 2.0, 2_f64.sqrt() / 4.0, 1.0],
        Point3::new(0.0, 0.5, 0.0),
        45.0,
        5.45711,
    );
}

#[test]
fn BOPCutSimpleTest__RotatedBoxes_G1() {
    cut_with_rotated_box(
        [0.0, 0.5, 0.0, 2.0 / 5_f64.sqrt(), 1.25, 1.0],
        Point3::new(0.0, 0.5, 0.0),
        martensite_cmath::atan2(1_f64, 2.0).to_degrees() - 90.0,
        4.55205,
    );
}

#[test]
fn BOPCutSimpleTest__RotatedBoxes_G2() {
    cut_with_rotated_box(
        [
            0.25,
            0.0,
            0.0,
            1.5 * 5_f64.sqrt() / 4.0,
            5_f64.sqrt() / 4.0,
            1.0,
        ],
        Point3::new(0.25, 0.0, 0.0),
        martensite_cmath::atan2(1_f64, 2.0).to_degrees(),
        7.85758,
    );
}

#[test]
fn BOPCutSimpleTest__RotatedBoxes_G3() {
    let side = 2_f64.sqrt() / 2.0;
    cut_with_rotated_box(
        [1.0 - side, 0.0, 0.0, 2.0 * side, 1.5 * side, 1.0],
        Point3::new(1.0, 0.0, 0.0),
        45.0,
        1.95711,
    );
}

#[test]
fn BOPCutSimpleTest__RotatedBoxes_G4() {
    cut_with_rotated_box(
        [-0.25, -0.25, 0.0, 2.0, 2_f64.sqrt() / 4.0, 1.0],
        Point3::new(-0.25, -0.25, 0.0),
        45.0,
        6.37132,
    );
}

#[test]
fn BOPCutSimpleTest__RotatedBoxes_G5() {
    cut_with_rotated_box(
        [0.0, 0.25, 0.0, 1.3, 1.5 / 5_f64.sqrt(), 1.0],
        Point3::new(0.0, 0.25, 0.0),
        martensite_cmath::atan2(1_f64, 2.0).to_degrees(),
        4.11803,
    );
}

#[test]
fn BOPCutSimpleTest__RotatedBoxes_G6() {
    cut_with_rotated_box(
        [
            0.0,
            0.45,
            0.0,
            5_f64.sqrt().recip(),
            5_f64.sqrt() / 2.0,
            1.0,
        ],
        Point3::new(0.0, 0.45, 0.0),
        martensite_cmath::atan2(1_f64, 2.0).to_degrees() - 90.0,
        7.05967,
    );
}

#[test]
fn BOPCutSimpleTest__RotatedBoxes_G7() {
    cut_with_rotated_box(
        [0.0, 0.25, 0.0, 0.5, 0.5, 1.0],
        Point3::new(0.0, 0.25, 0.0),
        45.0,
        6.04289,
    );
}

#[test]
fn BOPCutSimpleTest__RotatedBoxes_G8() {
    cut_with_rotated_box(
        [0.0, 0.25, 0.0, 0.5, 0.5, 1.0],
        Point3::new(0.0, 0.25, 0.0),
        -30.0,
        7.5,
    );
}

#[test]
fn BOPCutSimpleTest__RotatedBoxes_G9() {
    cut_with_rotated_box(
        [0.0, 0.25, 0.0, 0.5, 0.5, 1.0],
        Point3::new(0.0, 0.25, 0.0),
        240.0,
        5.70873,
    );
}

#[test]
fn BOPCutSimpleTest__RotatedBoxes_H1() {
    let side = 2_f64.sqrt() * 0.875;
    cut_with_rotated_box(
        [0.0, 0.75, 0.0, side, side, 1.0],
        Point3::new(0.0, 0.75, 0.0),
        -45.0,
        4.03921,
    );
}

#[test]
fn BOPCutSimpleTest__RotatedBoxes_H2() {
    let side = 5_f64.sqrt() * 0.5;
    cut_with_rotated_box(
        [0.0, 0.25, 0.0, side, side, 1.0],
        Point3::new(0.0, 0.25, 0.0),
        martensite_cmath::atan2(1_f64, 2.0).to_degrees(),
        4.80379,
    );
}

#[test]
fn BOPCutSimpleTest__RotatedBoxes_H3() {
    cut_with_rotated_box(
        [0.5, 0.0, 0.0, 1.0, 2_f64.sqrt() / 2.0, 1.0],
        Point3::new(0.5, 0.0, 0.0),
        45.0,
        6.17157,
    );
}

#[test]
fn BOPCutSimpleTest__RotatedBoxes_H4() {
    cut_with_rotated_box(
        [0.5, 0.0, 0.0, 1.5, 2_f64.sqrt() / 2.0, 1.0],
        Point3::new(0.5, 0.0, 0.0),
        45.0,
        5.87132,
    );
}

#[test]
fn BOPCutSimpleTest__RotatedBoxes_H5() {
    cut_with_rotated_box(
        [-0.1, 0.0, 0.0, 1.5, 5_f64.sqrt().recip(), 1.0],
        Point3::new(1.0, 0.0, 0.0),
        martensite_cmath::atan2(1_f64, 2.0).to_degrees(),
        5.11803,
    );
}

#[test]
fn BOPCutSimpleTest__RotatedBoxes_H6() {
    cut_with_rotated_box(
        [0.0, -0.25, 0.0, 2_f64.sqrt(), 0.5, 1.0],
        Point3::new(0.0, 0.0, 0.0),
        45.0,
        5.25,
    );
}

#[test]
fn BOPCutSimpleTest__RotatedBoxes_H7() {
    cut_with_rotated_box(
        [0.0, 0.25, 0.0, 0.1, 0.5, 1.0],
        Point3::new(0.0, 0.25, 0.0),
        240.0,
        6.37668,
    );
}

#[test]
fn BOPCutSimpleTest__RotatedBoxes_H8() {
    cut_with_rotated_box(
        [0.0, 0.25, 0.0, 5_f64.sqrt() / 2.0, 0.5, 1.0],
        Point3::new(0.0, 0.25, 0.0),
        martensite_cmath::atan2(1_f64, 2.0).to_degrees(),
        5.87674,
    );
}

#[test]
fn BOPCutSimpleTest__RotatedBoxes_H9() {
    cut_with_rotated_box(
        [0.25, 0.25, 0.0, 0.5, 0.5, 1.0],
        Point3::new(0.25, 0.25, 0.0),
        30.0,
        7.5,
    );
}

#[test]
fn BOPCutSimpleTest__RotatedBoxes_I1() {
    cut_with_rotated_box(
        [0.25, 0.25, 0.0, 0.5, 0.5, 1.0],
        Point3::new(0.25, 0.25, 0.0),
        120.0,
        6.06699,
    );
}

#[test]
fn BOPCutSimpleTest__RotatedBoxes_I2() {
    cut_with_rotated_box(
        [0.0, 0.25, 0.0, 0.5, 0.5, 1.0],
        Point3::new(0.0, 0.25, 0.0),
        35.0,
        6.21456,
    );
}

#[test]
fn BOPCutSimpleTest__RotatedBoxes_I3() {
    cut_with_rotated_box(
        [0.0, 0.25, 0.0, 0.5, 0.5, 1.0],
        Point3::new(0.0, 0.25, 0.0),
        115.0,
        6.0,
    );
}

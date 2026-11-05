//! Regressions of downward direct box cut regressions.
#![allow(non_snake_case)]

#[path = "bopcut_simple_box_support.rs"]
mod bopcut_simple_box_support;

use bopcut_simple_box_support::*;
use martensite_tkmath::gp::Point3;

fn downward_cut(second: [f64; 6], pivot: Point3, angle: f64) {
    cut_with_rotated_box(second, pivot, angle, 6.0);
}

#[test]
fn BOPCutSimpleTest__DownwardBoxes_N9() {
    downward_cut(
        [0.0, 0.5, 0.0, 1.5, 1.5, -1.0],
        Point3::new(0.0, 0.5, 0.0),
        -45.0,
    );
}

#[test]
fn BOPCutSimpleTest__DownwardBoxes_O1() {
    downward_cut(
        [0.0, 0.5, 0.0, 0.8, 0.8, -1.0],
        Point3::new(0.0, 0.5, 0.0),
        -45.0,
    );
}

#[test]
fn BOPCutSimpleTest__DownwardBoxes_O2() {
    downward_cut(
        [0.0, 0.25, 0.0, 0.8, 0.8, -1.0],
        Point3::new(0.0, 0.25, 0.0),
        -45.0,
    );
}

#[test]
fn BOPCutSimpleTest__DownwardBoxes_O3() {
    downward_cut(
        [0.0, 0.25, 0.0, 0.5, 0.5, -1.0],
        Point3::new(0.0, 0.25, 0.0),
        -45.0,
    );
}

#[test]
fn BOPCutSimpleTest__DownwardBoxes_O4() {
    let side = 2_f64.sqrt() * 0.75;
    downward_cut(
        [0.0, 0.625, 0.0, side, side, -1.0],
        Point3::new(0.0, 0.625, 0.0),
        -45.0,
    );
}

#[test]
fn BOPCutSimpleTest__DownwardBoxes_O5() {
    downward_cut(
        [-0.25, 0.0, 0.0, 0.5, 1.7, -1.0],
        Point3::new(0.0, 0.0, 0.0),
        -45.0,
    );
}

#[test]
fn BOPCutSimpleTest__DownwardBoxes_O6() {
    downward_cut(
        [-0.25, 0.0, 0.0, 0.5, 1.7, -1.0],
        Point3::new(0.0, 0.0, 0.0),
        -30.0,
    );
}

#[test]
fn BOPCutSimpleTest__DownwardBoxes_O7() {
    downward_cut(
        [-0.25, -0.25, 0.0, 0.25, 2.0, -1.0],
        Point3::new(0.0, 0.0, 0.0),
        -30.0,
    );
}

#[test]
fn BOPCutSimpleTest__DownwardBoxes_O8() {
    downward_cut(
        [-0.25, 0.0, 0.0, 0.5, 1.3, -1.0],
        Point3::new(0.0, 0.0, 0.0),
        -45.0,
    );
}

#[test]
fn BOPCutSimpleTest__DownwardBoxes_O9() {
    downward_cut(
        [0.0, -0.25, 0.0, 0.5, 1.55, -1.0],
        Point3::new(0.0, 0.0, 0.0),
        -35.0,
    );
}

//! Regressions of later cylinder and box cut regressions.
#![allow(non_snake_case)]
#[path = "bop_test_utilities/mod.rs"]
mod bop_test_utilities;
use bop_test_utilities::*;
use martensite_tkbo::bop_algo::Operation;
use martensite_tkmath::gp::Point3;
#[test]
fn BOPCutSimpleTest__ConeBoxes_ZF5() {
    let first = CreateBox(Point3::new(0.0, 0.0, 0.0), 4.0, 4.0, 4.0);
    let second = Translate(&CreateCone(1.0, 0.5, 2.0), 2.0, 2.0, 4.0);
    ValidateResult(
        &PerformDirectBOP(&first, &second, Operation::Cut),
        96.0,
        -1.0,
        false,
    );
}
fn cone_box_cut(height: f64, z: f64, area: f64) {
    let first = CreateBox(Point3::new(0.0, 0.0, 0.0), 4.0, 4.0, 4.0);
    let second = Translate(&CreateCone(1.0, 0.5, height), 2.0, 2.0, z);
    ValidateResult(
        &PerformDirectBOP(&first, &second, Operation::Cut),
        area,
        -1.0,
        false,
    );
}
#[test]
fn BOPCutSimpleTest__ConeBoxes_ZF6() {
    cone_box_cut(2.0, 2.0, 108.071);
}
#[test]
fn BOPCutSimpleTest__ConeBoxes_ZF7() {
    cone_box_cut(4.0, 0.0, 111.069);
}
#[test]
fn BOPCutSimpleTest__ConeBoxes_ZF8() {
    cone_box_cut(5.0, -1.0, 110.351);
}
#[test]
fn BOPCutSimpleTest__ConeBoxes_ZF9() {
    cone_box_cut(2.0, 1.0, 109.642);
}
fn large_cone_cut(offset: [f64; 3]) {
    let first = CreateBox(Point3::new(0.0, 0.0, 0.0), 4.0, 4.0, 4.0);
    let second = Translate(&CreateCone(3.0, 2.0, 4.0), offset[0], offset[1], offset[2]);
    ValidateResult(
        &PerformDirectBOP(&first, &second, Operation::Cut),
        70.8546,
        -1.0,
        false,
    );
}
#[test]
fn BOPCutSimpleTest__ConeBoxes_ZG1() {
    large_cone_cut([4.0, 2.0, 0.0]);
}
#[test]
fn BOPCutSimpleTest__ConeBoxes_ZG2() {
    large_cone_cut([2.0, 0.0, 0.0]);
}
#[test]
fn BOPCutSimpleTest__ConeBoxes_ZG3() {
    large_cone_cut([0.0, 2.0, 0.0]);
}
#[test]
fn BOPCutSimpleTest__ConeBoxes_ZG4() {
    let first = CreateBox(Point3::new(0.0, 0.0, 0.0), 4.0, 4.0, 4.0);
    let mut second = Translate(&CreateCone(3.0, 2.0, 4.0), 2.0, 0.0, 0.0);
    RotateAroundAxis(
        &mut second,
        Point3::new(2.0, 0.0, 0.0),
        [0.0, 0.0, 1.0],
        30.0,
    );
    ValidateResult(
        &PerformDirectBOP(&first, &second, Operation::Cut),
        70.8546,
        -1.0,
        false,
    );
}
#[test]
fn BOPCutSimpleTest__ConeBoxes_ZG5() {
    let first = CreateBox(Point3::new(0.0, 0.0, 0.0), 4.0, 4.0, 4.0);
    let second = Translate(&CreateCone(2.0, 1.0, 4.0), 4.0, 2.0, 0.0);
    ValidateResult(
        &PerformDirectBOP(&first, &second, Operation::Cut),
        95.5757,
        -1.0,
        false,
    );
}
#[test]
fn BOPCutSimpleTest__ConeBoxes_ZG6() {
    let first = CreateBox(Point3::new(0.0, 0.0, 0.0), 4.0, 4.0, 4.0);
    let second = Translate(&CreateCone(2.0, 1.0, 4.0), 2.0, 0.0, 0.0);
    ValidateResult(
        &PerformDirectBOP(&first, &second, Operation::Cut),
        95.5757,
        -1.0,
        false,
    );
}
#[test]
fn BOPCutSimpleTest__ConeBoxes_ZG7() {
    let first = CreateBox(Point3::new(0.0, 0.0, 0.0), 4.0, 4.0, 4.0);
    let second = Translate(&CreateCone(2.0, 1.0, 4.0), 0.0, 2.0, 0.0);
    ValidateResult(
        &PerformDirectBOP(&first, &second, Operation::Cut),
        95.5757,
        -1.0,
        false,
    );
}
#[test]
fn BOPCutSimpleTest__ConeBoxes_ZG8() {
    let first = CreateBox(Point3::new(0.0, 0.0, 0.0), 4.0, 4.0, 4.0);
    let second = Translate(&CreateCone(1.0, 0.5, 2.0), 3.0, 1.0, 4.0);
    ValidateResult(
        &PerformDirectBOP(&first, &second, Operation::Cut),
        96.0,
        -1.0,
        false,
    );
}
#[test]
fn BOPCutSimpleTest__ConeBoxes_ZG9() {
    let first = CreateBox(Point3::new(0.0, 0.0, 0.0), 4.0, 4.0, 4.0);
    let second = Translate(&CreateCone(1.0, 0.5, 2.0), 1.0, 1.0, 4.0);
    ValidateResult(
        &PerformDirectBOP(&first, &second, Operation::Cut),
        96.0,
        -1.0,
        false,
    );
}
#[test]
fn BOPCutSimpleTest__ConeBoxes_ZH1() {
    let first = CreateBox(Point3::new(0.0, 0.0, 0.0), 4.0, 4.0, 4.0);
    let second = Translate(&CreateCone(5.0, 4.0, 4.0), 6.0, 2.0, 0.0);
    ValidateResult(
        &PerformDirectBOP(&first, &second, Operation::Cut),
        62.1124,
        -1.0,
        false,
    );
}
#[test]
fn BOPCutSimpleTest__ConeBoxes_ZH2() {
    let first = CreateBox(Point3::new(0.0, 0.0, 0.0), 4.0, 4.0, 4.0);
    let second = Translate(&CreateCone(5.0, 4.0, 4.0), -2.0, 2.0, 0.0);
    ValidateResult(
        &PerformDirectBOP(&first, &second, Operation::Cut),
        62.1124,
        -1.0,
        false,
    );
}
#[test]
fn BOPCutSimpleTest__ConeBoxes_ZH3() {
    let first = CreateBox(Point3::new(0.0, 0.0, 0.0), 4.0, 4.0, 4.0);
    let mut second = Translate(&CreateCone(5.0, 4.0, 4.0), -2.0, 2.0, 0.0);
    RotateAroundAxis(
        &mut second,
        Point3::new(-2.0, 2.0, 0.0),
        [0.0, 0.0, 1.0],
        30.0,
    );
    ValidateResult(
        &PerformDirectBOP(&first, &second, Operation::Cut),
        62.1124,
        -1.0,
        false,
    );
}
#[test]
fn BOPCutSimpleTest__ConeBoxes_ZH4() {
    let first = CreateBox(Point3::new(0.0, 0.0, 0.0), 4.0, 4.0, 4.0);
    let mut second = Translate(&CreateCone(5.0, 3.5, 4.0), -2.0, 2.0, 0.0);
    RotateAroundAxis(
        &mut second,
        Point3::new(-2.0, 2.0, 0.0),
        [0.0, 0.0, 1.0],
        30.0,
    );
    ValidateResult(
        &PerformDirectBOP(&first, &second, Operation::Cut),
        67.2316,
        -1.0,
        false,
    );
}

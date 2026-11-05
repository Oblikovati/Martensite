//! Regressions of torus and cone direct-cut regressions.
#![allow(non_snake_case)]
#[path = "bop_test_utilities/mod.rs"]
mod bop_test_utilities;
use bop_test_utilities::*;
use martensite_tkbo::bop_algo::Operation;
use martensite_tkmath::gp::Point3;

fn cylinder_torus_cut(angle: Option<f64>) {
    let first = CreateCylinder(4.0, 8.0);
    let mut second = Translate(&CreateTorus(4.0, 1.0), 0.0, 0.0, 4.0);
    if let Some(angle) = angle {
        RotateAroundAxis(
            &mut second,
            Point3::new(0.0, 0.0, 4.0),
            [0.0, 0.0, 1.0],
            angle,
        );
    }
    ValidateResult(
        &PerformDirectBOP(&first, &second, Operation::Cut),
        317.718,
        -1.0,
        false,
    );
}
fn cone_cone_cut(angle: Option<f64>) {
    let first = CreateCone(8.0, 4.0, 8.0);
    let mut second = Translate(&CreateCone(4.0, 2.0, 4.0), 0.0, 0.0, 8.0);
    if let Some(angle) = angle {
        RotateAroundAxis(
            &mut second,
            Point3::new(0.0, 0.0, 2.0),
            [0.0, 0.0, 1.0],
            angle,
        );
    }
    ValidateResult(
        &PerformDirectBOP(&first, &second, Operation::Cut),
        588.519,
        -1.0,
        false,
    );
}
#[test]
fn BOPCutSimpleTest__CylinderCone_ZL1() {
    let first = CreateCylinder(4.0, 8.0);
    let mut second = Translate(&CreateCone(1.0, 6.0, 8.0), 0.0, 0.0, -1.0);
    RotateAroundAxis(
        &mut second,
        Point3::new(0.0, 0.0, 4.0),
        [0.0, 1.0, 0.0],
        -90.0,
    );
    ValidateResult(
        &PerformDirectBOP(&first, &second, Operation::Cut),
        355.82,
        -1.0,
        false,
    );
}
#[test]
fn BOPCutSimpleTest__CylinderTorus_ZL2() {
    cylinder_torus_cut(None);
}
#[test]
fn BOPCutSimpleTest__CylinderTorus_ZL3() {
    cylinder_torus_cut(Some(90.0));
}
#[test]
fn BOPCutSimpleTest__CylinderTorus_ZL4() {
    cylinder_torus_cut(Some(180.0));
}
#[test]
fn BOPCutSimpleTest__CylinderTorus_ZL5() {
    cylinder_torus_cut(Some(270.0));
}
#[test]
fn BOPCutSimpleTest__ConeCone_ZL6() {
    cone_cone_cut(None);
}
#[test]
fn BOPCutSimpleTest__ConeCone_ZL7() {
    cone_cone_cut(Some(90.0));
}
#[test]
fn BOPCutSimpleTest__ConeCone_ZL8() {
    cone_cone_cut(Some(180.0));
}
#[test]
fn BOPCutSimpleTest__ConeCone_ZL9() {
    cone_cone_cut(Some(270.0));
}

fn cone_pair_cut(second: [f64; 3], z: f64, rotation: Option<(f64, f64)>, area: f64) {
    let first = CreateCone(8.0, 4.0, 8.0);
    let mut second = Translate(&CreateCone(second[0], second[1], second[2]), 0.0, 0.0, z);
    if let Some((pivot_z, angle)) = rotation {
        RotateAroundAxis(
            &mut second,
            Point3::new(0.0, 0.0, pivot_z),
            [0.0, 0.0, 1.0],
            angle,
        );
    }
    ValidateResult(
        &PerformDirectBOP(&first, &second, Operation::Cut),
        area,
        -1.0,
        false,
    );
}
#[test]
fn BOPCutSimpleTest__ConeCone_ZM1() {
    cone_pair_cut([2.0, 1.0, 2.0], 8.0, None, 588.519);
}
#[test]
fn BOPCutSimpleTest__ConeCone_ZM2() {
    cone_pair_cut([2.0, 1.0, 2.0], 6.0, None, 619.018);
}
#[test]
fn BOPCutSimpleTest__ConeCone_ZM3() {
    cone_pair_cut([2.0, 1.0, 8.0], 0.0, None, 648.796);
}
#[test]
fn BOPCutSimpleTest__ConeCone_ZM4() {
    cone_pair_cut([2.0, 1.0, 9.0], -1.0, None, 647.221);
}
#[test]
fn BOPCutSimpleTest__ConeCone_ZM5() {
    cone_pair_cut([2.0, 1.0, 4.0], 2.0, None, 643.086);
}
#[test]
fn BOPCutSimpleTest__ConeCone_ZM6() {
    cone_pair_cut([2.0, 1.0, 10.0], -1.0, None, 649.15);
}
#[test]
fn BOPCutSimpleTest__ConeCone_ZM7() {
    cone_pair_cut([4.0, 8.0, 4.0], -4.0, None, 588.519);
}
#[test]
fn BOPCutSimpleTest__ConeCone_ZM8() {
    cone_pair_cut([4.0, 8.0, 4.0], -4.0, Some((-4.0, 90.0)), 588.519);
}
#[test]
fn BOPCutSimpleTest__ConeCone_ZM9() {
    cone_pair_cut([4.0, 8.0, 4.0], -4.0, Some((-4.0, 180.0)), 588.519);
}

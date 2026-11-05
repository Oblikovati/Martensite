//! Regressions of cylinder-cone direct-cut regressions.
#![allow(non_snake_case)]
#[path = "bop_test_utilities/mod.rs"]
mod bop_test_utilities;
use bop_test_utilities::*;
use martensite_tkbo::bop_algo::Operation;
use martensite_tkmath::gp::Point3;

fn cylinder_cone_cut(base_radius: f64, top_radius: f64, height: f64, z: f64, area: f64) {
    let first = CreateCylinder(4.0, 8.0);
    let second = Translate(&CreateCone(base_radius, top_radius, height), 0.0, 0.0, z);
    ValidateResult(
        &PerformDirectBOP(&first, &second, Operation::Cut),
        area,
        -1.0,
        false,
    );
}

#[test]
fn BOPCutSimpleTest__CylinderCone_ZJ4() {
    cylinder_cone_cut(2.0, 1.0, 4.0, 8.0, 301.593);
}
#[test]
fn BOPCutSimpleTest__CylinderCone_ZJ5() {
    cylinder_cone_cut(1.0, 2.0, 4.0, 4.0, 331.027);
}
#[test]
fn BOPCutSimpleTest__CylinderCone_ZJ6() {
    cylinder_cone_cut(2.0, 1.0, 8.0, 0.0, 361.87);
}
#[test]
fn BOPCutSimpleTest__CylinderCone_ZJ7() {
    cylinder_cone_cut(2.0, 1.0, 9.0, -1.0, 360.295);
}
#[test]
fn BOPCutSimpleTest__CylinderCone_ZJ8() {
    cylinder_cone_cut(2.0, 1.0, 4.0, 2.0, 356.16);
}
#[test]
fn BOPCutSimpleTest__CylinderCone_ZJ9() {
    cylinder_cone_cut(2.0, 1.0, 10.0, -1.0, 362.225);
}

fn cylinder_cone_rotated_cut(
    radii: [f64; 2],
    height: f64,
    z: f64,
    pivot_z: f64,
    axis: [f64; 3],
    angle: f64,
    area: f64,
) {
    let first = CreateCylinder(4.0, 8.0);
    let mut second = Translate(&CreateCone(radii[0], radii[1], height), 0.0, 0.0, z);
    RotateAroundAxis(&mut second, Point3::new(0.0, 0.0, pivot_z), axis, angle);
    ValidateResult(
        &PerformDirectBOP(&first, &second, Operation::Cut),
        area,
        -1.0,
        false,
    );
}

#[test]
fn BOPCutSimpleTest__CylinderCone_ZK1() {
    cylinder_cone_cut(4.0, 2.0, 4.0, 8.0, 301.593);
}
#[test]
fn BOPCutSimpleTest__CylinderCone_ZK2() {
    cylinder_cone_rotated_cut([4.0, 2.0], 4.0, 8.0, 8.0, [0.0, 0.0, 1.0], 90.0, 301.593);
}
#[test]
fn BOPCutSimpleTest__CylinderCone_ZK3() {
    cylinder_cone_rotated_cut([4.0, 2.0], 4.0, 8.0, 8.0, [0.0, 0.0, 1.0], 180.0, 301.593);
}
#[test]
fn BOPCutSimpleTest__CylinderCone_ZK4() {
    cylinder_cone_rotated_cut([4.0, 2.0], 4.0, 8.0, 8.0, [0.0, 0.0, 1.0], 270.0, 301.593);
}
#[test]
fn BOPCutSimpleTest__CylinderCone_ZK5() {
    cylinder_cone_rotated_cut([2.0, 1.0], 10.0, -1.0, 4.0, [0.0, 1.0, 0.0], 90.0, 358.727);
}
#[test]
fn BOPCutSimpleTest__CylinderCone_ZK6() {
    cylinder_cone_rotated_cut([2.0, 1.0], 10.0, -1.0, 4.0, [1.0, 0.0, 0.0], 90.0, 358.826);
}
#[test]
fn BOPCutSimpleTest__CylinderCone_ZK7() {
    cylinder_cone_rotated_cut([6.0, 1.0], 10.0, -1.0, 4.0, [0.0, 1.0, 0.0], 90.0, 308.057);
}
#[test]
fn BOPCutSimpleTest__CylinderCone_ZK8() {
    cylinder_cone_rotated_cut([6.0, 1.0], 10.0, -1.0, 4.0, [1.0, 0.0, 0.0], 90.0, 308.068);
}
#[test]
fn BOPCutSimpleTest__CylinderCone_ZK9() {
    cylinder_cone_rotated_cut([1.0, 6.0], 8.0, -1.0, 4.0, [0.0, 1.0, 0.0], 90.0, 355.69);
}

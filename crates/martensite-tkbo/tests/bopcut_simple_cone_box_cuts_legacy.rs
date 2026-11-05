//! Regressions of mixed primitive direct-cut regressions.
#![allow(non_snake_case)]
#[path = "bop_test_utilities/mod.rs"]
mod bop_test_utilities;
use bop_test_utilities::*;
use martensite_tkbo::bop_algo::Operation;
use martensite_tkbrep::topods::Shape;
use martensite_tkmath::gp::Point3;
fn cut(a: &Shape, b: &Shape, area: f64) {
    ValidateResult(&PerformDirectBOP(a, b, Operation::Cut), area, -1.0, false);
}
#[test]
fn BOPCutSimpleTest__ConeCone_ZN1() {
    let a = CreateCone(8., 4., 8.);
    let mut b = Translate(&CreateCone(4., 8., 4.), 0., 0., -4.);
    RotateAroundAxis(&mut b, Point3::new(0., 0., -4.), [0., 0., 1.], 270.);
    cut(&a, &b, 588.519);
}
#[test]
fn BOPCutSimpleTest__ConeCone_ZN2() {
    cut(
        &CreateCone(8., 4., 8.),
        &Translate(&CreateCone(8., 4., 4.), 4., 0., 0.),
        523.976,
    );
}
fn box_cylinder(r: f64, h: f64, p: [f64; 3], area: f64) {
    cut(
        &CreateBox(Point3::new(0., 0., 0.), 4., 4., 4.),
        &Translate(&CreateCylinder(r, h), p[0], p[1], p[2]),
        area,
    );
}
#[test]
fn BOPCutSimpleTest__BoxCylinder_ZN3() {
    box_cylinder(1., 6., [2., 2., -2.], 114.85);
}
#[test]
fn BOPCutSimpleTest__BoxCylinder_ZN4() {
    box_cylinder(1., 2., [2., 2., 1.], 114.85);
}
#[test]
fn BOPCutSimpleTest__BoxCylinder_ZN5() {
    box_cylinder(2., 4., [4., 2., 0.], 92.5664);
}
#[test]
fn BOPCutSimpleTest__BoxCylinder_ZN6() {
    box_cylinder(2., 4., [2., 0., 0.], 92.5664);
}
#[test]
fn BOPCutSimpleTest__BoxCylinder_ZN7() {
    box_cylinder(2., 4., [0., 2., 0.], 92.5664);
}
#[test]
fn BOPCutSimpleTest__BoxCylinder_ZN8() {
    box_cylinder(1., 4., [4., 2., 0.], 97.4248);
}
#[test]
fn BOPCutSimpleTest__BoxCylinder_ZN9() {
    box_cylinder(1., 4., [2., 0., 0.], 97.4248);
}

fn box_cylinder_rotated(radius: f64, position: [f64; 3], angle: Option<f64>, area: f64) {
    let first = CreateBox(Point3::new(0.0, 0.0, 0.0), 4.0, 4.0, 4.0);
    let mut second = Translate(
        &CreateCylinder(radius, 4.0),
        position[0],
        position[1],
        position[2],
    );
    if let Some(angle) = angle {
        RotateAroundAxis(
            &mut second,
            Point3::new(position[0], position[1], position[2]),
            [0.0, 0.0, 1.0],
            angle,
        );
    }
    cut(&first, &second, area);
}
#[test]
fn BOPCutSimpleTest__BoxCylinder_ZO1() {
    box_cylinder(1., 4., [0., 2., 0.], 97.4248);
}
#[test]
fn BOPCutSimpleTest__BoxCylinder_ZO2() {
    box_cylinder(1., 4., [4., 3., 0.], 97.4248);
}
#[test]
fn BOPCutSimpleTest__BoxCylinder_ZO3() {
    box_cylinder(1., 4., [3., 0., 0.], 97.4248);
}
#[test]
fn BOPCutSimpleTest__BoxCylinder_ZO4() {
    box_cylinder(1., 4., [0., 3., 0.], 97.4248);
}
#[test]
fn BOPCutSimpleTest__BoxCylinder_ZO5() {
    box_cylinder(1., 4., [1., 0., 0.], 97.4248);
}
#[test]
fn BOPCutSimpleTest__BoxCylinder_ZO6() {
    box_cylinder_rotated(4., [-2., 2., 0.], None, 70.4308);
}
#[test]
fn BOPCutSimpleTest__BoxCylinder_ZO7() {
    box_cylinder_rotated(4., [-2., 2., 0.], Some(30.), 70.4308);
}
#[test]
fn BOPCutSimpleTest__BoxCylinder_ZO8() {
    box_cylinder_rotated(4., [-2., 2., 0.], Some(90.), 70.4308);
}
#[test]
fn BOPCutSimpleTest__BoxCylinder_ZO9() {
    box_cylinder(1., 4., [3., 1., 4.], 96.);
}

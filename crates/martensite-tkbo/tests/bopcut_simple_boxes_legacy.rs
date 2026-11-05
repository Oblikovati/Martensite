//! Regressions of direct box cut regressions.
#![allow(non_snake_case)]
#[path = "bop_test_utilities/mod.rs"]
mod bop_test_utilities;
use bop_test_utilities::*;
use martensite_tkbrep::{brep::BRepBuilder, topexp::Explorer, topods::Shape};
use martensite_tkg3d::top_abs::{Orientation, ShapeEnum};
use martensite_tkmath::gp::Point3;
fn direct_cut(second: [f64; 6], area: f64, empty: bool) {
    let first = CreateBox(Point3::new(0., 0., 0.), 1., 1., 1.);
    let second = CreateBox(
        Point3::new(second[0], second[1], second[2]),
        second[3],
        second[4],
        second[5],
    );
    ValidateResult(
        &PerformDirectBOP(&first, &second, martensite_tkbo::bop_algo::Operation::Cut),
        area,
        -1.,
        empty,
    );
}
#[test]
fn BOPCutSimpleTest__Boxes_A1() {
    direct_cut([0., 0., 0., 1., 1., 1.], -1., true);
}
#[test]
fn BOPCutSimpleTest__Boxes_A2() {
    direct_cut([0., 0., 0., 1., 10.5, 1.], -1., true);
}
#[test]
fn BOPCutSimpleTest__Boxes_A3() {
    direct_cut([0., 1., 0., 1., 0.5, 1.], 6., false);
}
#[test]
fn BOPCutSimpleTest__Boxes_A4() {
    direct_cut([1., 1., 0., 1., 1., 1.], 6., false);
}
#[test]
fn BOPCutSimpleTest__Boxes_A5() {
    direct_cut([0., 0., 0., 0.5, 1., 0.5], 5.5, false);
}
#[test]
fn BOPCutSimpleTest__Boxes_A6() {
    direct_cut([0., -0.5, 0., 0.5, 0.5, 1.], 6., false);
}
#[test]
fn BOPCutSimpleTest__Boxes_A7() {
    direct_cut([0., -0.5, 0., 0.5, 1.5, 1.], 4., false);
}
#[test]
fn BOPCutSimpleTest__Boxes_A8() {
    direct_cut([0., 0.5, 0., 1., 1., 1.], 4., false);
}
#[test]
fn BOPCutSimpleTest__Boxes_A9() {
    direct_cut([0., 0.25, 0., 1., 0.5, 1.], 6., false);
}
#[test]
fn BOPCutSimpleTest__Boxes_B1() {
    direct_cut([0., 0., 0., 0.5, 0.5, 0.5], 6., false);
}
#[test]
fn BOPCutSimpleTest__Boxes_B2() {
    direct_cut([0., -0.5, 0., 0.5, 0.5, 0.5], 6., false);
}
#[test]
fn BOPCutSimpleTest__Boxes_B3() {
    direct_cut([0., -0.5, -0.5, 0.5, 0.5, 0.5], 6., false);
}
#[test]
fn BOPCutSimpleTest__Boxes_B4() {
    direct_cut([-0.5, -0.5, -0.5, 0.5, 0.5, 0.5], 6., false);
}
#[test]
fn BOPCutSimpleTest__Boxes_B5() {
    direct_cut([0., 0., 0., 1.5, 0.5, 0.5], 5.5, false);
}
#[test]
fn BOPCutSimpleTest__Boxes_B6() {
    direct_cut([0., -0.5, 0., 1.5, 0.5, 0.5], 6., false);
}
#[test]
fn BOPCutSimpleTest__Boxes_B7() {
    direct_cut([0.25, 0., 0., 0.5, 0.5, 1.], 6.5, false);
}
#[test]
fn BOPCutSimpleTest__Boxes_B8() {
    direct_cut([0.25, -0.5, 0., 0.5, 0.5, 1.], 6., false);
}
#[test]
fn BOPCutSimpleTest__Boxes_B9() {
    direct_cut([0.25, 0., 0., 0.5, 1.5, 1.], 6., false);
}
#[test]
fn BOPCutSimpleTest__Boxes_C1() {
    direct_cut([0.5, 0., 0., 1., 1., 0.5], 5.5, false);
}
#[test]
fn BOPCutSimpleTest__Boxes_C2() {
    direct_cut([0.5, 0., -0.5, 1., 1., 0.5], 6., false);
}
fn rotated_cut(width: f64, height: f64, area: f64) {
    let first = CreateBox(Point3::new(0., 0., 0.), 1., 1., 1.);
    let mut second = CreateBox(Point3::new(0., 0., 0.), width, height, 1.);
    RotateAroundAxis(&mut second, Point3::new(0., 0., 0.), [0., 0., 1.], 45.);
    ValidateResult(
        &PerformDirectBOP(&first, &second, martensite_tkbo::bop_algo::Operation::Cut),
        area,
        -1.,
        false,
    );
}
#[test]
fn BOPCutSimpleTest__Boxes_C3() {
    rotated_cut(2_f64.sqrt(), 2_f64.sqrt() / 2., 4.41421);
}
#[test]
fn BOPCutSimpleTest__Boxes_C4() {
    rotated_cut(2_f64.sqrt() / 2., 2_f64.sqrt() / 2., 5.91421);
}
#[test]
fn BOPCutSimpleTest__Boxes_C5() {
    rotated_cut(2_f64.sqrt(), 0.25, 7.03921);
}
#[test]
fn BOPCutSimpleTest__Boxes_C6() {
    let first = CreateBox(Point3::new(0., 0., 0.), 1., 1., 1.);
    let mut second = CreateBox(Point3::new(0., 0., 0.), 31_f64.sqrt() / 4., 0.25, 1.);
    RotateAroundAxis(&mut second, Point3::new(0., 0., 0.), [0., 0., 1.], 34.73);
    ValidateResult(
        &PerformDirectBOP(&first, &second, martensite_tkbo::bop_algo::Operation::Cut),
        7.21677,
        -1.,
        false,
    );
}
#[test]
fn BOPCutSimpleTest__Boxes_C8() {
    let first = CreateBox(Point3::new(0., 0., 0.), 1., 1., 1.);
    let root = 5_f64.sqrt();
    let angle = martensite_cmath::atan2(1_f64, 2.) * 180. / std::f64::consts::PI;
    let mut second = CreateBox(Point3::new(0., 0.5, 0.), root / 2., 1. / root, 1.);
    RotateAroundAxis(&mut second, Point3::new(0., 0.5, 0.), [0., 0., 1.], angle);
    ValidateResult(
        &PerformDirectBOP(&first, &second, martensite_tkbo::bop_algo::Operation::Cut),
        5.11803,
        -1.,
        false,
    );
}
#[test]
fn BOPCutSimpleTest__Boxes_D1() {
    let first = CreateBox(Point3::new(0., 0., 0.), 1., 1., 1.);
    let root = 5_f64.sqrt();
    let angle = martensite_cmath::atan2(1_f64, 2.) * 180. / std::f64::consts::PI - 90.;
    let mut second = CreateBox(Point3::new(0., 0.5, 0.), 1. / root, root / 2., 1.);
    RotateAroundAxis(&mut second, Point3::new(0., 0.5, 0.), [0., 0., 1.], angle);
    ValidateResult(
        &PerformDirectBOP(&first, &second, martensite_tkbo::bop_algo::Operation::Cut),
        7.05967,
        -1.,
        false,
    );
}
#[test]
fn BOPCutSimpleTest__Boxes_C7() {
    rotated_cut(1.5, 2_f64.sqrt() / 2., 4.41421);
}
#[test]
fn BOPCutSimpleTest__Boxes_C9() {
    rotated_cut(0.5, 0.5, 6.04289);
}
#[test]
fn BOPCutSimpleTest__Boxes_D2() {
    let first = CreateBox(Point3::new(0., 0., 0.), 1., 1., 1.);
    let mut second = CreateBox(Point3::new(-0.5, 0., 0.), 0.5, 1.7, 1.);
    RotateAroundAxis(&mut second, Point3::new(0., 0., 0.), [0., 0., 1.], -30.);
    ValidateResult(
        &PerformDirectBOP(&first, &second, martensite_tkbo::bop_algo::Operation::Cut),
        5.,
        -1.,
        false,
    );
}
#[test]
fn BOPCutSimpleTest__Boxes_D3() {
    rotated_cut(1.5, 0.25, 7.03921);
}
#[test]
fn BOPCutSimpleTest__Boxes_F1() {
    direct_cut([-0.25, 0., 0., 1.5, 0.5, 0.5], 5.5, false);
}
#[test]
fn BOPCutSimpleTest__Boxes_F2() {
    direct_cut([-0.25, -0.5, 0., 1.5, 0.5, 0.5], 6., false);
}
#[test]
fn BOPCutSimpleTest__Boxes_F3() {
    direct_cut([-0.25, -0.5, -0.5, 1.5, 0.5, 0.5], 6., false);
}
#[test]
fn BOPCutSimpleTest__Boxes_F4() {
    direct_cut([0.25, 0.25, 0., 1.5, 0.5, 0.5], 6.75, false);
}
fn pivoted_cut(width: f64, angle: f64, area: f64) {
    let first = CreateBox(Point3::new(0., 0., 0.), 1., 1., 1.);
    let mut second = CreateBox(Point3::new(0., 0.5, 0.), width, width, 1.);
    RotateAroundAxis(&mut second, Point3::new(0., 0.5, 0.), [0., 0., 1.], angle);
    ValidateResult(
        &PerformDirectBOP(&first, &second, martensite_tkbo::bop_algo::Operation::Cut),
        area,
        -1.,
        false,
    );
}
#[test]
fn BOPCutSimpleTest__Boxes_F5() {
    pivoted_cut(2_f64.sqrt() / 2., -45., 7.82843);
}
#[test]
fn BOPCutSimpleTest__Boxes_F6() {
    pivoted_cut(2_f64.sqrt() * 0.75, -45., 3.91421);
}

fn first_child(shape: &Shape, kind: ShapeEnum) -> Shape {
    let mut explorer = Explorer::new(shape, kind, ShapeEnum::Shape).unwrap();
    Iterator::next(&mut explorer).expect("expected a topological child")
}

fn reversed_cavity_shell(shape: &Shape) -> Shape {
    let mut shell = Explorer::new(shape, ShapeEnum::Shell, ShapeEnum::Shape)
        .unwrap()
        .nth(1)
        .expect("expected an inner cavity shell");
    shell.set_orientation(Orientation::Reversed);
    shell
}

#[test]
fn BOPCutSimpleTest__Topology_ZP9() {
    let outer = CreateBox(Point3::new(0., 0., 0.), 10., 10., 10.);
    let inner = CreateBox(Point3::new(2., 2., 2.), 4., 4., 4.);
    let cut = PerformCut(&outer, &inner);
    let mut cavity_only = outer.empty_copied().unwrap();
    BRepBuilder::default()
        .add(&mut cavity_only, &reversed_cavity_shell(&cut))
        .unwrap();
    let result = PerformCut(&first_child(&cut, ShapeEnum::Solid), &cavity_only);
    ValidateResult(&result, 696., -1., false);
}

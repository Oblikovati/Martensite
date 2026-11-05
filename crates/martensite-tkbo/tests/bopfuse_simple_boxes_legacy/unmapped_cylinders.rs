//! Native fuse regressions that combine transformed cylinders.
#![allow(non_snake_case)]

use super::*;

fn fused_cylinders(
    radius: f64,
    rotations: &[([f64; 3], f64)],
    translate: Option<[f64; 3]>,
    area: f64,
) {
    let first = CreateCylinder(1., 4.);
    let mut second = CreateCylinder(radius, 4.);
    for &(axis, angle) in rotations {
        RotateAroundAxis(&mut second, Point3::new(0., 0., 2.), axis, angle);
    }
    if let Some(offset) = translate {
        second = Translate(&second, offset[0], offset[1], offset[2]);
    }
    ValidateResult(&PerformFuse(&first, &second), area, -1., false);
}

#[test]
fn BOPFuseSimpleTest__Cylinders_ZD8() {
    fused_cylinders(1., &[([1., 0., 0.], 90.)], None, 46.8319);
}

#[test]
fn BOPFuseSimpleTest__Cylinders_ZD9() {
    fused_cylinders(
        1.,
        &[([1., 0., 0.], 90.), ([0., 1., 0.], 60.)],
        None,
        46.8319,
    );
}

#[test]
fn BOPFuseSimpleTest__Cylinders_ZE1() {
    fused_cylinders(
        1.,
        &[([1., 0., 0.], 90.), ([0., 1., 0.], 180.)],
        None,
        46.8319,
    );
}

#[test]
fn BOPFuseSimpleTest__Cylinders_ZE2() {
    fused_cylinders(
        1.,
        &[
            ([1., 0., 0.], 90.),
            ([0., 1., 0.], 60.),
            ([0., 0., 1.], 120.),
        ],
        None,
        46.8319,
    );
}

#[test]
fn BOPFuseSimpleTest__Cylinders_ZE3() {
    fused_cylinders(0.5, &[([1., 0., 0.], 90.)], None, 38.0586);
}

#[test]
fn BOPFuseSimpleTest__Cylinders_ZE4() {
    fused_cylinders(
        0.5,
        &[([1., 0., 0.], 90.), ([0., 1., 0.], 90.)],
        None,
        38.0584,
    );
}

#[test]
fn BOPFuseSimpleTest__Cylinders_ZE5() {
    fused_cylinders(
        0.5,
        &[([1., 0., 0.], 90.), ([0., 1., 0.], 180.)],
        None,
        38.058,
    );
}

#[test]
fn BOPFuseSimpleTest__Cylinders_ZE6() {
    fused_cylinders(
        0.5,
        &[([1., 0., 0.], 90.), ([0., 1., 0.], 270.)],
        None,
        38.0572,
    );
}

#[test]
fn BOPFuseSimpleTest__Cylinders_ZE7() {
    fused_cylinders(0.5, &[([1., 0., 0.], 90.)], Some([0.5, 0., 0.]), 38.8306);
}

#[test]
fn BOPFuseSimpleTest__Cylinders_ZE8() {
    fused_cylinders(
        0.5,
        &[([1., 0., 0.], 90.), ([0., 1., 0.], 90.)],
        Some([0.5, 0., 0.]),
        38.8304,
    );
}

#[test]
fn BOPFuseSimpleTest__Cylinders_ZE9() {
    fused_cylinders(
        0.5,
        &[([1., 0., 0.], 90.), ([0., 1., 0.], 180.)],
        Some([0.5, 0., 0.]),
        38.8309,
    );
}

#[test]
fn BOPFuseSimpleTest__Cylinders_ZF1() {
    fused_cylinders(
        0.5,
        &[([1., 0., 0.], 90.), ([0., 1., 0.], 270.)],
        Some([0.5, 0., 0.]),
        38.8307,
    );
}

fn fused_rotated_cone(radii: [f64; 2], height: f64, axis: [f64; 3], angle: f64, area: f64) {
    let first = CreateCylinder(4., 8.);
    let mut second = Translate(&CreateCone(radii[0], radii[1], height), 0., 0., -1.);
    RotateAroundAxis(&mut second, Point3::new(0., 0., 4.), axis, angle);
    ValidateResult(&PerformFuse(&first, &second), area, -1., false);
}

#[test]
fn BOPFuseSimpleTest__CylinderCone_ZK5() {
    fused_rotated_cone([2., 1.], 10., [0., 1., 0.], 90., 324.106);
}

#[test]
fn BOPFuseSimpleTest__CylinderCone_ZK6() {
    fused_rotated_cone([2., 1.], 10., [1., 0., 0.], 90., 324.207);
}

#[test]
fn BOPFuseSimpleTest__CylinderCone_ZK7() {
    fused_rotated_cone([6., 1.], 10., [0., 1., 0.], 90., 447.132);
}

#[test]
fn BOPFuseSimpleTest__CylinderCone_ZK8() {
    fused_rotated_cone([6., 1.], 10., [1., 0., 0.], 90., 447.125);
}

#[test]
fn BOPFuseSimpleTest__CylinderCone_ZK9() {
    fused_rotated_cone([1., 6.], 8., [0., 1., 0.], 90., 396.65);
}

#[test]
fn BOPFuseSimpleTest__CylinderCone_ZL1() {
    fused_rotated_cone([1., 6.], 8., [0., 1., 0.], -90., 396.823);
}

#[test]
fn BOPFuseSimpleTest__ConeCone_ZN2() {
    let first = CreateCone(8., 4., 8.);
    let second = Translate(&CreateCone(8., 4., 4.), 4., 0., 0.);
    ValidateResult(&PerformFuse(&first, &second), 712.125, -1., false);
}

fn first_solid(shape: &martensite_tkbrep::topods::Shape) -> martensite_tkbrep::topods::Shape {
    let mut explorer = martensite_tkbrep::topexp::Explorer::new(
        shape,
        martensite_tkg3d::top_abs::ShapeEnum::Solid,
        martensite_tkg3d::top_abs::ShapeEnum::Shape,
    )
    .unwrap();
    std::iter::Iterator::next(&mut explorer).expect("cut result must contain an outer solid")
}

fn reversed_cavity_shell(
    shape: &martensite_tkbrep::topods::Shape,
) -> martensite_tkbrep::topods::Shape {
    let mut explorer = martensite_tkbrep::topexp::Explorer::new(
        shape,
        martensite_tkg3d::top_abs::ShapeEnum::Shell,
        martensite_tkg3d::top_abs::ShapeEnum::Shape,
    )
    .unwrap();
    let mut shell =
        std::iter::Iterator::nth(&mut explorer, 1).expect("cut result must contain a cavity shell");
    shell.set_orientation(martensite_tkg3d::top_abs::Orientation::Reversed);
    shell
}

#[test]
fn BOPFuseSimpleTest__ReversedCavityShell_ZP5() {
    let outer = CreateBox(Point3::new(0., 0., 0.), 10., 10., 10.);
    let inner = Translate(&CreateBox(Point3::new(0., 0., 0.), 4., 4., 4.), 2., 2., 2.);
    let cut = PerformCut(&outer, &inner);
    let mut carrier = outer.empty_copied().unwrap();
    martensite_tkbrep::brep::BRepBuilder::default()
        .add(&mut carrier, &reversed_cavity_shell(&cut))
        .unwrap();
    ValidateResult(&PerformFuse(&first_solid(&cut), &carrier), 600., -1., false);
}

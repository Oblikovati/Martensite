//! Native replay of the GDML orthogonal complex-prism construction.
#![allow(non_snake_case)]

#[path = "bop_test_utilities/mod.rs"]
mod bop_test_utilities;

use bop_test_utilities::{
    CreateBox, CreateCylinder, PerformCut, PerformFuse, RotateAroundAxis, Translate,
};
use martensite_tkmath::gp::Point3;

fn final_placement(
    mut shape: martensite_tkbrep::topods::Shape,
) -> martensite_tkbrep::topods::Shape {
    RotateAroundAxis(&mut shape, Point3::default(), [0., 1., 0.], 90.);
    Translate(&shape, 508., 127., -63.5)
}

fn body_corner(z: f64, angle: f64, y: f64, height: f64) -> martensite_tkbrep::topods::Shape {
    let mut shape = CreateBox(
        Point3::new(-137.397918795, -137.397918795, z),
        274.79583759,
        274.79583759,
        height,
    );
    RotateAroundAxis(&mut shape, Point3::default(), [0., 0., 1.], angle);
    final_placement(Translate(&shape, 192.4050001, y, 0.))
}

fn cylinder_feature(y: f64, box_angle: f64, box_y: f64) -> martensite_tkbrep::topods::Shape {
    let mut box_shape = CreateBox(
        Point3::new(-192.405, -28.177059885, -388.62),
        384.81,
        56.35411977,
        777.24,
    );
    RotateAroundAxis(&mut box_shape, Point3::default(), [0., 0., 1.], box_angle);
    let box_shape = final_placement(Translate(&box_shape, 75.32580988, box_y, 0.));
    let cylinder = final_placement(Translate(
        &Translate(&CreateCylinder(190.5, 762.), 0., 0., -381.),
        190.5,
        y,
        0.,
    ));
    PerformCut(&cylinder, &box_shape)
}

#[test]
fn GdmlPublic__B7() {
    let source = "tests/boolean/gdml_public/B7";
    let body = final_placement(CreateBox(
        Point3::new(-190.5000001, -254.0000001, -381.),
        381.0000002,
        508.0000002,
        762.,
    ));
    let first_cut = PerformCut(&body, &body_corner(-396.24, 135., 255.9050001, 792.48));
    assert!(!first_cut.is_null(), "{source}: first body cut");
    let body = PerformCut(&first_cut, &body_corner(-411.48, 45., -255.9050001, 822.96));
    assert!(!body.is_null(), "{source}: second body cut");
    let lower = cylinder_feature(-254., 45., -138.82580988);
    assert!(!lower.is_null(), "{source}: lower cylinder cut");
    let upper = cylinder_feature(254., 135., 138.82580988);
    assert!(!upper.is_null(), "{source}: upper cylinder cut");
    let fused = PerformFuse(&body, &lower);
    assert!(!fused.is_null(), "{source}: first fuse");
    let result = PerformFuse(&fused, &upper);
    assert!(!result.is_null(), "{source}: second fuse");
}

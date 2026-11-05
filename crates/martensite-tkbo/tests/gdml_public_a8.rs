//! Native replay of the GDML eye construction with partial cylindrical features.
#![allow(non_snake_case)]

#[path = "bop_test_utilities/mod.rs"]
mod bop_test_utilities;

use bop_test_utilities::{
    CreateBox, CreateCylinder, PerformCut, PerformFuse, RotateAroundAxis, Translate,
};
use martensite_tkmath::gp::Point3;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeCylinder;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;

const ASSEMBLY_AXIS: [f64; 3] = [
    0.57735026918962562,
    0.57735026918962573,
    0.57735026918962573,
];

fn assembled(mut shape: martensite_tkbrep::topods::Shape) -> martensite_tkbrep::topods::Shape {
    RotateAroundAxis(&mut shape, Point3::default(), ASSEMBLY_AXIS, 120.);
    Translate(&shape, 2.5, -5., 0.)
}

fn partial_cylinder(radius: f64, height: f64, angle: f64) -> martensite_tkbrep::topods::Shape {
    let mut maker = BRepPrimAPIMakeCylinder::with_angle(radius, height, angle.to_radians())
        .expect("GDML A8 partial cylinder");
    maker
        .shape()
        .expect("GDML A8 partial cylinder shape")
        .clone()
}

fn side_cylinder(radius: f64, angle: f64) -> martensite_tkbrep::topods::Shape {
    let mut shape = if angle == 360. {
        CreateCylinder(radius, 5.)
    } else {
        partial_cylinder(radius, 5., angle)
    };
    shape = Translate(&shape, 0., 0., -2.5);
    RotateAroundAxis(&mut shape, Point3::default(), [0., 0., -1.], 90.);
    assembled(Translate(&shape, 10., 0., 0.))
}

fn panel_cylinder(
    radius: f64,
    height: f64,
    base_z: f64,
    translation: [f64; 3],
) -> martensite_tkbrep::topods::Shape {
    let mut shape = Translate(&CreateCylinder(radius, height), 0., 0., base_z);
    RotateAroundAxis(&mut shape, Point3::default(), [0., -1., 0.], 90.);
    assembled(Translate(
        &shape,
        translation[0],
        translation[1],
        translation[2],
    ))
}

#[test]
fn GdmlPublic__A8() {
    let source = "tests/boolean/gdml_public/A8";
    let outer = side_cylinder(10., 180.);
    let inner = side_cylinder(5., 360.);
    let half_disc = PerformCut(&outer, &inner);
    assert!(!half_disc.is_null(), "{source}: half-disc cut");

    let mut quarter_outer = partial_cylinder(10., 20., 90.);
    RotateAroundAxis(&mut quarter_outer, Point3::default(), [0., 1., 0.], 180.);
    quarter_outer = Translate(&quarter_outer, 0., 0., -10.);
    RotateAroundAxis(&mut quarter_outer, Point3::default(), ASSEMBLY_AXIS, -120.);
    quarter_outer = assembled(Translate(&quarter_outer, -10., 0., 7.5));
    let mut quarter_inner = partial_cylinder(5., 20., 90.);
    RotateAroundAxis(&mut quarter_inner, Point3::default(), [0., 1., 0.], 180.);
    quarter_inner = Translate(&quarter_inner, 0., 0., -10.);
    RotateAroundAxis(&mut quarter_inner, Point3::default(), ASSEMBLY_AXIS, -120.);
    quarter_inner = assembled(Translate(&quarter_inner, -10., 0., 7.5));
    let quarter_ring = PerformCut(&quarter_outer, &quarter_inner);
    assert!(!quarter_ring.is_null(), "{source}: quarter-ring cut");

    let mut panel = CreateBox(Point3::new(-10., -2.5, -10.), 20., 5., 20.);
    RotateAroundAxis(&mut panel, Point3::default(), [0., -1., 0.], 90.);
    panel = assembled(Translate(&panel, -17.5, 0., 17.5));
    let mut panel_result = panel;
    for (radius, height, base_z, translation) in [
        (1.5, 5., -2.5, [-17.5, 5., 22.5]),
        (1.5, 5., -2.5, [-17.5, -5., 22.5]),
        (1.5, 20., -10., [-20., 5., 12.5]),
        (1.5, 10., -5., [-15., -5., 12.5]),
        (2.5, 2.5, -1.25, [-18.75, 0., 17.5]),
    ] {
        panel_result = PerformCut(
            &panel_result,
            &panel_cylinder(radius, height, base_z, translation),
        );
        assert!(!panel_result.is_null(), "{source}: panel cut");
    }

    let fused = PerformFuse(&half_disc, &quarter_ring);
    assert!(!fused.is_null(), "{source}: quarter-ring fuse");
    let result = PerformFuse(&fused, &panel_result);
    assert!(!result.is_null(), "{source}: panel fuse");
}

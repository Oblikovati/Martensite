//! Native replays of fixture-free GDML Boolean regressions.
#![allow(non_snake_case)]

#[path = "bop_test_utilities/mod.rs"]
mod bop_test_utilities;

use bop_test_utilities::{CreateBox, PerformFuse, RotateAroundAxis, Translate};
use martensite_tkmath::gp::Point3;

#[test]
fn GdmlPublic__C4() {
    let primary = Translate(
        &CreateBox(Point3::new(-20., -20., -2.5), 40., 40., 5.),
        20.,
        20.,
        2.5,
    );
    let mut secondary = CreateBox(Point3::new(-5., -5., -7.0985), 10., 10., 14.197);
    RotateAroundAxis(&mut secondary, Point3::new(0., 0., 0.), [0., 0., 1.], 90.);
    let secondary = Translate(&secondary, 0., 0., 9.5985);
    let secondary = Translate(&secondary, 20., 20., 2.5);
    let result = PerformFuse(&primary, &secondary);
    assert!(!result.is_null());
}

fn gdml_c2_cylinder(radius: f64) -> martensite_tkbrep::topods::Shape {
    let mut cylinder = Translate(
        &bop_test_utilities::CreateCylinder(radius, 762.),
        0.,
        0.,
        -381.,
    );
    RotateAroundAxis(&mut cylinder, Point3::new(0., 0., 0.), [-1., 0., 0.], 90.);
    Translate(&cylinder, 63.5, 0., 254.)
}

#[test]
fn GdmlPublic__C2() {
    let result = bop_test_utilities::PerformCut(&gdml_c2_cylinder(635.), &gdml_c2_cylinder(127.));
    assert!(!result.is_null());
}

fn gdml_c3_cylinder(radius: f64) -> martensite_tkbrep::topods::Shape {
    use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeCylinder;
    use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;
    use std::f64::consts::FRAC_PI_2;

    let frame = martensite_tkmath::gp::Axis2::with_x_direction(
        Point3::new(0., 0., -381.),
        martensite_tkmath::gp::Direction3::Z,
        martensite_tkmath::gp::Direction3::new(-1., 0., 0.).unwrap(),
    )
    .unwrap();
    let mut cylinder =
        BRepPrimAPIMakeCylinder::with_axes_angle(frame, radius, 762., 3. * FRAC_PI_2)
            .unwrap()
            .shape()
            .unwrap()
            .clone();
    RotateAroundAxis(&mut cylinder, Point3::new(0., 0., 0.), [-1., 0., 0.], 90.);
    Translate(&cylinder, 63.5, 0., 254.)
}

#[test]
fn GdmlPublic__C3() {
    let result = bop_test_utilities::PerformCut(&gdml_c3_cylinder(635.), &gdml_c3_cylinder(127.));
    assert!(!result.is_null());
}

fn gdml_c5_cylinder(radius: f64) -> martensite_tkbrep::topods::Shape {
    use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeCylinder;
    use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;
    use std::f64::consts::PI;

    let frame = martensite_tkmath::gp::Axis2::with_x_direction(
        Point3::new(0., 0., -12.5),
        martensite_tkmath::gp::Direction3::Z,
        martensite_tkmath::gp::Direction3::new(-1., 0., 0.).unwrap(),
    )
    .unwrap();
    let mut cylinder = BRepPrimAPIMakeCylinder::with_axes_angle(frame, radius, 25., PI)
        .unwrap()
        .shape()
        .unwrap()
        .clone();
    RotateAroundAxis(&mut cylinder, Point3::new(0., 0., 0.), [-1., 0., 0.], 90.);
    Translate(&cylinder, 25., 0., 0.)
}

#[test]
fn GdmlPublic__C5() {
    let result = bop_test_utilities::PerformCut(&gdml_c5_cylinder(45.), &gdml_c5_cylinder(5.));
    assert!(!result.is_null());
}

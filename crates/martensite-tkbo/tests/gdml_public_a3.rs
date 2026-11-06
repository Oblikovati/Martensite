//! Native replay of the GDML nested cylinder-cut construction.
#![allow(non_snake_case)]

#[path = "bop_test_utilities/mod.rs"]
mod bop_test_utilities;

use bop_test_utilities::{CreateBox, CreateCylinder, PerformCut, RotateAroundAxis, Translate};
use martensite_tkmath::gp::Point3;

#[test]
fn GdmlPublic__A3() {
    let source = "tests/boolean/gdml_public/A3";
    let mut cylinder = Translate(&CreateCylinder(419.1, 1270.), 0., 0., -635.);
    RotateAroundAxis(
        &mut cylinder,
        Point3::default(),
        [
            0.70710678118654757,
            -4.329780281177467e-17,
            0.70710678118654757,
        ],
        180.,
    );
    cylinder = Translate(&cylinder, -12.7, 0., 0.);
    RotateAroundAxis(&mut cylinder, Point3::default(), [0., 1., 0.], 90.);
    cylinder = Translate(&cylinder, 635., 127., -139.7);

    let mut inner = CreateBox(Point3::new(-571.5, -457.2, -635.), 1143., 914.4, 1270.);
    RotateAroundAxis(&mut inner, Point3::default(), [0., -1., 0.], 90.);
    inner = Translate(&inner, -12.7, 0., 0.);
    RotateAroundAxis(&mut inner, Point3::default(), [0., 1., 0.], 90.);
    inner = Translate(&inner, 635., 127., -139.7);
    let cavity = PerformCut(&inner, &cylinder);
    assert!(!cavity.is_null(), "{source}: inner cut");

    let mut outer = CreateBox(Point3::new(-317.5, -304.8, -381.), 635., 609.6, 762.);
    RotateAroundAxis(&mut outer, Point3::default(), [0., 1., 0.], 90.);
    outer = Translate(&outer, 635., 127., -139.7);
    let result = PerformCut(&outer, &cavity);
    assert!(!result.is_null(), "{source}: outer cut");
}

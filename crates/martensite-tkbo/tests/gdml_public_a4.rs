//! Native replay of the GDML nested transformed-cut construction.
#![allow(non_snake_case)]
#[path = "bop_test_utilities/mod.rs"]
mod bop_test_utilities;
use bop_test_utilities::{CreateBox, CreateCylinder, PerformCut, RotateAroundAxis, Translate};
use martensite_tkmath::gp::Point3;
#[test]
fn GdmlPublic__A4() {
    let source = "tests/boolean/gdml_public/A4";
    let mut c = Translate(&CreateCylinder(419.1, 762.), 0., 0., -381.);
    RotateAroundAxis(
        &mut c,
        Point3::default(),
        [
            0.70710678118654757,
            -4.329780281177467e-17,
            0.70710678118654757,
        ],
        180.,
    );
    c = Translate(&c, -266.7, 0., 0.);
    RotateAroundAxis(&mut c, Point3::default(), [0., 1., 0.], 90.);
    c = Translate(&c, 635., 127., -139.7);
    let mut inner = CreateBox(Point3::new(-571.5, -457.2, -381.), 1143., 914.4, 762.);
    RotateAroundAxis(&mut inner, Point3::default(), [0., -1., 0.], 90.);
    inner = Translate(&inner, -266.7, 0., 0.);
    RotateAroundAxis(&mut inner, Point3::default(), [0., 1., 0.], 90.);
    inner = Translate(&inner, 635., 127., -139.7);
    let cut = PerformCut(&inner, &c);
    assert!(!cut.is_null(), "{source}: inner cut");
    let mut outer = CreateBox(Point3::new(-317.5, -304.8, -381.), 635., 609.6, 762.);
    RotateAroundAxis(&mut outer, Point3::default(), [0., 1., 0.], 90.);
    outer = Translate(&outer, 635., 127., -139.7);
    assert!(!PerformCut(&outer, &cut).is_null(), "{source}: outer cut");
}

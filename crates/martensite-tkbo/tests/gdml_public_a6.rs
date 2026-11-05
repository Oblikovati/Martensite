//! Native replay of the GDML cone-truncation fuse construction.
#![allow(non_snake_case)]

#[path = "bop_test_utilities/mod.rs"]
mod bop_test_utilities;

use bop_test_utilities::{CreateCone, CreateCylinder, PerformFuse, RotateAroundAxis, Translate};
use martensite_tkmath::gp::Point3;

#[test]
fn GdmlPublic__A6() {
    let source = "tests/boolean/gdml_public/A6";
    let mut cylinder = Translate(
        &CreateCylinder(555.80733567, 196.31876072),
        0.,
        0.,
        -98.15938036,
    );
    let mut cone = CreateCone(0., 555.80733567, 386.81270656);
    RotateAroundAxis(&mut cone, Point3::default(), [1., 0., 0.], 180.);
    cone = Translate(&cone, 0., 3.4891115774371428e-8, 484.97208690922002);
    RotateAroundAxis(&mut cone, Point3::default(), [1., 0., 0.], 130.8818103);
    cone = Translate(&cone, 254., -34.79562771, 204.22696378);
    RotateAroundAxis(&mut cylinder, Point3::default(), [1., 0., 0.], 130.8818103);
    cylinder = Translate(&cylinder, 254., -34.79562771, 204.22696378);
    let result = PerformFuse(&cylinder, &cone);
    assert!(!result.is_null(), "{source}: fuse result");
}

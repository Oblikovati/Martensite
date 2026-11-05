//! Native replay of the GDML multi-revolution torus construction.
#![allow(non_snake_case)]

#[path = "bop_test_utilities/mod.rs"]
mod bop_test_utilities;

use bop_test_utilities::{CreateTorus, PerformFuse, RotateAroundAxis, Translate};
use martensite_tkmath::gp::Point3;

fn rotated_torus(z: f64) -> martensite_tkbrep::topods::Shape {
    let mut shape = Translate(&CreateTorus(127., 51.71806365), 0., 0., z);
    RotateAroundAxis(&mut shape, Point3::default(), [1., 0., 0.], 90.);
    shape
}

#[test]
fn GdmlPublic__B6() {
    let source = "tests/boolean/gdml_public/B6";
    let mut result = rotated_torus(0.);
    for z in [-254., -508., 254., 508.] {
        result = PerformFuse(&result, &rotated_torus(z));
        assert!(!result.is_null(), "{source}: fuse at z={z}");
    }
}

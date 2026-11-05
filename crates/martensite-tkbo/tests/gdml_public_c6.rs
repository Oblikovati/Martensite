//! Native replay of the GDML rotor construction.
#![allow(non_snake_case)]

#[path = "bop_test_utilities/mod.rs"]
mod bop_test_utilities;

use bop_test_utilities::{
    CreateCone, CreateCylinder, PerformCut, PerformFuse, RotateAroundAxis, Translate,
};
use martensite_tkmath::gp::Point3;

fn rotor_placement(
    mut shape: martensite_tkbrep::topods::Shape,
    z: f64,
) -> martensite_tkbrep::topods::Shape {
    shape = Translate(&shape, 0., 0., z);
    RotateAroundAxis(&mut shape, Point3::default(), [1., 0., 0.], 90.);
    Translate(&shape, 0., 342.9, 0.)
}

#[test]
fn GdmlPublic__C6() {
    let source = "tests/boolean/gdml_public/C6";
    let outer_ring = rotor_placement(
        Translate(&CreateCylinder(63.5, 63.5), 0., 0., -31.75),
        247.65,
    );
    let inner_ring = rotor_placement(
        Translate(&CreateCylinder(38.1, 63.5), 0., 0., -31.75),
        247.65,
    );
    let ring = PerformCut(&outer_ring, &inner_ring);
    assert!(!ring.is_null(), "{source}: ring cut");

    let spindle = rotor_placement(
        Translate(&CreateCylinder(38.1, 152.4), 0., 0., -76.2),
        292.1,
    );
    let fused = PerformFuse(&spindle, &ring);
    assert!(!fused.is_null(), "{source}: spindle fuse");

    let mut cone = Translate(&CreateCone(76.2, 101.6, 76.2), 0., 0., 228.6);
    RotateAroundAxis(&mut cone, Point3::default(), [1., 0., 0.], 90.);
    cone = Translate(&cone, 0., 342.9, 0.);
    let result = PerformCut(&cone, &fused);
    assert!(!result.is_null(), "{source}: final cut");
}

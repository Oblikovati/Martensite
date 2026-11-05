//! Native replay of the GDML transformed-box cut construction.
#![allow(non_snake_case)]
#[path = "bop_test_utilities/mod.rs"]
mod bop_test_utilities;
use bop_test_utilities::{CreateBox, PerformCut, RotateAroundAxis, Translate};
use martensite_tkmath::gp::Point3;
#[test]
fn GdmlPublic__A7() {
    let source = "tests/boolean/gdml_public/A7";
    let mut tool = CreateBox(Point3::new(-254., -190.5, -381.), 508., 381., 762.);
    RotateAroundAxis(
        &mut tool,
        Point3::default(),
        [
            -0.63331020410333672,
            0.75474971096672405,
            -0.17108786974603546,
        ],
        91.727941072350504,
    );
    tool = Translate(&tool, 12.702649262360318, -217.06072879313561, 182.19951927);
    RotateAroundAxis(
        &mut tool,
        Point3::default(),
        [0., 0., 1.],
        150.00000000000003,
    );
    tool = Translate(&tool, -97.52954744, -202.95567884, 508.);
    let mut object = CreateBox(Point3::new(-254., -381., -381.), 508., 762., 762.);
    RotateAroundAxis(
        &mut object,
        Point3::default(),
        [0., 0., 1.],
        150.00000000000003,
    );
    object = Translate(&object, -97.52954744, -202.95567884, 508.);
    assert!(
        !PerformCut(&object, &tool).is_null(),
        "{source}: cut result"
    );
}

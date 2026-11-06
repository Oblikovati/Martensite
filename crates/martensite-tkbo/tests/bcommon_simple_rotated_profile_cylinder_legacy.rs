//! Regression of a rotated profile-prism and cylinder common regression.
#![allow(non_snake_case)]

#[path = "bop_test_utilities/mod.rs"]
mod bop_test_utilities;
use bop_test_utilities::*;
use martensite_tkmath::gp::{Point3, Vector3};
#[test]
fn BCommonSimpleTest__RotatedProfileCylinder_H9() {
    let first = CreatePolygonWire(
        &[
            Point3::new(-1., -1., 0.),
            Point3::new(1.8660254037844386, -1., 0.),
            Point3::new(1.8660254037844386, 2., 0.),
            Point3::new(-1., 2., 0.),
        ],
        true,
    );
    let mut first = CreatePrism(&CreateFaceFromWire(&first), Vector3::new(0., 0., 1.));
    RotateAroundAxis(&mut first, Point3::new(0., 0., 0.), [0., 0., 1.], 60.);
    ValidateResult(
        &PerformCommon(&first, &CreateCylinder(1., 2.)),
        12.5664,
        -1.,
        false,
    );
}

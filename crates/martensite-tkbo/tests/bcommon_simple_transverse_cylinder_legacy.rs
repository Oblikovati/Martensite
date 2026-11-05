//! Regression of a transverse circular-profile prism common regression.
#![allow(non_snake_case)]

#[path = "bop_test_utilities/mod.rs"]
mod bop_test_utilities;

use bop_test_utilities::*;
use martensite_tkmath::gp::{Direction3, Plane3, Point3};

#[test]
fn BCommonSimpleTest__TransverseCylinder_I9() {
    let axial = CreateCylinder(100.0, 400.0);
    let plane = Plane3::new(
        Point3::new(-150.0, 0.0, 200.0),
        Direction3::new(1.0, 0.0, 0.0).unwrap(),
    )
    .unwrap();
    let transverse = CreateCylinderOnPlane(&plane, 100.0, 300.0);
    ValidateResult(&PerformCommon(&axial, &transverse), 160000.0, -1.0, false);
}

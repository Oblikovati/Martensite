//! Regressions of transformed cylinder-torus common regressions.
#![allow(non_snake_case)]

#[path = "bop_test_utilities/mod.rs"]
mod bop_test_utilities;
use bop_test_utilities::*;
use martensite_tkmath::gp::Point3;

fn common_cylinder_torus(angle: f64) {
    let cylinder = CreateCylinder(4.0, 8.0);
    let mut torus = Translate(&CreateTorus(4.0, 1.0), 0.0, 0.0, 4.0);
    RotateAroundAxis(
        &mut torus,
        Point3::new(0.0, 0.0, 4.0),
        [0.0, 0.0, 1.0],
        angle,
    );
    ValidateResult(&PerformCommon(&cylinder, &torus), 116.656, -1.0, false);
}

#[test]
fn BOPCommonSimpleTest__CylinderTorus_ZL3() {
    common_cylinder_torus(90.0);
}
#[test]
fn BOPCommonSimpleTest__CylinderTorus_ZL4() {
    common_cylinder_torus(180.0);
}
#[test]
fn BOPCommonSimpleTest__CylinderTorus_ZL5() {
    common_cylinder_torus(270.0);
}

// Test bopcommon_simple/ZL2: translated cylinder and torus; common; checkprops result -s 116.656
#[test]
fn BOPCommonSimpleTest__CylinderTorus_ZL2() {
    let cylinder = CreateCylinder(4.0, 8.0);
    let torus = Translate(&CreateTorus(4.0, 1.0), 0.0, 0.0, 4.0);
    ValidateResult(&PerformCommon(&cylinder, &torus), 116.656, -1.0, false);
}

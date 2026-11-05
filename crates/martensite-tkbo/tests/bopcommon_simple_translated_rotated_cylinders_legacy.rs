//! Regressions of translated and rotated cylinder common regressions.
#![allow(non_snake_case)]

#[path = "bop_test_utilities/mod.rs"]
mod bop_test_utilities;
use bop_test_utilities::*;
use martensite_tkmath::gp::Point3;

fn common_translated_rotated_cylinders(angle: f64) {
    let first = CreateCylinder(1.0, 2.0);
    let mut second = Translate(&CreateCylinder(1.0, 2.0), 0.0, 0.0, 2.0);
    RotateAroundAxis(
        &mut second,
        Point3::new(0.0, 0.0, 0.0),
        [0.0, 0.0, 1.0],
        angle,
    );
    ValidateResult(&PerformCommon(&first, &second), -1.0, -1.0, true);
}

// Test bopcommon_simple/ZD5: translated cylinder rotated 90 degrees; common; checkprops result -s empty
#[test]
fn BOPCommonSimpleTest__TranslatedRotatedCylinders_ZD5() {
    common_translated_rotated_cylinders(90.0);
}

// Test bopcommon_simple/ZD6: translated cylinder rotated 180 degrees; common; checkprops result -s empty
#[test]
fn BOPCommonSimpleTest__TranslatedRotatedCylinders_ZD6() {
    common_translated_rotated_cylinders(180.0);
}

// Test bopcommon_simple/ZD7: translated cylinder rotated 270 degrees; common; checkprops result -s empty
#[test]
fn BOPCommonSimpleTest__TranslatedRotatedCylinders_ZD7() {
    common_translated_rotated_cylinders(270.0);
}

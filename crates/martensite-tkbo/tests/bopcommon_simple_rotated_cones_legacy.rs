//! Regressions of translated and rotated cone common regressions.
#![allow(non_snake_case)]

#[path = "bop_test_utilities/mod.rs"]
mod bop_test_utilities;
use bop_test_utilities::*;
use martensite_tkmath::gp::Point3;

fn common_rotated_cones(
    first: [f64; 3],
    second: [f64; 3],
    translation: [f64; 3],
    pivot: [f64; 3],
    angle: f64,
    expected_area: f64,
    empty: bool,
) {
    let first = CreateCone(first[0], first[1], first[2]);
    let mut second = Translate(
        &CreateCone(second[0], second[1], second[2]),
        translation[0],
        translation[1],
        translation[2],
    );
    RotateAboutZ(
        &mut second,
        Point3::new(pivot[0], pivot[1], pivot[2]),
        angle,
    );
    let result = PerformCommon(&first, &second);
    ValidateResult(&result, expected_area, -1.0, empty);
}

macro_rules! common_rotated_cones {
    ($name:ident, $first:expr, $second:expr, $translation:expr, $pivot:expr, $angle:expr, $area:expr, $empty:expr) => {
        #[test]
        fn $name() {
            common_rotated_cones($first, $second, $translation, $pivot, $angle, $area, $empty);
        }
    };
}

// Test bopcommon_simple/ZL7: translated and rotated cones; common; checkprops result -s empty
common_rotated_cones!(
    BOPCommonSimpleTest__RotatedCones_ZL7,
    [8.0, 4.0, 8.0],
    [4.0, 2.0, 4.0],
    [0.0, 0.0, 8.0],
    [0.0, 0.0, 2.0],
    90.0,
    -1.0,
    true
);

// Test bopcommon_simple/ZL8: translated and rotated cones; common; checkprops result -s empty
common_rotated_cones!(
    BOPCommonSimpleTest__RotatedCones_ZL8,
    [8.0, 4.0, 8.0],
    [4.0, 2.0, 4.0],
    [0.0, 0.0, 8.0],
    [0.0, 0.0, 2.0],
    180.0,
    -1.0,
    true
);

// Test bopcommon_simple/ZL9: translated and rotated cones; common; checkprops result -s empty
common_rotated_cones!(
    BOPCommonSimpleTest__RotatedCones_ZL9,
    [8.0, 4.0, 8.0],
    [4.0, 2.0, 4.0],
    [0.0, 0.0, 8.0],
    [0.0, 0.0, 2.0],
    270.0,
    -1.0,
    true
);

// Test bopcommon_simple/ZM8: translated and rotated cones; common; checkprops result -s empty
common_rotated_cones!(
    BOPCommonSimpleTest__RotatedCones_ZM8,
    [8.0, 4.0, 8.0],
    [4.0, 8.0, 4.0],
    [0.0, 0.0, -4.0],
    [0.0, 0.0, -4.0],
    90.0,
    -1.0,
    true
);

// Test bopcommon_simple/ZM9: translated and rotated cones; common; checkprops result -s empty
common_rotated_cones!(
    BOPCommonSimpleTest__RotatedCones_ZM9,
    [8.0, 4.0, 8.0],
    [4.0, 8.0, 4.0],
    [0.0, 0.0, -4.0],
    [0.0, 0.0, -4.0],
    180.0,
    -1.0,
    true
);

// Test bopcommon_simple/ZN1: translated and rotated cones; common; checkprops result -s empty
common_rotated_cones!(
    BOPCommonSimpleTest__RotatedCones_ZN1,
    [8.0, 4.0, 8.0],
    [4.0, 8.0, 4.0],
    [0.0, 0.0, -4.0],
    [0.0, 0.0, -4.0],
    270.0,
    -1.0,
    true
);

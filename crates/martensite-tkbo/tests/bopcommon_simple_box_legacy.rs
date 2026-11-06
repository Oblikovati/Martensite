//! Regressions of the self-contained `bopcommon_simple` box regressions.
#![allow(non_snake_case)]

#[path = "bop_test_utilities/mod.rs"]
mod bop_test_utilities;
use bop_test_utilities::*;
use martensite_tkmath::gp::Point3;

fn common_box_case(first: [f64; 6], second: [f64; 6], expected_area: f64, empty: bool) {
    let first = CreateBox(
        Point3::new(first[0], first[1], first[2]),
        first[3],
        first[4],
        first[5],
    );
    let second = CreateBox(
        Point3::new(second[0], second[1], second[2]),
        second[3],
        second[4],
        second[5],
    );
    let result = PerformCommon(&first, &second);
    ValidateResult(&result, expected_area, -1.0, empty);
}

macro_rules! common_box_case {
    ($name:ident, $first:expr, $second:expr, $area:expr, $empty:expr) => {
        #[test]
        fn $name() {
            common_box_case($first, $second, $area, $empty);
        }
    };
}

// Test bopcommon_simple/A2: two axis-aligned boxes; common; checkprops result -s 6
common_box_case!(
    BOPCommonSimpleTest__Boxes_A2,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.0, 0.0, 0.0, 1.0, 1.5, 1.0],
    6.0,
    false
);

// Test bopcommon_simple/A3: two axis-aligned boxes; common; checkprops result -s empty
common_box_case!(
    BOPCommonSimpleTest__Boxes_A3,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.0, 1.0, 0.0, 1.0, 0.5, 1.0],
    -1.0,
    true
);

// Test bopcommon_simple/A4: two axis-aligned boxes; common; checkprops result -s empty
common_box_case!(
    BOPCommonSimpleTest__Boxes_A4,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [1.0, 1.0, 0.0, 1.0, 1.0, 1.0],
    -1.0,
    true
);

// Test bopcommon_simple/A5: two axis-aligned boxes; common; checkprops result -s 2.5
common_box_case!(
    BOPCommonSimpleTest__Boxes_A5,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.0, 0.0, 0.0, 0.5, 1.0, 0.5],
    2.5,
    false
);

// Test bopcommon_simple/A6: two axis-aligned boxes; common; checkprops result -s empty
common_box_case!(
    BOPCommonSimpleTest__Boxes_A6,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.0, -0.5, 0.0, 0.5, 0.5, 1.0],
    -1.0,
    true
);

// Test bopcommon_simple/A7: two axis-aligned boxes; common; checkprops result -s 4
common_box_case!(
    BOPCommonSimpleTest__Boxes_A7,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.0, -0.5, 0.0, 0.5, 1.5, 1.0],
    4.0,
    false
);

// Test bopcommon_simple/A8: two axis-aligned boxes; common; checkprops result -s 4
common_box_case!(
    BOPCommonSimpleTest__Boxes_A8,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.0, 0.5, 0.0, 1.0, 1.0, 1.0],
    4.0,
    false
);

// Test bopcommon_simple/A9: two axis-aligned boxes; common; checkprops result -s 4
common_box_case!(
    BOPCommonSimpleTest__Boxes_A9,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.0, 0.25, 0.0, 1.0, 0.5, 1.0],
    4.0,
    false
);

// Test bopcommon_simple/B1: two axis-aligned boxes; common; checkprops result -s 1.5
common_box_case!(
    BOPCommonSimpleTest__Boxes_B1,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.0, 0.0, 0.0, 0.5, 0.5, 0.5],
    1.5,
    false
);

// Test bopcommon_simple/B2: two axis-aligned boxes; common; checkprops result -s empty
common_box_case!(
    BOPCommonSimpleTest__Boxes_B2,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.0, -0.5, 0.0, 0.5, 0.5, 0.5],
    -1.0,
    true
);

// Test bopcommon_simple/B3: two axis-aligned boxes; common; checkprops result -s empty
common_box_case!(
    BOPCommonSimpleTest__Boxes_B3,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.0, -0.5, -0.5, 0.5, 0.5, 0.5],
    -1.0,
    true
);

// Test bopcommon_simple/B4: two axis-aligned boxes; common; checkprops result -s empty
common_box_case!(
    BOPCommonSimpleTest__Boxes_B4,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [-0.5, -0.5, -0.5, 0.5, 0.5, 0.5],
    -1.0,
    true
);

// Test bopcommon_simple/B5: two axis-aligned boxes; common; checkprops result -s 2.5
common_box_case!(
    BOPCommonSimpleTest__Boxes_B5,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.0, 0.0, 0.0, 1.5, 0.5, 0.5],
    2.5,
    false
);

// Test bopcommon_simple/B6: two axis-aligned boxes; common; checkprops result -s empty
common_box_case!(
    BOPCommonSimpleTest__Boxes_B6,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.0, -0.5, 0.0, 1.5, 0.5, 0.5],
    -1.0,
    true
);

// Test bopcommon_simple/B7: two axis-aligned boxes; common; checkprops result -s 2.5
common_box_case!(
    BOPCommonSimpleTest__Boxes_B7,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.25, 0.0, 0.0, 0.5, 0.5, 1.0],
    2.5,
    false
);

// Test bopcommon_simple/B8: two axis-aligned boxes; common; checkprops result -s empty
common_box_case!(
    BOPCommonSimpleTest__Boxes_B8,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.25, -0.5, 0.0, 0.5, 0.5, 1.0],
    -1.0,
    true
);

// Test bopcommon_simple/B9: two axis-aligned boxes; common; checkprops result -s 4
common_box_case!(
    BOPCommonSimpleTest__Boxes_B9,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.25, 0.0, 0.0, 0.5, 1.5, 1.0],
    4.0,
    false
);

// Test bopcommon_simple/C1: two axis-aligned boxes; common; checkprops result -s 2.5
common_box_case!(
    BOPCommonSimpleTest__Boxes_C1,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.5, 0.0, 0.0, 1.0, 1.0, 0.5],
    2.5,
    false
);

// Test bopcommon_simple/C2: two axis-aligned boxes; common; checkprops result -s empty
common_box_case!(
    BOPCommonSimpleTest__Boxes_C2,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.5, 0.0, -0.5, 1.0, 1.0, 0.5],
    -1.0,
    true
);

// Test bopcommon_simple/E2: two axis-aligned boxes; common; checkprops result -s 1.5
common_box_case!(
    BOPCommonSimpleTest__Boxes_E2,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.25, 0.0, 0.0, 0.5, 0.5, 0.5],
    1.5,
    false
);

// Test bopcommon_simple/E3: two axis-aligned boxes; common; checkprops result -s empty
common_box_case!(
    BOPCommonSimpleTest__Boxes_E3,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.25, -0.5, 0.0, 0.5, 0.5, 0.5],
    -1.0,
    true
);

// Test bopcommon_simple/E4: two axis-aligned boxes; common; checkprops result -s empty
common_box_case!(
    BOPCommonSimpleTest__Boxes_E4,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.25, -0.5, -0.5, 0.5, 0.5, 0.5],
    -1.0,
    true
);

// Test bopcommon_simple/E5: two axis-aligned boxes; common; checkprops result -s 2.5
common_box_case!(
    BOPCommonSimpleTest__Boxes_E5,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.25, 0.0, 0.0, 0.5, 1.5, 0.5],
    2.5,
    false
);

// Test bopcommon_simple/E6: two axis-aligned boxes; common; checkprops result -s 2
common_box_case!(
    BOPCommonSimpleTest__Boxes_E6,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.25, 0.0, 0.0, 1.5, 0.5, 0.5],
    2.0,
    false
);

// Test bopcommon_simple/E7: two axis-aligned boxes; common; checkprops result -s empty
common_box_case!(
    BOPCommonSimpleTest__Boxes_E7,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.25, -0.5, 0.0, 1.5, 0.5, 0.5],
    -1.0,
    true
);

// Test bopcommon_simple/E8: two axis-aligned boxes; common; checkprops result -s empty
common_box_case!(
    BOPCommonSimpleTest__Boxes_E8,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.25, -0.5, -0.5, 1.5, 0.5, 0.5],
    -1.0,
    true
);

// Test bopcommon_simple/E9: two axis-aligned boxes; common; checkprops result -s 3.25
common_box_case!(
    BOPCommonSimpleTest__Boxes_E9,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.25, 0.0, 0.0, 1.5, 1.5, 0.5],
    3.25,
    false
);

// Test bopcommon_simple/F1: two axis-aligned boxes; common; checkprops result -s 2.5
common_box_case!(
    BOPCommonSimpleTest__Boxes_F1,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [-0.25, 0.0, 0.0, 1.5, 0.5, 0.5],
    2.5,
    false
);

// Test bopcommon_simple/F2: two axis-aligned boxes; common; checkprops result -s empty
common_box_case!(
    BOPCommonSimpleTest__Boxes_F2,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [-0.25, -0.5, 0.0, 1.5, 0.5, 0.5],
    -1.0,
    true
);

// Test bopcommon_simple/F3: two axis-aligned boxes; common; checkprops result -s empty
common_box_case!(
    BOPCommonSimpleTest__Boxes_F3,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [-0.25, -0.5, -0.5, 1.5, 0.5, 0.5],
    -1.0,
    true
);

// Test bopcommon_simple/F4: two axis-aligned boxes; common; checkprops result -s 2
common_box_case!(
    BOPCommonSimpleTest__Boxes_F4,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.25, 0.25, 0.0, 1.5, 0.5, 0.5],
    2.0,
    false
);

// Test bopcommon_simple/J9: two axis-aligned boxes; common; checkprops result -s 1.125
common_box_case!(
    BOPCommonSimpleTest__Boxes_J9,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [-0.25, -0.25, 0.0, 0.5, 0.5, 1.0],
    1.125,
    false
);

// Test bopcommon_simple/K1: two axis-aligned boxes; common; checkprops result -s 2.5
common_box_case!(
    BOPCommonSimpleTest__Boxes_K1,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.25, 0.25, 0.0, 0.5, 0.5, 1.0],
    2.5,
    false
);

// Test bopcommon_simple/K5: two axis-aligned boxes; common; checkprops result -s 4
common_box_case!(
    BOPCommonSimpleTest__Boxes_K5,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.25, -0.25, 0.0, 0.5, 1.5, 1.0],
    4.0,
    false
);

// Test bopcommon_simple/P3: two axis-aligned boxes; common; checkprops result -s 0.625
common_box_case!(
    BOPCommonSimpleTest__Boxes_P3,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [-0.25, -0.25, 0.5, 0.5, 0.5, 1.0],
    0.625,
    false
);

// Test bopcommon_simple/P4: two axis-aligned boxes; common; checkprops result -s 1.5
common_box_case!(
    BOPCommonSimpleTest__Boxes_P4,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.25, 0.5, 0.5, 0.5, 0.5, 1.0],
    1.5,
    false
);

// Test bopcommon_simple/P8: two axis-aligned boxes; common; checkprops result -s 2.5
common_box_case!(
    BOPCommonSimpleTest__Boxes_P8,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.25, -0.25, 0.5, 0.5, 1.5, 1.0],
    2.5,
    false
);

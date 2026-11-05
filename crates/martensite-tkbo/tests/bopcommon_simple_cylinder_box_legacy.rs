//! Regressions of self-contained cylinder-box common regressions.
#![allow(non_snake_case)]

#[path = "bop_test_utilities/mod.rs"]
mod bop_test_utilities;
use bop_test_utilities::*;
use martensite_tkmath::gp::Point3;

fn common_cylinder_box_case(
    radius: f64,
    height: f64,
    box_values: [f64; 6],
    expected_area: f64,
    empty: bool,
) {
    let cylinder = CreateCylinder(radius, height);
    let box_shape = CreateBox(
        Point3::new(box_values[0], box_values[1], box_values[2]),
        box_values[3],
        box_values[4],
        box_values[5],
    );
    let result = PerformCommon(&cylinder, &box_shape);
    ValidateResult(&result, expected_area, -1.0, empty);
}

macro_rules! common_cylinder_box_case {
    ($name:ident, $radius:expr, $height:expr, $box_values:expr, $area:expr, $empty:expr) => {
        #[test]
        fn $name() {
            common_cylinder_box_case($radius, $height, $box_values, $area, $empty);
        }
    };
}

// Test bopcommon_simple/R1: cylinder and box; common; checkprops result -s 18.8496
common_cylinder_box_case!(
    BOPCommonSimpleTest__CylinderBox_R1,
    1.0,
    2.0,
    [-1.0, -1.0, 0.0, 2.0, 2.0, 2.0],
    18.8496,
    false
);

// Test bopcommon_simple/R3: cylinder and box; common; checkprops result -s 16.8965
common_cylinder_box_case!(
    BOPCommonSimpleTest__CylinderBox_R3,
    1.0,
    2.0,
    [-1.0, -1.0, 0.0, 2.0, 1.5, 2.0],
    16.8965,
    false
);

// Test bopcommon_simple/R5: cylinder and box; common; checkprops result -s 16.8965
common_cylinder_box_case!(
    BOPCommonSimpleTest__CylinderBox_R5,
    1.0,
    2.0,
    [-1.0, -1.0, 0.0, 1.5, 2.0, 2.0],
    16.8965,
    false
);

// Test bopcommon_simple/R6: cylinder and box; common; checkprops result -s 18.8496
common_cylinder_box_case!(
    BOPCommonSimpleTest__CylinderBox_R6,
    1.0,
    2.0,
    [-1.0, -1.0, 0.0, 2.0, 2.5, 2.0],
    18.8496,
    false
);

// Test bopcommon_simple/R7: cylinder and box; common; checkprops result -s 18.8496
common_cylinder_box_case!(
    BOPCommonSimpleTest__CylinderBox_R7,
    1.0,
    2.0,
    [-1.0, -1.0, 0.0, 2.5, 2.0, 2.0],
    18.8496,
    false
);

// Test bopcommon_simple/R8: cylinder and box; common; checkprops result -s 13.4248
common_cylinder_box_case!(
    BOPCommonSimpleTest__CylinderBox_R8,
    1.0,
    2.0,
    [-1.0, -1.0, 0.0, 1.0, 2.0, 2.0],
    13.4248,
    false
);

// Test bopcommon_simple/R9: cylinder and box; common; checkprops result -s 13.4248
common_cylinder_box_case!(
    BOPCommonSimpleTest__CylinderBox_R9,
    1.0,
    2.0,
    [0.0, -1.0, 0.0, 1.0, 2.0, 2.0],
    13.4248,
    false
);

// Test bopcommon_simple/S1: cylinder and box; common; checkprops result -s 13.4248
common_cylinder_box_case!(
    BOPCommonSimpleTest__CylinderBox_S1,
    1.0,
    2.0,
    [-1.0, -1.0, 0.0, 2.0, 1.0, 2.0],
    13.4248,
    false
);

// Test bopcommon_simple/S4: cylinder and box; common; checkprops result -s 17.0816
common_cylinder_box_case!(
    BOPCommonSimpleTest__CylinderBox_S4,
    1.0,
    2.0,
    [-1.0, -1.0, 0.0, 1.7071067811865475, 1.7071067811865475, 2.0],
    17.0816,
    false
);

// Test bopcommon_simple/S5: cylinder and box; common; checkprops result -s 17.0816
common_cylinder_box_case!(
    BOPCommonSimpleTest__CylinderBox_S5,
    1.0,
    2.0,
    [
        -0.70710678118654757,
        -0.70710678118654757,
        0.0,
        1.7071067811865475,
        1.7071067811865475,
        2.0
    ],
    17.0816,
    false
);

// Test bopcommon_simple/S6: cylinder and box; common; checkprops result -s 17.8475
common_cylinder_box_case!(
    BOPCommonSimpleTest__CylinderBox_S6,
    1.0,
    2.0,
    [
        -0.80000000000000004,
        -0.80000000000000004,
        0.0,
        1.8,
        1.8,
        2.0
    ],
    17.8475,
    false
);

// Test bopcommon_simple/S7: cylinder and box; common; checkprops result -s 17.8475
common_cylinder_box_case!(
    BOPCommonSimpleTest__CylinderBox_S7,
    1.0,
    2.0,
    [-1.0, -0.80000000000000004, 0.0, 1.8, 1.8, 2.0],
    17.8475,
    false
);

// Test bopcommon_simple/S9: cylinder and box; common; checkprops result -s 17.8475
common_cylinder_box_case!(
    BOPCommonSimpleTest__CylinderBox_S9,
    1.0,
    2.0,
    [
        -1.0,
        -0.80000000000000004,
        0.0,
        2.0,
        1.6000000000000001,
        2.0
    ],
    17.8475,
    false
);

// Test bopcommon_simple/T1: cylinder and box; common; checkprops result -s 17.8475
common_cylinder_box_case!(
    BOPCommonSimpleTest__CylinderBox_T1,
    1.0,
    2.0,
    [
        -0.80000000000000004,
        -1.0,
        0.0,
        1.6000000000000001,
        2.0,
        2.0
    ],
    17.8475,
    false
);

// Test bopcommon_simple/T3: cylinder and box; common; checkprops result -s 14.6841
common_cylinder_box_case!(
    BOPCommonSimpleTest__CylinderBox_T3,
    1.0,
    2.0,
    [-0.5, -1.0, 0.0, 1.5, 1.5, 2.0],
    14.6841,
    false
);

// Test bopcommon_simple/T5: cylinder and box; common; checkprops result -s 14.6841
common_cylinder_box_case!(
    BOPCommonSimpleTest__CylinderBox_T5,
    1.0,
    2.0,
    [-1.0, -1.0, 0.0, 1.5, 1.5, 2.0],
    14.6841,
    false
);

// Test bopcommon_simple/T6: cylinder and box; common; checkprops result -s 16.1977
common_cylinder_box_case!(
    BOPCommonSimpleTest__CylinderBox_T6,
    1.0,
    2.0,
    [
        -0.70710678118654757,
        -0.70710678118654757,
        0.0,
        1.7071067811865475,
        1.4142135623730951,
        2.0
    ],
    16.1977,
    false
);

// Test bopcommon_simple/T9: cylinder and box; common; checkprops result -s 16.1977
common_cylinder_box_case!(
    BOPCommonSimpleTest__CylinderBox_T9,
    1.0,
    2.0,
    [
        -0.70710678118654757,
        -0.70710678118654757,
        0.0,
        1.4142135623730951,
        1.7071067811865475,
        2.0
    ],
    16.1977,
    false
);

// Test bopcommon_simple/U3: cylinder and box; common; checkprops result -s 17.3465
common_cylinder_box_case!(
    BOPCommonSimpleTest__CylinderBox_U3,
    1.0,
    2.0,
    [
        -0.80000000000000004,
        -1.0,
        0.0,
        1.6000000000000001,
        1.8,
        2.0
    ],
    17.3465,
    false
);

// Test bopcommon_simple/U4: cylinder and box; common; checkprops result -s 13.4248
common_cylinder_box_case!(
    BOPCommonSimpleTest__CylinderBox_U4,
    1.0,
    2.0,
    [-1.0, -1.1000000000000001, 0.0, 1.0, 2.2000000000000002, 2.0],
    13.4248,
    false
);

// Test bopcommon_simple/U5: cylinder and box; common; checkprops result -s 13.4248
common_cylinder_box_case!(
    BOPCommonSimpleTest__CylinderBox_U5,
    1.0,
    2.0,
    [0.0, -1.1000000000000001, 0.0, 1.0, 2.2000000000000002, 2.0],
    13.4248,
    false
);

// Test bopcommon_simple/U6: cylinder and box; common; checkprops result -s 13.4248
common_cylinder_box_case!(
    BOPCommonSimpleTest__CylinderBox_U6,
    1.0,
    2.0,
    [-1.1000000000000001, -1.0, 0.0, 2.2000000000000002, 1.0, 2.0],
    13.4248,
    false
);

// Test bopcommon_simple/U7: cylinder and box; common; checkprops result -s empty
common_cylinder_box_case!(
    BOPCommonSimpleTest__CylinderBox_U7,
    1.0,
    2.0,
    [-1.0, 1.0, 0.0, 2.0, 2.0, 2.0],
    -1.0,
    true
);

// Test bopcommon_simple/U8: cylinder and box; common; checkprops result -s empty
common_cylinder_box_case!(
    BOPCommonSimpleTest__CylinderBox_U8,
    1.0,
    2.0,
    [1.0, -1.0, 0.0, 2.0, 2.0, 2.0],
    -1.0,
    true
);

// Test bopcommon_simple/V1: cylinder and box; common; checkprops result -s 15.3137
common_cylinder_box_case!(
    BOPCommonSimpleTest__CylinderBox_V1,
    1.0,
    2.0,
    [
        -0.70710678118654757,
        -0.70710678118654757,
        0.0,
        1.4142135623730951,
        1.4142135623730951,
        2.0
    ],
    15.3137,
    false
);

// Test bopcommon_simple/V4: cylinder and box; common; checkprops result -s 16.0004
common_cylinder_box_case!(
    BOPCommonSimpleTest__CylinderBox_V4,
    1.0,
    2.0,
    [
        -0.70710678118654757,
        -0.70710678118654757,
        0.0,
        1.6000000000000001,
        1.4142135623730951,
        2.0
    ],
    16.0004,
    false
);

// Test bopcommon_simple/V5: cylinder and box; common; checkprops result -s 12.5664
common_cylinder_box_case!(
    BOPCommonSimpleTest__CylinderBox_V5,
    1.0,
    2.0,
    [-1.0, -1.0, 0.0, 2.0, 2.0, 1.0],
    12.5664,
    false
);

// Test bopcommon_simple/V7: cylinder and box; common; checkprops result -s 10.9757
common_cylinder_box_case!(
    BOPCommonSimpleTest__CylinderBox_V7,
    1.0,
    2.0,
    [-1.0, -1.0, 0.0, 2.0, 1.5, 1.0],
    10.9757,
    false
);

// Test bopcommon_simple/V9: cylinder and box; common; checkprops result -s 10.9757
common_cylinder_box_case!(
    BOPCommonSimpleTest__CylinderBox_V9,
    1.0,
    2.0,
    [-1.0, -1.0, 0.0, 1.5, 2.0, 1.0],
    10.9757,
    false
);

// Test bopcommon_simple/W1: cylinder and box; common; checkprops result -s 12.5664
common_cylinder_box_case!(
    BOPCommonSimpleTest__CylinderBox_W1,
    1.0,
    2.0,
    [-1.0, -1.0, 0.0, 2.0, 2.5, 1.0],
    12.5664,
    false
);

// Test bopcommon_simple/W2: cylinder and box; common; checkprops result -s 12.5664
common_cylinder_box_case!(
    BOPCommonSimpleTest__CylinderBox_W2,
    1.0,
    2.0,
    [-1.0, -1.0, 0.0, 2.5, 2.0, 1.0],
    12.5664,
    false
);

// Test bopcommon_simple/W3: cylinder and box; common; checkprops result -s 8.28319
common_cylinder_box_case!(
    BOPCommonSimpleTest__CylinderBox_W3,
    1.0,
    2.0,
    [-1.0, -1.0, 0.0, 1.0, 2.0, 1.0],
    8.28319,
    false
);

// Test bopcommon_simple/W4: cylinder and box; common; checkprops result -s 8.28319
common_cylinder_box_case!(
    BOPCommonSimpleTest__CylinderBox_W4,
    1.0,
    2.0,
    [0.0, -1.0, 0.0, 1.0, 2.0, 1.0],
    8.28319,
    false
);

// Test bopcommon_simple/W5: cylinder and box; common; checkprops result -s 8.28319
common_cylinder_box_case!(
    BOPCommonSimpleTest__CylinderBox_W5,
    1.0,
    2.0,
    [-1.0, -1.0, 0.0, 2.0, 1.0, 1.0],
    8.28319,
    false
);

// Test bopcommon_simple/W8: cylinder and box; common; checkprops result -s 11.1116
common_cylinder_box_case!(
    BOPCommonSimpleTest__CylinderBox_W8,
    1.0,
    2.0,
    [-1.0, -1.0, 0.0, 1.7071067811865475, 1.7071067811865475, 1.0],
    11.1116,
    false
);

// Test bopcommon_simple/W9: cylinder and box; common; checkprops result -s 11.1116
common_cylinder_box_case!(
    BOPCommonSimpleTest__CylinderBox_W9,
    1.0,
    2.0,
    [
        -0.70710678118654757,
        -0.70710678118654757,
        0.0,
        1.7071067811865475,
        1.7071067811865475,
        1.0
    ],
    11.1116,
    false
);

// Test bopcommon_simple/X1: cylinder and box; common; checkprops result -s 11.7384
common_cylinder_box_case!(
    BOPCommonSimpleTest__CylinderBox_X1,
    1.0,
    2.0,
    [
        -0.80000000000000004,
        -0.80000000000000004,
        0.0,
        1.8,
        1.8,
        1.0
    ],
    11.7384,
    false
);

// Test bopcommon_simple/X2: cylinder and box; common; checkprops result -s 11.7384
common_cylinder_box_case!(
    BOPCommonSimpleTest__CylinderBox_X2,
    1.0,
    2.0,
    [-1.0, -0.80000000000000004, 0.0, 1.8, 1.8, 1.0],
    11.7384,
    false
);

// Test bopcommon_simple/X4: cylinder and box; common; checkprops result -s 11.7384
common_cylinder_box_case!(
    BOPCommonSimpleTest__CylinderBox_X4,
    1.0,
    2.0,
    [
        -1.0,
        -0.80000000000000004,
        0.0,
        2.0,
        1.6000000000000001,
        1.0
    ],
    11.7384,
    false
);

// Test bopcommon_simple/X5: cylinder and box; common; checkprops result -s 11.7384
common_cylinder_box_case!(
    BOPCommonSimpleTest__CylinderBox_X5,
    1.0,
    2.0,
    [
        -0.80000000000000004,
        -1.0,
        0.0,
        1.6000000000000001,
        2.0,
        1.0
    ],
    11.7384,
    false
);

// Test bopcommon_simple/X7: cylinder and box; common; checkprops result -s 9.33406
common_cylinder_box_case!(
    BOPCommonSimpleTest__CylinderBox_X7,
    1.0,
    2.0,
    [-0.5, -1.0, 0.0, 1.5, 1.5, 1.0],
    9.33406,
    false
);

// Test bopcommon_simple/X9: cylinder and box; common; checkprops result -s 9.33406
common_cylinder_box_case!(
    BOPCommonSimpleTest__CylinderBox_X9,
    1.0,
    2.0,
    [-1.0, -1.0, 0.0, 1.5, 1.5, 1.0],
    9.33406,
    false
);

// Test bopcommon_simple/Y1: cylinder and box; common; checkprops result -s 10.3842
common_cylinder_box_case!(
    BOPCommonSimpleTest__CylinderBox_Y1,
    1.0,
    2.0,
    [
        -0.70710678118654757,
        -0.70710678118654757,
        0.0,
        1.7071067811865475,
        1.4142135623730951,
        1.0
    ],
    10.3842,
    false
);

// Test bopcommon_simple/Y4: cylinder and box; common; checkprops result -s 10.3842
common_cylinder_box_case!(
    BOPCommonSimpleTest__CylinderBox_Y4,
    1.0,
    2.0,
    [
        -0.70710678118654757,
        -0.70710678118654757,
        0.0,
        1.4142135623730951,
        1.7071067811865475,
        1.0
    ],
    10.3842,
    false
);

// Test bopcommon_simple/Y7: cylinder and box; common; checkprops result -s 11.3244
common_cylinder_box_case!(
    BOPCommonSimpleTest__CylinderBox_Y7,
    1.0,
    2.0,
    [
        -0.80000000000000004,
        -1.0,
        0.0,
        1.6000000000000001,
        1.8,
        1.0
    ],
    11.3244,
    false
);

// Test bopcommon_simple/Y8: cylinder and box; common; checkprops result -s 8.28319
common_cylinder_box_case!(
    BOPCommonSimpleTest__CylinderBox_Y8,
    1.0,
    2.0,
    [-1.0, -1.1000000000000001, 0.0, 1.0, 2.2000000000000002, 1.0],
    8.28319,
    false
);

// Test bopcommon_simple/Y9: cylinder and box; common; checkprops result -s 8.28319
common_cylinder_box_case!(
    BOPCommonSimpleTest__CylinderBox_Y9,
    1.0,
    2.0,
    [0.0, -1.1000000000000001, 0.0, 1.0, 2.2000000000000002, 1.0],
    8.28319,
    false
);

// Test bopcommon_simple/Z1: cylinder and box; common; checkprops result -s 8.28319
common_cylinder_box_case!(
    BOPCommonSimpleTest__CylinderBox_Z1,
    1.0,
    2.0,
    [-1.1000000000000001, -1.0, 0.0, 2.2000000000000002, 1.0, 1.0],
    8.28319,
    false
);

// Test bopcommon_simple/Z2: cylinder and box; common; checkprops result -s empty
common_cylinder_box_case!(
    BOPCommonSimpleTest__CylinderBox_Z2,
    1.0,
    2.0,
    [-1.0, 1.0, 0.0, 2.0, 2.0, 1.0],
    -1.0,
    true
);

// Test bopcommon_simple/Z3: cylinder and box; common; checkprops result -s empty
common_cylinder_box_case!(
    BOPCommonSimpleTest__CylinderBox_Z3,
    1.0,
    2.0,
    [1.0, -1.0, 0.0, 2.0, 2.0, 1.0],
    -1.0,
    true
);

// Test bopcommon_simple/Z5: cylinder and box; common; checkprops result -s 9.65685
common_cylinder_box_case!(
    BOPCommonSimpleTest__CylinderBox_Z5,
    1.0,
    2.0,
    [
        -0.70710678118654757,
        -0.70710678118654757,
        0.0,
        1.4142135623730951,
        1.4142135623730951,
        1.0
    ],
    9.65685,
    false
);

// Test bopcommon_simple/Z8: cylinder and box; common; checkprops result -s 10.2206
common_cylinder_box_case!(
    BOPCommonSimpleTest__CylinderBox_Z8,
    1.0,
    2.0,
    [
        -0.70710678118654757,
        -0.70710678118654757,
        0.0,
        1.6000000000000001,
        1.4142135623730951,
        1.0
    ],
    10.2206,
    false
);

// Test bopcommon_simple/ZA1: cylinder and box; common; checkprops result -s 16.6872
common_cylinder_box_case!(
    BOPCommonSimpleTest__CylinderBox_ZA1,
    1.0,
    2.0,
    [
        -0.70710678118654757,
        -0.70710678118654757,
        0.0,
        1.6000000000000001,
        1.6000000000000001,
        2.0
    ],
    16.6872,
    false
);

// Test bopcommon_simple/ZA5: cylinder and box; common; checkprops result -s 16.8455
common_cylinder_box_case!(
    BOPCommonSimpleTest__CylinderBox_ZA5,
    1.0,
    2.0,
    [
        -0.80000000000000004,
        -0.80000000000000004,
        0.0,
        1.6000000000000001,
        1.6000000000000001,
        2.0
    ],
    16.8455,
    false
);

// Test bopcommon_simple/ZA6: cylinder and box; common; checkprops result -s 9.47172
common_cylinder_box_case!(
    BOPCommonSimpleTest__CylinderBox_ZA6,
    1.0,
    2.0,
    [0.0, -0.5, 0.0, 2.0, 1.0, 2.0],
    9.47172,
    false
);

// Test bopcommon_simple/ZA8: cylinder and box; common; checkprops result -s 8.71239
common_cylinder_box_case!(
    BOPCommonSimpleTest__CylinderBox_ZA8,
    1.0,
    2.0,
    [0.0, 0.0, 0.0, 2.0, 2.0, 2.0],
    8.71239,
    false
);

// Test bopcommon_simple/ZA9: cylinder and box; common; checkprops result -s 8.71239
common_cylinder_box_case!(
    BOPCommonSimpleTest__CylinderBox_ZA9,
    1.0,
    2.0,
    [-2.0, -2.0, 0.0, 2.0, 2.0, 2.0],
    8.71239,
    false
);

// Test bopcommon_simple/ZB1: cylinder and box; common; checkprops result -s 7.90092
common_cylinder_box_case!(
    BOPCommonSimpleTest__CylinderBox_ZB1,
    1.0,
    2.0,
    [0.0, 0.0, 0.0, 0.8660254037844386, 0.8660254037844386, 2.0],
    7.90092,
    false
);

// Test bopcommon_simple/ZB3: cylinder and box; common; checkprops result -s 10.7843
common_cylinder_box_case!(
    BOPCommonSimpleTest__CylinderBox_ZB3,
    1.0,
    2.0,
    [
        -0.70710678118654757,
        -0.70710678118654757,
        0.0,
        1.6000000000000001,
        1.6000000000000001,
        1.0
    ],
    10.7843,
    false
);

// Test bopcommon_simple/ZB7: cylinder and box; common; checkprops result -s 10.9104
common_cylinder_box_case!(
    BOPCommonSimpleTest__CylinderBox_ZB7,
    1.0,
    2.0,
    [
        -0.80000000000000004,
        -0.80000000000000004,
        0.0,
        1.6000000000000001,
        1.6000000000000001,
        1.0
    ],
    10.9104,
    false
);

// Test bopcommon_simple/ZB8: cylinder and box; common; checkprops result -s 5.69247
common_cylinder_box_case!(
    BOPCommonSimpleTest__CylinderBox_ZB8,
    1.0,
    2.0,
    [0.0, -0.5, 0.0, 2.0, 1.0, 1.0],
    5.69247,
    false
);

// Test bopcommon_simple/ZC1: cylinder and box; common; checkprops result -s 5.14159
common_cylinder_box_case!(
    BOPCommonSimpleTest__CylinderBox_ZC1,
    1.0,
    2.0,
    [0.0, 0.0, 0.0, 2.0, 2.0, 1.0],
    5.14159,
    false
);

// Test bopcommon_simple/ZC2: cylinder and box; common; checkprops result -s 5.14159
common_cylinder_box_case!(
    BOPCommonSimpleTest__CylinderBox_ZC2,
    1.0,
    2.0,
    [-2.0, -2.0, 0.0, 2.0, 2.0, 1.0],
    5.14159,
    false
);

// Test bopcommon_simple/ZC3: cylinder and box; common; checkprops result -s 4.64527
common_cylinder_box_case!(
    BOPCommonSimpleTest__CylinderBox_ZC3,
    1.0,
    2.0,
    [0.0, 0.0, 0.0, 0.8660254037844386, 0.8660254037844386, 1.0],
    4.64527,
    false
);

// Test bopcommon_simple/ZC6: cylinder and box; common; checkprops result -s 8.28319
common_cylinder_box_case!(
    BOPCommonSimpleTest__CylinderBox_ZC6,
    1.0,
    2.0,
    [-1.0, -2.0, 0.5, 2.0, 2.0, 1.0],
    8.28319,
    false
);

// Test bopcommon_simple/ZC7: cylinder and box; common; checkprops result -s 13.4248
common_cylinder_box_case!(
    BOPCommonSimpleTest__CylinderBox_ZC7,
    1.0,
    4.0,
    [-1.0, 0.0, 1.0, 2.0, 2.0, 2.0],
    13.4248,
    false
);

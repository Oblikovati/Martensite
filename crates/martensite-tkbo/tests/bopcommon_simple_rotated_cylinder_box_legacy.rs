//! Regressions of self-contained rotated cylinder-box common regressions.
#![allow(non_snake_case)]

#[path = "bop_test_utilities/mod.rs"]
mod bop_test_utilities;
use bop_test_utilities::*;
use martensite_tkmath::gp::Point3;

fn common_rotated_cylinder_box_case(
    radius: f64,
    height: f64,
    box_values: [f64; 6],
    pivot: [f64; 3],
    angle: f64,
    expected_area: f64,
    empty: bool,
) {
    let cylinder = CreateCylinder(radius, height);
    let mut box_shape = CreateBox(
        Point3::new(box_values[0], box_values[1], box_values[2]),
        box_values[3],
        box_values[4],
        box_values[5],
    );
    RotateAboutZ(
        &mut box_shape,
        Point3::new(pivot[0], pivot[1], pivot[2]),
        angle,
    );
    let result = PerformCommon(&cylinder, &box_shape);
    ValidateResult(&result, expected_area, -1.0, empty);
}

macro_rules! common_rotated_cylinder_box_case {
    ($name:ident, $radius:expr, $height:expr, $box_values:expr, $pivot:expr, $angle:expr, $area:expr, $empty:expr) => {
        #[test]
        fn $name() {
            common_rotated_cylinder_box_case(
                $radius,
                $height,
                $box_values,
                $pivot,
                $angle,
                $area,
                $empty,
            );
        }
    };
}

// Test bopcommon_simple/R2: cylinder and rotated box; common; checkprops result -s 18.8496
common_rotated_cylinder_box_case!(
    BOPCommonSimpleTest__RotatedCylinderBox_R2,
    1.0,
    2.0,
    [-1.0, -1.0, 0.0, 2.0, 2.0, 2.0],
    [0.0, 0.0, 0.0],
    45.0,
    18.8496,
    false
);

// Test bopcommon_simple/R4: cylinder and rotated box; common; checkprops result -s 18.574
common_rotated_cylinder_box_case!(
    BOPCommonSimpleTest__RotatedCylinderBox_R4,
    1.0,
    2.0,
    [-1.0, -1.0, 0.0, 1.8660254037844386, 2.0, 2.0],
    [0.0, 0.0, 0.0],
    60.0,
    18.574,
    false
);

// Test bopcommon_simple/S2: cylinder and rotated box; common; checkprops result -s 17.0816
common_rotated_cylinder_box_case!(
    BOPCommonSimpleTest__RotatedCylinderBox_S2,
    1.0,
    2.0,
    [-1.0, -1.0, 0.0, 1.7071067811865475, 1.7071067811865475, 2.0],
    [0.0, 0.0, 0.0],
    45.0,
    17.0816,
    false
);

// Test bopcommon_simple/S3: cylinder and rotated box; common; checkprops result -s 17.0816
common_rotated_cylinder_box_case!(
    BOPCommonSimpleTest__RotatedCylinderBox_S3,
    1.0,
    2.0,
    [-1.0, -1.0, 0.0, 1.7071067811865475, 1.7071067811865475, 2.0],
    [0.0, 0.0, 0.0],
    -45.0,
    17.0816,
    false
);

// Test bopcommon_simple/S8: cylinder and rotated box; common; checkprops result -s 18.2984
common_rotated_cylinder_box_case!(
    BOPCommonSimpleTest__RotatedCylinderBox_S8,
    1.0,
    2.0,
    [
        -1.0,
        -0.8660254037844386,
        0.0,
        1.8660254037844386,
        1.8660254037844386,
        2.0
    ],
    [0.0, 0.0, 0.0],
    30.0,
    18.2984,
    false
);

// Test bopcommon_simple/T2: cylinder and rotated box; common; checkprops result -s 18.2984
common_rotated_cylinder_box_case!(
    BOPCommonSimpleTest__RotatedCylinderBox_T2,
    1.0,
    2.0,
    [-0.8660254037844386, -1.0, 0.0, 1.7320508075688772, 2.0, 2.0],
    [0.0, 0.0, 0.0],
    30.0,
    18.2984,
    false
);

// Test bopcommon_simple/T4: cylinder and rotated box; common; checkprops result -s 14.6841
common_rotated_cylinder_box_case!(
    BOPCommonSimpleTest__RotatedCylinderBox_T4,
    1.0,
    2.0,
    [-0.5, -1.0, 0.0, 1.5, 1.5, 2.0],
    [0.0, 0.0, 0.0],
    30.0,
    14.6841,
    false
);

// Test bopcommon_simple/T7: cylinder and rotated box; common; checkprops result -s 16.1977
common_rotated_cylinder_box_case!(
    BOPCommonSimpleTest__RotatedCylinderBox_T7,
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
    [0.0, 0.0, 0.0],
    135.0,
    16.1977,
    false
);

// Test bopcommon_simple/T8: cylinder and rotated box; common; checkprops result -s 16.3454
common_rotated_cylinder_box_case!(
    BOPCommonSimpleTest__RotatedCylinderBox_T8,
    1.0,
    2.0,
    [-0.5, -0.8660254037844386, 0.0, 1.5, 1.7320508075688772, 2.0],
    [0.0, 0.0, 0.0],
    60.0,
    16.3454,
    false
);

// Test bopcommon_simple/U1: cylinder and rotated box; common; checkprops result -s 18.0229
common_rotated_cylinder_box_case!(
    BOPCommonSimpleTest__RotatedCylinderBox_U1,
    1.0,
    2.0,
    [
        -0.8660254037844386,
        -1.0,
        0.0,
        1.7320508075688772,
        1.8660254037844386,
        2.0
    ],
    [0.0, 0.0, 0.0],
    30.0,
    18.0229,
    false
);

// Test bopcommon_simple/U2: cylinder and rotated box; common; checkprops result -s 17.3465
common_rotated_cylinder_box_case!(
    BOPCommonSimpleTest__RotatedCylinderBox_U2,
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
    [0.0, 0.0, 0.0],
    90.0,
    17.3465,
    false
);

// Test bopcommon_simple/U9: cylinder and rotated box; common; checkprops result -s 15.3137
common_rotated_cylinder_box_case!(
    BOPCommonSimpleTest__RotatedCylinderBox_U9,
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
    [0.0, 0.0, 0.0],
    135.0,
    15.3137,
    false
);

// Test bopcommon_simple/V2: cylinder and rotated box; common; checkprops result -s 16.0004
common_rotated_cylinder_box_case!(
    BOPCommonSimpleTest__RotatedCylinderBox_V2,
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
    [0.0, 0.0, 0.0],
    135.0,
    16.0004,
    false
);

// Test bopcommon_simple/V3: cylinder and rotated box; common; checkprops result -s 16.1674
common_rotated_cylinder_box_case!(
    BOPCommonSimpleTest__RotatedCylinderBox_V3,
    1.0,
    2.0,
    [
        -0.5,
        -0.8660254037844386,
        0.0,
        1.3999999999999999,
        1.7320508075688772,
        2.0
    ],
    [0.0, 0.0, 0.0],
    60.0,
    16.1674,
    false
);

// Test bopcommon_simple/V6: cylinder and rotated box; common; checkprops result -s 12.5664
common_rotated_cylinder_box_case!(
    BOPCommonSimpleTest__RotatedCylinderBox_V6,
    1.0,
    2.0,
    [-1.0, -1.0, 0.0, 2.0, 2.0, 1.0],
    [0.0, 0.0, 0.0],
    45.0,
    12.5664,
    false
);

// Test bopcommon_simple/V8: cylinder and rotated box; common; checkprops result -s 12.338
common_rotated_cylinder_box_case!(
    BOPCommonSimpleTest__RotatedCylinderBox_V8,
    1.0,
    2.0,
    [-1.0, -1.0, 0.0, 1.8660254037844386, 2.0, 1.0],
    [0.0, 0.0, 0.0],
    60.0,
    12.338,
    false
);

// Test bopcommon_simple/W6: cylinder and rotated box; common; checkprops result -s 11.1116
common_rotated_cylinder_box_case!(
    BOPCommonSimpleTest__RotatedCylinderBox_W6,
    1.0,
    2.0,
    [-1.0, -1.0, 0.0, 1.7071067811865475, 1.7071067811865475, 1.0],
    [0.0, 0.0, 0.0],
    45.0,
    11.1116,
    false
);

// Test bopcommon_simple/W7: cylinder and rotated box; common; checkprops result -s 11.1116
common_rotated_cylinder_box_case!(
    BOPCommonSimpleTest__RotatedCylinderBox_W7,
    1.0,
    2.0,
    [-1.0, -1.0, 0.0, 1.7071067811865475, 1.7071067811865475, 1.0],
    [0.0, 0.0, 0.0],
    -45.0,
    11.1116,
    false
);

// Test bopcommon_simple/X3: cylinder and rotated box; common; checkprops result -s 12.1096
common_rotated_cylinder_box_case!(
    BOPCommonSimpleTest__RotatedCylinderBox_X3,
    1.0,
    2.0,
    [
        -1.0,
        -0.8660254037844386,
        0.0,
        1.8660254037844386,
        1.8660254037844386,
        1.0
    ],
    [0.0, 0.0, 0.0],
    30.0,
    12.1096,
    false
);

// Test bopcommon_simple/X6: cylinder and rotated box; common; checkprops result -s 12.1096
common_rotated_cylinder_box_case!(
    BOPCommonSimpleTest__RotatedCylinderBox_X6,
    1.0,
    2.0,
    [-0.8660254037844386, -1.0, 0.0, 1.7320508075688772, 2.0, 1.0],
    [0.0, 0.0, 0.0],
    30.0,
    12.1096,
    false
);

// Test bopcommon_simple/X8: cylinder and rotated box; common; checkprops result -s 9.33406
common_rotated_cylinder_box_case!(
    BOPCommonSimpleTest__RotatedCylinderBox_X8,
    1.0,
    2.0,
    [-0.5, -1.0, 0.0, 1.5, 1.5, 1.0],
    [0.0, 0.0, 0.0],
    30.0,
    9.33406,
    false
);

// Test bopcommon_simple/Y2: cylinder and rotated box; common; checkprops result -s 10.3842
common_rotated_cylinder_box_case!(
    BOPCommonSimpleTest__RotatedCylinderBox_Y2,
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
    [0.0, 0.0, 0.0],
    135.0,
    10.3842,
    false
);

// Test bopcommon_simple/Y3: cylinder and rotated box; common; checkprops result -s 10.5189
common_rotated_cylinder_box_case!(
    BOPCommonSimpleTest__RotatedCylinderBox_Y3,
    1.0,
    2.0,
    [-0.5, -0.8660254037844386, 0.0, 1.5, 1.7320508075688772, 1.0],
    [0.0, 0.0, 0.0],
    60.0,
    10.5189,
    false
);

// Test bopcommon_simple/Y5: cylinder and rotated box; common; checkprops result -s 11.8813
common_rotated_cylinder_box_case!(
    BOPCommonSimpleTest__RotatedCylinderBox_Y5,
    1.0,
    2.0,
    [
        -0.8660254037844386,
        -1.0,
        0.0,
        1.7320508075688772,
        1.8660254037844386,
        1.0
    ],
    [0.0, 0.0, 0.0],
    30.0,
    11.8813,
    false
);

// Test bopcommon_simple/Y6: cylinder and rotated box; common; checkprops result -s 11.3244
common_rotated_cylinder_box_case!(
    BOPCommonSimpleTest__RotatedCylinderBox_Y6,
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
    [0.0, 0.0, 0.0],
    90.0,
    11.3244,
    false
);

// Test bopcommon_simple/Z4: cylinder and rotated box; common; checkprops result -s 9.65685
common_rotated_cylinder_box_case!(
    BOPCommonSimpleTest__RotatedCylinderBox_Z4,
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
    [0.0, 0.0, 0.0],
    135.0,
    9.65685,
    false
);

// Test bopcommon_simple/Z6: cylinder and rotated box; common; checkprops result -s 10.2206
common_rotated_cylinder_box_case!(
    BOPCommonSimpleTest__RotatedCylinderBox_Z6,
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
    [0.0, 0.0, 0.0],
    135.0,
    10.2206,
    false
);

// Test bopcommon_simple/Z7: cylinder and rotated box; common; checkprops result -s 10.3712
common_rotated_cylinder_box_case!(
    BOPCommonSimpleTest__RotatedCylinderBox_Z7,
    1.0,
    2.0,
    [
        -0.5,
        -0.8660254037844386,
        0.0,
        1.3999999999999999,
        1.7320508075688772,
        1.0
    ],
    [0.0, 0.0, 0.0],
    60.0,
    10.3712,
    false
);

// Test bopcommon_simple/ZA2: cylinder and rotated box; common; checkprops result -s 16.4622
common_rotated_cylinder_box_case!(
    BOPCommonSimpleTest__RotatedCylinderBox_ZA2,
    1.0,
    2.0,
    [-0.5, -0.8660254037844386, 0.0, 1.45, 1.8, 2.0],
    [0.0, 0.0, 0.0],
    60.0,
    16.4622,
    false
);

// Test bopcommon_simple/ZA3: cylinder and rotated box; common; checkprops result -s 16.6872
common_rotated_cylinder_box_case!(
    BOPCommonSimpleTest__RotatedCylinderBox_ZA3,
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
    [0.0, 0.0, 0.0],
    135.0,
    16.6872,
    false
);

// Test bopcommon_simple/ZA4: cylinder and rotated box; common; checkprops result -s 17.7473
common_rotated_cylinder_box_case!(
    BOPCommonSimpleTest__RotatedCylinderBox_ZA4,
    1.0,
    2.0,
    [
        -0.8660254037844386,
        -0.8660254037844386,
        0.0,
        1.7320508075688772,
        1.7320508075688772,
        2.0
    ],
    [0.0, 0.0, 0.0],
    60.0,
    17.7473,
    false
);

// Test bopcommon_simple/ZA7: cylinder and rotated box; common; checkprops result -s 9.47172
common_rotated_cylinder_box_case!(
    BOPCommonSimpleTest__RotatedCylinderBox_ZA7,
    1.0,
    2.0,
    [0.0, -0.5, 0.0, 2.0, 1.0, 2.0],
    [0.0, 0.0, 0.0],
    30.0,
    9.47172,
    false
);

// Test bopcommon_simple/ZB2: cylinder and rotated box; common; checkprops result -s 7.90092
common_rotated_cylinder_box_case!(
    BOPCommonSimpleTest__RotatedCylinderBox_ZB2,
    1.0,
    2.0,
    [0.0, 0.0, 0.0, 0.8660254037844386, 0.8660254037844386, 2.0],
    [0.0, 0.0, 0.0],
    -30.0,
    7.90092,
    false
);

// Test bopcommon_simple/ZB4: cylinder and rotated box; common; checkprops result -s 10.6153
common_rotated_cylinder_box_case!(
    BOPCommonSimpleTest__RotatedCylinderBox_ZB4,
    1.0,
    2.0,
    [-0.5, -0.8660254037844386, 0.0, 1.45, 1.8, 1.0],
    [0.0, 0.0, 0.0],
    60.0,
    10.6153,
    false
);

// Test bopcommon_simple/ZB5: cylinder and rotated box; common; checkprops result -s 10.7843
common_rotated_cylinder_box_case!(
    BOPCommonSimpleTest__RotatedCylinderBox_ZB5,
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
    [0.0, 0.0, 0.0],
    135.0,
    10.7843,
    false
);

// Test bopcommon_simple/ZB6: cylinder and rotated box; common; checkprops result -s 11.6529
common_rotated_cylinder_box_case!(
    BOPCommonSimpleTest__RotatedCylinderBox_ZB6,
    1.0,
    2.0,
    [
        -0.8660254037844386,
        -0.8660254037844386,
        0.0,
        1.7320508075688772,
        1.7320508075688772,
        1.0
    ],
    [0.0, 0.0, 0.0],
    60.0,
    11.6529,
    false
);

// Test bopcommon_simple/ZB9: cylinder and rotated box; common; checkprops result -s 5.69247
common_rotated_cylinder_box_case!(
    BOPCommonSimpleTest__RotatedCylinderBox_ZB9,
    1.0,
    2.0,
    [0.0, -0.5, 0.0, 2.0, 1.0, 1.0],
    [0.0, 0.0, 0.0],
    30.0,
    5.69247,
    false
);

// Test bopcommon_simple/ZC4: cylinder and rotated box; common; checkprops result -s 4.64527
common_rotated_cylinder_box_case!(
    BOPCommonSimpleTest__RotatedCylinderBox_ZC4,
    1.0,
    2.0,
    [0.0, 0.0, 0.0, 0.8660254037844386, 0.8660254037844386, 1.0],
    [0.0, 0.0, 0.0],
    -30.0,
    4.64527,
    false
);

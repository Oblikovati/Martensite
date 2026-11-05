//! Regressions of self-contained rotated-box common regressions.
#![allow(non_snake_case)]

#[path = "bop_test_utilities/mod.rs"]
mod bop_test_utilities;
use bop_test_utilities::*;
use martensite_tkmath::gp::Point3;

fn common_rotated_box_case(
    first: [f64; 6],
    second: [f64; 6],
    pivot: [f64; 3],
    angle: f64,
    expected_area: f64,
    empty: bool,
) {
    let first = CreateBox(
        Point3::new(first[0], first[1], first[2]),
        first[3],
        first[4],
        first[5],
    );
    let mut second = CreateBox(
        Point3::new(second[0], second[1], second[2]),
        second[3],
        second[4],
        second[5],
    );
    RotateAboutZ(
        &mut second,
        Point3::new(pivot[0], pivot[1], pivot[2]),
        angle,
    );
    let result = PerformCommon(&first, &second);
    ValidateResult(&result, expected_area, -1.0, empty);
}

macro_rules! common_rotated_box_case {
    ($name:ident, $first:expr, $second:expr, $pivot:expr, $angle:expr, $area:expr, $empty:expr) => {
        #[test]
        fn $name() {
            common_rotated_box_case($first, $second, $pivot, $angle, $area, $empty);
        }
    };
}

// Test bopcommon_simple/C3: rotated second box; common; checkprops result -s 4.41421
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_C3,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.0, 0.0, 0.0, 1.4142135623730951, 0.70710678118654757, 1.0],
    [0.0, 0.0, 0.0],
    45.0,
    4.41421,
    false
);

// Test bopcommon_simple/C4: rotated second box; common; checkprops result -s 2.91421
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_C4,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.0, 0.0, 0.0, 0.70710678118654757, 0.70710678118654757, 1.0],
    [0.0, 0.0, 0.0],
    45.0,
    2.91421,
    false
);

// Test bopcommon_simple/C5: rotated second box; common; checkprops result -s 3.61764
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_C5,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.0, 0.0, 0.0, 1.4142135623730951, 0.25, 1.0],
    [0.0, 0.0, 0.0],
    45.0,
    3.61764,
    false
);

// Test bopcommon_simple/C6: rotated second box; common; checkprops result -s 3.65032
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_C6,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.0, 0.0, 0.0, 1.3919410907075054, 0.25, 1.0],
    [0.0, 0.0, 0.0],
    34.729999999999997,
    3.65032,
    false
);

// Test bopcommon_simple/C7: rotated second box; common; checkprops result -s 4.41421
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_C7,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.0, 0.0, 0.0, 1.5, 0.70710678118654757, 1.0],
    [0.0, 0.0, 0.0],
    45.0,
    4.41421,
    false
);

// Test bopcommon_simple/C8: rotated second box; common; checkprops result -s 3.11803
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_C8,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.0, 0.5, 0.0, 1.1180339887498949, 0.44721359549995793, 1.0],
    [0.0, 0.5, 0.0],
    26.56505117707799,
    3.11803,
    false
);

// Test bopcommon_simple/C9: rotated second box; common; checkprops result -s 1.95711
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_C9,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.0, 0.0, 0.0, 0.5, 0.5, 1.0],
    [0.0, 0.0, 0.0],
    45.0,
    1.95711,
    false
);

// Test bopcommon_simple/D1: rotated second box; common; checkprops result -s 3.85967
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_D1,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.0, 0.5, 0.0, 0.44721359549995793, 1.1180339887498949, 1.0],
    [0.0, 0.5, 0.0],
    -63.43494882292201,
    3.85967,
    false
);

// Test bopcommon_simple/D2: rotated second box; common; checkprops result -s 3.3094
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_D2,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [-0.5, 0.0, 0.0, 0.5, 1.7, 1.0],
    [0.0, 0.0, 0.0],
    -30.0,
    3.3094,
    false
);

// Test bopcommon_simple/D3: rotated second box; common; checkprops result -s 3.61764
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_D3,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.0, 0.0, 0.0, 1.5, 0.25, 1.0],
    [0.0, 0.0, 0.0],
    45.0,
    3.61764,
    false
);

// Test bopcommon_simple/D4: rotated second box; common; checkprops result -s 4.33997
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_D4,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.0, 0.0, 0.0, 1.3919410907075054, 0.5, 1.0],
    [0.0, 0.0, 0.0],
    34.729999999999997,
    4.33997,
    false
);

// Test bopcommon_simple/D5: rotated second box; common; checkprops result -s 2.14434
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_D5,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.0, 0.0, 0.0, 0.5, 0.5, 1.0],
    [0.0, 0.0, 0.0],
    30.0,
    2.14434,
    false
);

// Test bopcommon_simple/D6: rotated second box; common; checkprops result -s 1.51036
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_D6,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.0, 0.0, 0.0, 0.5, 0.5, 1.0],
    [0.0, 0.0, 0.0],
    60.0,
    1.51036,
    false
);

// Test bopcommon_simple/D7: rotated second box; common; checkprops result -s empty
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_D7,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [1.0, 1.0, 0.0, 1.0, 1.0, 1.0],
    [1.0, 1.0, 1.0],
    30.0,
    -1.0,
    true
);

// Test bopcommon_simple/D8: rotated second box; common; checkprops result -s 2.95374
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_D8,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [-0.25, 0.0, 0.0, 0.25, 1.7, 1.0],
    [0.0, 0.0, 0.0],
    -30.0,
    2.95374,
    false
);

// Test bopcommon_simple/D9: rotated second box; common; checkprops result -s 4.26257
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_D9,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [-0.5, 0.0, 0.0, 0.5, 1.7, 1.0],
    [0.0, 0.0, 0.0],
    -50.0,
    4.26257,
    false
);

// Test bopcommon_simple/E1: rotated second box; common; checkprops result -s 3.62194
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_E1,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.0, 0.0, 0.0, 1.3693063937629153, 0.25, 1.0],
    [0.0, 0.0, 0.0],
    40.0,
    3.62194,
    false
);

// Test bopcommon_simple/F5: rotated second box; common; checkprops result -s 3.82843
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_F5,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.0, 0.5, 0.0, 0.70710678118654757, 0.70710678118654757, 1.0],
    [0.0, 0.5, 0.0],
    -45.0,
    3.82843,
    false
);

// Test bopcommon_simple/F6: rotated second box; common; checkprops result -s 4.91421
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_F6,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.0, 0.5, 0.0, 1.0606601717798214, 1.0606601717798214, 1.0],
    [0.0, 0.5, 0.0],
    -45.0,
    4.91421,
    false
);

// Test bopcommon_simple/F7: rotated second box; common; checkprops result -s 3.11803
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_F7,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.0, 0.5, 0.0, 1.25, 0.44721359549995793, 1.0],
    [0.0, 0.5, 0.0],
    26.56505117707799,
    3.11803,
    false
);

// Test bopcommon_simple/F8: rotated second box; common; checkprops result -s 4.41421
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_F8,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [-0.25, -0.25, 0.0, 2.0, 0.70710678118654757, 1.0],
    [-0.25, -0.25, 0.0],
    45.0,
    4.41421,
    false
);

// Test bopcommon_simple/F9: rotated second box; common; checkprops result -s 1.95711
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_F9,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.0, 0.5, 0.0, 0.70710678118654757, 0.35355339059327379, 1.0],
    [0.0, 0.5, 0.0],
    45.0,
    1.95711,
    false
);

// Test bopcommon_simple/G1: rotated second box; common; checkprops result -s 4.80205
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_G1,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.0, 0.5, 0.0, 0.89442719099991586, 1.25, 1.0],
    [0.0, 0.5, 0.0],
    -63.43494882292201,
    4.80205,
    false
);

// Test bopcommon_simple/G2: rotated second box; common; checkprops result -s 3.73258
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_G2,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [
        0.25,
        0.0,
        0.0,
        0.83852549156242118,
        0.55901699437494745,
        1.0
    ],
    [0.25, 0.0, 0.0],
    26.56505117707799,
    3.73258,
    false
);

// Test bopcommon_simple/G3: rotated second box; common; checkprops result -s 5.45711
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_G3,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [
        0.29289321881345243,
        0.0,
        0.0,
        1.4142135623730951,
        1.0606601717798214,
        1.0
    ],
    [1.0, 0.0, 0.0],
    45.0,
    5.45711,
    false
);

// Test bopcommon_simple/G4: rotated second box; common; checkprops result -s 3.87132
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_G4,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [-0.25, -0.25, 0.0, 2.0, 0.35355339059327379, 1.0],
    [-0.25, -0.25, 0.0],
    45.0,
    3.87132,
    false
);

// Test bopcommon_simple/G5: rotated second box; common; checkprops result -s 4.11803
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_G5,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.0, 0.25, 0.0, 1.3, 0.67082039324993692, 1.0],
    [0.0, 0.25, 0.0],
    26.56505117707799,
    4.11803,
    false
);

// Test bopcommon_simple/G6: rotated second box; common; checkprops result -s 3.85967
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_G6,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [
        0.0,
        0.45000000000000001,
        0.0,
        0.44721359549995793,
        1.1180339887498949,
        1.0
    ],
    [0.0, 0.45000000000000001, 0.0],
    -63.43494882292201,
    3.85967,
    false
);

// Test bopcommon_simple/G7: rotated second box; common; checkprops result -s 1.95711
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_G7,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.0, 0.25, 0.0, 0.5, 0.5, 1.0],
    [0.0, 0.25, 0.0],
    45.0,
    1.95711,
    false
);

// Test bopcommon_simple/G8: rotated second box; common; checkprops result -s 2.5
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_G8,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.0, 0.25, 0.0, 0.5, 0.5, 1.0],
    [0.0, 0.25, 0.0],
    -30.0,
    2.5,
    false
);

// Test bopcommon_simple/G9: rotated second box; common; checkprops result -s 1.29127
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_G9,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.0, 0.25, 0.0, 0.5, 0.5, 1.0],
    [0.0, 0.25, 0.0],
    240.0,
    1.29127,
    false
);

// Test bopcommon_simple/H1: rotated second box; common; checkprops result -s 4.78921
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_H1,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.0, 0.75, 0.0, 1.2374368670764582, 1.2374368670764582, 1.0],
    [0.0, 0.75, 0.0],
    -45.0,
    4.78921,
    false
);

// Test bopcommon_simple/H2: rotated second box; common; checkprops result -s 3.99129
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_H2,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.0, 0.25, 0.0, 1.1180339887498949, 1.1180339887498949, 1.0],
    [0.0, 0.25, 0.0],
    26.56505117707799,
    3.99129,
    false
);

// Test bopcommon_simple/H3: rotated second box; common; checkprops result -s 4.31371
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_H3,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.5, 0.0, 0.0, 1.0, 0.70710678118654757, 1.0],
    [0.5, 0.0, 0.0],
    45.0,
    4.31371,
    false
);

// Test bopcommon_simple/H4: rotated second box; common; checkprops result -s 4.37132
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_H4,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.5, 0.0, 0.0, 1.5, 0.70710678118654757, 1.0],
    [0.5, 0.0, 0.0],
    45.0,
    4.37132,
    false
);

// Test bopcommon_simple/H5: rotated second box; common; checkprops result -s 3.11803
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_H5,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [
        -0.10000000000000001,
        0.0,
        0.0,
        1.5,
        0.44721359549995793,
        1.0
    ],
    [1.0, 0.0, 0.0],
    26.56505117707799,
    3.11803,
    false
);

// Test bopcommon_simple/H6: rotated second box; common; checkprops result -s 4.40685
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_H6,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.0, -0.25, 0.0, 1.4142135623730951, 0.5, 1.0],
    [0.0, 0.0, 0.0],
    45.0,
    4.40685,
    false
);

// Test bopcommon_simple/H7: rotated second box; common; checkprops result -s 1.16144
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_H7,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.0, 0.25, 0.0, 0.10000000000000001, 0.5, 1.0],
    [0.0, 0.25, 0.0],
    240.0,
    1.16144,
    false
);

// Test bopcommon_simple/H8: rotated second box; common; checkprops result -s 3.77245
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_H8,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.0, 0.25, 0.0, 1.1180339887498949, 0.5, 1.0],
    [0.0, 0.25, 0.0],
    26.56505117707799,
    3.77245,
    false
);

// Test bopcommon_simple/H9: rotated second box; common; checkprops result -s 2.5
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_H9,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.25, 0.25, 0.0, 0.5, 0.5, 1.0],
    [0.25, 0.25, 0.0],
    30.0,
    2.5,
    false
);

// Test bopcommon_simple/I1: rotated second box; common; checkprops result -s 1.51036
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_I1,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.25, 0.25, 0.0, 0.5, 0.5, 1.0],
    [0.25, 0.25, 0.0],
    120.0,
    1.51036,
    false
);

// Test bopcommon_simple/I2: rotated second box; common; checkprops result -s 2.08523
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_I2,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.0, 0.25, 0.0, 0.5, 0.5, 1.0],
    [0.0, 0.25, 0.0],
    35.0,
    2.08523,
    false
);

// Test bopcommon_simple/I3: rotated second box; common; checkprops result -s empty
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_I3,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.0, 0.25, 0.0, 0.5, 0.5, 1.0],
    [0.0, 0.25, 0.0],
    115.0,
    -1.0,
    true
);

// Test bopcommon_simple/I4: rotated second box; common; checkprops result -s 1.01135
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_I4,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.0, 0.25, 0.0, 0.5, 0.5, 1.0],
    [0.0, 0.25, 0.0],
    230.0,
    1.01135,
    false
);

// Test bopcommon_simple/I5: rotated second box; common; checkprops result -s 1.42746
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_I5,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.0, 0.25, 0.0, 0.5, 0.5, 1.0],
    [0.0, 0.25, 0.0],
    250.0,
    1.42746,
    false
);

// Test bopcommon_simple/I6: rotated second box; common; checkprops result -s 4.91421
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_I6,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.0, 0.5, 0.0, 1.5, 1.5, 1.0],
    [0.0, 0.5, 0.0],
    -45.0,
    4.91421,
    false
);

// Test bopcommon_simple/I7: rotated second box; common; checkprops result -s 4.21056
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_I7,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.0, 0.5, 0.0, 0.80000000000000004, 0.80000000000000004, 1.0],
    [0.0, 0.5, 0.0],
    -45.0,
    4.21056,
    false
);

// Test bopcommon_simple/I8: rotated second box; common; checkprops result -s 3.87582
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_I8,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [
        0.0,
        0.25,
        0.0,
        0.80000000000000004,
        0.80000000000000004,
        1.0
    ],
    [0.0, 0.25, 0.0],
    -45.0,
    3.87582,
    false
);

// Test bopcommon_simple/I9: rotated second box; common; checkprops result -s 2.39277
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_I9,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.0, 0.25, 0.0, 0.5, 0.5, 1.0],
    [0.0, 0.25, 0.0],
    -45.0,
    2.39277,
    false
);

// Test bopcommon_simple/J1: rotated second box; common; checkprops result -s 4.79412
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_J1,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.0, 0.625, 0.0, 1.0606601717798214, 1.0606601717798214, 1.0],
    [0.0, 0.625, 0.0],
    -45.0,
    4.79412,
    false
);

// Test bopcommon_simple/J2: rotated second box; common; checkprops result -s 4.40685
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_J2,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [-0.25, 0.0, 0.0, 0.5, 1.7, 1.0],
    [0.0, 0.0, 0.0],
    -45.0,
    4.40685,
    false
);

// Test bopcommon_simple/J3: rotated second box; common; checkprops result -s 4.10844
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_J3,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [-0.25, 0.0, 0.0, 0.5, 1.7, 1.0],
    [0.0, 0.0, 0.0],
    -30.0,
    4.10844,
    false
);

// Test bopcommon_simple/J4: rotated second box; common; checkprops result -s 2.95374
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_J4,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [-0.25, -0.25, 0.0, 0.25, 2.0, 1.0],
    [0.0, 0.0, 0.0],
    -30.0,
    2.95374,
    false
);

// Test bopcommon_simple/J5: rotated second box; common; checkprops result -s 4.28615
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_J5,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [-0.25, 0.0, 0.0, 0.5, 1.3, 1.0],
    [0.0, 0.0, 0.0],
    -45.0,
    4.28615,
    false
);

// Test bopcommon_simple/J6: rotated second box; common; checkprops result -s 4.24099
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_J6,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.0, -0.25, 0.0, 0.5, 1.55, 1.0],
    [0.0, 0.0, 0.0],
    -35.0,
    4.24099,
    false
);

// Test bopcommon_simple/J7: rotated second box; common; checkprops result -s 0.464678
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_J7,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.10000000000000001, 0.25, 0.0, 0.5, 0.5, 1.0],
    [0.0, 0.25, 0.0],
    230.0,
    0.464678,
    false
);

// Test bopcommon_simple/J8: rotated second box; common; checkprops result -s 1.90425
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_J8,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.10000000000000001, 0.40000000000000002, 0.0, 0.5, 0.5, 1.0],
    [0.10000000000000001, 0.40000000000000002, 0.0],
    245.0,
    1.90425,
    false
);

// Test bopcommon_simple/K2: rotated second box; common; checkprops result -s 2.39277
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_K2,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.25, 0.25, 0.0, 0.5, 0.5, 1.0],
    [0.25, 0.25, 0.0],
    45.0,
    2.39277,
    false
);

// Test bopcommon_simple/K3: rotated second box; common; checkprops result -s 1.75
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_K3,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.25, 0.25, 0.0, 0.5, 0.5, 1.0],
    [0.25, 0.25, 0.0],
    90.0,
    1.75,
    false
);

// Test bopcommon_simple/K4: rotated second box; common; checkprops result -s 1.40617
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_K4,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.25, 0.25, 0.0, 0.5, 0.5, 1.0],
    [0.25, 0.25, 0.0],
    125.0,
    1.40617,
    false
);

// Test bopcommon_simple/K6: rotated second box; common; checkprops result -s 4.40683
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_K6,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.90000000000000002, -0.25, 0.0, 0.5, 1.7, 1.0],
    [0.90000000000000002, -0.25, 0.0],
    45.0,
    4.40683,
    false
);

// Test bopcommon_simple/K7: rotated second box; common; checkprops result -s 2.51409
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_K7,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [1.25, 0.0, 0.0, 0.25, 1.7, 1.0],
    [1.1000000000000001, 0.0, 0.0],
    45.0,
    2.51409,
    false
);

// Test bopcommon_simple/K8: rotated second box; common; checkprops result -s 4.10844
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_K8,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [-0.25, -0.25, 0.0, 0.5, 1.7, 1.0],
    [0.0, 0.0, 0.0],
    -30.0,
    4.10844,
    false
);

// Test bopcommon_simple/K9: rotated second box; common; checkprops result -s 4.28615
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_K9,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [-0.25, -0.25, 0.0, 0.5, 1.55, 1.0],
    [0.0, 0.0, 0.0],
    -45.0,
    4.28615,
    false
);

// Test bopcommon_simple/L1: rotated second box; common; checkprops result -s 4.9632
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_L1,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.5, -0.25, 0.0, 1.0, 1.0, 1.0],
    [0.5, -0.25, 0.0],
    45.0,
    4.9632,
    false
);

// Test bopcommon_simple/L2: rotated second box; common; checkprops result -s 4.00937
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_L2,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [
        0.0,
        0.14999999999999999,
        0.0,
        0.80000000000000004,
        0.90000000000000002,
        1.0
    ],
    [0.0, 0.25, 0.0],
    -45.0,
    4.00937,
    false
);

// Test bopcommon_simple/L3: rotated second box; common; checkprops result -s 3.41053
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_L3,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.5, -0.25, 0.0, 0.5, 1.0, 1.0],
    [0.5, -0.25, 0.0],
    45.0,
    3.41053,
    false
);

// Test bopcommon_simple/L4: rotated second box; common; checkprops result -s 2.70711
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_L4,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.0, 0.0, 0.0, 1.4142135623730951, 0.70710678118654757, 0.5],
    [0.0, 0.0, 0.0],
    45.0,
    2.70711,
    false
);

// Test bopcommon_simple/L5: rotated second box; common; checkprops result -s 1.70711
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_L5,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.0, 0.0, 0.0, 0.70710678118654757, 0.70710678118654757, 0.5],
    [0.0, 0.0, 0.0],
    45.0,
    1.70711,
    false
);

// Test bopcommon_simple/L6: rotated second box; common; checkprops result -s 2.09987
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_L6,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.0, 0.0, 0.0, 1.4142135623730951, 0.25, 0.5],
    [0.0, 0.0, 0.0],
    45.0,
    2.09987,
    false
);

// Test bopcommon_simple/L7: rotated second box; common; checkprops result -s 2.12935
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_L7,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.0, 0.0, 0.0, 1.3919410907075054, 0.25, 0.5],
    [0.0, 0.0, 0.0],
    34.729999999999997,
    2.12935,
    false
);

// Test bopcommon_simple/L8: rotated second box; common; checkprops result -s 2.70711
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_L8,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.0, 0.0, 0.0, 1.5, 0.70710678118654757, 0.5],
    [0.0, 0.0, 0.0],
    45.0,
    2.70711,
    false
);

// Test bopcommon_simple/L9: rotated second box; common; checkprops result -s 1.80902
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_L9,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.0, 0.5, 0.0, 1.1180339887498949, 0.44721359549995793, 0.5],
    [0.0, 0.5, 0.0],
    26.56505117707799,
    1.80902,
    false
);

// Test bopcommon_simple/M1: rotated second box; common; checkprops result -s 1.10355
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_M1,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.0, 0.0, 0.0, 0.5, 0.5, 0.5],
    [0.0, 0.0, 0.0],
    45.0,
    1.10355,
    false
);

// Test bopcommon_simple/M2: rotated second box; common; checkprops result -s 2.37984
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_M2,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.0, 0.5, 0.0, 0.44721359549995793, 1.1180339887498949, 0.5],
    [0.0, 0.5, 0.0],
    -63.43494882292201,
    2.37984,
    false
);

// Test bopcommon_simple/M3: rotated second box; common; checkprops result -s 1.94338
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_M3,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [-0.5, 0.0, 0.0, 0.5, 1.7, 0.5],
    [0.0, 0.0, 0.0],
    -30.0,
    1.94338,
    false
);

// Test bopcommon_simple/M4: rotated second box; common; checkprops result -s 2.09987
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_M4,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.0, 0.0, 0.0, 1.5, 0.25, 0.5],
    [0.0, 0.0, 0.0],
    45.0,
    2.09987,
    false
);

// Test bopcommon_simple/M5: rotated second box; common; checkprops result -s 2.71277
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_M5,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.0, 0.0, 0.0, 1.3919410907075054, 0.5, 0.5],
    [0.0, 0.0, 0.0],
    34.729999999999997,
    2.71277,
    false
);

// Test bopcommon_simple/M6: rotated second box; common; checkprops result -s 1.25
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_M6,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.0, 0.0, 0.0, 0.5, 0.5, 0.5],
    [0.0, 0.0, 0.0],
    30.0,
    1.25,
    false
);

// Test bopcommon_simple/M7: rotated second box; common; checkprops result -s 0.82735
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_M7,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.0, 0.0, 0.0, 0.5, 0.5, 0.5],
    [0.0, 0.0, 0.0],
    60.0,
    0.82735,
    false
);

// Test bopcommon_simple/M8: rotated second box; common; checkprops result -s empty
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_M8,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [1.0, 1.0, 0.0, 1.0, 1.0, 0.5],
    [1.0, 1.0, 1.0],
    30.0,
    -1.0,
    true
);

// Test bopcommon_simple/M9: rotated second box; common; checkprops result -s 1.69338
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_M9,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [-0.25, 0.0, 0.0, 0.25, 1.7, 0.5],
    [0.0, 0.0, 0.0],
    -30.0,
    1.69338,
    false
);

// Test bopcommon_simple/N1: rotated second box; common; checkprops result -s 2.63987
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_N1,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [-0.5, 0.0, 0.0, 0.5, 1.7, 0.5],
    [0.0, 0.0, 0.0],
    -50.0,
    2.63987,
    false
);

// Test bopcommon_simple/N2: rotated second box; common; checkprops result -s 2.11942
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_N2,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.0, 0.0, 0.0, 1.3693063937629153, 0.25, 0.5],
    [0.0, 0.0, 0.0],
    40.0,
    2.11942,
    false
);

// Test bopcommon_simple/N3: rotated second box; common; checkprops result -s empty
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_N3,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.25, 0.25, 0.0, 0.5, 0.5, -1.0],
    [0.25, 0.25, 0.0],
    30.0,
    -1.0,
    true
);

// Test bopcommon_simple/N4: rotated second box; common; checkprops result -s empty
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_N4,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.25, 0.25, 0.0, 0.5, 0.5, -1.0],
    [0.25, 0.25, 0.0],
    120.0,
    -1.0,
    true
);

// Test bopcommon_simple/N5: rotated second box; common; checkprops result -s empty
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_N5,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.0, 0.25, 0.0, 0.5, 0.5, -1.0],
    [0.0, 0.25, 0.0],
    35.0,
    -1.0,
    true
);

// Test bopcommon_simple/N6: rotated second box; common; checkprops result -s empty
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_N6,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.0, 0.25, 0.0, 0.5, 0.5, -1.0],
    [0.0, 0.25, 0.0],
    115.0,
    -1.0,
    true
);

// Test bopcommon_simple/N7: rotated second box; common; checkprops result -s empty
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_N7,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.0, 0.25, 0.0, 0.5, 0.5, -1.0],
    [0.0, 0.25, 0.0],
    230.0,
    -1.0,
    true
);

// Test bopcommon_simple/N8: rotated second box; common; checkprops result -s empty
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_N8,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.0, 0.25, 0.0, 0.5, 0.5, -1.0],
    [0.0, 0.25, 0.0],
    250.0,
    -1.0,
    true
);

// Test bopcommon_simple/N9: rotated second box; common; checkprops result -s empty
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_N9,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.0, 0.5, 0.0, 1.5, 1.5, -1.0],
    [0.0, 0.5, 0.0],
    -45.0,
    -1.0,
    true
);

// Test bopcommon_simple/O1: rotated second box; common; checkprops result -s empty
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_O1,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [
        0.0,
        0.5,
        0.0,
        0.80000000000000004,
        0.80000000000000004,
        -1.0
    ],
    [0.0, 0.5, 0.0],
    -45.0,
    -1.0,
    true
);

// Test bopcommon_simple/O2: rotated second box; common; checkprops result -s empty
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_O2,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [
        0.0,
        0.25,
        0.0,
        0.80000000000000004,
        0.80000000000000004,
        -1.0
    ],
    [0.0, 0.25, 0.0],
    -45.0,
    -1.0,
    true
);

// Test bopcommon_simple/O3: rotated second box; common; checkprops result -s empty
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_O3,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.0, 0.25, 0.0, 0.5, 0.5, -1.0],
    [0.0, 0.25, 0.0],
    -45.0,
    -1.0,
    true
);

// Test bopcommon_simple/O4: rotated second box; common; checkprops result -s empty
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_O4,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [
        0.0,
        0.625,
        0.0,
        1.0606601717798214,
        1.0606601717798214,
        -1.0
    ],
    [0.0, 0.625, 0.0],
    -45.0,
    -1.0,
    true
);

// Test bopcommon_simple/O5: rotated second box; common; checkprops result -s empty
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_O5,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [-0.25, 0.0, 0.0, 0.5, 1.7, -1.0],
    [0.0, 0.0, 0.0],
    -45.0,
    -1.0,
    true
);

// Test bopcommon_simple/O6: rotated second box; common; checkprops result -s empty
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_O6,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [-0.25, 0.0, 0.0, 0.5, 1.7, -1.0],
    [0.0, 0.0, 0.0],
    -30.0,
    -1.0,
    true
);

// Test bopcommon_simple/O7: rotated second box; common; checkprops result -s empty
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_O7,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [-0.25, -0.25, 0.0, 0.25, 2.0, -1.0],
    [0.0, 0.0, 0.0],
    -30.0,
    -1.0,
    true
);

// Test bopcommon_simple/O8: rotated second box; common; checkprops result -s empty
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_O8,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [-0.25, 0.0, 0.0, 0.5, 1.3, -1.0],
    [0.0, 0.0, 0.0],
    -45.0,
    -1.0,
    true
);

// Test bopcommon_simple/O9: rotated second box; common; checkprops result -s empty
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_O9,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.0, -0.25, 0.0, 0.5, 1.55, -1.0],
    [0.0, 0.0, 0.0],
    -35.0,
    -1.0,
    true
);

// Test bopcommon_simple/P1: rotated second box; common; checkprops result -s 0.542919
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_P1,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.0, 0.25, 0.5, 0.5, 0.5, 1.0],
    [0.0, 0.25, 0.0],
    230.0,
    0.542919,
    false
);

// Test bopcommon_simple/P2: rotated second box; common; checkprops result -s 1.10649
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_P2,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.10000000000000001, 0.40000000000000002, 0.5, 0.5, 0.5, 1.0],
    [0.10000000000000001, 0.40000000000000002, 0.0],
    245.0,
    1.10649,
    false
);

// Test bopcommon_simple/P5: rotated second box; common; checkprops result -s 1.12132
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_P5,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.25, 0.5, 0.5, 0.5, 0.5, 1.0],
    [0.25, 0.25, 0.0],
    45.0,
    1.12132,
    false
);

// Test bopcommon_simple/P6: rotated second box; common; checkprops result -s empty
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_P6,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.25, 0.5, 0.5, 0.5, 0.5, 1.0],
    [0.25, 0.25, 0.0],
    90.0,
    -1.0,
    true
);

// Test bopcommon_simple/P7: rotated second box; common; checkprops result -s 0.769594
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_P7,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.25, 0.25, 0.5, 0.5, 0.5, 1.0],
    [0.25, 0.25, 0.0],
    125.0,
    0.769594,
    false
);

// Test bopcommon_simple/P9: rotated second box; common; checkprops result -s 2.78551
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_P9,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.90000000000000002, -0.25, 0.5, 0.5, 1.7, 1.0],
    [0.90000000000000002, -0.25, 0.0],
    45.0,
    2.78551,
    false
);

// Test bopcommon_simple/Q1: rotated second box; common; checkprops result -s 1.43774
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_Q1,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [1.25, 0.0, 0.5, 0.25, 1.7, 1.0],
    [1.1000000000000001, 0.0, 0.0],
    45.0,
    1.43774,
    false
);

// Test bopcommon_simple/Q2: rotated second box; common; checkprops result -s 2.5594
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_Q2,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [-0.25, -0.25, 0.5, 0.5, 1.7, 1.0],
    [0.0, 0.0, 0.0],
    -30.0,
    2.5594,
    false
);

// Test bopcommon_simple/Q3: rotated second box; common; checkprops result -s 2.71214
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_Q3,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [-0.25, -0.25, 0.5, 0.5, 1.55, 1.0],
    [0.0, 0.0, 0.0],
    -45.0,
    2.71214,
    false
);

// Test bopcommon_simple/Q4: rotated second box; common; checkprops result -s 3.30635
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_Q4,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.5, -0.25, 0.5, 1.0, 1.0, 1.0],
    [0.5, -0.25, 0.0],
    45.0,
    3.30635,
    false
);

// Test bopcommon_simple/Q5: rotated second box; common; checkprops result -s 2.55312
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_Q5,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [
        0.0,
        0.14999999999999999,
        0.5,
        0.80000000000000004,
        0.90000000000000002,
        1.0
    ],
    [0.0, 0.25, 0.0],
    -45.0,
    2.55312,
    false
);

// Test bopcommon_simple/Q6: rotated second box; common; checkprops result -s 2.09987
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_Q6,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.5, -0.25, 0.5, 0.5, 1.0, 1.0],
    [0.5, -0.25, 0.0],
    45.0,
    2.09987,
    false
);

// Test bopcommon_simple/Q8: rotated second box; common; checkprops result -s 2.7099
common_rotated_box_case!(
    BOPCommonSimpleTest__RotatedBoxes_Q8,
    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    [0.0, -0.25, 0.25, 0.5, 1.7, 0.5],
    [0.0, 0.0, 0.0],
    -35.0,
    2.7099,
    false
);

//! Regressions of translated cylinder-box common regressions.
#![allow(non_snake_case)]

#[path = "bop_test_utilities/mod.rs"]
mod bop_test_utilities;
use bop_test_utilities::*;
use martensite_tkmath::gp::Point3;

fn common_translated_cylinder_box_case(
    box_size: [f64; 3],
    radius: f64,
    height: f64,
    translation: [f64; 3],
    expected_area: f64,
    empty: bool,
) {
    let box_shape = CreateBox(
        Point3::new(0.0, 0.0, 0.0),
        box_size[0],
        box_size[1],
        box_size[2],
    );
    let cylinder = Translate(
        &CreateCylinder(radius, height),
        translation[0],
        translation[1],
        translation[2],
    );
    let result = PerformCommon(&box_shape, &cylinder);
    ValidateResult(&result, expected_area, -1.0, empty);
}

macro_rules! common_translated_cylinder_box_case {
    ($name:ident, $box_size:expr, $radius:expr, $height:expr, $translation:expr, $area:expr, $empty:expr) => {
        #[test]
        fn $name() {
            common_translated_cylinder_box_case(
                $box_size,
                $radius,
                $height,
                $translation,
                $area,
                $empty,
            );
        }
    };
}

// Test bopcommon_simple/Z9: translated cylinder and box; common; checkprops result -s 34.8496
common_translated_cylinder_box_case!(
    BOPCommonSimpleTest__TranslatedCylinderBox_Z9,
    [4.0, 4.0, 4.0],
    2.0,
    6.0,
    [0.0, 0.0, -2.0],
    34.8496,
    false
);

// Test bopcommon_simple/ZN3: translated cylinder and box; common; checkprops result -s 31.4159
common_translated_cylinder_box_case!(
    BOPCommonSimpleTest__TranslatedCylinderBox_ZN3,
    [4.0, 4.0, 4.0],
    1.0,
    6.0,
    [2.0, 2.0, -2.0],
    31.4159,
    false
);

// Test bopcommon_simple/ZN4: translated cylinder and box; common; checkprops result -s 18.8496
common_translated_cylinder_box_case!(
    BOPCommonSimpleTest__TranslatedCylinderBox_ZN4,
    [4.0, 4.0, 4.0],
    1.0,
    2.0,
    [2.0, 2.0, 1.0],
    18.8496,
    false
);

// Test bopcommon_simple/ZN5: translated cylinder and box; common; checkprops result -s 53.6991
common_translated_cylinder_box_case!(
    BOPCommonSimpleTest__TranslatedCylinderBox_ZN5,
    [4.0, 4.0, 4.0],
    2.0,
    4.0,
    [4.0, 2.0, 0.0],
    53.6991,
    false
);

// Test bopcommon_simple/ZN6: translated cylinder and box; common; checkprops result -s 53.6991
common_translated_cylinder_box_case!(
    BOPCommonSimpleTest__TranslatedCylinderBox_ZN6,
    [4.0, 4.0, 4.0],
    2.0,
    4.0,
    [2.0, 0.0, 0.0],
    53.6991,
    false
);

// Test bopcommon_simple/ZN7: translated cylinder and box; common; checkprops result -s 53.6991
common_translated_cylinder_box_case!(
    BOPCommonSimpleTest__TranslatedCylinderBox_ZN7,
    [4.0, 4.0, 4.0],
    2.0,
    4.0,
    [0.0, 2.0, 0.0],
    53.6991,
    false
);

// Test bopcommon_simple/ZN8: translated cylinder and box; common; checkprops result -s 23.708
common_translated_cylinder_box_case!(
    BOPCommonSimpleTest__TranslatedCylinderBox_ZN8,
    [4.0, 4.0, 4.0],
    1.0,
    4.0,
    [4.0, 2.0, 0.0],
    23.708,
    false
);

// Test bopcommon_simple/ZN9: translated cylinder and box; common; checkprops result -s 23.708
common_translated_cylinder_box_case!(
    BOPCommonSimpleTest__TranslatedCylinderBox_ZN9,
    [4.0, 4.0, 4.0],
    1.0,
    4.0,
    [2.0, 0.0, 0.0],
    23.708,
    false
);

// Test bopcommon_simple/ZO1: translated cylinder and box; common; checkprops result -s 23.708
common_translated_cylinder_box_case!(
    BOPCommonSimpleTest__TranslatedCylinderBox_ZO1,
    [4.0, 4.0, 4.0],
    1.0,
    4.0,
    [0.0, 2.0, 0.0],
    23.708,
    false
);

// Test bopcommon_simple/ZO2: translated cylinder and box; common; checkprops result -s 23.708
common_translated_cylinder_box_case!(
    BOPCommonSimpleTest__TranslatedCylinderBox_ZO2,
    [4.0, 4.0, 4.0],
    1.0,
    4.0,
    [4.0, 3.0, 0.0],
    23.708,
    false
);

// Test bopcommon_simple/ZO3: translated cylinder and box; common; checkprops result -s 23.708
common_translated_cylinder_box_case!(
    BOPCommonSimpleTest__TranslatedCylinderBox_ZO3,
    [4.0, 4.0, 4.0],
    1.0,
    4.0,
    [3.0, 0.0, 0.0],
    23.708,
    false
);

// Test bopcommon_simple/ZO4: translated cylinder and box; common; checkprops result -s 23.708
common_translated_cylinder_box_case!(
    BOPCommonSimpleTest__TranslatedCylinderBox_ZO4,
    [4.0, 4.0, 4.0],
    1.0,
    4.0,
    [0.0, 3.0, 0.0],
    23.708,
    false
);

// Test bopcommon_simple/ZO5: translated cylinder and box; common; checkprops result -s 23.708
common_translated_cylinder_box_case!(
    BOPCommonSimpleTest__TranslatedCylinderBox_ZO5,
    [4.0, 4.0, 4.0],
    1.0,
    4.0,
    [1.0, 0.0, 0.0],
    23.708,
    false
);

// Test bopcommon_simple/ZO6: translated cylinder and box; common; checkprops result -s 59.0795
common_translated_cylinder_box_case!(
    BOPCommonSimpleTest__TranslatedCylinderBox_ZO6,
    [4.0, 4.0, 4.0],
    4.0,
    4.0,
    [-2.0, 2.0, 0.0],
    59.0795,
    false
);

// Test bopcommon_simple/ZO9: translated cylinder and box; common; checkprops result -s empty
common_translated_cylinder_box_case!(
    BOPCommonSimpleTest__TranslatedCylinderBox_ZO9,
    [4.0, 4.0, 4.0],
    1.0,
    4.0,
    [3.0, 1.0, 4.0],
    -1.0,
    true
);

// Test bopcommon_simple/ZP1: translated cylinder and box; common; checkprops result -s empty
common_translated_cylinder_box_case!(
    BOPCommonSimpleTest__TranslatedCylinderBox_ZP1,
    [4.0, 4.0, 4.0],
    1.0,
    4.0,
    [1.0, 1.0, 4.0],
    -1.0,
    true
);

// Test bopcommon_simple/ZP2: translated cylinder and box; common; checkprops result -s 34.8496
common_translated_cylinder_box_case!(
    BOPCommonSimpleTest__TranslatedCylinderBox_ZP2,
    [4.0, 4.0, 4.0],
    2.0,
    6.0,
    [0.0, 0.0, -2.0],
    34.8496,
    false
);

//! Regressions of translated cone-box common regressions.
#![allow(non_snake_case)]

#[path = "bop_test_utilities/mod.rs"]
mod bop_test_utilities;
use bop_test_utilities::*;
use martensite_tkmath::gp::Point3;

fn common_translated_cone_box_case(
    box_size: [f64; 3],
    radii: [f64; 2],
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
    let cone = Translate(
        &CreateCone(radii[0], radii[1], height),
        translation[0],
        translation[1],
        translation[2],
    );
    let result = PerformCommon(&box_shape, &cone);
    ValidateResult(&result, expected_area, -1.0, empty);
}

macro_rules! common_translated_cone_box_case {
    ($name:ident, $box_size:expr, $radii:expr, $height:expr, $translation:expr, $area:expr, $empty:expr) => {
        #[test]
        fn $name() {
            common_translated_cone_box_case(
                $box_size,
                $radii,
                $height,
                $translation,
                $area,
                $empty,
            );
        }
    };
}

// Test bopcommon_simple/ZF5: translated cone and box; common; checkprops result -s empty
common_translated_cone_box_case!(
    BOPCommonSimpleTest__TranslatedConeBox_ZF5,
    [4.0, 4.0, 4.0],
    [1.0, 0.5],
    2.0,
    [2.0, 2.0, 4.0],
    -1.0,
    true
);

// Test bopcommon_simple/ZF6: translated cone and box; common; checkprops result -s 13.6418
common_translated_cone_box_case!(
    BOPCommonSimpleTest__TranslatedConeBox_ZF6,
    [4.0, 4.0, 4.0],
    [1.0, 0.5],
    2.0,
    [2.0, 2.0, 2.0],
    13.6418,
    false
);

// Test bopcommon_simple/ZF7: translated cone and box; common; checkprops result -s 22.9232
common_translated_cone_box_case!(
    BOPCommonSimpleTest__TranslatedConeBox_ZF7,
    [4.0, 4.0, 4.0],
    [1.0, 0.5],
    4.0,
    [2.0, 2.0, 0.0],
    22.9232,
    false
);

// Test bopcommon_simple/ZF8: translated cone and box; common; checkprops result -s 21.0108
common_translated_cone_box_case!(
    BOPCommonSimpleTest__TranslatedConeBox_ZF8,
    [4.0, 4.0, 4.0],
    [1.0, 0.5],
    5.0,
    [2.0, 2.0, -1.0],
    21.0108,
    false
);

// Test bopcommon_simple/ZF9: translated cone and box; common; checkprops result -s 13.6418
common_translated_cone_box_case!(
    BOPCommonSimpleTest__TranslatedConeBox_ZF9,
    [4.0, 4.0, 4.0],
    [1.0, 0.5],
    2.0,
    [2.0, 2.0, 1.0],
    13.6418,
    false
);

// Test bopcommon_simple/ZG1: translated cone and box; common; checkprops result -s 64.3685
common_translated_cone_box_case!(
    BOPCommonSimpleTest__TranslatedConeBox_ZG1,
    [4.0, 4.0, 4.0],
    [3.0, 2.0],
    4.0,
    [4.0, 2.0, 0.0],
    64.3685,
    false
);

// Test bopcommon_simple/ZG2: translated cone and box; common; checkprops result -s 64.3685
common_translated_cone_box_case!(
    BOPCommonSimpleTest__TranslatedConeBox_ZG2,
    [4.0, 4.0, 4.0],
    [3.0, 2.0],
    4.0,
    [2.0, 0.0, 0.0],
    64.3685,
    false
);

// Test bopcommon_simple/ZG3: translated cone and box; common; checkprops result -s 64.3685
common_translated_cone_box_case!(
    BOPCommonSimpleTest__TranslatedConeBox_ZG3,
    [4.0, 4.0, 4.0],
    [3.0, 2.0],
    4.0,
    [0.0, 2.0, 0.0],
    64.3685,
    false
);

// Test bopcommon_simple/ZG5: translated cone and box; common; checkprops result -s 39.2837
common_translated_cone_box_case!(
    BOPCommonSimpleTest__TranslatedConeBox_ZG5,
    [4.0, 4.0, 4.0],
    [2.0, 1.0],
    4.0,
    [4.0, 2.0, 0.0],
    39.2837,
    false
);

// Test bopcommon_simple/ZG6: translated cone and box; common; checkprops result -s 39.2837
common_translated_cone_box_case!(
    BOPCommonSimpleTest__TranslatedConeBox_ZG6,
    [4.0, 4.0, 4.0],
    [2.0, 1.0],
    4.0,
    [2.0, 0.0, 0.0],
    39.2837,
    false
);

// Test bopcommon_simple/ZG7: translated cone and box; common; checkprops result -s 39.2837
common_translated_cone_box_case!(
    BOPCommonSimpleTest__TranslatedConeBox_ZG7,
    [4.0, 4.0, 4.0],
    [2.0, 1.0],
    4.0,
    [0.0, 2.0, 0.0],
    39.2837,
    false
);

// Test bopcommon_simple/ZG8: translated cone and box; common; checkprops result -s empty
common_translated_cone_box_case!(
    BOPCommonSimpleTest__TranslatedConeBox_ZG8,
    [4.0, 4.0, 4.0],
    [1.0, 0.5],
    2.0,
    [3.0, 1.0, 4.0],
    -1.0,
    true
);

// Test bopcommon_simple/ZG9: translated cone and box; common; checkprops result -s empty
common_translated_cone_box_case!(
    BOPCommonSimpleTest__TranslatedConeBox_ZG9,
    [4.0, 4.0, 4.0],
    [1.0, 0.5],
    2.0,
    [1.0, 1.0, 4.0],
    -1.0,
    true
);

// Test bopcommon_simple/ZH1: translated cone and box; common; checkprops result -s 68.087
common_translated_cone_box_case!(
    BOPCommonSimpleTest__TranslatedConeBox_ZH1,
    [4.0, 4.0, 4.0],
    [5.0, 4.0],
    4.0,
    [6.0, 2.0, 0.0],
    68.087,
    false
);

// Test bopcommon_simple/ZH2: translated cone and box; common; checkprops result -s 68.087
common_translated_cone_box_case!(
    BOPCommonSimpleTest__TranslatedConeBox_ZH2,
    [4.0, 4.0, 4.0],
    [5.0, 4.0],
    4.0,
    [-2.0, 2.0, 0.0],
    68.087,
    false
);

//! Regressions of translated cone-cone common regressions.
#![allow(non_snake_case)]

#[path = "bop_test_utilities/mod.rs"]
mod bop_test_utilities;
use bop_test_utilities::*;

fn common_translated_cones(
    first: [f64; 3],
    second: [f64; 3],
    translation: [f64; 3],
    expected_area: f64,
    empty: bool,
) {
    let first = CreateCone(first[0], first[1], first[2]);
    let second = Translate(
        &CreateCone(second[0], second[1], second[2]),
        translation[0],
        translation[1],
        translation[2],
    );
    let result = PerformCommon(&first, &second);
    ValidateResult(&result, expected_area, -1.0, empty);
}

macro_rules! common_translated_cones {
    ($name:ident, $first:expr, $second:expr, $translation:expr, $area:expr, $empty:expr) => {
        #[test]
        fn $name() {
            common_translated_cones($first, $second, $translation, $area, $empty);
        }
    };
}

// Test bopcommon_simple/ZL6: translated cones; common; checkprops result -s empty
common_translated_cones!(
    BOPCommonSimpleTest__TranslatedCones_ZL6,
    [8.0, 4.0, 8.0],
    [4.0, 2.0, 4.0],
    [0.0, 0.0, 8.0],
    -1.0,
    true
);

// Test bopcommon_simple/ZM1: translated cones; common; checkprops result -s empty
common_translated_cones!(
    BOPCommonSimpleTest__TranslatedCones_ZM1,
    [8.0, 4.0, 8.0],
    [2.0, 1.0, 2.0],
    [0.0, 0.0, 8.0],
    -1.0,
    true
);

// Test bopcommon_simple/ZM2: translated cones; common; checkprops result -s 36.7824
common_translated_cones!(
    BOPCommonSimpleTest__TranslatedCones_ZM2,
    [8.0, 4.0, 8.0],
    [2.0, 1.0, 2.0],
    [0.0, 0.0, 6.0],
    36.7824,
    false
);

// Test bopcommon_simple/ZM4: translated cones; common; checkprops result -s 87.403
common_translated_cones!(
    BOPCommonSimpleTest__TranslatedCones_ZM4,
    [8.0, 4.0, 8.0],
    [2.0, 1.0, 9.0],
    [0.0, 0.0, -1.0],
    87.403,
    false
);

// Test bopcommon_simple/ZM5: translated cones; common; checkprops result -s 54.5673
common_translated_cones!(
    BOPCommonSimpleTest__TranslatedCones_ZM5,
    [8.0, 4.0, 8.0],
    [2.0, 1.0, 4.0],
    [0.0, 0.0, 2.0],
    54.5673,
    false
);

// Test bopcommon_simple/ZM6: translated cones; common; checkprops result -s 90.9168
common_translated_cones!(
    BOPCommonSimpleTest__TranslatedCones_ZM6,
    [8.0, 4.0, 8.0],
    [2.0, 1.0, 10.0],
    [0.0, 0.0, -1.0],
    90.9168,
    false
);

// Test bopcommon_simple/ZM7: translated cones; common; checkprops result -s empty
common_translated_cones!(
    BOPCommonSimpleTest__TranslatedCones_ZM7,
    [8.0, 4.0, 8.0],
    [4.0, 8.0, 4.0],
    [0.0, 0.0, -4.0],
    -1.0,
    true
);

// Test bopcommon_simple/ZN2: translated cones; common; checkprops result -s 340.979
common_translated_cones!(
    BOPCommonSimpleTest__TranslatedCones_ZN2,
    [8.0, 4.0, 8.0],
    [8.0, 4.0, 4.0],
    [4.0, 0.0, 0.0],
    340.979,
    false
);

//! Regressions of translated cylinder common regressions.
#![allow(non_snake_case)]

#[path = "bop_test_utilities/mod.rs"]
mod bop_test_utilities;
use bop_test_utilities::*;

fn common_translated_cylinders(
    first: [f64; 2],
    second: [f64; 2],
    translation: [f64; 3],
    area: f64,
    empty: bool,
) {
    let first = CreateCylinder(first[0], first[1]);
    let second = Translate(
        &CreateCylinder(second[0], second[1]),
        translation[0],
        translation[1],
        translation[2],
    );
    ValidateResult(&PerformCommon(&first, &second), area, -1.0, empty);
}

macro_rules! common_translated_cylinders {
    ($name:ident, $first:expr, $second:expr, $translation:expr, $area:expr, $empty:expr) => {
        #[test]
        fn $name() {
            common_translated_cylinders($first, $second, $translation, $area, $empty);
        }
    };
}

// Test bopcommon_simple/ZC8: translated cylinders; common; checkprops result -s empty
common_translated_cylinders!(
    BOPCommonSimpleTest__TranslatedCylinders_ZC8,
    [1.0, 2.0],
    [0.5, 1.0],
    [0.0, 0.0, 2.0],
    -1.0,
    true
);

// Test bopcommon_simple/ZC9: translated cylinders; common; checkprops result -s 4.71239
common_translated_cylinders!(
    BOPCommonSimpleTest__TranslatedCylinders_ZC9,
    [1.0, 2.0],
    [0.5, 1.0],
    [0.0, 0.0, 1.0],
    4.71239,
    false
);

// Test bopcommon_simple/ZD1: translated cylinders; common; checkprops result -s 4.71239
common_translated_cylinders!(
    BOPCommonSimpleTest__TranslatedCylinders_ZD1,
    [1.0, 2.0],
    [0.5, 1.0],
    [0.0, 0.0, 0.5],
    4.71239,
    false
);

// Test bopcommon_simple/ZD3: translated cylinders; common; checkprops result -s 7.85398
common_translated_cylinders!(
    BOPCommonSimpleTest__TranslatedCylinders_ZD3,
    [1.0, 2.0],
    [0.5, 3.0],
    [0.0, 0.0, -1.0],
    7.85398,
    false
);

// Test bopcommon_simple/ZD4: translated cylinders; common; checkprops result -s empty
common_translated_cylinders!(
    BOPCommonSimpleTest__TranslatedCylinders_ZD4,
    [1.0, 2.0],
    [1.0, 2.0],
    [0.0, 0.0, 2.0],
    -1.0,
    true
);

// Test bopcommon_simple/ZF4: translated cylinders; common; checkprops result -s 19.2119
common_translated_cylinders!(
    BOPCommonSimpleTest__TranslatedCylinders_ZF4,
    [1.0, 4.0],
    [1.0, 4.0],
    [1.0, 0.0, 0.0],
    19.2119,
    false
);

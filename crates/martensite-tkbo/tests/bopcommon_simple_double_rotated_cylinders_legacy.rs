//! Regressions of doubly rotated cylinder common regressions.
#![allow(non_snake_case)]

#[path = "bop_test_utilities/mod.rs"]
mod bop_test_utilities;
use bop_test_utilities::*;
use martensite_tkmath::gp::Point3;

fn common_double_rotated_cylinders(
    first: [f64; 2],
    second: [f64; 2],
    rotation_a: ([f64; 3], [f64; 3], f64),
    rotation_b: ([f64; 3], [f64; 3], f64),
    area: f64,
) {
    let first = CreateCylinder(first[0], first[1]);
    let mut second = CreateCylinder(second[0], second[1]);
    RotateAroundAxis(
        &mut second,
        Point3::new(rotation_a.0[0], rotation_a.0[1], rotation_a.0[2]),
        rotation_a.1,
        rotation_a.2,
    );
    RotateAroundAxis(
        &mut second,
        Point3::new(rotation_b.0[0], rotation_b.0[1], rotation_b.0[2]),
        rotation_b.1,
        rotation_b.2,
    );
    ValidateResult(&PerformCommon(&first, &second), area, -1.0, false);
}

macro_rules! common_double_rotated_cylinders {
    ($name:ident, $first:expr, $second:expr, $a:expr, $b:expr, $area:expr) => {
        #[test]
        fn $name() {
            common_double_rotated_cylinders($first, $second, $a, $b, $area);
        }
    };
}

// Test bopcommon_simple/ZD9: twice-rotated cylinders; common; checkprops result -s 16
common_double_rotated_cylinders!(
    BOPCommonSimpleTest__DoubleRotatedCylinders_ZD9,
    [1.0, 4.0],
    [1.0, 4.0],
    ([0.0, 0.0, 2.0], [1.0, 0.0, 0.0], 90.0),
    ([0.0, 0.0, 2.0], [0.0, 1.0, 0.0], 60.0),
    16.0
);

// Test bopcommon_simple/ZE1: twice-rotated cylinders; common; checkprops result -s 16
common_double_rotated_cylinders!(
    BOPCommonSimpleTest__DoubleRotatedCylinders_ZE1,
    [1.0, 4.0],
    [1.0, 4.0],
    ([0.0, 0.0, 2.0], [1.0, 0.0, 0.0], 90.0),
    ([0.0, 0.0, 2.0], [0.0, 1.0, 0.0], 180.0),
    16.0
);

// Test bopcommon_simple/ZE4: twice-rotated cylinders; common; checkprops result -s 7.49533
common_double_rotated_cylinders!(
    BOPCommonSimpleTest__DoubleRotatedCylinders_ZE4,
    [1.0, 4.0],
    [0.5, 4.0],
    ([0.0, 0.0, 2.0], [1.0, 0.0, 0.0], 90.0),
    ([0.0, 0.0, 2.0], [0.0, 1.0, 0.0], 90.0),
    7.49533
);

// Test bopcommon_simple/ZE5: twice-rotated cylinders; common; checkprops result -s 7.49478
common_double_rotated_cylinders!(
    BOPCommonSimpleTest__DoubleRotatedCylinders_ZE5,
    [1.0, 4.0],
    [0.5, 4.0],
    ([0.0, 0.0, 2.0], [1.0, 0.0, 0.0], 90.0),
    ([0.0, 0.0, 2.0], [0.0, 1.0, 0.0], 180.0),
    7.49478
);

// Test bopcommon_simple/ZE6: twice-rotated cylinders; common; checkprops result -s 7.49533
common_double_rotated_cylinders!(
    BOPCommonSimpleTest__DoubleRotatedCylinders_ZE6,
    [1.0, 4.0],
    [0.5, 4.0],
    ([0.0, 0.0, 2.0], [1.0, 0.0, 0.0], 90.0),
    ([0.0, 0.0, 2.0], [0.0, 1.0, 0.0], 270.0),
    7.49533
);

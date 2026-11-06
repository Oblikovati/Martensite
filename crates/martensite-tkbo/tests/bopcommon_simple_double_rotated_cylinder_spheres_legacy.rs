//! Regressions of doubly transformed cylinder-sphere common regressions.
#![allow(non_snake_case)]

#[path = "bop_test_utilities/mod.rs"]
mod bop_test_utilities;
use bop_test_utilities::*;
use martensite_tkmath::gp::Point3;

fn common_double_rotated_cylinder_sphere(
    cylinder: [f64; 2],
    radius: f64,
    translation: [f64; 3],
    first: ([f64; 3], [f64; 3], f64),
    second: ([f64; 3], [f64; 3], f64),
    area: f64,
) {
    let cylinder = CreateCylinder(cylinder[0], cylinder[1]);
    let mut sphere = Translate(
        &CreateSphere(Point3::new(0.0, 0.0, 0.0), radius),
        translation[0],
        translation[1],
        translation[2],
    );
    RotateAroundAxis(
        &mut sphere,
        Point3::new(first.0[0], first.0[1], first.0[2]),
        first.1,
        first.2,
    );
    RotateAroundAxis(
        &mut sphere,
        Point3::new(second.0[0], second.0[1], second.0[2]),
        second.1,
        second.2,
    );
    ValidateResult(&PerformCommon(&cylinder, &sphere), area, -1.0, false);
}

macro_rules! common_double_rotated_cylinder_sphere {
    ($name:ident, $cylinder:expr, $radius:expr, $translation:expr, $first:expr, $second:expr, $area:expr) => {
        #[test]
        fn $name() {
            common_double_rotated_cylinder_sphere(
                $cylinder,
                $radius,
                $translation,
                $first,
                $second,
                $area,
            );
        }
    };
}

// Test bopcommon_simple/ZI1: cylinder and twice-rotated sphere; common; checkprops result -s 150.796
common_double_rotated_cylinder_sphere!(
    BOPCommonSimpleTest__DoubleRotatedCylinderSphere_ZI1,
    [4.0, 8.0],
    4.0,
    [0.0, 0.0, 8.0],
    ([0.0, 0.0, 8.0], [1.0, 0.0, 0.0], 90.0),
    ([0.0, 0.0, 8.0], [0.0, 0.0, 1.0], 90.0),
    150.796
);

// Test bopcommon_simple/ZI2: cylinder and twice-rotated sphere; common; checkprops result -s 150.796
common_double_rotated_cylinder_sphere!(
    BOPCommonSimpleTest__DoubleRotatedCylinderSphere_ZI2,
    [4.0, 8.0],
    4.0,
    [0.0, 0.0, 8.0],
    ([0.0, 0.0, 8.0], [1.0, 0.0, 0.0], 90.0),
    ([0.0, 0.0, 8.0], [0.0, 0.0, 1.0], 180.0),
    150.796
);

// Test bopcommon_simple/ZI3: cylinder and twice-rotated sphere; common; checkprops result -s 150.796
common_double_rotated_cylinder_sphere!(
    BOPCommonSimpleTest__DoubleRotatedCylinderSphere_ZI3,
    [4.0, 8.0],
    4.0,
    [0.0, 0.0, 8.0],
    ([0.0, 0.0, 8.0], [1.0, 0.0, 0.0], 90.0),
    ([0.0, 0.0, 8.0], [0.0, 0.0, 1.0], 270.0),
    150.796
);

// Test bopcommon_simple/ZI5: cylinder and twice-rotated sphere; common; checkprops result -s 37.6989
common_double_rotated_cylinder_sphere!(
    BOPCommonSimpleTest__DoubleRotatedCylinderSphere_ZI5,
    [4.0, 8.0],
    2.0,
    [0.0, 0.0, 8.0],
    ([0.0, 0.0, 8.0], [1.0, 0.0, 0.0], 90.0),
    ([0.0, 0.0, 8.0], [0.0, 1.0, 0.0], 90.0),
    37.6989
);

// Test bopcommon_simple/ZI6: cylinder and twice-rotated sphere; common; checkprops result -s 37.6989
common_double_rotated_cylinder_sphere!(
    BOPCommonSimpleTest__DoubleRotatedCylinderSphere_ZI6,
    [4.0, 8.0],
    2.0,
    [0.0, 0.0, 8.0],
    ([0.0, 0.0, 8.0], [1.0, 0.0, 0.0], 90.0),
    ([0.0, 0.0, 8.0], [0.0, 1.0, 0.0], 180.0),
    37.6989
);

// Test bopcommon_simple/ZI7: cylinder and twice-rotated sphere; common; checkprops result -s 37.6989
common_double_rotated_cylinder_sphere!(
    BOPCommonSimpleTest__DoubleRotatedCylinderSphere_ZI7,
    [4.0, 8.0],
    2.0,
    [0.0, 0.0, 8.0],
    ([0.0, 0.0, 8.0], [1.0, 0.0, 0.0], 90.0),
    ([0.0, 0.0, 8.0], [0.0, 1.0, 0.0], 270.0),
    37.6989
);

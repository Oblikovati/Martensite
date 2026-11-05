//! Regressions of transformed cylinder-sphere common regressions.
#![allow(non_snake_case)]

#[path = "bop_test_utilities/mod.rs"]
mod bop_test_utilities;
use bop_test_utilities::*;
use martensite_tkmath::gp::Point3;

fn common_rotated_cylinder_sphere(
    cylinder: [f64; 2],
    radius: f64,
    translation: [f64; 3],
    pivot: [f64; 3],
    angle: f64,
    area: f64,
    empty: bool,
) {
    let cylinder = CreateCylinder(cylinder[0], cylinder[1]);
    let mut sphere = Translate(
        &CreateSphere(Point3::new(0.0, 0.0, 0.0), radius),
        translation[0],
        translation[1],
        translation[2],
    );
    RotateAboutZ(
        &mut sphere,
        Point3::new(pivot[0], pivot[1], pivot[2]),
        angle,
    );
    ValidateResult(&PerformCommon(&cylinder, &sphere), area, -1.0, empty);
}

macro_rules! common_rotated_cylinder_sphere {
    ($name:ident, $cylinder:expr, $radius:expr, $translation:expr, $pivot:expr, $angle:expr, $area:expr, $empty:expr) => {
        #[test]
        fn $name() {
            common_rotated_cylinder_sphere(
                $cylinder,
                $radius,
                $translation,
                $pivot,
                $angle,
                $area,
                $empty,
            );
        }
    };
}

// Test bopcommon_simple/ZH6: transformed cylinder and sphere; common; checkprops result -s 150.796
common_rotated_cylinder_sphere!(
    BOPCommonSimpleTest__RotatedCylinderSphere_ZH6,
    [4.0, 8.0],
    4.0,
    [0.0, 0.0, 8.0],
    [0.0, 0.0, 8.0],
    90.0,
    150.796,
    false
);

// Test bopcommon_simple/ZH7: transformed cylinder and sphere; common; checkprops result -s 150.796
common_rotated_cylinder_sphere!(
    BOPCommonSimpleTest__RotatedCylinderSphere_ZH7,
    [4.0, 8.0],
    4.0,
    [0.0, 0.0, 8.0],
    [0.0, 0.0, 8.0],
    180.0,
    150.796,
    false
);

// Test bopcommon_simple/ZH8: transformed cylinder and sphere; common; checkprops result -s 150.796
common_rotated_cylinder_sphere!(
    BOPCommonSimpleTest__RotatedCylinderSphere_ZH8,
    [4.0, 8.0],
    4.0,
    [0.0, 0.0, 8.0],
    [0.0, 0.0, 8.0],
    270.0,
    150.796,
    false
);

// Test bopcommon_simple/ZI9: transformed cylinder and sphere; common; checkprops result -s 220.262
common_rotated_cylinder_sphere!(
    BOPCommonSimpleTest__RotatedCylinderSphere_ZI9,
    [4.0, 8.0],
    6.0,
    [0.0, 0.0, 8.0],
    [0.0, 0.0, 8.0],
    90.0,
    220.262,
    false
);

// Test bopcommon_simple/ZJ1: transformed cylinder and sphere; common; checkprops result -s 220.262
common_rotated_cylinder_sphere!(
    BOPCommonSimpleTest__RotatedCylinderSphere_ZJ1,
    [4.0, 8.0],
    6.0,
    [0.0, 0.0, 8.0],
    [0.0, 0.0, 8.0],
    180.0,
    220.262,
    false
);

// Test bopcommon_simple/ZJ2: transformed cylinder and sphere; common; checkprops result -s 220.262
common_rotated_cylinder_sphere!(
    BOPCommonSimpleTest__RotatedCylinderSphere_ZJ2,
    [4.0, 8.0],
    6.0,
    [0.0, 0.0, 8.0],
    [0.0, 0.0, 8.0],
    270.0,
    220.262,
    false
);

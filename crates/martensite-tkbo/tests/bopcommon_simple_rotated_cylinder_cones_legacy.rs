//! Regressions of transformed cylinder-cone common regressions.
#![allow(non_snake_case)]

#[path = "bop_test_utilities/mod.rs"]
mod bop_test_utilities;
use bop_test_utilities::*;
use martensite_tkmath::gp::Point3;

fn common_rotated_cylinder_cone(
    cylinder: [f64; 2],
    cone: [f64; 3],
    translation: [f64; 3],
    pivot: [f64; 3],
    angle: f64,
    area: f64,
    empty: bool,
) {
    let cylinder = CreateCylinder(cylinder[0], cylinder[1]);
    let mut cone = Translate(
        &CreateCone(cone[0], cone[1], cone[2]),
        translation[0],
        translation[1],
        translation[2],
    );
    RotateAboutZ(&mut cone, Point3::new(pivot[0], pivot[1], pivot[2]), angle);
    ValidateResult(&PerformCommon(&cylinder, &cone), area, -1.0, empty);
}

macro_rules! common_rotated_cylinder_cone {
    ($name:ident, $cylinder:expr, $cone:expr, $translation:expr, $pivot:expr, $angle:expr, $area:expr, $empty:expr) => {
        #[test]
        fn $name() {
            common_rotated_cylinder_cone(
                $cylinder,
                $cone,
                $translation,
                $pivot,
                $angle,
                $area,
                $empty,
            );
        }
    };
}

// Test bopcommon_simple/ZK2: transformed cylinder and cone; common; checkprops result -s empty
common_rotated_cylinder_cone!(
    BOPCommonSimpleTest__RotatedCylinderCone_ZK2,
    [4.0, 8.0],
    [4.0, 2.0, 4.0],
    [0.0, 0.0, 8.0],
    [0.0, 0.0, 8.0],
    90.0,
    -1.0,
    true
);

// Test bopcommon_simple/ZK3: transformed cylinder and cone; common; checkprops result -s empty
common_rotated_cylinder_cone!(
    BOPCommonSimpleTest__RotatedCylinderCone_ZK3,
    [4.0, 8.0],
    [4.0, 2.0, 4.0],
    [0.0, 0.0, 8.0],
    [0.0, 0.0, 8.0],
    180.0,
    -1.0,
    true
);

// Test bopcommon_simple/ZK4: transformed cylinder and cone; common; checkprops result -s empty
common_rotated_cylinder_cone!(
    BOPCommonSimpleTest__RotatedCylinderCone_ZK4,
    [4.0, 8.0],
    [4.0, 2.0, 4.0],
    [0.0, 0.0, 8.0],
    [0.0, 0.0, 8.0],
    270.0,
    -1.0,
    true
);

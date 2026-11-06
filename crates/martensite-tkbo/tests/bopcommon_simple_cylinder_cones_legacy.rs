//! Regressions of translated cylinder-cone common regressions.
#![allow(non_snake_case)]

#[path = "bop_test_utilities/mod.rs"]
mod bop_test_utilities;
use bop_test_utilities::*;

fn common_cylinder_cone(
    cylinder: [f64; 2],
    cone: [f64; 3],
    translation: [f64; 3],
    area: f64,
    empty: bool,
) {
    let cylinder = CreateCylinder(cylinder[0], cylinder[1]);
    let cone = Translate(
        &CreateCone(cone[0], cone[1], cone[2]),
        translation[0],
        translation[1],
        translation[2],
    );
    ValidateResult(&PerformCommon(&cylinder, &cone), area, -1.0, empty);
}

macro_rules! common_cylinder_cone {
    ($name:ident, $cylinder:expr, $cone:expr, $translation:expr, $area:expr, $empty:expr) => {
        #[test]
        fn $name() {
            common_cylinder_cone($cylinder, $cone, $translation, $area, $empty);
        }
    };
}

// Test bopcommon_simple/ZJ4: translated cylinder and cone; common; checkprops result -s empty
common_cylinder_cone!(
    BOPCommonSimpleTest__CylinderCone_ZJ4,
    [4.0, 8.0],
    [2.0, 1.0, 4.0],
    [0.0, 0.0, 8.0],
    -1.0,
    true
);

// Test bopcommon_simple/ZJ5: translated cylinder and cone; common; checkprops result -s 54.5673
common_cylinder_cone!(
    BOPCommonSimpleTest__CylinderCone_ZJ5,
    [4.0, 8.0],
    [1.0, 2.0, 4.0],
    [0.0, 0.0, 4.0],
    54.5673,
    false
);

// Test bopcommon_simple/ZJ7: translated cylinder and cone; common; checkprops result -s 87.403
common_cylinder_cone!(
    BOPCommonSimpleTest__CylinderCone_ZJ7,
    [4.0, 8.0],
    [2.0, 1.0, 9.0],
    [0.0, 0.0, -1.0],
    87.403,
    false
);

// Test bopcommon_simple/ZJ8: translated cylinder and cone; common; checkprops result -s 54.5673
common_cylinder_cone!(
    BOPCommonSimpleTest__CylinderCone_ZJ8,
    [4.0, 8.0],
    [2.0, 1.0, 4.0],
    [0.0, 0.0, 2.0],
    54.5673,
    false
);

// Test bopcommon_simple/ZJ9: translated cylinder and cone; common; checkprops result -s 90.9168
common_cylinder_cone!(
    BOPCommonSimpleTest__CylinderCone_ZJ9,
    [4.0, 8.0],
    [2.0, 1.0, 10.0],
    [0.0, 0.0, -1.0],
    90.9168,
    false
);

// Test bopcommon_simple/ZK1: translated cylinder and cone; common; checkprops result -s empty
common_cylinder_cone!(
    BOPCommonSimpleTest__CylinderCone_ZK1,
    [4.0, 8.0],
    [4.0, 2.0, 4.0],
    [0.0, 0.0, 8.0],
    -1.0,
    true
);

//! Regressions of remaining transformed cylinder-sphere common regressions.
#![allow(non_snake_case)]

#[path = "bop_test_utilities/mod.rs"]
mod bop_test_utilities;
use bop_test_utilities::*;
use martensite_tkmath::gp::Point3;

fn common_cylinder_sphere(
    cylinder: [f64; 2],
    radius: f64,
    translation: [f64; 3],
    rotation: Option<([f64; 3], [f64; 3], f64)>,
    area: f64,
) {
    let cylinder = CreateCylinder(cylinder[0], cylinder[1]);
    let mut sphere = Translate(
        &CreateSphere(Point3::new(0.0, 0.0, 0.0), radius),
        translation[0],
        translation[1],
        translation[2],
    );
    if let Some((pivot, axis, angle)) = rotation {
        RotateAroundAxis(
            &mut sphere,
            Point3::new(pivot[0], pivot[1], pivot[2]),
            axis,
            angle,
        );
    }
    ValidateResult(&PerformCommon(&cylinder, &sphere), area, -1.0, false);
}

macro_rules! common_cylinder_sphere {
    ($name:ident, $cylinder:expr, $radius:expr, $translation:expr, $rotation:expr, $area:expr) => {
        #[test]
        fn $name() {
            common_cylinder_sphere($cylinder, $radius, $translation, $rotation, $area);
        }
    };
}

// Test bopcommon_simple/ZH5: transformed cylinder and sphere; common; checkprops result -s 150.796
common_cylinder_sphere!(
    BOPCommonSimpleTest__RemainingCylinderSphere_ZH5,
    [4.0, 8.0],
    4.0,
    [0.0, 0.0, 8.0],
    None,
    150.796
);

// Test bopcommon_simple/ZH9: transformed cylinder and sphere; common; checkprops result -s 150.796
common_cylinder_sphere!(
    BOPCommonSimpleTest__RemainingCylinderSphere_ZH9,
    [4.0, 8.0],
    4.0,
    [0.0, 0.0, 8.0],
    Some(([0.0, 0.0, 8.0], [1.0, 0.0, 0.0], 90.0)),
    150.796
);

// Test bopcommon_simple/ZI4: transformed cylinder and sphere; common; checkprops result -s 37.6989
common_cylinder_sphere!(
    BOPCommonSimpleTest__RemainingCylinderSphere_ZI4,
    [4.0, 8.0],
    2.0,
    [0.0, 0.0, 8.0],
    Some(([0.0, 0.0, 8.0], [1.0, 0.0, 0.0], 90.0)),
    37.6989
);

// Test bopcommon_simple/ZI8: transformed cylinder and sphere; common; checkprops result -s 220.262
common_cylinder_sphere!(
    BOPCommonSimpleTest__RemainingCylinderSphere_ZI8,
    [4.0, 8.0],
    6.0,
    [0.0, 0.0, 8.0],
    None,
    220.262
);

// Test bopcommon_simple/ZJ3: transformed cylinder and sphere; common; checkprops result -s 220.262
common_cylinder_sphere!(
    BOPCommonSimpleTest__RemainingCylinderSphere_ZJ3,
    [4.0, 8.0],
    6.0,
    [0.0, 0.0, 8.0],
    Some(([0.0, 0.0, 8.0], [0.0, 1.0, 0.0], 90.0)),
    220.262
);

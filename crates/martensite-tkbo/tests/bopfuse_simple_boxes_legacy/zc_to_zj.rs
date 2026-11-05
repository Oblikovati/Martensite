#![allow(non_snake_case)]
use super::*;
fn cylinder_box(radius: f64, height: f64, origin: [f64; 3], size: [f64; 3], area: f64) {
    let cylinder = CreateCylinder(radius, height);
    let box_shape = CreateBox(
        Point3::new(origin[0], origin[1], origin[2]),
        size[0],
        size[1],
        size[2],
    );
    ValidateResult(&PerformFuse(&cylinder, &box_shape), area, -1., false);
}
macro_rules! cb {
    ($n:ident,$r:expr,$h:expr,$o:expr,$s:expr,$a:expr) => {
        #[test]
        fn $n() {
            cylinder_box($r, $h, $o, $s, $a);
        }
    };
}
cb!(
    BOPFuseSimpleTest__CylinderPositiveHalfZC1,
    1.,
    2.,
    [0., 0., 0.],
    [2., 2., 1.],
    29.708
);
cb!(
    BOPFuseSimpleTest__CylinderNegativeHalfZC2,
    1.,
    2.,
    [-2., -2., 0.],
    [2., 2., 1.],
    29.708
);
cb!(
    BOPFuseSimpleTest__CylinderCornerHalfZC3,
    1.,
    2.,
    [0., 0., 0.],
    [3f64.sqrt() / 2., 3f64.sqrt() / 2., 1.],
    19.1684
);
#[test]
fn BOPFuseSimpleTest__CylinderCornerHalfZC4() {
    let cylinder = CreateCylinder(1., 2.);
    let mut b = CreateBox(
        Point3::new(0., 0., 0.),
        3f64.sqrt() / 2.,
        3f64.sqrt() / 2.,
        1.,
    );
    RotateAboutZ(&mut b, Point3::new(0., 0., 0.), -30.);
    ValidateResult(&PerformFuse(&cylinder, &b), 19.1684, -1., false);
}
cb!(
    BOPFuseSimpleTest__CylinderRaisedBoxZC6,
    1.,
    2.,
    [-1., -2., 0.5],
    [2., 2., 1.],
    26.5664
);
cb!(
    BOPFuseSimpleTest__TallCylinderRaisedBoxZC7,
    1.,
    4.,
    [-1., 0., 1.],
    [2., 2., 2.],
    41.9911
);
#[test]
fn BOPFuseSimpleTest__CylinderStackZC8() {
    let a = CreateCylinder(1., 2.);
    let b = Translate(&CreateCylinder(0.5, 1.), 0., 0., 2.);
    ValidateResult(&PerformFuse(&a, &b), 21.9911, -1., false);
}
#[test]
fn BOPFuseSimpleTest__CylinderOverlapZC9() {
    let a = CreateCylinder(1., 2.);
    let b = Translate(&CreateCylinder(0.5, 1.), 0., 0., 1.);
    ValidateResult(&PerformFuse(&a, &b), 18.8496, -1., false);
}
#[test]
fn BOPFuseSimpleTest__CylinderTiltedBox_ZC5() {
    let cylinder = CreateCylinder(1., 2.);
    let mut box_shape = CreateBox(Point3::new(0., 0., -0.2), 1.4, 1.4, 1.4);
    RotateAroundAxis(
        &mut box_shape,
        Point3::new(0., 0., -0.2),
        [-1., 1., 0.],
        -45.,
    );
    ValidateResult(&PerformFuse(&cylinder, &box_shape), 19.7221, -1., false);
}
fn fuse_cylinders(
    second_radius: f64,
    second_height: f64,
    translation: [f64; 3],
    angle: f64,
    area: f64,
) {
    let first = CreateCylinder(1., 2.);
    let mut second = Translate(
        &CreateCylinder(second_radius, second_height),
        translation[0],
        translation[1],
        translation[2],
    );
    RotateAboutZ(&mut second, Point3::new(0., 0., 0.), angle);
    ValidateResult(&PerformFuse(&first, &second), area, -1., false);
}
macro_rules! cc {
    ($n:ident,$r:expr,$h:expr,$t:expr,$a:expr,$area:expr) => {
        #[test]
        fn $n() {
            fuse_cylinders($r, $h, $t, $a, $area);
        }
    };
}
cc!(
    BOPFuseSimpleTest__CylinderNestedZD1,
    0.5,
    1.,
    [0., 0., 0.5],
    0.,
    18.8496
);
cc!(
    BOPFuseSimpleTest__CylinderNestedZD2,
    0.5,
    2.,
    [0., 0., 0.],
    0.,
    18.8496
);
cc!(
    BOPFuseSimpleTest__CylinderExtendedZD3,
    0.5,
    3.,
    [0., 0., -1.],
    0.,
    21.9911
);
cc!(
    BOPFuseSimpleTest__CylinderStackZD4,
    1.,
    2.,
    [0., 0., 2.],
    0.,
    31.4159
);
cc!(
    BOPFuseSimpleTest__CylinderStackZD5,
    1.,
    2.,
    [0., 0., 2.],
    90.,
    31.4159
);
cc!(
    BOPFuseSimpleTest__CylinderStackZD6,
    1.,
    2.,
    [0., 0., 2.],
    180.,
    31.4159
);
cc!(
    BOPFuseSimpleTest__CylinderStackZD7,
    1.,
    2.,
    [0., 0., 2.],
    270.,
    31.4159
);

fn fuse_cone_box(radii: [f64; 2], height: f64, translation: [f64; 3], expected_area: f64) {
    let box_shape = CreateBox(Point3::new(0., 0., 0.), 4., 4., 4.);
    let cone = Translate(
        &CreateCone(radii[0], radii[1], height),
        translation[0],
        translation[1],
        translation[2],
    );
    ValidateResult(&PerformFuse(&box_shape, &cone), expected_area, -1., false);
}

macro_rules! cone_box_fuse {
    ($name:ident, $radii:expr, $height:expr, $translation:expr, $area:expr) => {
        #[test]
        fn $name() {
            fuse_cone_box($radii, $height, $translation, $area);
        }
    };
}

cone_box_fuse!(
    BOPFuseSimpleTest__ConeOverBoxZF5,
    [1., 0.5],
    2.,
    [2., 2., 4.],
    103.359
);
cone_box_fuse!(
    BOPFuseSimpleTest__ConeInsideBoxZF6,
    [1., 0.5],
    2.,
    [2., 2., 2.],
    96.
);
cone_box_fuse!(
    BOPFuseSimpleTest__ConeBoxZF7,
    [1., 0.5],
    4.,
    [2., 2., 0.],
    96.
);
cone_box_fuse!(
    BOPFuseSimpleTest__ConeBelowBoxZF8,
    [1., 0.5],
    5.,
    [2., 2., -1.],
    102.596
);
cone_box_fuse!(
    BOPFuseSimpleTest__ConeContainedBoxZF9,
    [1., 0.5],
    2.,
    [2., 2., 1.],
    96.
);
cone_box_fuse!(
    BOPFuseSimpleTest__LargeConeSideZG1,
    [3., 2.],
    4.,
    [4., 2., 0.],
    137.238
);
cone_box_fuse!(
    BOPFuseSimpleTest__LargeConeBaseZG2,
    [3., 2.],
    4.,
    [2., 0., 0.],
    137.238
);
cone_box_fuse!(
    BOPFuseSimpleTest__LargeConeSideZG3,
    [3., 2.],
    4.,
    [0., 2., 0.],
    137.238
);
cone_box_fuse!(
    BOPFuseSimpleTest__MediumConeSideZG5,
    [2., 1.],
    4.,
    [4., 2., 0.],
    111.284
);
cone_box_fuse!(
    BOPFuseSimpleTest__MediumConeBaseZG6,
    [2., 1.],
    4.,
    [2., 0., 0.],
    111.284
);
cone_box_fuse!(
    BOPFuseSimpleTest__MediumConeSideZG7,
    [2., 1.],
    4.,
    [0., 2., 0.],
    111.284
);
cone_box_fuse!(
    BOPFuseSimpleTest__ConeTopCornerZG8,
    [1., 0.5],
    2.,
    [3., 1., 4.],
    103.359
);
cone_box_fuse!(
    BOPFuseSimpleTest__ConeTopInsetZG9,
    [1., 0.5],
    2.,
    [1., 1., 4.],
    103.359
);

#[test]
fn BOPFuseSimpleTest__OffsetCylinderZF2() {
    let first = CreateCylinder(1., 4.);
    let mut second = Translate(&CreateCylinder(1., 4.), 1., 0., 0.);
    RotateAboutZ(&mut second, Point3::new(1., 0., 0.), -120.);
    RotateAboutZ(&mut second, Point3::new(0., 0., 0.), 60.);
    ValidateResult(&PerformFuse(&first, &second), 43.62, -1., false);
}

#[test]
fn BOPFuseSimpleTest__OffsetCylinderZF3() {
    let first = CreateCylinder(1., 4.);
    let mut second = Translate(&CreateCylinder(1., 4.), 1., 0., 0.);
    RotateAboutZ(&mut second, Point3::new(1., 0., 0.), 120.);
    RotateAboutZ(&mut second, Point3::new(0., 0., 0.), 60.);
    ValidateResult(&PerformFuse(&first, &second), 43.62, -1., false);
}

#[test]
fn BOPFuseSimpleTest__OffsetCylinderZF4() {
    let first = CreateCylinder(1., 4.);
    let second = Translate(&CreateCylinder(1., 4.), 1., 0., 0.);
    ValidateResult(&PerformFuse(&first, &second), 43.62, -1., false);
}

#[test]
fn BOPFuseSimpleTest__LargeConeBaseRotatedZG4() {
    let box_shape = CreateBox(Point3::new(0., 0., 0.), 4., 4., 4.);
    let mut cone = Translate(&CreateCone(3., 2., 4.), 2., 0., 0.);
    RotateAboutZ(&mut cone, Point3::new(2., 0., 0.), 30.);
    ValidateResult(&PerformFuse(&box_shape, &cone), 137.238, -1., false);
}

#![allow(non_snake_case)]
use super::*;

fn fuse_cone_box(radii: [f64; 2], translation: [f64; 3], area: f64) {
    let box_shape = CreateBox(Point3::new(0., 0., 0.), 4., 4., 4.);
    let cone = Translate(
        &CreateCone(radii[0], radii[1], 4.),
        translation[0],
        translation[1],
        translation[2],
    );
    ValidateResult(&PerformFuse(&box_shape, &cone), area, -1., false);
}

#[test]
fn BOPFuseSimpleTest__LargeConeSideZH1() {
    fuse_cone_box([5., 4.], [6., 2., 0.], 273.296);
}

#[test]
fn BOPFuseSimpleTest__LargeConeBaseZH2() {
    fuse_cone_box([5., 4.], [-2., 2., 0.], 273.296);
}

#[test]
fn BOPFuseSimpleTest__LargeConeBaseRotatedZH3() {
    let box_shape = CreateBox(Point3::new(0., 0., 0.), 4., 4., 4.);
    let mut cone = Translate(&CreateCone(5., 4., 4.), -2., 2., 0.);
    RotateAboutZ(&mut cone, Point3::new(-2., 2., 0.), 30.);
    ValidateResult(&PerformFuse(&box_shape, &cone), 273.296, -1., false);
}

#[test]
fn BOPFuseSimpleTest__TaperedConeBaseRotatedZH4() {
    let box_shape = CreateBox(Point3::new(0., 0., 0.), 4., 4., 4.);
    let mut cone = Translate(&CreateCone(5., 3.5, 4.), -2., 2., 0.);
    RotateAboutZ(&mut cone, Point3::new(-2., 2., 0.), 30.);
    ValidateResult(&PerformFuse(&box_shape, &cone), 262.691, -1., false);
}

fn fuse_cylinder_sphere(radius: f64, rotations: &[[f64; 4]], area: f64) {
    let cylinder = CreateCylinder(4., 8.);
    let mut sphere = CreateSphere(Point3::new(0., 0., 8.), radius);
    for [x, y, z, angle] in rotations {
        RotateAroundAxis(&mut sphere, Point3::new(0., 0., 8.), [*x, *y, *z], *angle);
    }
    ValidateResult(&PerformFuse(&cylinder, &sphere), area, -1., false);
}

macro_rules! cylinder_sphere_fuse {
    ($name:ident, $radius:expr, $rotations:expr, $area:expr) => {
        #[test]
        fn $name() {
            fuse_cylinder_sphere($radius, $rotations, $area);
        }
    };
}

cylinder_sphere_fuse!(BOPFuseSimpleTest__CylinderSphereZH5, 4., &[], 351.858);
cylinder_sphere_fuse!(
    BOPFuseSimpleTest__CylinderSphereZH6,
    4.,
    &[[0., 0., 1., 90.]],
    351.858
);
cylinder_sphere_fuse!(
    BOPFuseSimpleTest__CylinderSphereZH7,
    4.,
    &[[0., 0., 1., 180.]],
    351.858
);
cylinder_sphere_fuse!(
    BOPFuseSimpleTest__CylinderSphereZH8,
    4.,
    &[[0., 0., 1., 270.]],
    351.858
);
cylinder_sphere_fuse!(
    BOPFuseSimpleTest__CylinderSphereZH9,
    4.,
    &[[1., 0., 0., 90.]],
    351.858
);
cylinder_sphere_fuse!(
    BOPFuseSimpleTest__CylinderSphereZI1,
    4.,
    &[[1., 0., 0., 90.], [0., 0., 1., 90.]],
    351.858
);
cylinder_sphere_fuse!(
    BOPFuseSimpleTest__CylinderSphereZI2,
    4.,
    &[[1., 0., 0., 90.], [0., 0., 1., 180.]],
    351.858
);
cylinder_sphere_fuse!(
    BOPFuseSimpleTest__CylinderSphereZI3,
    4.,
    &[[1., 0., 0., 90.], [0., 0., 1., 270.]],
    351.858
);
cylinder_sphere_fuse!(
    BOPFuseSimpleTest__SmallCylinderSphereZI4,
    2.,
    &[[1., 0., 0., 90.]],
    314.159
);
cylinder_sphere_fuse!(
    BOPFuseSimpleTest__SmallCylinderSphereZI5,
    2.,
    &[[1., 0., 0., 90.], [0., 1., 0., 90.]],
    314.159
);
cylinder_sphere_fuse!(
    BOPFuseSimpleTest__SmallCylinderSphereZI6,
    2.,
    &[[1., 0., 0., 90.], [0., 1., 0., 180.]],
    314.159
);
cylinder_sphere_fuse!(
    BOPFuseSimpleTest__SmallCylinderSphereZI7,
    2.,
    &[[1., 0., 0., 90.], [0., 1., 0., 270.]],
    314.159
);
cylinder_sphere_fuse!(BOPFuseSimpleTest__LargeCylinderSphereZI8, 6., &[], 533.72);
cylinder_sphere_fuse!(
    BOPFuseSimpleTest__LargeCylinderSphereZI9,
    6.,
    &[[0., 0., 1., 90.]],
    533.72
);

cylinder_sphere_fuse!(
    BOPFuseSimpleTest__LargeCylinderSphereZJ1,
    6.,
    &[[0., 0., 1., 180.]],
    533.72
);
cylinder_sphere_fuse!(
    BOPFuseSimpleTest__LargeCylinderSphereZJ2,
    6.,
    &[[0., 0., 1., 270.]],
    533.72
);
cylinder_sphere_fuse!(
    BOPFuseSimpleTest__LargeCylinderSphereZJ3,
    6.,
    &[[0., 1., 0., 90.]],
    533.721
);

fn fuse_cylinder_cone(radii: [f64; 2], height: f64, z: f64, area: f64) {
    let cylinder = CreateCylinder(4., 8.);
    let cone = Translate(&CreateCone(radii[0], radii[1], height), 0., 0., z);
    ValidateResult(&PerformFuse(&cylinder, &cone), area, -1., false);
}

macro_rules! cylinder_cone_fuse {
    ($name:ident, $radii:expr, $height:expr, $z:expr, $area:expr) => {
        #[test]
        fn $name() {
            fuse_cylinder_cone($radii, $height, $z, $area);
        }
    };
}

cylinder_cone_fuse!(
    BOPFuseSimpleTest__CylinderConeZJ4,
    [2., 1.],
    4.,
    8.,
    331.027
);
cylinder_cone_fuse!(
    BOPFuseSimpleTest__CylinderConeZJ5,
    [1., 2.],
    4.,
    4.,
    301.593
);
cylinder_cone_fuse!(
    BOPFuseSimpleTest__CylinderConeZJ6,
    [2., 1.],
    8.,
    0.,
    301.593
);
cylinder_cone_fuse!(
    BOPFuseSimpleTest__CylinderConeZJ7,
    [2., 1.],
    9.,
    -1.,
    315.243
);
cylinder_cone_fuse!(
    BOPFuseSimpleTest__CylinderConeZJ8,
    [2., 1.],
    4.,
    2.,
    301.593
);
cylinder_cone_fuse!(
    BOPFuseSimpleTest__CylinderConeZJ9,
    [2., 1.],
    10.,
    -1.,
    321.102
);
cylinder_cone_fuse!(
    BOPFuseSimpleTest__CylinderConeZK1,
    [4., 2.],
    4.,
    8.,
    348.192
);

fn fuse_rotated_cylinder_cone(angle: f64, area: f64) {
    let cylinder = CreateCylinder(4., 8.);
    let mut cone = Translate(&CreateCone(4., 2., 4.), 0., 0., 8.);
    RotateAboutZ(&mut cone, Point3::new(0., 0., 8.), angle);
    ValidateResult(&PerformFuse(&cylinder, &cone), area, -1., false);
}

#[test]
fn BOPFuseSimpleTest__CylinderConeZK2() {
    fuse_rotated_cylinder_cone(90., 348.192);
}

#[test]
fn BOPFuseSimpleTest__CylinderConeZK3() {
    fuse_rotated_cylinder_cone(180., 348.192);
}

#[test]
fn BOPFuseSimpleTest__CylinderConeZK4() {
    fuse_rotated_cylinder_cone(270., 348.192);
}

fn fuse_rotated_cylinder_torus(angle: f64) {
    let cylinder = CreateCylinder(4., 8.);
    let mut torus = Translate(&CreateTorus(4., 1.), 0., 0., 4.);
    RotateAboutZ(&mut torus, Point3::new(0., 0., 4.), angle);
    ValidateResult(&PerformFuse(&cylinder, &torus), 342.851, -1., false);
}

#[test]
fn BOPFuseSimpleTest__CylinderTorusZL2() {
    fuse_rotated_cylinder_torus(0.);
}

#[test]
fn BOPFuseSimpleTest__CylinderTorusZL3() {
    fuse_rotated_cylinder_torus(90.);
}

#[test]
fn BOPFuseSimpleTest__CylinderTorusZL4() {
    fuse_rotated_cylinder_torus(180.);
}

#[test]
fn BOPFuseSimpleTest__CylinderTorusZL5() {
    fuse_rotated_cylinder_torus(270.);
}

fn fuse_rotated_cones(angle: f64) {
    let first = CreateCone(8., 4., 8.);
    let mut second = Translate(&CreateCone(4., 2., 4.), 0., 0., 8.);
    RotateAboutZ(&mut second, Point3::new(0., 0., 2.), angle);
    ValidateResult(&PerformFuse(&first, &second), 635.117, -1., false);
}

#[test]
fn BOPFuseSimpleTest__ConeConeZL6() {
    fuse_rotated_cones(0.);
}

#[test]
fn BOPFuseSimpleTest__ConeConeZL7() {
    fuse_rotated_cones(90.);
}

#[test]
fn BOPFuseSimpleTest__ConeConeZL8() {
    fuse_rotated_cones(180.);
}

#[test]
fn BOPFuseSimpleTest__ConeConeZL9() {
    fuse_rotated_cones(270.);
}

fn fuse_cones(second: [f64; 3], z: f64, area: f64) {
    let first = CreateCone(8., 4., 8.);
    let second = Translate(&CreateCone(second[0], second[1], second[2]), 0., 0., z);
    ValidateResult(&PerformFuse(&first, &second), area, -1., false);
}

macro_rules! cone_fuse {
    ($name:ident, $second:expr, $z:expr, $area:expr) => {
        #[test]
        fn $name() {
            fuse_cones($second, $z, $area);
        }
    };
}

cone_fuse!(BOPFuseSimpleTest__ConeConeZM1, [2., 1., 2.], 8., 600.168);
cone_fuse!(BOPFuseSimpleTest__ConeConeZM2, [2., 1., 2.], 6., 588.519);
cone_fuse!(BOPFuseSimpleTest__ConeConeZM3, [2., 1., 8.], 0., 588.519);
cone_fuse!(BOPFuseSimpleTest__ConeConeZM4, [2., 1., 9.], -1., 602.168);
cone_fuse!(BOPFuseSimpleTest__ConeConeZM5, [2., 1., 4.], 2., 588.519);
cone_fuse!(BOPFuseSimpleTest__ConeConeZM6, [2., 1., 10.], -1., 608.028);
cone_fuse!(BOPFuseSimpleTest__ConeConeZM7, [4., 8., 4.], -4., 650.98);
fn fuse_box_cylinder(radius: f64, height: f64, translation: [f64; 3], area: f64) {
    let box_shape = CreateBox(Point3::new(0., 0., 0.), 4., 4., 4.);
    let cylinder = Translate(
        &CreateCylinder(radius, height),
        translation[0],
        translation[1],
        translation[2],
    );
    ValidateResult(&PerformFuse(&box_shape, &cylinder), area, -1., false);
}

macro_rules! box_cylinder_fuse {
    ($name:ident, $radius:expr, $height:expr, $translation:expr, $area:expr) => {
        #[test]
        fn $name() {
            fuse_box_cylinder($radius, $height, $translation, $area);
        }
    };
}

box_cylinder_fuse!(
    BOPFuseSimpleTest__BoxCylinderZN3,
    1.,
    6.,
    [2., 2., -2.],
    108.566
);
box_cylinder_fuse!(BOPFuseSimpleTest__BoxCylinderZN4, 1., 2., [2., 2., 1.], 96.);
box_cylinder_fuse!(
    BOPFuseSimpleTest__BoxCylinderZN5,
    2.,
    4.,
    [4., 2., 0.],
    117.699
);
box_cylinder_fuse!(
    BOPFuseSimpleTest__BoxCylinderZN6,
    2.,
    4.,
    [2., 0., 0.],
    117.699
);
box_cylinder_fuse!(
    BOPFuseSimpleTest__BoxCylinderZN7,
    2.,
    4.,
    [0., 2., 0.],
    117.699
);
box_cylinder_fuse!(
    BOPFuseSimpleTest__BoxCylinderZN8,
    1.,
    4.,
    [4., 2., 0.],
    103.708
);
box_cylinder_fuse!(
    BOPFuseSimpleTest__BoxCylinderZN9,
    1.,
    4.,
    [2., 0., 0.],
    103.708
);

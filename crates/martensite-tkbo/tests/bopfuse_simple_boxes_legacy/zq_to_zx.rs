#![allow(non_snake_case)]
use super::*;
use martensite_tkbrep::{topexp::Explorer, topods};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::Vector3;

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
    BOPFuseSimpleTest__BoxCylinderZO1,
    1.,
    4.,
    [0., 2., 0.],
    103.708
);
box_cylinder_fuse!(
    BOPFuseSimpleTest__BoxCylinderZO2,
    1.,
    4.,
    [4., 3., 0.],
    103.708
);
box_cylinder_fuse!(
    BOPFuseSimpleTest__BoxCylinderZO3,
    1.,
    4.,
    [3., 0., 0.],
    103.708
);
box_cylinder_fuse!(
    BOPFuseSimpleTest__BoxCylinderZO4,
    1.,
    4.,
    [0., 3., 0.],
    103.708
);
box_cylinder_fuse!(
    BOPFuseSimpleTest__BoxCylinderZO5,
    1.,
    4.,
    [1., 0., 0.],
    103.708
);
box_cylinder_fuse!(
    BOPFuseSimpleTest__BoxCylinderZO6,
    4.,
    4.,
    [-2., 2., 0.],
    237.982
);
box_cylinder_fuse!(
    BOPFuseSimpleTest__BoxCylinderZO9,
    1.,
    4.,
    [3., 1., 4.],
    121.133
);
box_cylinder_fuse!(
    BOPFuseSimpleTest__BoxCylinderZP1,
    1.,
    4.,
    [1., 1., 4.],
    121.133
);
box_cylinder_fuse!(
    BOPFuseSimpleTest__BoxCylinderZP2,
    2.,
    6.,
    [0., 0., -2.],
    161.681
);

fn fuse_rotated_box_cylinder(angle: f64) {
    let box_shape = CreateBox(Point3::new(0., 0., 0.), 4., 4., 4.);
    let mut cylinder = Translate(&CreateCylinder(4., 4.), -2., 2., 0.);
    RotateAboutZ(&mut cylinder, Point3::new(-2., 2., 0.), angle);
    ValidateResult(&PerformFuse(&box_shape, &cylinder), 237.982, -1., false);
}

#[test]
fn BOPFuseSimpleTest__BoxCylinderZO7() {
    fuse_rotated_box_cylinder(30.);
}

#[test]
fn BOPFuseSimpleTest__BoxCylinderZO8() {
    fuse_rotated_box_cylinder(90.);
}

fn fuse_rotated_cones(angle: f64) {
    let first = CreateCone(8., 4., 8.);
    let mut second = Translate(&CreateCone(4., 8., 4.), 0., 0., -4.);
    RotateAboutZ(&mut second, Point3::new(0., 0., -4.), angle);
    ValidateResult(&PerformFuse(&first, &second), 650.98, -1., false);
}

#[test]
fn BOPFuseSimpleTest__ConeConeZM8() {
    fuse_rotated_cones(90.);
}

#[test]
fn BOPFuseSimpleTest__ConeConeZM9() {
    fuse_rotated_cones(180.);
}

#[test]
fn BOPFuseSimpleTest__ConeConeZN1() {
    fuse_rotated_cones(270.);
}

#[test]
fn BOPFuseSimpleTest__PrismCylinderZP3() {
    let points = [
        Point3::new(0., -1., 0.),
        Point3::new(0., 1., 0.),
        Point3::new(0., 1., 2.),
        Point3::new(0., -1., 2.),
    ];
    let wire = CreatePolygonWire(&points, true);
    let face = CreateFaceFromWire(&wire);
    let prism = CreatePrism(&face, Vector3::new(1., 0., 0.));
    ValidateResult(
        &PerformFuse(&prism, &CreateCylinder(1., 4.)),
        33.9911,
        -1.,
        false,
    );
}

#[test]
fn BOPFuseSimpleTest__ThreeToriZP6() {
    let first = CreateTorus(100., 20.);
    let mut second = first.clone();
    let mut third = first.clone();
    RotateAroundAxis(&mut second, Point3::new(0., 0., 0.), [1., 0., 0.], 90.);
    RotateAroundAxis(&mut third, Point3::new(0., 0., 0.), [0., 1., 0.], 90.);
    let first_fuse = PerformFuse(&first, &second);
    ValidateResult(&PerformFuse(&first_fuse, &third), 197700., -1., false);
}

#[test]
fn BOPFuseSimpleTest__ConeFacePrismZP4() {
    let cone = CreateCone(10., 0., 20.);
    let face_shape = Explorer::new(&cone, ShapeEnum::Face, ShapeEnum::Shape)
        .unwrap()
        .nth(1)
        .unwrap();
    let face = topods::face(&face_shape).unwrap();
    let prism = CreatePrism(&face, Vector3::new(0., 0., 10.));
    ValidateResult(&PerformFuse(&cone, &prism), 1353.72, -1., false);
}

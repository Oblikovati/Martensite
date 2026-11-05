#![allow(non_snake_case)]
use super::*;
fn fuse_cylinder_box(origin: [f64; 3], size: [f64; 3], angle: f64, area: f64) {
    let cylinder = CreateCylinder(1., 2.);
    let mut box_shape = CreateBox(
        Point3::new(origin[0], origin[1], origin[2]),
        size[0],
        size[1],
        size[2],
    );
    RotateAboutZ(&mut box_shape, Point3::new(0., 0., 0.), angle);
    ValidateResult(&PerformFuse(&cylinder, &box_shape), area, -1., false);
}
macro_rules! case {
    ($name:ident,$origin:expr,$size:expr,$angle:expr,$area:expr) => {
        #[test]
        fn $name() {
            fuse_cylinder_box($origin, $size, $angle, $area);
        }
    };
}
case!(
    BOPFuseSimpleTest__CylinderDiagonalHalfY1,
    [-2f64.sqrt() / 2., -2f64.sqrt() / 2., 0.],
    [1. + 2f64.sqrt() / 2., 2f64.sqrt(), 1.],
    0.,
    19.5364
);
case!(
    BOPFuseSimpleTest__CylinderDiagonalHalfY2,
    [-2f64.sqrt() / 2., -2f64.sqrt() / 2., 0.],
    [1. + 2f64.sqrt() / 2., 2f64.sqrt(), 1.],
    135.,
    19.5364
);
case!(
    BOPFuseSimpleTest__CylinderSixtyHalfY3,
    [-0.5, -3f64.sqrt() / 2., 0.],
    [1.5, 3f64.sqrt(), 1.],
    60.,
    19.9909
);
case!(
    BOPFuseSimpleTest__CylinderDiagonalHalfY4,
    [-2f64.sqrt() / 2., -2f64.sqrt() / 2., 0.],
    [2f64.sqrt(), 1. + 2f64.sqrt() / 2., 1.],
    0.,
    19.5364
);
case!(
    BOPFuseSimpleTest__CylinderThirtyHalfY5,
    [-3f64.sqrt() / 2., -1., 0.],
    [3f64.sqrt(), 1. + 3f64.sqrt() / 2., 1.],
    30.,
    20.6285
);
case!(
    BOPFuseSimpleTest__CylinderRotatedInsetHalfY6,
    [-0.8, -1., 0.],
    [1.6, 1.8, 1.],
    90.,
    20.0852
);
case!(
    BOPFuseSimpleTest__CylinderInsetHalfY7,
    [-0.8, -1., 0.],
    [1.6, 1.8, 1.],
    0.,
    20.0852
);
case!(
    BOPFuseSimpleTest__CylinderVerticalHalfY8,
    [-1., -1.1, 0.],
    [1., 2.2, 1.],
    0.,
    21.3664
);
case!(
    BOPFuseSimpleTest__CylinderVerticalSideHalfY9,
    [0., -1.1, 0.],
    [1., 2.2, 1.],
    0.,
    21.3664
);
case!(
    BOPFuseSimpleTest__CylinderHorizontalHalfZ1,
    [-1.1, -1., 0.],
    [2.2, 1., 1.],
    0.,
    21.3664
);
case!(
    BOPFuseSimpleTest__CylinderExternalHalfZ2,
    [-1., 1., 0.],
    [2., 2., 1.],
    0.,
    34.8496
);
case!(
    BOPFuseSimpleTest__CylinderExternalHalfZ3,
    [1., -1., 0.],
    [2., 2., 1.],
    0.,
    34.8496
);
case!(
    BOPFuseSimpleTest__CylinderSquareHalfZ4,
    [-2f64.sqrt() / 2., -2f64.sqrt() / 2., 0.],
    [2f64.sqrt(), 2f64.sqrt(), 1.],
    135.,
    18.8496
);
case!(
    BOPFuseSimpleTest__CylinderSquareHalfZ5,
    [-2f64.sqrt() / 2., -2f64.sqrt() / 2., 0.],
    [2f64.sqrt(), 2f64.sqrt(), 1.],
    0.,
    18.8496
);
case!(
    BOPFuseSimpleTest__CylinderDiagonalHalfZ6,
    [-2f64.sqrt() / 2., -2f64.sqrt() / 2., 0.],
    [1.6, 2f64.sqrt(), 1.],
    135.,
    19.1829
);
case!(
    BOPFuseSimpleTest__CylinderSixtyHalfZ7,
    [-0.5, -3f64.sqrt() / 2., 0.],
    [1.4, 3f64.sqrt(), 1.],
    60.,
    19.5922
);
case!(
    BOPFuseSimpleTest__CylinderRectHalfZ8,
    [-2f64.sqrt() / 2., -2f64.sqrt() / 2., 0.],
    [1.6, 2f64.sqrt(), 1.],
    0.,
    19.1829
);
#[test]
fn BOPFuseSimpleTest__BoxCylinder_Z9() {
    let first = CreateBox(Point3::new(0., 0., 0.), 4., 4., 4.);
    let cylinder = Translate(&CreateCylinder(2., 6.), 0., 0., -2.);
    ValidateResult(&PerformFuse(&first, &cylinder), 161.681, -1., false);
}
case!(
    BOPFuseSimpleTest__CylinderInsetZA1,
    [-2f64.sqrt() / 2., -2f64.sqrt() / 2., 0.],
    [1.6, 1.6, 2.],
    0.,
    20.0824
);
case!(
    BOPFuseSimpleTest__CylinderSixtyZA2,
    [-0.5, -3f64.sqrt() / 2., 0.],
    [1.45, 1.8, 2.],
    60.,
    20.6074
);
case!(
    BOPFuseSimpleTest__CylinderInsetZA3,
    [-2f64.sqrt() / 2., -2f64.sqrt() / 2., 0.],
    [1.6, 1.6, 2.],
    135.,
    20.0824
);
case!(
    BOPFuseSimpleTest__CylinderSquareZA4,
    [-3f64.sqrt() / 2., -3f64.sqrt() / 2., 0.],
    [3f64.sqrt(), 3f64.sqrt(), 2.],
    60.,
    20.9587
);
case!(
    BOPFuseSimpleTest__CylinderSmallInsetZA5,
    [-0.8, -0.8, 0.],
    [1.6, 1.6, 2.],
    0.,
    19.924
);
case!(
    BOPFuseSimpleTest__CylinderSideBoxZA6,
    [0., -0.5, 0.],
    [2., 1., 2.],
    0.,
    25.3778
);
case!(
    BOPFuseSimpleTest__CylinderSideBoxZA7,
    [0., -0.5, 0.],
    [2., 1., 2.],
    30.,
    25.3778
);
case!(
    BOPFuseSimpleTest__CylinderPositiveBoxZA8,
    [0., 0., 0.],
    [2., 2., 2.],
    0.,
    34.1372
);
case!(
    BOPFuseSimpleTest__CylinderNegativeBoxZA9,
    [-2., -2., 0.],
    [2., 2., 2.],
    0.,
    34.1372
);
case!(
    BOPFuseSimpleTest__CylinderCornerSquareZB1,
    [0., 0., 0.],
    [3f64.sqrt() / 2., 3f64.sqrt() / 2., 2.],
    0.,
    19.3768
);
case!(
    BOPFuseSimpleTest__CylinderCornerSquareZB2,
    [0., 0., 0.],
    [3f64.sqrt() / 2., 3f64.sqrt() / 2., 2.],
    -30.,
    19.3768
);
case!(
    BOPFuseSimpleTest__CylinderInsetHalfZB3,
    [-2f64.sqrt() / 2., -2f64.sqrt() / 2., 0.],
    [1.6, 1.6, 1.],
    0.,
    19.5852
);
case!(
    BOPFuseSimpleTest__CylinderSixtyHalfZB4,
    [-0.5, -3f64.sqrt() / 2., 0.],
    [1.45, 1.8, 1.],
    60.,
    19.9542
);
case!(
    BOPFuseSimpleTest__CylinderInsetHalfZB5,
    [-2f64.sqrt() / 2., -2f64.sqrt() / 2., 0.],
    [1.6, 1.6, 1.],
    135.,
    19.5852
);
case!(
    BOPFuseSimpleTest__CylinderSquareHalfZB6,
    [-3f64.sqrt() / 2., -3f64.sqrt() / 2., 0.],
    [3f64.sqrt(), 3f64.sqrt(), 1.],
    60.,
    20.1249
);
case!(
    BOPFuseSimpleTest__CylinderSmallInsetHalfZB7,
    [-0.8, -0.8, 0.],
    [1.6, 1.6, 1.],
    0.,
    19.4592
);
case!(
    BOPFuseSimpleTest__CylinderSideHalfZB8,
    [0., -0.5, 0.],
    [2., 1., 1.],
    0.,
    23.1571
);
case!(
    BOPFuseSimpleTest__CylinderSideHalfZB9,
    [0., -0.5, 0.],
    [2., 1., 1.],
    30.,
    23.1571
);

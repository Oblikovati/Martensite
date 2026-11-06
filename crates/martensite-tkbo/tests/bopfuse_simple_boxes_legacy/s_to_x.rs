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
    BOPFuseSimpleTest__CylinderHalfBoxS1,
    [-1., -1., 0.],
    [2., 1., 2.],
    0.,
    21.4248
);
case!(
    BOPFuseSimpleTest__CylinderDiagonalBoxS2,
    [-1., -1., 0.],
    [1. + 2f64.sqrt() / 2., 1. + 2f64.sqrt() / 2., 2.],
    45.,
    21.2532
);
case!(
    BOPFuseSimpleTest__CylinderDiagonalBoxS3,
    [-1., -1., 0.],
    [1. + 2f64.sqrt() / 2., 1. + 2f64.sqrt() / 2., 2.],
    -45.,
    21.2532
);
case!(
    BOPFuseSimpleTest__CylinderSquareBoxS4,
    [-1., -1., 0.],
    [1. + 2f64.sqrt() / 2., 1. + 2f64.sqrt() / 2., 2.],
    0.,
    21.2532
);
case!(
    BOPFuseSimpleTest__CylinderCenteredSquareS5,
    [-2f64.sqrt() / 2., -2f64.sqrt() / 2., 0.],
    [1. + 2f64.sqrt() / 2., 1. + 2f64.sqrt() / 2., 2.],
    0.,
    21.2532
);
case!(
    BOPFuseSimpleTest__CylinderInsetBoxS6,
    [-0.8, -0.8, 0.],
    [1.8, 1.8, 2.],
    0.,
    21.882
);
case!(
    BOPFuseSimpleTest__CylinderOffsetInsetBoxS7,
    [-1., -0.8, 0.],
    [1.8, 1.8, 2.],
    0.,
    21.882
);
case!(
    BOPFuseSimpleTest__CylinderAngledSquareS8,
    [-1., -3f64.sqrt() / 2., 0.],
    [1. + 3f64.sqrt() / 2., 1. + 3f64.sqrt() / 2., 2.],
    30.,
    22.4434
);
case!(
    BOPFuseSimpleTest__CylinderShortInsetBoxS9,
    [-1., -0.8, 0.],
    [2., 1.6, 2.],
    0.,
    21.802
);
case!(
    BOPFuseSimpleTest__CylinderInsetBoxT1,
    [-0.8, -1., 0.],
    [1.6, 2., 2.],
    0.,
    21.802
);
case!(
    BOPFuseSimpleTest__CylinderAngledBoxT2,
    [-3f64.sqrt() / 2., -1., 0.],
    [3f64.sqrt(), 2., 2.],
    30.,
    22.4075
);
case!(
    BOPFuseSimpleTest__CylinderOffsetBoxT3,
    [-0.5, -1., 0.],
    [1.5, 1.5, 2.],
    0.,
    20.6654
);
case!(
    BOPFuseSimpleTest__CylinderOffsetBoxT4,
    [-0.5, -1., 0.],
    [1.5, 1.5, 2.],
    30.,
    20.6654
);
case!(
    BOPFuseSimpleTest__CylinderCornerBoxT5,
    [-1., -1., 0.],
    [1.5, 1.5, 2.],
    0.,
    20.6654
);
case!(
    BOPFuseSimpleTest__CylinderDiagonalBoxT6,
    [-2f64.sqrt() / 2., -2f64.sqrt() / 2., 0.],
    [1. + 2f64.sqrt() / 2., 2f64.sqrt(), 2.],
    0.,
    19.9656
);
case!(
    BOPFuseSimpleTest__CylinderDiagonalBoxT7,
    [-2f64.sqrt() / 2., -2f64.sqrt() / 2., 0.],
    [1. + 2f64.sqrt() / 2., 2f64.sqrt(), 2.],
    135.,
    19.9656
);
case!(
    BOPFuseSimpleTest__CylinderSixtyBoxT8,
    [-0.5, -3f64.sqrt() / 2., 0.],
    [1.5, 3f64.sqrt(), 2.],
    60.,
    20.6285
);
case!(
    BOPFuseSimpleTest__CylinderDiagonalBoxT9,
    [-2f64.sqrt() / 2., -2f64.sqrt() / 2., 0.],
    [2f64.sqrt(), 1. + 2f64.sqrt() / 2., 2.],
    0.,
    19.9656
);
case!(
    BOPFuseSimpleTest__CylinderAngledOverlapU1,
    [-3f64.sqrt() / 2., -1., 0.],
    [3f64.sqrt(), 1. + 3f64.sqrt() / 2., 2.],
    30.,
    21.6831
);
case!(
    BOPFuseSimpleTest__CylinderRotatedInsetU2,
    [-0.8, -1., 0.],
    [1.6, 1.8, 2.],
    90.,
    20.863
);
case!(
    BOPFuseSimpleTest__CylinderInsetU3,
    [-0.8, -1., 0.],
    [1.6, 1.8, 2.],
    0.,
    20.863
);
case!(
    BOPFuseSimpleTest__CylinderVerticalSlabU4,
    [-1., -1.1, 0.],
    [1., 2.2, 2.],
    0.,
    22.6248
);
case!(
    BOPFuseSimpleTest__CylinderVerticalSideSlabU5,
    [0., -1.1, 0.],
    [1., 2.2, 2.],
    0.,
    22.6248
);
case!(
    BOPFuseSimpleTest__CylinderHorizontalSlabU6,
    [-1.1, -1., 0.],
    [2.2, 1., 2.],
    0.,
    22.6248
);
case!(
    BOPFuseSimpleTest__CylinderExternalBoxU7,
    [-1., 1., 0.],
    [2., 2., 2.],
    0.,
    42.8496
);
case!(
    BOPFuseSimpleTest__CylinderExternalBoxU8,
    [1., -1., 0.],
    [2., 2., 2.],
    0.,
    42.8496
);
case!(
    BOPFuseSimpleTest__CylinderDiagonalSquareU9,
    [-2f64.sqrt() / 2., -2f64.sqrt() / 2., 0.],
    [2f64.sqrt(), 2f64.sqrt(), 2.],
    135.,
    18.8496
);
case!(
    BOPFuseSimpleTest__CylinderSquareV1,
    [-2f64.sqrt() / 2., -2f64.sqrt() / 2., 0.],
    [2f64.sqrt(), 2f64.sqrt(), 2.],
    0.,
    18.8496
);
case!(
    BOPFuseSimpleTest__CylinderDiagonalRectV2,
    [-2f64.sqrt() / 2., -2f64.sqrt() / 2., 0.],
    [1.6, 2f64.sqrt(), 2.],
    135.,
    19.4315
);
case!(
    BOPFuseSimpleTest__CylinderSixtyRectV3,
    [-0.5, -3f64.sqrt() / 2., 0.],
    [1.4, 3f64.sqrt(), 2.],
    60.,
    20.0601
);
case!(
    BOPFuseSimpleTest__CylinderRectV4,
    [-2f64.sqrt() / 2., -2f64.sqrt() / 2., 0.],
    [1.6, 2f64.sqrt(), 2.],
    0.,
    19.4315
);
case!(
    BOPFuseSimpleTest__CylinderHalfHeightBoxV5,
    [-1., -1., 0.],
    [2., 2., 1.],
    0.,
    22.2832
);
case!(
    BOPFuseSimpleTest__CylinderHalfHeightBoxV6,
    [-1., -1., 0.],
    [2., 2., 1.],
    45.,
    22.2832
);
case!(
    BOPFuseSimpleTest__CylinderHalfHeightShortV7,
    [-1., -1., 0.],
    [2., 1.5, 1.],
    0.,
    20.8739
);
case!(
    BOPFuseSimpleTest__CylinderHalfHeightAngledV8,
    [-1., -1., 0.],
    [1. + 3f64.sqrt() / 2., 2., 1.],
    60.,
    21.7077
);
case!(
    BOPFuseSimpleTest__CylinderHalfHeightNarrowV9,
    [-1., -1., 0.],
    [1.5, 2., 1.],
    0.,
    20.8739
);
case!(
    BOPFuseSimpleTest__CylinderWideHalfBoxW1,
    [-1., -1., 0.],
    [2., 2.5, 1.],
    0.,
    25.2832
);
case!(
    BOPFuseSimpleTest__CylinderLongHalfBoxW2,
    [-1., -1., 0.],
    [2.5, 2., 1.],
    0.,
    25.2832
);
case!(
    BOPFuseSimpleTest__CylinderHalfBoxW3,
    [-1., -1., 0.],
    [1., 2., 1.],
    0.,
    20.5664
);
case!(
    BOPFuseSimpleTest__CylinderSideHalfBoxW4,
    [0., -1., 0.],
    [1., 2., 1.],
    0.,
    20.5664
);
case!(
    BOPFuseSimpleTest__CylinderNarrowHalfBoxW5,
    [-1., -1., 0.],
    [2., 1., 1.],
    0.,
    20.5664
);
case!(
    BOPFuseSimpleTest__CylinderDiagonalHalfBoxW6,
    [-1., -1., 0.],
    [1. + 2f64.sqrt() / 2., 1. + 2f64.sqrt() / 2., 1.],
    45.,
    20.3948
);
case!(
    BOPFuseSimpleTest__CylinderDiagonalHalfBoxW7,
    [-1., -1., 0.],
    [1. + 2f64.sqrt() / 2., 1. + 2f64.sqrt() / 2., 1.],
    -45.,
    20.3948
);
case!(
    BOPFuseSimpleTest__CylinderSquareHalfBoxW8,
    [-1., -1., 0.],
    [1. + 2f64.sqrt() / 2., 1. + 2f64.sqrt() / 2., 1.],
    0.,
    20.3948
);
case!(
    BOPFuseSimpleTest__CylinderCenteredHalfBoxW9,
    [-2f64.sqrt() / 2., -2f64.sqrt() / 2., 0.],
    [1. + 2f64.sqrt() / 2., 1. + 2f64.sqrt() / 2., 1.],
    0.,
    20.3948
);
case!(
    BOPFuseSimpleTest__CylinderInsetHalfBoxX1,
    [-0.8, -0.8, 0.],
    [1.8, 1.8, 1.],
    0.,
    20.7912
);
case!(
    BOPFuseSimpleTest__CylinderOffsetInsetHalfBoxX2,
    [-1., -0.8, 0.],
    [1.8, 1.8, 1.],
    0.,
    20.7912
);
case!(
    BOPFuseSimpleTest__CylinderAngledHalfBoxX3,
    [-1., -3f64.sqrt() / 2., 0.],
    [1. + 3f64.sqrt() / 2., 1. + 3f64.sqrt() / 2., 1.],
    30.,
    21.1681
);
case!(
    BOPFuseSimpleTest__CylinderShortInsetHalfBoxX4,
    [-1., -0.8, 0.],
    [2., 1.6, 1.],
    0.,
    20.7112
);
case!(
    BOPFuseSimpleTest__CylinderWideInsetHalfBoxX5,
    [-0.8, -1., 0.],
    [1.6, 2., 1.],
    0.,
    20.7112
);
case!(
    BOPFuseSimpleTest__CylinderSixtyHalfBoxX6,
    [-3f64.sqrt() / 2., -1., 0.],
    [3f64.sqrt(), 2., 1.],
    30.,
    21.1322
);
case!(
    BOPFuseSimpleTest__CylinderOffsetHalfBoxX7,
    [-0.5, -1., 0.],
    [1.5, 1.5, 1.],
    0.,
    20.0155
);
case!(
    BOPFuseSimpleTest__CylinderOffsetHalfBoxX8,
    [-0.5, -1., 0.],
    [1.5, 1.5, 1.],
    30.,
    20.0155
);
case!(
    BOPFuseSimpleTest__CylinderCornerHalfBoxX9,
    [-1., -1., 0.],
    [1.5, 1.5, 1.],
    0.,
    20.0155
);

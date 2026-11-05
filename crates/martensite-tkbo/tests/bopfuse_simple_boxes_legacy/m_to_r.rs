#![allow(non_snake_case)]
use super::*;
rotated_box_fuse_case!(
    BOPFuseSimpleTest__CornerCubeM1,
    [0., 0., 0.],
    [0.5, 0.5, 0.5],
    [0., 0., 0.],
    45.,
    6.39645
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__PivotedHalfAngledBoxM2,
    [0., 0.5, 0.],
    [1. / 5f64.sqrt(), 5f64.sqrt() / 2., 0.5],
    [0., 0.5, 0.],
    martensite_cmath::atan2(1f64, 2.) * 180. / std::f64::consts::PI - 90.,
    6.18541
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__OffsetHalfTallBoxM3,
    [-0.5, 0., 0.],
    [0.5, 1.7, 0.5],
    [0., 0., 0.],
    -30.,
    7.95662
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__DiagonalHalfNarrowBoxM4,
    [0., 0., 0.],
    [1.5, 0.25, 0.5],
    [0., 0., 0.],
    45.,
    6.40013
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__AngledHalfWideBoxM5,
    [0., 0., 0.],
    [31f64.sqrt() / 4., 0.5, 0.5],
    [0., 0., 0.],
    34.73,
    6.57112
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__ThirtyDegreeCornerCubeM6,
    [0., 0., 0.],
    [0.5, 0.5, 0.5],
    [0., 0., 0.],
    30.,
    6.25
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__SixtyDegreeCornerCubeM7,
    [0., 0., 0.],
    [0.5, 0.5, 0.5],
    [0., 0., 0.],
    60.,
    6.67265
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__PivotedCornerHalfBoxM8,
    [1., 1., 0.],
    [1., 1., 0.5],
    [1., 1., 1.],
    30.,
    10.
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__OffsetThinHalfBoxM9,
    [-0.25, 0., 0.],
    [0.25, 1.7, 0.5],
    [0., 0., 0.],
    -30.,
    7.10662
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__OffsetHalfTallBoxN1,
    [-0.5, 0., 0.],
    [0.5, 1.7, 0.5],
    [0., 0., 0.],
    -50.,
    7.26013
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__FortyDegreeHalfBoxN2,
    [0., 0., 0.],
    [30f64.sqrt() / 4., 0.25, 0.5],
    [0., 0., 0.],
    40.,
    6.18454
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__InvertedInsetSquareN3,
    [0.25, 0.25, 0.],
    [0.5, 0.5, -1.],
    [0.25, 0.25, 0.],
    30.,
    8.
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__InvertedInsetSquareN4,
    [0.25, 0.25, 0.],
    [0.5, 0.5, -1.],
    [0.25, 0.25, 0.],
    120.,
    8.35566
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__InvertedSmallSquareN5,
    [0., 0.25, 0.],
    [0.5, 0.5, -1.],
    [0., 0.25, 0.],
    35.,
    8.17505
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__InvertedSmallSquareN6,
    [0., 0.25, 0.],
    [0.5, 0.5, -1.],
    [0., 0.25, 0.],
    115.,
    8.5
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__InvertedSmallSquareN7,
    [0., 0.25, 0.],
    [0.5, 0.5, -1.],
    [0., 0.25, 0.],
    230.,
    8.42552
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__InvertedSmallSquareN8,
    [0., 0.25, 0.],
    [0.5, 0.5, -1.],
    [0., 0.25, 0.],
    250.,
    8.3477
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__InvertedLargeSquareN9,
    [0., 0.5, 0.],
    [1.5, 1.5, -1.],
    [0., 0.5, 0.],
    -45.,
    15.
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__InvertedMediumSquareO1,
    [0., 0.5, 0.],
    [0.8, 0.8, -1.],
    [0., 0.5, 0.],
    -45.,
    9.25177
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__InvertedMediumSquareO2,
    [0., 0.25, 0.],
    [0.8, 0.8, -1.],
    [0., 0.25, 0.],
    -45.,
    9.43383
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__InvertedSmallSquareO3,
    [0., 0.25, 0.],
    [0.5, 0.5, -1.],
    [0., 0.25, 0.],
    -45.,
    8.02145
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__InvertedDiamondO4,
    [0., 0.625, 0.],
    [2f64.sqrt() * 0.75, 2f64.sqrt() * 0.75, -1.],
    [0., 0.625, 0.],
    -45.,
    11.0395
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__InvertedLongBoxO5,
    [-0.25, 0., 0.],
    [0.5, 1.7, -1.],
    [0., 0., 0.],
    -45.,
    10.9358
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__InvertedLongBoxO6,
    [-0.25, 0., 0.],
    [0.5, 1.7, -1.],
    [0., 0., 0.],
    -30.,
    11.0896
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__InvertedTallBoxO7,
    [-0.25, -0.25, 0.],
    [0.25, 2., -1.],
    [0., 0., 0.],
    -30.,
    11.067
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__InvertedShortBoxO8,
    [-0.25, 0., 0.],
    [0.5, 1.3, -1.],
    [0., 0., 0.],
    -45.,
    9.76188
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__InvertedSlenderBoxO9,
    [0., -0.25, 0.],
    [0.5, 1.55, -1.],
    [0., 0., 0.],
    -35.,
    10.5853
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__RaisedSquareP1,
    [0., 0.25, 0.5],
    [0.5, 0.5, 1.],
    [0., 0.25, 0.],
    230.,
    7.95708
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__RaisedInsetSquareP2,
    [0.1, 0.4, 0.5],
    [0.5, 0.5, 1.],
    [0.1, 0.4, 0.],
    245.,
    7.39351
);
box_fuse_case!(
    BOPFuseSimpleTest__RaisedCornerSquareP3,
    [-0.25, -0.25, 0.5],
    [0.5, 0.5, 1.],
    7.875
);
box_fuse_case!(
    BOPFuseSimpleTest__RaisedContainedSquareP4,
    [0.25, 0.5, 0.5],
    [0.5, 0.5, 1.],
    7.
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__RaisedContainedSquareP5,
    [0.25, 0.5, 0.5],
    [0.5, 0.5, 1.],
    [0.25, 0.25, 0.],
    45.,
    7.37868
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__RaisedContainedSquareP6,
    [0.25, 0.5, 0.5],
    [0.5, 0.5, 1.],
    [0.25, 0.25, 0.],
    90.,
    8.
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__RaisedInsetSquareP7,
    [0.25, 0.25, 0.5],
    [0.5, 0.5, 1.],
    [0.25, 0.25, 0.],
    125.,
    7.73041
);
box_fuse_case!(
    BOPFuseSimpleTest__RaisedLongBoxP8,
    [0.25, -0.25, 0.5],
    [0.5, 1.5, 1.],
    9.
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__RaisedOffsetLongBoxP9,
    [0.9, -0.25, 0.5],
    [0.5, 1.7, 1.],
    [0.9, -0.25, 0.],
    45.,
    9.31449
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__RaisedOffsetSlabQ1,
    [1.25, 0., 0.5],
    [0.25, 1.7, 1.],
    [1.1, 0., 0.],
    45.,
    9.31226
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__RaisedLongBoxQ2,
    [-0.25, -0.25, 0.5],
    [0.5, 1.7, 1.],
    [0., 0., 0.],
    -30.,
    9.5406
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__RaisedShortBoxQ3,
    [-0.25, -0.25, 0.5],
    [0.5, 1.55, 1.],
    [0., 0., 0.],
    -45.,
    8.93786
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__RaisedWideBoxQ4,
    [0.5, -0.25, 0.5],
    [1., 1., 1.],
    [0.5, -0.25, 0.],
    45.,
    8.69365
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__RaisedPivotedBoxQ5,
    [0., 0.15, 0.5],
    [0.8, 0.9, 1.],
    [0., 0.25, 0.],
    -45.,
    8.28688
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__RaisedNarrowBoxQ6,
    [0.5, -0.25, 0.5],
    [0.5, 1., 1.],
    [0.5, -0.25, 0.],
    45.,
    7.90013
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__PartialHeightBoxQ7,
    [-0.25, -0.25, 0.25],
    [0.25, 1., 0.5],
    [0., 0., 0.],
    -30.,
    6.57476
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__RaisedSlenderBoxQ8,
    [0., -0.25, 0.25],
    [0.5, 1.7, 0.5],
    [0., 0., 0.],
    -35.,
    7.1901
);
#[test]
fn BOPFuseSimpleTest__RotatedTranslatedBoxes_Q9() {
    let first = CreateBox(Point3::new(0., 0., 0.), 3., 3., 3.);
    let mut second = CreateBox(Point3::new(0., 0., 0.), 1., 1., 1.);
    RotateAroundAxis(&mut second, Point3::new(0., 0., 0.), [1., 0., 0.], -30.);
    let second = Translate(&second, 1., 1., 3.);
    ValidateResult(&PerformFuse(&first, &second), 56.6906, -1., false);
}
fn fused_cylinder_box(origin: [f64; 3], size: [f64; 3], angle: f64, area: f64) {
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
macro_rules! cylinder_box_fuse_case {
    ($name:ident,$origin:expr,$size:expr,$angle:expr,$area:expr) => {
        #[test]
        fn $name() {
            fused_cylinder_box($origin, $size, $angle, $area);
        }
    };
}
cylinder_box_fuse_case!(
    BOPFuseSimpleTest__CylinderBoxR1,
    [-1., -1., 0.],
    [2., 2., 2.],
    0.,
    24.
);
cylinder_box_fuse_case!(
    BOPFuseSimpleTest__CylinderBoxR2,
    [-1., -1., 0.],
    [2., 2., 2.],
    45.,
    24.
);
cylinder_box_fuse_case!(
    BOPFuseSimpleTest__CylinderShortBoxR3,
    [-1., -1., 0.],
    [2., 1.5, 2.],
    0.,
    21.9531
);
cylinder_box_fuse_case!(
    BOPFuseSimpleTest__CylinderAngledBoxR4,
    [-1., -1., 0.],
    [1. + 3f64.sqrt() / 2., 2., 2.],
    60.,
    23.2038
);
cylinder_box_fuse_case!(
    BOPFuseSimpleTest__CylinderNarrowBoxR5,
    [-1., -1., 0.],
    [1.5, 2., 2.],
    0.,
    21.9531
);
cylinder_box_fuse_case!(
    BOPFuseSimpleTest__CylinderWideBoxR6,
    [-1., -1., 0.],
    [2., 2.5, 2.],
    0.,
    28.
);
cylinder_box_fuse_case!(
    BOPFuseSimpleTest__CylinderLongBoxR7,
    [-1., -1., 0.],
    [2.5, 2., 2.],
    0.,
    28.
);
cylinder_box_fuse_case!(
    BOPFuseSimpleTest__CylinderHalfBoxR8,
    [-1., -1., 0.],
    [1., 2., 2.],
    0.,
    21.4248
);
cylinder_box_fuse_case!(
    BOPFuseSimpleTest__CylinderSideBoxR9,
    [0., -1., 0.],
    [1., 2., 2.],
    0.,
    21.4248
);

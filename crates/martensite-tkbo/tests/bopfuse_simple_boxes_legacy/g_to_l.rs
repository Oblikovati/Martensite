#![allow(non_snake_case)]
use super::*;
rotated_box_fuse_case!(
    BOPFuseSimpleTest__PivotedWideAngledBox_G1,
    [0., 0.5, 0.],
    [2. / 5f64.sqrt(), 1.25, 1.],
    [0., 0.5, 0.],
    martensite_cmath::atan2(1f64, 2.) * 180. / std::f64::consts::PI - 90.,
    7.72287
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__OffsetSmallAngledBox_G2,
    [0.25, 0., 0.],
    [1.5 * 5f64.sqrt() / 4., 5f64.sqrt() / 4., 1.],
    [0.25, 0., 0.],
    martensite_cmath::atan2(1f64, 2.) * 180. / std::f64::consts::PI,
    6.
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__PivotedDiagonalBox_G3,
    [1. - 2f64.sqrt() / 2., 0., 0.],
    [2f64.sqrt(), 1.5 * 2f64.sqrt() / 2., 1.],
    [1., 0., 0.],
    45.,
    8.49264
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__OffsetNarrowDiagonalBox_G4,
    [-0.25, -0.25, 0.],
    [2., 2f64.sqrt() / 4., 1.],
    [-0.25, -0.25, 0.],
    45.,
    8.25
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__PivotedMediumAngledBox_G5,
    [0., 0.25, 0.],
    [1.3, 3. / 5f64.sqrt() / 2., 1.],
    [0., 0.25, 0.],
    martensite_cmath::atan2(1f64, 2.) * 180. / std::f64::consts::PI,
    7.56774
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__PivotedOffsetAngledBox_G6,
    [0., 0.45, 0.],
    [1. / 5f64.sqrt(), 5f64.sqrt() / 2., 1.],
    [0., 0.45, 0.],
    martensite_cmath::atan2(1f64, 2.) * 180. / std::f64::consts::PI - 90.,
    6.27082
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__PivotedSmallSquare_G7,
    [0., 0.25, 0.],
    [0.5, 0.5, 1.],
    [0., 0.25, 0.],
    45.,
    6.54289
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__PivotedSmallSquareNegative_G8,
    [0., 0.25, 0.],
    [0.5, 0.5, 1.],
    [0., 0.25, 0.],
    -30.,
    6.
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__PivotedSmallSquareReflex_G9,
    [0., 0.25, 0.],
    [0.5, 0.5, 1.],
    [0., 0.25, 0.],
    240.,
    7.20873
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__PivotedLargeDiamond_H1,
    [0., 0.75, 0.],
    [2f64.sqrt() * 0.875, 2f64.sqrt() * 0.875, 1.],
    [0., 0.75, 0.],
    -45.,
    9.22303
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__PivotedHalfSquare_H2,
    [0., 0.25, 0.],
    [5f64.sqrt() * 0.5, 5f64.sqrt() * 0.5, 1.],
    [0., 0.25, 0.],
    martensite_cmath::atan2(1f64, 2.) * 180. / std::f64::consts::PI,
    8.98084
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__PivotedDiagonalSideBox_H3,
    [0.5, 0., 0.],
    [1., 2f64.sqrt() * 0.5, 1.],
    [0.5, 0., 0.],
    45.,
    6.51472
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__PivotedDiagonalLongSideBox_H4,
    [0.5, 0., 0.],
    [1.5, 2f64.sqrt() * 0.5, 1.],
    [0.5, 0., 0.],
    45.,
    8.16421
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__OffsetPivotAngledBox_H5,
    [-0.1, 0., 0.],
    [1.5, 1. / 5f64.sqrt(), 1.],
    [1., 0., 0.],
    martensite_cmath::atan2(1f64, 2.) * 180. / std::f64::consts::PI,
    8.11803
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__OffsetDiagonalSlab_H6,
    [0., -0.25, 0.],
    [2f64.sqrt(), 0.5, 1.],
    [0., 0., 0.],
    45.,
    6.83579
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__PivotedThinSquare_H7,
    [0., 0.25, 0.],
    [0.1, 0.5, 1.],
    [0., 0.25, 0.],
    240.,
    6.13856
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__PivotedHalfAngledBox_H8,
    [0., 0.25, 0.],
    [5f64.sqrt() * 0.5, 0.5, 1.],
    [0., 0.25, 0.],
    martensite_cmath::atan2(1f64, 2.) * 180. / std::f64::consts::PI,
    6.58166
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__PivotedInsetSquare_H9,
    [0.25, 0.25, 0.],
    [0.5, 0.5, 1.],
    [0.25, 0.25, 0.],
    30.,
    6.
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__PivotedInsetSquareI1,
    [0.25, 0.25, 0.],
    [0.5, 0.5, 1.],
    [0.25, 0.25, 0.],
    120.,
    6.98964
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__PivotedSmallSquareI2,
    [0., 0.25, 0.],
    [0.5, 0.5, 1.],
    [0., 0.25, 0.],
    35.,
    6.41477
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__PivotedSmallSquareI3,
    [0., 0.25, 0.],
    [0.5, 0.5, 1.],
    [0., 0.25, 0.],
    115.,
    8.5
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__PivotedSmallSquareI4,
    [0., 0.25, 0.],
    [0.5, 0.5, 1.],
    [0., 0.25, 0.],
    230.,
    7.48865
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__PivotedSmallSquareI5,
    [0., 0.25, 0.],
    [0.5, 0.5, 1.],
    [0., 0.25, 0.],
    250.,
    7.07254
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__PivotedLargeSquareI6,
    [0., 0.5, 0.],
    [1.5, 1.5, 1.],
    [0., 0.5, 0.],
    -45.,
    11.5858
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__PivotedMediumSquareI7,
    [0., 0.5, 0.],
    [0.8, 0.8, 1.],
    [0., 0.5, 0.],
    -45.,
    6.26944
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__PivotedMediumSquareI8,
    [0., 0.25, 0.],
    [0.8, 0.8, 1.],
    [0., 0.25, 0.],
    -45.,
    6.60418
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__PivotedHalfSquareI9,
    [0., 0.25, 0.],
    [0.5, 0.5, 1.],
    [0., 0.25, 0.],
    -45.,
    6.10723
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__PivotedDiamondJ1,
    [0., 0.625, 0.],
    [2f64.sqrt() * 0.75, 2f64.sqrt() * 0.75, 1.],
    [0., 0.625, 0.],
    -45.,
    7.69853
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__OffsetLongBoxJ2,
    [-0.25, 0., 0.],
    [0.5, 1.7, 1.],
    [0., 0., 0.],
    -45.,
    7.69315
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__OffsetLongBoxJ3,
    [-0.25, 0., 0.],
    [0.5, 1.7, 1.],
    [0., 0., 0.],
    -30.,
    7.99156
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__OffsetTallBoxJ4,
    [-0.25, -0.25, 0.],
    [0.25, 2., 1.],
    [0., 0., 0.],
    -30.,
    8.54626
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__OffsetShortBoxJ5,
    [-0.25, 0., 0.],
    [0.5, 1.3, 1.],
    [0., 0., 0.],
    -45.,
    6.61385
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__OffsetSlenderBoxJ6,
    [0., -0.25, 0.],
    [0.5, 1.55, 1.],
    [0., 0., 0.],
    -35.,
    7.40901
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__PivotedOffsetSquareJ7,
    [0.1, 0.25, 0.],
    [0.5, 0.5, 1.],
    [0., 0.25, 0.],
    230.,
    8.03532
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__PivotedInsetSquareJ8,
    [0.1, 0.4, 0.],
    [0.5, 0.5, 1.],
    [0.1, 0.4, 0.],
    245.,
    6.59575
);
box_fuse_case!(
    BOPFuseSimpleTest__CornerBoxJ9,
    [-0.25, -0.25, 0.],
    [0.5, 0.5, 1.],
    7.375
);
box_fuse_case!(
    BOPFuseSimpleTest__ContainedSquareK1,
    [0.25, 0.25, 0.],
    [0.5, 0.5, 1.],
    6.
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__PivotedContainedSquareK2,
    [0.25, 0.25, 0.],
    [0.5, 0.5, 1.],
    [0.25, 0.25, 0.],
    45.,
    6.10723
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__PivotedContainedSquareK3,
    [0.25, 0.25, 0.],
    [0.5, 0.5, 1.],
    [0.25, 0.25, 0.],
    90.,
    6.75
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__PivotedContainedSquareK4,
    [0.25, 0.25, 0.],
    [0.5, 0.5, 1.],
    [0.25, 0.25, 0.],
    125.,
    7.09383
);
box_fuse_case!(
    BOPFuseSimpleTest__LongOffsetBoxK5,
    [0.25, -0.25, 0.],
    [0.5, 1.5, 1.],
    7.5
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__PivotedLongOffsetBoxK6,
    [0.9, -0.25, 0.],
    [0.5, 1.7, 1.],
    [0.9, -0.25, 0.],
    45.,
    7.69317
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__OffsetPivotedSlabK7,
    [1.25, 0., 0.],
    [0.25, 1.7, 1.],
    [1.1, 0., 0.],
    45.,
    8.23591
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__OffsetLongBoxK8,
    [-0.25, -0.25, 0.],
    [0.5, 1.7, 1.],
    [0., 0., 0.],
    -30.,
    7.99156
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__OffsetShortBoxK9,
    [-0.25, -0.25, 0.],
    [0.5, 1.55, 1.],
    [0., 0., 0.],
    -45.,
    7.36385
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__PivotedWideBoxL1,
    [0.5, -0.25, 0.],
    [1., 1., 1.],
    [0.5, -0.25, 0.],
    45.,
    7.0368
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__OffsetPivotedBoxL2,
    [0., 0.15, 0.],
    [0.8, 0.9, 1.],
    [0., 0.25, 0.],
    -45.,
    6.83063
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__PivotedNarrowBoxL3,
    [0.5, -0.25, 0.],
    [0.5, 1., 1.],
    [0.5, -0.25, 0.],
    45.,
    6.58947
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__DiagonalHalfHeightL4,
    [0., 0., 0.],
    [2f64.sqrt(), 2f64.sqrt() / 2., 0.5],
    [0., 0., 0.],
    45.,
    7.41421
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__DiagonalSquareHalfHeightL5,
    [0., 0., 0.],
    [2f64.sqrt() / 2., 2f64.sqrt() / 2., 0.5],
    [0., 0., 0.],
    45.,
    6.70711
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__DiagonalNarrowHalfHeightL6,
    [0., 0., 0.],
    [2f64.sqrt(), 0.25, 0.5],
    [0., 0., 0.],
    45.,
    6.27145
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__AngledNarrowHalfHeightL7,
    [0., 0., 0.],
    [31f64.sqrt() / 4., 0.25, 0.5],
    [0., 0., 0.],
    34.73,
    6.20856
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__DiagonalExtendedHalfHeightL8,
    [0., 0., 0.],
    [1.5, 2f64.sqrt() / 2., 0.5],
    [0., 0., 0.],
    45.,
    7.62132
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__PivotedHalfAngledBoxL9,
    [0., 0.5, 0.],
    [5f64.sqrt() / 2., 1. / 5f64.sqrt(), 0.5],
    [0., 0.5, 0.],
    martensite_cmath::atan2(1f64, 2.) * 180. / std::f64::consts::PI,
    6.75623
);

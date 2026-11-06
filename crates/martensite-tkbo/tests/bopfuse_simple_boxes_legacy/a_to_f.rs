#![allow(non_snake_case)]
use super::*;
box_fuse_case!(
    BOPFuseSimpleTest__CoincidentBoxes_A1,
    [0., 0., 0.],
    [1., 1., 1.],
    6.
);
box_fuse_case!(
    BOPFuseSimpleTest__ExtendedBox_A2,
    [0., 0., 0.],
    [1., 1.5, 1.],
    8.
);
box_fuse_case!(
    BOPFuseSimpleTest__TouchingSideBox_A3,
    [0., 1., 0.],
    [1., 0.5, 1.],
    8.
);
box_fuse_case!(
    BOPFuseSimpleTest__TouchingEdgeBox_A4,
    [1., 1., 0.],
    [1., 1., 1.],
    12.
);
box_fuse_case!(
    BOPFuseSimpleTest__ContainedHalfBox_A5,
    [0., 0., 0.],
    [0.5, 1., 0.5],
    6.
);
box_fuse_case!(
    BOPFuseSimpleTest__OffsetHalfBox_A6,
    [0., -0.5, 0.],
    [0.5, 0.5, 1.],
    7.5
);
box_fuse_case!(
    BOPFuseSimpleTest__OverlappingHalfBox_A7,
    [0., -0.5, 0.],
    [0.5, 1.5, 1.],
    7.5
);
box_fuse_case!(
    BOPFuseSimpleTest__ShiftedBox_A8,
    [0., 0.5, 0.],
    [1., 1., 1.],
    8.
);
box_fuse_case!(
    BOPFuseSimpleTest__CenteredThinBox_A9,
    [0., 0.25, 0.],
    [1., 0.5, 1.],
    6.
);
box_fuse_case!(
    BOPFuseSimpleTest__ContainedCornerBox_B1,
    [0., 0., 0.],
    [0.5, 0.5, 0.5],
    6.
);
box_fuse_case!(
    BOPFuseSimpleTest__SideOverlapBox_B2,
    [0., -0.5, 0.],
    [0.5, 0.5, 0.5],
    7.
);
box_fuse_case!(
    BOPFuseSimpleTest__CornerOverlapBox_B3,
    [0., -0.5, -0.5],
    [0.5, 0.5, 0.5],
    7.5
);
box_fuse_case!(
    BOPFuseSimpleTest__CenteredCornerOverlapBox_B4,
    [-0.5, -0.5, -0.5],
    [0.5, 0.5, 0.5],
    7.5
);
box_fuse_case!(
    BOPFuseSimpleTest__ExtendedContainedBox_B5,
    [0., 0., 0.],
    [1.5, 0.5, 0.5],
    7.
);
box_fuse_case!(
    BOPFuseSimpleTest__OffsetExtendedBox_B6,
    [0., -0.5, 0.],
    [1.5, 0.5, 0.5],
    8.5
);
box_fuse_case!(
    BOPFuseSimpleTest__CenteredContainedSlab_B7,
    [0.25, 0., 0.],
    [0.5, 0.5, 1.],
    6.
);
box_fuse_case!(
    BOPFuseSimpleTest__OffsetContainedSlab_B8,
    [0.25, -0.5, 0.],
    [0.5, 0.5, 1.],
    7.5
);
box_fuse_case!(
    BOPFuseSimpleTest__ExtendedContainedSlab_B9,
    [0.25, 0., 0.],
    [0.5, 1.5, 1.],
    7.5
);
box_fuse_case!(
    BOPFuseSimpleTest__PartialSideExtension_C1,
    [0.5, 0., 0.],
    [1., 1., 0.5],
    7.5
);
box_fuse_case!(
    BOPFuseSimpleTest__OffsetPartialSideExtension_C2,
    [0.5, 0., -0.5],
    [1., 1., 0.5],
    9.
);

rotated_box_fuse_case!(
    BOPFuseSimpleTest__DiagonalHalfHeight_C3,
    [0., 0., 0.],
    [2f64.sqrt(), 2f64.sqrt() / 2., 1.],
    [0., 0., 0.],
    45.,
    7.82843
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__DiagonalSquare_C4,
    [0., 0., 0.],
    [2f64.sqrt() / 2., 2f64.sqrt() / 2., 1.],
    [0., 0., 0.],
    45.,
    6.91421
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__DiagonalNarrow_C5,
    [0., 0., 0.],
    [2f64.sqrt(), 0.25, 1.],
    [0., 0., 0.],
    45.,
    6.41789
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__AngledNarrow_C6,
    [0., 0., 0.],
    [31f64.sqrt() / 4., 0.25, 1.],
    [0., 0., 0.],
    34.73,
    6.32953
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__DiagonalExtended_C7,
    [0., 0., 0.],
    [1.5, 2f64.sqrt() / 2., 1.],
    [0., 0., 0.],
    45.,
    8.12132
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__PivotedThirtyDegrees_C8,
    [0., 0.5, 0.],
    [5f64.sqrt() / 2., 1. / 5f64.sqrt(), 1.],
    [0., 0.5, 0.],
    martensite_cmath::atan2(1f64, 2.) * 180. / std::f64::consts::PI,
    7.01246
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__DiagonalSmallSquare_C9,
    [0., 0., 0.],
    [0.5, 0.5, 1.],
    [0., 0., 0.],
    45.,
    6.54289
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__PivotedMinusSixtyDegrees_D1,
    [0., 0.5, 0.],
    [1. / 5f64.sqrt(), 5f64.sqrt() / 2., 1.],
    [0., 0.5, 0.],
    martensite_cmath::atan2(1f64, 2.) * 180. / std::f64::consts::PI - 90.,
    6.27082
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__OffsetLongBox_D2,
    [-0.5, 0., 0.],
    [0.5, 1.7, 1.],
    [0., 0., 0.],
    -30.,
    8.7906
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__DiagonalLongNarrow_D3,
    [0., 0., 0.],
    [1.5, 0.25, 1.],
    [0., 0., 0.],
    45.,
    6.63236
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__AngledWideNarrow_D4,
    [0., 0., 0.],
    [31f64.sqrt() / 4., 0.5, 1.],
    [0., 0., 0.],
    34.73,
    6.83585
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__ThirtyDegreeSmallSquare_D5,
    [0., 0., 0.],
    [0.5, 0.5, 1.],
    [0., 0., 0.],
    30.,
    6.35566
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__SixtyDegreeSmallSquare_D6,
    [0., 0., 0.],
    [0.5, 0.5, 1.],
    [0., 0., 0.],
    60.,
    6.98964
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__PivotedCornerBox_D7,
    [1., 1., 0.],
    [1., 1., 1.],
    [1., 1., 1.],
    30.,
    12.
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__NarrowOffsetLongBox_D8,
    [-0.25, 0., 0.],
    [0.25, 1.7, 1.],
    [0., 0., 0.],
    -30.,
    7.79626
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__FiftyDegreeOffsetLongBox_D9,
    [-0.5, 0., 0.],
    [0.5, 1.7, 1.],
    [0., 0., 0.],
    -50.,
    7.83743
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__FortyDegreeNarrowBox_E1,
    [0., 0., 0.],
    [30f64.sqrt() / 4., 0.25, 1.],
    [0., 0., 0.],
    40.,
    6.30133
);
box_fuse_case!(
    BOPFuseSimpleTest__ContainedOffsetBox_E2,
    [0.25, 0., 0.],
    [0.5, 0.5, 0.5],
    6.
);
box_fuse_case!(
    BOPFuseSimpleTest__OffsetSideBox_E3,
    [0.25, -0.5, 0.],
    [0.5, 0.5, 0.5],
    7.
);
box_fuse_case!(
    BOPFuseSimpleTest__OffsetCornerBox_E4,
    [0.25, -0.5, -0.5],
    [0.5, 0.5, 0.5],
    7.5
);
box_fuse_case!(
    BOPFuseSimpleTest__ExtendedOffsetBox_E5,
    [0.25, 0., 0.],
    [0.5, 1.5, 0.5],
    7.
);
box_fuse_case!(
    BOPFuseSimpleTest__LongOffsetBox_E6,
    [0.25, 0., 0.],
    [1.5, 0.5, 0.5],
    7.5
);
box_fuse_case!(
    BOPFuseSimpleTest__OffsetLongSideBox_E7,
    [0.25, -0.5, 0.],
    [1.5, 0.5, 0.5],
    8.75
);
box_fuse_case!(
    BOPFuseSimpleTest__OffsetLongCornerBox_E8,
    [0.25, -0.5, -0.5],
    [1.5, 0.5, 0.5],
    9.5
);
box_fuse_case!(
    BOPFuseSimpleTest__LargeOffsetBox_E9,
    [0.25, 0., 0.],
    [1.5, 1.5, 0.5],
    10.25
);
box_fuse_case!(
    BOPFuseSimpleTest__LeftLongBox_F1,
    [-0.25, 0., 0.],
    [1.5, 0.5, 0.5],
    7.
);
box_fuse_case!(
    BOPFuseSimpleTest__LeftLongSideBox_F2,
    [-0.25, -0.5, 0.],
    [1.5, 0.5, 0.5],
    8.5
);
box_fuse_case!(
    BOPFuseSimpleTest__LeftLongCornerBox_F3,
    [-0.25, -0.5, -0.5],
    [1.5, 0.5, 0.5],
    9.5
);
box_fuse_case!(
    BOPFuseSimpleTest__InsetLongBox_F4,
    [0.25, 0.25, 0.],
    [1.5, 0.5, 0.5],
    7.5
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__PivotedSmallDiamond_F5,
    [0., 0.5, 0.],
    [2f64.sqrt() / 2., 2f64.sqrt() / 2., 1.],
    [0., 0.5, 0.],
    -45.,
    6.
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__PivotedLargeDiamond_F6,
    [0., 0.5, 0.],
    [2f64.sqrt() * 0.75, 2f64.sqrt() * 0.75, 1.],
    [0., 0.5, 0.],
    -45.,
    7.57843
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__PivotedLongAngledBox_F7,
    [0., 0.5, 0.],
    [1.25, 1. / 5f64.sqrt(), 1.],
    [0., 0.5, 0.],
    martensite_cmath::atan2(1f64, 2.) * 180. / std::f64::consts::PI,
    7.39443
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__OffsetDiagonalLongBox_F8,
    [-0.25, -0.25, 0.],
    [2., 2f64.sqrt() / 2., 1.],
    [-0.25, -0.25, 0.],
    45.,
    9.82843
);
rotated_box_fuse_case!(
    BOPFuseSimpleTest__PivotedHalfDiamond_F9,
    [0., 0.5, 0.],
    [2f64.sqrt() / 2., 2f64.sqrt() / 4., 1.],
    [0., 0.5, 0.],
    45.,
    6.66421
);

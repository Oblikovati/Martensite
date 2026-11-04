//! Closed forms use integer pixel-cell sets and rational texel centers, without a GPU.
use super::*;
use crate::{PointStyle, RenderVertex};
fn sprite() -> PrimitiveDraw {
    let mut style = PointStyle::new(11., [11, 11], vec![[1.; 4]; 121]).unwrap();
    style.color_policy = PointColorPolicy::AlphaSprite;
    PrimitiveDraw::points(
        vec![RenderVertex::new([0.; 3], [1., 1., 0., 1.])],
        vec![0],
        crate::identity_clip(),
        style,
    )
    .unwrap()
}
fn rectangle(center: [f32; 2], size: f32) -> ([i32; 2], i32) {
    let width = crate::fixed_grid_point::fixed_grid_point_extent(size).unwrap() as i32;
    let snapped = center.map(|v| (v * 4096.).round_ties_even() as i64);
    let half = i64::from(width / 2);
    let even = if width % 2 == 0 { 2048 } else { 0 };
    (
        [
            (snapped[0] + even).div_euclid(4096) as i32 - half as i32,
            (snapped[1] + even).div_euclid(4096) as i32 - half as i32,
        ],
        width,
    )
}
#[test]
fn odd_even_extents_follow_independent_cell_tables_at_signed_ties() {
    for (center, size, expected) in [
        ([10., 20.], 11., ([5, 15], 11)),
        ([10., 20.], 4., ([8, 18], 4)),
        ([10.5, 20.5], 4., ([9, 19], 4)),
        ([-10., -20.], 11., ([-15, -25], 11)),
        ([-10.5, -20.5], 4., ([-12, -22], 4)),
        ([10., 20.], 3.5, ([9, 19], 3)),
    ] {
        assert_eq!(rectangle(center, size), expected);
    }
}
#[test]
fn marker_center_integer_boundary_has_native_seventeen_cell_cross() {
    let (minimum, width) = rectangle([200., 209.], 11.);
    let mut actual = std::collections::BTreeSet::new();
    for y in 1..10 {
        actual.insert((minimum[0] + 5, 409 - (minimum[1] + y)));
    }
    for x in 1..10 {
        actual.insert((minimum[0] + x, 409 - (minimum[1] + 5)));
    }
    let mut expected = std::collections::BTreeSet::new();
    for y in 196..205 {
        expected.insert((200, y));
    }
    for x in 196..205 {
        expected.insert((x, 200));
    }
    assert_eq!(width, 11);
    assert_eq!(actual, expected);
}
#[test]
fn explicit_uv_texel_centers_preserve_asymmetric_rows_across_viewport_signs() {
    for height in [409., -409.] {
        for cell in 0..11 {
            let lower = (2 * cell + 1) * 3 / (2 * 11);
            let physical = if height < 0. { 10 - cell } else { cell };
            let logical = if height < 0. { 10 - physical } else { physical };
            assert_eq!((2 * logical + 1) * 3 / (2 * 11), lower);
        }
    }
    assert_eq!(
        (0..5).map(|i| (2 * i + 1) * 3 / 10).collect::<Vec<_>>(),
        vec![0, 0, 1, 2, 2]
    );
}
#[test]
fn selection_preserves_centers_depth_clamp_styles_and_source_ownership() {
    let mut source = sprite();
    source.depth_clamp = true;
    source.vertices[0].weight = 2.;
    source.vertices[0].position[2] = -3.;
    let before = source.clone();
    let selected = selected_draw(&source, vk::SampleCountFlags::TYPE_1)
        .unwrap()
        .unwrap();
    assert!(
        selected.fixed_grid_point
            && selected.fixed_grid_point_unit12
            && selected.fixed_grid_point_sprite
    );
    assert_eq!(selected.vertices, source.vertices);
    assert_eq!(selected.clip_transform, source.clip_transform);
    assert_eq!(selected.point_style, source.point_style);
    assert_eq!(selected.depth_clamp, source.depth_clamp);
    assert_eq!(source, before);
}
#[test]
fn hardware_and_earlier_versions_leave_alpha_sprites_unchanged() {
    for behavior in [
        RenderPointRasterBehavior::HardwareQuadV1,
        RenderPointRasterBehavior::FixedGridAliasedV2,
        RenderPointRasterBehavior::FixedGridUnit12V3,
    ] {
        assert!(
            super::super::fixed_grid_upload::selected_point_draw(
                behavior,
                &sprite(),
                vk::SampleCountFlags::TYPE_1
            )
            .unwrap()
            .is_none()
        );
    }
    assert!(
        selected_draw(&sprite(), vk::SampleCountFlags::TYPE_4)
            .unwrap()
            .is_none()
    );
    assert_eq!(
        RenderPointRasterBehavior::default(),
        RenderPointRasterBehavior::HardwareQuadV1
    );
    assert_eq!(
        RenderPointRasterBehavior::FixedGridSprites12V4.behavior_version(),
        4
    );
}
#[test]
fn surface_and_registered_shader_inputs_refuse_before_publication() {
    let mut source = sprite();
    source.fixed_function_lighting = true;
    let before = source.clone();
    assert!(selected_draw(&source, vk::SampleCountFlags::TYPE_1).is_err());
    assert_eq!(source, before);
}

#[test]
fn uploaded_marker_matrix_uses_reflected_viewport_and_crop_before_cell_assertion() {
    let point = [-1.7763568e-15f32, -0.51078486, 0.59985614];
    let clip_y =
        ((0.02475763 * point[1] + -0.02475763 * point[0]) + 0.04951526 * point[2]) + 0.000058643876;
    let physical_y = clip_y * -204.5 + 205.5;
    let gl_y = 411. - physical_y;
    assert_eq!((gl_y * 4096.).round_ties_even(), 856064.);
    let (minimum, _) = rectangle([200., gl_y], 11.);
    let attachment_top = 411 - minimum[1] - 11;
    let logical_cross_row = attachment_top + 5 - 1;
    assert_eq!(logical_cross_row, 200);
}

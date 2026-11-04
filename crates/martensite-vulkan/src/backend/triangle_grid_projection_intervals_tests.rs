use super::*;

#[test]
fn prebounded_clips_preserve_existing_projection_and_refuse_zero_w() {
    let draw = crate::backend::triangle_grid_coverage_upload::tests::bug29791_triangle_nine();
    let viewport = [0.0, 410.0, 409.0, -409.0];
    let clips =
        crate::window_depth_bounds::clip_transform_bounds(draw.clip_transform, draw.vertices[0])
            .unwrap();
    let expected = projected_q12_intervals(&draw, &[0, 1, 2], viewport).unwrap()[0];
    assert_eq!(projected_clip_bounds(clips, viewport), Some(expected));
    let mut zero_w = clips;
    zero_w[3] = crate::window_depth_bounds::DepthShaderBound::source(0.0).unwrap();
    assert_eq!(projected_clip_bounds(zero_w, viewport), None);
}

#[test]
fn witnessed_phong_triangle_maps_each_corner_to_outward_q12_endpoints() {
    let draw = crate::backend::triangle_grid_coverage_upload::tests::bug29791_triangle_nine();
    let intervals =
        projected_q12_intervals(&draw, &[0, 1, 2], [0.0, 410.0, 409.0, -409.0]).unwrap();
    assert_eq!(
        intervals.map(GridVertexInterval::coordinates),
        [
            [[1014062, 1014062], [278330, 278332]],
            [[1014063, 1014064], [278312, 278314]],
            [[1012647, 1012648], [308581, 308583]],
        ]
    );
}

#[test]
fn projected_q12_intervals_refuse_invalid_transform_extent_and_index() {
    let mut draw = crate::backend::triangle_grid_coverage_upload::tests::bug29791_triangle_nine();
    draw.clip_transform[0] = f32::NAN;
    assert!(projected_q12_intervals(&draw, &[0, 1, 2], [0.0, 410.0, 409.0, -409.0]).is_none());
    let draw = crate::backend::triangle_grid_coverage_upload::tests::bug29791_triangle_nine();
    assert!(projected_q12_intervals(&draw, &[0, 1], [0.0, 410.0, 409.0, -409.0]).is_none());
    assert!(projected_q12_intervals(&draw, &[0, 1, 2], [0.0, 0.0, f32::MAX, 1.0]).is_none());
}

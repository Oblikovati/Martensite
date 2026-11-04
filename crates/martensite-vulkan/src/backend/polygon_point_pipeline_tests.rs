//! Pure fixed-state/cache-key controls do not create a device.
use super::*;
use crate::{RenderVertex, identity_clip};

fn triangle() -> PrimitiveDraw {
    let vertices = [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]]
        .map(|position| RenderVertex::new(position, [1.; 4]));
    PrimitiveDraw::new(vertices.to_vec(), vec![0, 1, 2], identity_clip()).unwrap()
}
#[test]
fn polygon_point_cache_key_is_distinct_from_fill_and_lines() {
    let mut draw = triangle();
    let filled = special_variant_key(&draw);
    draw.polygon_points = true;
    let points = special_variant_key(&draw);
    assert_ne!(points, filled);
    draw.polygon_points = false;
    draw.polygon_edge_style = Some(crate::LineStyle::new(1., 0xffff, 1).unwrap());
    assert_ne!(points, special_variant_key(&draw));
}
#[test]
fn polygon_points_keep_triangle_assembly_face_shader_and_culling() {
    let mut draw = triangle().with_face_culling(crate::RenderFaceCulling::Back);
    draw.polygon_points = true;
    let kind = special_variant(&draw, false);
    let state = super::super::pipeline_state::PipelineState::new(kind);
    assert_eq!(
        state.raster_description().polygon_mode,
        vk::PolygonMode::POINT
    );
    assert!(!kind.point && !kind.line && !kind.text);
    assert_eq!(draw.face_culling, crate::RenderFaceCulling::Back);
}
#[test]
fn ordinary_pipeline_polygon_modes_remain_fill_and_line() {
    let mut kind = PipelineVariant::from_index(0, false);
    assert_eq!(kind.polygon_raster_mode(), vk::PolygonMode::FILL);
    kind.polygon_edges = true;
    assert_eq!(kind.polygon_raster_mode(), vk::PolygonMode::LINE);
}

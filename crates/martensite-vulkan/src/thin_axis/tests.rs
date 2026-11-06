use super::*;
use crate::{LineStyle, RenderLineGrid::*, RenderLineOrigin::*, RenderVertex};
fn mode(grid: crate::RenderLineGrid, origin: crate::RenderLineOrigin) -> RenderThinAxisCoverage {
    RenderThinAxisCoverage::FixedGridV2 { grid, origin }
}
fn segment(a: [f32; 2], b: [f32; 2]) -> PrimitiveDraw {
    let vertices =
        [a, b].map(|p| RenderVertex::new([p[0] / 32. - 1., p[1] / 32. - 1., 0.], [1.; 4]));
    PrimitiveDraw::segments(
        vertices.into(),
        vec![0, 1],
        crate::identity_clip(),
        LineStyle::new(1., 0xffff, 1).unwrap(),
    )
    .unwrap()
}
fn selected(draw: &PrimitiveDraw) -> PrimitiveDraw {
    selected_draw(mode(Fractional12, BottomUp), draw, [64, 64], true)
        .unwrap()
        .unwrap()
}
fn positions(draw: &PrimitiveDraw) -> Vec<[f32; 2]> {
    draw.vertices
        .iter()
        .map(|v| [0, 1].map(|i| (v.position[i] / v.weight + 1.) * 32.))
        .collect()
}
#[test]
fn integer_closed_form_precision_matches_all_translations() {
    for integer in -50..50 {
        let p = integer as f32 + 1. / 1024.;
        assert_eq!(cells::minor_cell(p, Fractional12), integer as f32);
        assert_eq!(cells::minor_cell(p, Fractional8), (integer - 1) as f32);
        for grid in [Fractional8, Fractional12] {
            assert_eq!(
                cells::minor_cell(integer as f32 + 0.25, grid),
                integer as f32
            );
        }
    }
}
#[test]
fn coordinate_direction_changes_only_the_boundary_cell() {
    let y = 44. - 1. / 1024.;
    assert_eq!(cells::minor_cell(y, Fractional8), 43.);
    let bottom = cells::oriented([0., y], 400, TopDown)[1];
    assert_eq!(399. - cells::minor_cell(bottom, Fractional8), 44.);
    assert_eq!(399. - cells::minor_cell(bottom, Fractional12), 43.);
    assert_eq!(
        cells::oriented(cells::oriented([2., 5.], 400, TopDown), 400, TopDown),
        [2., 5.]
    );
}
#[test]
fn vertical_cells_retain_source_and_reverse_half_open_ends() {
    let source = segment([4. + 1. / 1024., 10.25], [4. + 1. / 1024., 20.25]);
    let before = source.clone();
    assert_eq!(
        positions(&selected(&source)),
        vec![[4., 10.], [5., 10.], [5., 20.], [4., 20.]]
    );
    let reverse = segment([4. + 1. / 1024., 20.25], [4. + 1. / 1024., 10.25]);
    assert_eq!(
        positions(&selected(&reverse)),
        vec![[4., 10.], [5., 10.], [5., 20.], [4., 20.]]
    );
    assert_eq!(source, before);
}
#[test]
fn interpolation_retains_homogeneous_depth_and_vertex_colors() {
    let mut source = segment([4.25, 10.25], [4.25, 20.25]);
    source.vertices[0].position[2] = -0.5;
    source.vertices[1].position[2] = 0.5;
    source.vertices[0].color = [0.; 4];
    source.vertices[1].color = [1.; 4];
    let output = selected(&source);
    for vertex in &output.vertices {
        let y = (vertex.position[1] / vertex.weight + 1.) * 32.;
        assert!((vertex.position[2] - ((y - 10.25) / 10. - 0.5)).abs() < 1e-6);
        assert!((vertex.color[0] - (y - 10.25) / 10.).abs() < 1e-6);
    }
}
#[test]
fn hardware_nonaxis_wide_stipple_and_multisample_are_unchanged() {
    let source = segment([4.25, 10.25], [4.25, 20.25]);
    assert!(
        selected_draw(Default::default(), &source, [64, 64], true)
            .unwrap()
            .is_none()
    );
    assert!(
        selected_draw(mode(Fractional12, BottomUp), &source, [64, 64], false)
            .unwrap()
            .is_none()
    );
    let diagonal = segment([4.25, 10.25], [5.25, 20.25]);
    assert!(
        selected_draw(mode(Fractional12, BottomUp), &diagonal, [64, 64], true)
            .unwrap()
            .is_none()
    );
}
#[test]
fn wide_and_stippled_lines_keep_existing_geometry() {
    let source = segment([4.25, 10.25], [4.25, 20.25]);
    for style in [
        LineStyle::new(2., 0xffff, 1).unwrap(),
        LineStyle::new(1., 0xff00, 1).unwrap(),
    ] {
        let mut other = source.clone();
        other.line_style = Some(style);
        assert!(
            selected_draw(mode(Fractional12, BottomUp), &other, [64, 64], true)
                .unwrap()
                .is_none()
        );
    }
}
#[test]
fn unsupported_projection_or_style_refuses_without_mutation() {
    let source = segment([4.25, 10.25], [4.25, 20.25]);
    let mut behind = source.clone();
    behind.clip_transform[15] = -1.;
    let mut clipped = source.clone();
    clipped.vertices[0].position[2] = -2.;
    let mut blend = source.clone();
    blend.line_style.as_mut().unwrap().alpha_blend = true;
    for invalid in [behind, clipped, blend] {
        let before = invalid.clone();
        assert!(selected_draw(mode(Fractional12, BottomUp), &invalid, [64, 64], true).is_err());
        assert_eq!(invalid, before);
    }
    assert!(selected_draw(mode(Fractional12, BottomUp), &source, [0, 64], true).is_err());
}
#[test]
fn degenerate_lines_cover_no_cells() {
    let output = selected(&segment([4.25, 10.25], [4.25, 10.25]));
    assert!(output.vertices.is_empty());
    assert!(output.indices.is_empty());
}
#[test]
fn projected_line_interpolation_uses_perspective_parameter() {
    let w = [2., 4.];
    let s = crate::wide_line_columns::perspective_parameter(0.5, w[0], w[1]);
    assert!((s - 1. / 3.).abs() < 1e-7);
    let interpolated_w = w[0] + (w[1] - w[0]) * s;
    assert!((s * w[1] / interpolated_w - 0.5).abs() < 1e-7);
}
#[test]
fn nonaxis_style_and_clip_state_stay_on_the_legacy_path() {
    let mut source = segment([4.25, 10.25], [5.25, 20.25]);
    source.vertices[0].position[2] = -2.;
    source.line_style.as_mut().unwrap().alpha_blend = true;
    let before = source.clone();
    let output = selected_draw(mode(Fractional12, BottomUp), &source, [64, 64], true).unwrap();
    assert!(output.is_none());
    assert_eq!(source, before);
}
#[test]
fn semantic_surface_blending_refuses_axis_encoding() {
    let mut source = segment([4.25, 10.25], [4.25, 20.25]);
    source.surface_blend = true;
    let before = source.clone();
    assert!(selected_draw(mode(Fractional12, BottomUp), &source, [64, 64], true).is_err());
    assert_eq!(source, before);
}
#[test]
fn semantic_depth_clamp_preserves_outside_z_bits_but_not_xy_clipping() {
    let mut source = segment([4.25, 10.25], [4.25, 20.25]);
    for v in &mut source.vertices {
        v.position[2] = -1.000_000_2;
    }
    assert!(selected_draw(mode(Fractional12, BottomUp), &source, [64, 64], true).is_err());
    source.depth_clamp = true;
    let output = selected(&source);
    assert!(output.depth_clamp);
    assert!(
        output
            .vertices
            .iter()
            .all(|v| v.position[2].to_bits() == source.vertices[0].position[2].to_bits())
    );
    source.vertices[0].position[0] = -2.;
    source.vertices[1].position[0] = -2.;
    assert!(selected_draw(mode(Fractional12, BottomUp), &source, [64, 64], true).is_err());
}
#[test]
fn ignored_polygon_offset_preserves_line_and_following_fill_state() {
    let source = segment([4.25, 10.25], [4.25, 20.25])
        .with_polygon_offset(crate::RenderPolygonOffset::new(1, 2., 3.).unwrap());
    let output = selected(&source);
    assert_eq!(output.polygon_offset, source.polygon_offset);
    let mut state = crate::polygon_offset::PolygonOffsetState::default();
    assert_eq!(state.advance(output.polygon_offset, false), [0., 0.]);
    assert!(!state.raster_bias_enabled(false, false));
    assert_eq!(state.advance(None, true), [3., 2.]);
}

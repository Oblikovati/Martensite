//! Pin the old clone-and-replace result independently of how geometry storage is copied.
use super::*;
use crate::{LineStyle, RenderCappingPass, RenderClipping, RenderFaceCulling, RenderPolygonOffset};

fn stateful_source() -> PrimitiveDraw {
    let vertices = vec![RenderVertex::new([0., 0., 0.], [0.25, 0.5, 0.75, 1.]); 4];
    let style = LineStyle::new(2., 0x39c7, 3)
        .unwrap()
        .with_alpha_blending(true);
    let mut source =
        PrimitiveDraw::segments(vertices, vec![2, 3], crate::identity_clip(), style).unwrap();
    source.quad_corners = vec![[3, 2, 1, 0]];
    source.native_flat_sources = vec![2, 3, 1];
    source.polygon_offset = Some(RenderPolygonOffset::new(7, 1.5, -2.).unwrap());
    source.face_culling = RenderFaceCulling::Front;
    source.capping_pass = RenderCappingPass::Surface;
    source.clipping =
        Some(RenderClipping::new(vec![vec![[1., 0., 0., -0.5]]], crate::identity_clip()).unwrap());
    source.native_material_cutoff = Some(0.3);
    source.unlit_back_color = Some([0.2, 0.4, 0.6, 0.8]);
    source.surface_cutoff = Some(0.6);
    stateful_flags(&mut source);
    source
}

fn stateful_flags(source: &mut PrimitiveDraw) {
    source.hatch_rows = Some([0xa53c_0ff0; 32]);
    source.depth_clamp = true;
    source.mirrored = true;
    source.fixed_function_lighting = true;
    source.compatibility_color_target = true;
    source.source_color_array = true;
    source.source_line_strip = true;
    source.annotation = true;
    source.text_shadow_bias = true;
    source.text_logic_xor = true;
    source.text_background_blend = true;
    source.surface_blend = true;
    source.surface_forced_blend = true;
    source.surface_texture_binding_only = true;
    source.edge_background_fallback = true;
}

fn old_prepared_result(source: &PrimitiveDraw, upload: [[f32; 4]; 2]) -> PrimitiveDraw {
    let mut expected = source.clone();
    expected.vertices = upload
        .map(|point| RenderVertex::homogeneous(point, source.vertices[2].color))
        .to_vec();
    expected.indices = vec![0, 1];
    expected.clip_transform = crate::identity_clip();
    expected.projected_dash = Some(PreparedAxis {
        extent: [96, 64],
        axis: 2,
    });
    expected
}

#[test]
fn prepared_geometry_replacement_preserves_all_old_draw_state() {
    // Exercise rich metadata directly: ordinary finite preflight deliberately rejects some of it.
    let source = stateful_source();
    let upload = [[-0.5, 0.25, 0.75, 1.], [0.5, -0.25, 0.5, 1.]];
    let expected = old_prepared_result(&source, upload);
    let actual = prepared_draw(&source, upload, [96, 64], 2, 2);
    assert_eq!(actual, expected);
    println!("PREPARED_STATE_PIN={actual:?}");
}

#[test]
fn prepared_geometry_replacement_keeps_independent_source_storage() {
    let mut source = stateful_source();
    let upload = [[0., 0., 0., 1.], [1., 0., 0., 1.]];
    let actual = prepared_draw(&source, upload, [96, 64], 2, 2);
    let expected = actual.clone();
    source.vertices.clear();
    source.indices.clear();
    source.quad_corners.clear();
    source.native_flat_sources.clear();
    source.clipping = None;
    source.hatch_rows = None;
    assert_eq!(actual, expected);
}

use super::*;
use crate::{RasterImage, RenderVertex, TexturedStyle, identity_clip};

fn glyph_triangle() -> PrimitiveDraw {
    let vertices = [[-1., -1., 0.], [1., -1., 0.], [-1., 1., 0.]]
        .map(|position| RenderVertex::new(position, [1.; 4]))
        .to_vec();
    let image = RasterImage::new([1, 1], vec![[1.; 4]]).unwrap();
    let style = TexturedStyle::new(image, vec![[0., 0.], [1., 0.], [0., 1.]], 0.285).unwrap();
    PrimitiveDraw::textured(vertices, vec![0, 1, 2], identity_clip(), style).unwrap()
}

#[test]
fn selected_triangles_are_admitted_without_changing_submission_inputs() {
    let draw = glyph_triangle();
    let original = draw.clone();
    assert!(admit(Behavior::ProjectedGrid12V2, &draw, [0., 0., 10., 10.]).is_ok());
    assert_eq!(draw, original);
    assert!(require_interface(Behavior::ProjectedGrid12V2, &draw, true).is_ok());
    assert!(
        require_interface(Behavior::ProjectedGrid12V2, &draw, false)
            .unwrap_err()
            .text_interpolation_failure()
            .is_some()
    );
}

#[test]
fn hardware_replay_skips_grid_domain_and_interface_checks() {
    let mut draw = glyph_triangle();
    draw.clip_transform[15] = -1.;
    assert!(admit(Behavior::HardwareV1, &draw, [0., 0., 10., 10.]).is_ok());
    assert!(require_interface(Behavior::HardwareV1, &draw, false).is_ok());
}

#[test]
fn selected_behavior_accepts_signed_height_and_refuses_invalid_viewports() {
    let draw = glyph_triangle();
    for viewport in [[0., 8., 8., -8.], [0., 0., 8., 8.]] {
        assert!(admit(Behavior::ProjectedGrid12V2, &draw, viewport).is_ok());
    }
    for viewport in [
        [0., 0., 0., 8.],
        [0., 0., 8., 0.],
        [0., 0., -8., 8.],
        [0., 0., f32::NAN, 8.],
        [0., 0., 8., f32::NAN],
        [0., 0., f32::INFINITY, 8.],
        [0., 0., f32::NEG_INFINITY, 8.],
        [0., 0., 8., f32::INFINITY],
        [0., 0., 8., f32::NEG_INFINITY],
        [f32::NAN, 0., 8., 8.],
        [0., f32::INFINITY, 8., 8.],
    ] {
        assert!(
            admit(Behavior::ProjectedGrid12V2, &draw, viewport).is_err(),
            "unexpected accepted viewport {viewport:?}"
        );
    }
}

#[test]
fn selected_behavior_refuses_nonpositive_w_outside_grid_and_degenerate_triangles() {
    let mut draw = glyph_triangle();
    for bad in [-1., 0., f32::INFINITY, f32::NAN] {
        draw.clip_transform[15] = bad;
        assert!(
            admit(Behavior::ProjectedGrid12V2, &draw, [0., 0., 10., 10.])
                .unwrap_err()
                .text_interpolation_failure()
                .is_some()
        );
    }
    draw.clip_transform = identity_clip();
    draw.clip_transform[0] = 1000.;
    assert!(admit(Behavior::ProjectedGrid12V2, &draw, [0., 0., 10., 10.]).is_err());
    draw.clip_transform = identity_clip();
    for vertex in &mut draw.vertices {
        vertex.position[0] = 0.;
    }
    assert!(admit(Behavior::ProjectedGrid12V2, &draw, [0., 0., 10., 10.]).is_err());
}

#[test]
fn invalid_indices_and_nontriangle_topology_refuse_with_typed_error() {
    let mut draw = glyph_triangle();
    draw.indices = vec![0, 1, 8];
    assert!(
        admit(Behavior::ProjectedGrid12V2, &draw, [0., 0., 10., 10.])
            .unwrap_err()
            .text_interpolation_failure()
            .is_some()
    );
    let mut draw = glyph_triangle();
    draw.line_style = Some(crate::LineStyle::new(1., 0xffff, 1).unwrap());
    assert!(
        admit(Behavior::ProjectedGrid12V2, &draw, [0., 0., 10., 10.])
            .unwrap_err()
            .text_interpolation_failure()
            .is_some()
    );
}

#[test]
fn selected_mode_refuses_text_draws_that_require_a_different_shader_interface() {
    let mut draw = glyph_triangle();
    draw.fixed_function_lighting = true;
    let error = admit(Behavior::ProjectedGrid12V2, &draw, [0., 0., 10., 10.]).unwrap_err();
    assert!(error.text_interpolation_failure().is_some());
    assert!(error.to_string().contains("plain glyph-text pipeline"));
}

#[test]
fn shader_clip_w_domain_excludes_nonpositive_nonfinite_and_subnormal_reciprocals() {
    for value in [0.0, -1.0, f32::INFINITY, f32::NAN, f32::MAX] {
        assert!(
            !supported_clip_w(value),
            "unexpected accepted clip w {value}"
        );
    }
    for value in [f32::MIN_POSITIVE, 1.0, 4.0] {
        assert!(
            supported_clip_w(value),
            "unexpected rejected clip w {value}"
        );
    }
    assert!(!supported_world_point([f32::MAX, 0.0, 0.0, 0.5]));
    assert!(supported_world_point([2.0, -4.0, 8.0, 2.0]));
}

#[test]
fn clipped_text_refuses_world_w_before_the_clipping_shader_division() {
    let mut draw = glyph_triangle();
    for (vertex, position) in
        draw.vertices
            .iter_mut()
            .zip([[1., 1., 0.], [2., 1., 0.], [1., 2., 0.]])
    {
        vertex.position = position;
        vertex.weight = 0.0;
    }
    draw.clip_transform[3] = 1.0;
    draw.clip_transform[7] = 1.0;
    draw.clip_transform[15] = 0.0;
    let clipping =
        crate::RenderClipping::new(vec![vec![[1.0, 0.0, 0.0, 0.0]]], identity_clip()).unwrap();
    let draw = draw.with_clipping(clipping);
    let error = admit(Behavior::ProjectedGrid12V2, &draw, [0., 0., 10., 10.]).unwrap_err();
    assert!(error.text_interpolation_failure().is_some());
}

#[test]
fn clipped_text_refuses_finite_world_values_that_overflow_when_divided() {
    let mut draw = glyph_triangle();
    for (vertex, position) in draw.vertices.iter_mut().zip([
        [f32::MAX, -1.0, -1.0],
        [f32::MAX, 1.0, -1.0],
        [f32::MAX, -1.0, 1.0],
    ]) {
        vertex.position = position;
        vertex.weight = 0.5;
    }
    draw.clip_transform = [0.0; 16];
    draw.clip_transform[4] = 1.0;
    draw.clip_transform[9] = 1.0;
    draw.clip_transform[15] = 1.0;
    let clipping =
        crate::RenderClipping::new(vec![vec![[1.0, 0.0, 0.0, 0.0]]], identity_clip()).unwrap();
    let draw = draw.with_clipping(clipping);
    let error = admit(Behavior::ProjectedGrid12V2, &draw, [0., 0., 10., 10.]).unwrap_err();
    assert!(error.text_interpolation_failure().is_some());
}

#[test]
fn core_flat_transport_preserves_inputs_and_requires_its_registered_interface() {
    let draw = glyph_triangle();
    let original = draw.clone();
    assert!(
        admit(
            Behavior::ProjectedGrid12FlatCornersV4,
            &draw,
            [0., 8., 8., -8.]
        )
        .is_ok()
    );
    assert_eq!(draw, original);
    assert!(require_interface(Behavior::ProjectedGrid12FlatCornersV4, &draw, true).is_ok());
    assert!(require_interface(Behavior::ProjectedGrid12FlatCornersV4, &draw, false).is_err());
}

#[test]
fn core_flat_transport_refuses_corrupt_index_sources_before_packing() {
    let mut draw = glyph_triangle();
    draw.indices[1] = 99;
    assert!(
        admit(
            Behavior::ProjectedGrid12FlatCornersV4,
            &draw,
            [0., 0., 8., 8.]
        )
        .is_err()
    );
}

#[test]
fn core_flat_transport_refuses_incomplete_uv_sources_before_packing() {
    let mut draw = glyph_triangle();
    draw.textured_style.as_mut().unwrap().uv.pop();
    assert!(
        admit(
            Behavior::ProjectedGrid12FlatCornersV4,
            &draw,
            [0., 0., 8., 8.]
        )
        .is_err()
    );
}

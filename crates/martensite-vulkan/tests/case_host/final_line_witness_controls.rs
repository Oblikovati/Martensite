//! Independent closed-form clipping fixtures; no driver or raster-policy acceptance.
use super::*;
use martensite_vulkan::{LineStyle, RenderVertex};

fn segment(a: [f32; 4], b: [f32; 4]) -> PrimitiveDraw {
    let vertices = [a, b]
        .map(|p| RenderVertex {
            position: [p[0], p[1], p[2]],
            weight: p[3],
            color: [1.; 4],
        })
        .to_vec();
    let identity = [
        1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.,
    ];
    PrimitiveDraw::segments(
        vertices,
        vec![0, 1],
        identity,
        LineStyle::new(1., 65535, 1).unwrap(),
    )
    .unwrap()
}

fn crossing() -> PrimitiveDraw {
    segment([-2., 0., 0., -1.], [0., 0., 0., 1.])
}

#[test]
fn mixed_w_segment_is_clipped_to_independent_guard_plane() {
    let result = prepare(vec![crossing()], [64., 64.]).unwrap();
    let a = result[0].vertices()[0];
    assert!(a.weight > 0.);
    // For x(t)=-2+2t,w(t)=-1+2t and guard x=-256w, t=258/514.
    assert!((f64::from(a.position[0]) / f64::from(a.weight) + 256.).abs() < 0.01);
    assert_eq!(result[0].vertices()[1], crossing().vertices()[1]);
}

#[test]
fn fully_behind_and_shared_outside_halfspace_are_rejected() {
    let behind = segment([-1., 0., 0., -1.], [1., 0., 0., -1.]);
    let outside = segment([1000., 0., 0., -1.], [1000., 0., 0., 1.]);
    for draw in [behind, outside] {
        let result = prepare(vec![draw], [64., 64.]).unwrap();
        assert!(result[0].vertices().is_empty());
    }
}

#[test]
fn endpoint_reversal_preserves_derived_clipped_locus() {
    let forward = prepare(vec![crossing()], [64., 64.]).unwrap();
    let reverse = segment([0., 0., 0., 1.], [-2., 0., 0., -1.]);
    let reverse = prepare(vec![reverse], [64., 64.]).unwrap();
    for (a, b) in forward[0]
        .vertices()
        .iter()
        .zip(reverse[0].vertices().iter().rev())
    {
        assert!((a.position[0] - b.position[0]).abs() < 1e-6);
        assert!((a.weight - b.weight).abs() < 1e-6);
    }
}

#[test]
fn near_far_depth_is_retained_not_claimed_as_gpu_clipped() {
    let draw = segment([-0.5, 0., -3., 1.], [0.5, 0., 3., 1.]);
    let before = format!("{draw:?}");
    let result = prepare(vec![draw], [64., 64.]).unwrap();
    assert_eq!(format!("{:?}", result[0]), before);
}

#[test]
fn zero_w_degenerate_and_nonfinite_or_invalid_extent_refuse() {
    let zero = segment([0., 0., 0., 0.], [0., 0., 0., 0.]);
    assert!(prepare(vec![zero], [64., 64.]).is_err());
    for size in [
        [0., 64.],
        [64.5, 64.],
        [f64::NAN, 64.],
        [f64::INFINITY, 64.],
    ] {
        assert!(prepare(vec![crossing()], size).is_err());
    }
}

#[test]
fn transparent_crossing_context_is_incomplete_not_empty_success() {
    let source = crossing();
    let draw = PrimitiveDraw::segments(
        source.vertices().to_vec(),
        source.indices().to_vec(),
        *source.clip_transform(),
        LineStyle::new(1., 65535, 1)
            .unwrap()
            .with_alpha_blending(true),
    )
    .unwrap();
    assert!(prepare(vec![draw], [64., 64.]).is_err());
}

#[test]
fn nonunit_width_context_is_refused_without_guessing_device_policy() {
    let source = crossing();
    let wide = PrimitiveDraw::segments(
        source.vertices().to_vec(),
        source.indices().to_vec(),
        *source.clip_transform(),
        LineStyle::new(2., 65535, 1).unwrap(),
    )
    .unwrap();
    assert!(prepare(vec![wide], [64., 64.]).is_err());
}

#[test]
fn opaque_layer_split_preserves_derived_ordered_line_geometry() {
    let first = crossing();
    let second = segment([0., -2., 0., -1.], [0., 0., 0., 1.]);
    let layers = vec![
        martensite_vulkan::RenderLayer::new(vec![first.clone()], true, true, true),
        martensite_vulkan::RenderLayer::new(vec![second.clone()], true, true, false),
    ];
    let actual = RenderFrame::with_layers(64, 64, [0.; 4], layers).unwrap();
    let derived = prepare(vec![first, second], [64., 64.]).unwrap();
    assert_eq!(format!("{derived:?}"), format!("{:?}", actual.draws()));
}

#[test]
fn all_front_nonunit_draw_is_preserved_without_replay() {
    let source = segment([0., 0., 0., 1.], [0.5, 0.5, 0., 1.]);
    let wide = PrimitiveDraw::segments(
        source.vertices().to_vec(),
        source.indices().to_vec(),
        *source.clip_transform(),
        LineStyle::new(2., 65535, 1).unwrap(),
    )
    .unwrap();
    let before = format!("{wide:?}");
    assert_eq!(
        format!("{:?}", prepare(vec![wide], [64., 64.]).unwrap()[0]),
        before
    );
}

#[test]
fn mixed_surface_context_is_incomplete_instead_of_assumed_flat_order() {
    let points = vec![RenderVertex::new([0., 0., 0.], [1.; 4]); 3];
    let surface = PrimitiveDraw::new(points, vec![0, 1, 2], *crossing().clip_transform()).unwrap();
    assert!(prepare(vec![crossing(), surface], [64., 64.]).is_err());
}

#[test]
fn nonfinite_primitive_inputs_are_rejected_before_replay() {
    let source = crossing();
    let mut vertices = source.vertices().to_vec();
    vertices[0].position[0] = f32::NAN;
    assert!(
        PrimitiveDraw::segments(
            vertices,
            vec![0, 1],
            *source.clip_transform(),
            LineStyle::new(1., 65535, 1).unwrap()
        )
        .is_err()
    );
}

#[test]
fn earlier_wide_context_prevents_partial_unit_cohort_replay() {
    let source = segment([0., 0., 0., 1.], [0.5, 0.5, 0., 1.]);
    let wide = PrimitiveDraw::segments(
        source.vertices().to_vec(),
        source.indices().to_vec(),
        *source.clip_transform(),
        LineStyle::new(2., 65535, 1).unwrap(),
    )
    .unwrap();
    assert!(prepare(vec![wide, crossing()], [64., 64.]).is_err());
}

#[test]
fn effective_single_pixel_policy_does_not_erase_requested_width_refusal() {
    let source = crossing();
    let wide = PrimitiveDraw::segments(
        source.vertices().to_vec(),
        source.indices().to_vec(),
        *source.clip_transform(),
        LineStyle::new(2., 65535, 1).unwrap(),
    )
    .unwrap();
    let frame = RenderFrame::new(64, 64, [0.; 4], vec![wide])
        .unwrap()
        .with_line_width_policy(martensite_vulkan::RenderLineWidthPolicy::SinglePixelV1);
    assert_eq!(frame.draws()[0].line_style().unwrap().raster_width(), 1.);
    assert!(!unit_opaque_style(frame.draws()[0].line_style().unwrap()));
}

#[test]
fn replay_preserves_owned_source_draws_on_success_and_refusal() {
    let source = crossing();
    let before = format!("{source:?}");
    let _ = prepare(vec![source.clone()], [64., 64.]).unwrap();
    assert_eq!(format!("{source:?}"), before);
    assert!(prepare(vec![source.clone()], [0., 64.]).is_err());
    assert_eq!(format!("{source:?}"), before);
}

#[test]
fn float32_cancellation_cannot_bypass_crossing_eye_guard() {
    let source = segment([1., 1., 1., 1.], [1., 2., 1., 1.]);
    let mut matrix = *source.clip_transform();
    for (index, value) in [(3, 16777216.), (7, 1.), (11, -16777216.), (15, -0.5)] {
        matrix[index] = value;
    }
    let draw = PrimitiveDraw::segments(
        source.vertices().to_vec(),
        source.indices().to_vec(),
        matrix,
        LineStyle::new(1., 65535, 1).unwrap(),
    )
    .unwrap();
    assert_eq!(clip_weight(&draw, &draw.vertices()[0]), -0.5);
    assert!(crosses_eye(&draw));
    let actual = RenderFrame::new(64, 64, [0.; 4], vec![draw.clone()]).unwrap();
    let derived = prepare(vec![draw], [64., 64.]).unwrap();
    assert_eq!(format!("{derived:?}"), format!("{:?}", actual.draws()));
}

#[test]
fn dashed_crossing_context_refuses_instead_of_rewriting_stipple_geometry() {
    let source = crossing();
    let draw = PrimitiveDraw::segments(
        source.vertices().to_vec(),
        source.indices().to_vec(),
        *source.clip_transform(),
        LineStyle::new(1., 0xFF00, 1).unwrap(),
    )
    .unwrap();
    assert!(prepare(vec![draw], [64., 64.]).is_err());
}

#[test]
fn finite_inputs_with_overflowing_clip_product_refuse() {
    let source = segment([2., 0., 0., 1.], [3., 0., 0., 1.]);
    let mut matrix = *source.clip_transform();
    matrix[0] = f32::MAX;
    let draw = PrimitiveDraw::segments(
        source.vertices().to_vec(),
        source.indices().to_vec(),
        matrix,
        LineStyle::new(1., 65535, 1).unwrap(),
    )
    .unwrap();
    assert!(prepare(vec![draw], [64., 64.]).is_err());
}

fn opaque_surface() -> PrimitiveDraw {
    let points = [[0., 0., 0.], [0.5, 0., 0.], [0., 0.5, 0.]];
    let vertices = points.map(|p| RenderVertex::new(p, [1.; 4])).to_vec();
    PrimitiveDraw::new(vertices, vec![0, 1, 2], *crossing().clip_transform()).unwrap()
}

#[test]
fn complete_mixed_layers_preserve_surface_and_clipped_line_geometry() {
    let surface = opaque_surface();
    let layers = vec![RenderLayer::new(
        vec![surface.clone(), crossing()],
        true,
        true,
        false,
    )];
    let before = format!("{layers:?}");
    let result = prepare_layers(layers.clone(), [64., 64.]).unwrap();
    assert_eq!(result[0], surface);
    assert_eq!(result[1], prepare(vec![crossing()], [64., 64.]).unwrap()[0]);
    assert_eq!(format!("{layers:?}"), before);
}

#[test]
fn complete_mixed_depth_layers_match_actual_frame_preparation() {
    let offset = martensite_vulkan::RenderPolygonOffset::new(1, 2., 3.).unwrap();
    let layers = vec![
        RenderLayer::new(vec![opaque_surface()], true, true, true).with_polygon_offset(offset),
        RenderLayer::new(vec![crossing()], false, false, true),
        RenderLayer::new(vec![opaque_surface()], true, false, false),
    ];
    let actual = RenderFrame::with_layers(64, 64, [0.; 4], layers.clone()).unwrap();
    let prepared = prepare_layers(layers, [64., 64.]).unwrap();
    assert_eq!(prepared, actual.draws());
    assert_eq!(actual.layers()[0].polygon_offset(), offset);
    assert_eq!(actual.layers()[1].depth_policy(), (false, false, true));
}

#[test]
fn layered_replay_refuses_nonunit_lines_without_mutating_source() {
    let source = crossing();
    let wide = PrimitiveDraw::segments(
        source.vertices().to_vec(),
        source.indices().to_vec(),
        *source.clip_transform(),
        LineStyle::new(2., 65535, 1).unwrap(),
    )
    .unwrap();
    let layers = vec![RenderLayer::new(
        vec![opaque_surface(), wide],
        true,
        true,
        false,
    )];
    let before = format!("{layers:?}");
    assert!(prepare_layers(layers.clone(), [64., 64.]).is_err());
    assert_eq!(format!("{layers:?}"), before);
}

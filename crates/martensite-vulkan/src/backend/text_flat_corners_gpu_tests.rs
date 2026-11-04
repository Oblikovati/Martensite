//! These real-device tests require an explicit GPU lease; CPU selectors must exclude them.
use super::{VulkanOptions, VulkanRenderer};
use crate::{
    FrameCapture, PrimitiveDraw, RasterImage, RenderBackend, RenderBehaviorConfiguration,
    RenderCoordinateConvention as Origin, RenderFrame, RenderRasterPolicy as Provoking,
    RenderTextInterpolationBehavior as Text, RenderVertex, TexturedStyle, identity_clip,
};

fn renderer(text: Text, origin: Origin, provoking: Provoking) -> VulkanRenderer {
    VulkanRenderer::new_with_render_behavior(
        VulkanOptions {
            validation: true,
            device_index: Some(0),
        },
        provoking,
        origin,
        Default::default(),
        RenderBehaviorConfiguration::default().with_text_interpolation(text),
    )
    .unwrap()
}

pub(super) fn quad(perspective: bool) -> PrimitiveDraw {
    let positions = [[-1., -1., 0.], [1., -1., 0.], [1., 1., 0.], [-1., 1., 0.]];
    let vertices = positions.map(|[x, y, z]| {
        let w = if perspective && x > 0. { 2. } else { 1. };
        RenderVertex::homogeneous([x * w, y * w, z, w], [1.; 4])
    });
    PrimitiveDraw::textured(
        vertices.to_vec(),
        vec![0, 1, 2, 0, 2, 3],
        identity_clip(),
        style(),
    )
    .unwrap()
}

fn style() -> TexturedStyle {
    let image = RasterImage::new(
        [2, 2],
        vec![
            [0., 0., 0., 1.],
            [1., 0., 0., 1.],
            [0., 1., 0., 1.],
            [1., 1., 0., 1.],
        ],
    )
    .unwrap();
    TexturedStyle::new(image, vec![[0., 0.], [1., 0.], [1., 1.], [0., 1.]], 0.285).unwrap()
}

fn frame(draw: PrimitiveDraw) -> RenderFrame {
    RenderFrame::new(8, 8, [0., 0., 0., 1.], vec![draw]).unwrap()
}

fn srgb_byte(linear: f32) -> u8 {
    let encoded = if linear <= 0.003_130_8 {
        12.92 * linear
    } else {
        1.055 * martensite_cmath::powf(linear, 1. / 2.4) - 0.055
    };
    (encoded * 255.).round() as u8
}

pub(super) fn capture(renderer: &mut VulkanRenderer, draw: PrimitiveDraw) -> FrameCapture {
    let captured = renderer.render(&frame(draw)).unwrap();
    assert!(renderer.validation_errors().unwrap().is_empty());
    captured
}

#[test]
fn flat_corner_affine_and_perspective_uvs_match_independent_closed_forms_on_both_origins() {
    for origin in [Origin::LegacyBottomUpV1, Origin::TopLeftLogicalV2] {
        let mut renderer = renderer(
            Text::ProjectedGrid12FlatCornersV4,
            origin,
            Provoking::LegacyFirstV1,
        );
        for (perspective, linear) in [(false, [0.625, 0.375]), (true, [13. / 46., 5. / 46.])] {
            let actual = capture(&mut renderer, quad(perspective));
            // Top-left pixel(4,4) has UV=(9/16,7/16); perspective UV=(9/23,7/23).
            assert_eq!(
                actual.pixel(4, 4).unwrap(),
                [srgb_byte(linear[0]), srgb_byte(linear[1]), 0, 255]
            );
        }
    }
}

#[test]
fn flat_corner_first_and_last_provoking_policies_preserve_identical_color_and_depth() {
    let origin = Origin::TopLeftLogicalV2;
    let mut first = renderer(
        Text::ProjectedGrid12FlatCornersV4,
        origin,
        Provoking::LegacyFirstV1,
    );
    let mut last = renderer(
        Text::ProjectedGrid12FlatCornersV4,
        origin,
        Provoking::LastProvokingVertexV2,
    );
    let left = capture(&mut first, quad(true));
    let right = capture(&mut last, quad(true));
    assert_eq!(left.rgba, right.rgba);
    assert_eq!(left.depth, right.depth);
}

#[test]
fn flat_corner_clipping_preserves_world_halfspace_and_unclipped_samples() {
    let clipping =
        crate::RenderClipping::new(vec![vec![[1., 0., 0., 0.]]], identity_clip()).unwrap();
    let mut renderer = renderer(
        Text::ProjectedGrid12FlatCornersV4,
        Origin::TopLeftLogicalV2,
        Provoking::LegacyFirstV1,
    );
    let baseline = capture(&mut renderer, quad(true));
    let clipped = capture(&mut renderer, quad(true).with_clipping(clipping));
    assert_eq!(clipped.pixel(3, 4).unwrap(), [0, 0, 0, 255]);
    assert_ne!(baseline.pixel(3, 4).unwrap(), [0, 0, 0, 255]);
    assert_eq!(clipped.pixel(6, 4), baseline.pixel(6, 4));
    assert_eq!(clipped.depth[4 * 8 + 6], baseline.depth[4 * 8 + 6]);
}

#[test]
fn flat_corner_integer_lattice_geometry_keeps_hardware_rgba_and_depth() {
    let origin = Origin::TopLeftLogicalV2;
    let mut stock = renderer(Text::HardwareV1, origin, Provoking::LegacyFirstV1);
    let mut flat = renderer(
        Text::ProjectedGrid12FlatCornersV4,
        origin,
        Provoking::LegacyFirstV1,
    );
    let baseline = capture(&mut stock, quad(false));
    let candidate = capture(&mut flat, quad(false));
    assert_eq!(baseline.rgba, candidate.rgba);
    assert_eq!(baseline.depth, candidate.depth);
}

#[test]
fn flat_corner_clip_generated_vertices_keep_the_original_uv_plane() {
    let draw = clipped_quad();
    let mut renderer = renderer(
        Text::ProjectedGrid12FlatCornersV4,
        Origin::TopLeftLogicalV2,
        Provoking::LegacyFirstV1,
    );
    let actual = capture(&mut renderer, draw);
    // The reviewed source-derived format reference is limited to this new owned test.
    // d653's nearest-only red assertion and actual HardwareV1/V4 captures remain retained.
    super::text_flat_corners_reference::require_clipped_pixel(actual.pixel(4, 4).unwrap());
}

#[test]
fn flat_corner_near_clip_preserves_visibility_and_the_unclipped_uv_field() {
    let mut draw = quad(false);
    for vertex in &mut draw.vertices {
        vertex.position[2] = vertex.position[0] - 1.;
    }
    let mut renderer = renderer(
        Text::ProjectedGrid12FlatCornersV4,
        Origin::TopLeftLogicalV2,
        Provoking::LegacyFirstV1,
    );
    let actual = capture(&mut renderer, draw);
    assert_eq!(actual.pixel(2, 4).unwrap(), [0, 0, 0, 255]);
    assert_eq!(
        actual.pixel(4, 4).unwrap(),
        [srgb_byte(0.625), srgb_byte(0.375), 0, 255]
    );
    assert!(actual.depth[4 * 8 + 4] < 1.);
}

fn clipped_quad() -> PrimitiveDraw {
    let mut draw = quad(false);
    for vertex in &mut draw.vertices {
        vertex.position[0] *= 2.;
    }
    draw
}

fn clipped_capture(text: Text) -> FrameCapture {
    let mut renderer = renderer(text, Origin::TopLeftLogicalV2, Provoking::LegacyFirstV1);
    capture(&mut renderer, clipped_quad())
}

#[test]
fn flat_corner_same_clipped_hardware_v1_frame_matches_full_rgba_and_depth() {
    let hardware = clipped_capture(Text::HardwareV1);
    let flat = clipped_capture(Text::ProjectedGrid12FlatCornersV4);
    super::triangle_grid_coverage_gpu_tests::retain_capture("text-clipped-hardware-v1", &hardware);
    super::triangle_grid_coverage_gpu_tests::retain_capture("text-clipped-flat-v4", &flat);
    println!(
        "{{\"hardware_pixel\":{:?},\"flat_pixel\":{:?}}}",
        hardware.pixel(4, 4).unwrap(),
        flat.pixel(4, 4).unwrap()
    );
    assert_eq!(hardware.rgba, flat.rgba);
    assert_eq!(hardware.depth, flat.depth);
}

use super::expand::selected_draw;
use crate::{
    LineStyle, PrimitiveDraw, RenderThinAxisCoverage, RenderThinLineCoverage, RenderVertex,
    identity_clip,
};
use ash::vk;

fn line_draw(start: [f32; 3], end: [f32; 3], colors: [[f32; 4]; 2], pattern: u16) -> PrimitiveDraw {
    PrimitiveDraw::segments(
        vec![
            RenderVertex::new(start, colors[0]),
            RenderVertex::new(end, colors[1]),
        ],
        vec![0, 1],
        identity_clip(),
        LineStyle::new(1., pattern, 1).unwrap(),
    )
    .unwrap()
}

fn perspective_line_draw() -> PrimitiveDraw {
    PrimitiveDraw::segments(
        vec![
            RenderVertex::homogeneous([-0.5, 0.03125, -0.5, 1.], [1., 0., 0., 1.]),
            RenderVertex::homogeneous([1.5, 0.09375, 1.5, 3.], [0., 0., 1., 1.]),
        ],
        vec![0, 1],
        identity_clip(),
        LineStyle::new(1., 0xffff, 1).unwrap(),
    )
    .unwrap()
}

fn expand(draw: &PrimitiveDraw) -> Result<PrimitiveDraw, crate::RenderError> {
    expand_with_behavior(draw, RenderThinLineCoverage::DiamondExitV2)
}

fn expand_with_behavior(
    draw: &PrimitiveDraw,
    behavior: RenderThinLineCoverage,
) -> Result<PrimitiveDraw, crate::RenderError> {
    selected_draw(
        behavior,
        RenderThinAxisCoverage::HardwareV1,
        draw,
        vk::Extent2D {
            width: 32,
            height: 32,
        },
        true,
    )
    .map(Option::unwrap)
}

#[test]
fn expansion_preserves_the_unlit_per_pixel_source_color() {
    let colors = [[1., 0., 0., 1.], [0., 1., 0., 1.]];
    let source = line_draw([-0.5, 0., 0.], [0.5, 0., 0.], colors, 0xffff);
    let selected = expand(&source).unwrap();
    assert_eq!(selected.topology(), crate::PrimitiveTopology::Triangles);
    assert_eq!(selected.clip_transform, identity_clip());
    assert!(selected.line_style.is_none());
    assert!(
        selected
            .vertices
            .iter()
            .any(|vertex| vertex.color != colors[0])
    );
}

#[test]
fn color_and_depth_match_hand_computed_unclamped_line_parameter() {
    let source = perspective_line_draw();
    let before = source.clone();
    let selected = expand(&source).unwrap();
    assert_eq!(source, before);
    let pixel = pixel_at(&selected, 12., 16.);
    assert_pixel_output(pixel);
}

#[test]
fn major_axis_v3_perspective_attributes_use_the_selected_parameter() {
    let source = perspective_line_draw();
    let selected =
        expand_with_behavior(&source, RenderThinLineCoverage::DiamondExitMajorAxisV3).unwrap();
    assert_pixel_output(pixel_at(&selected, 12., 16.));
}

#[test]
fn major_axis_v3_clamps_extrapolated_depth_at_both_clip_planes() {
    assert_endpoint_depth_clamp_for(RenderThinLineCoverage::DiamondExitMajorAxisV3, 1., 0., 1.);
    assert_endpoint_depth_clamp_for(RenderThinLineCoverage::DiamondExitMajorAxisV3, -1., 0., -1.);
}

#[test]
fn varying_parameter_uses_dominant_axis_for_both_line_slopes_and_directions() {
    let behavior = RenderThinLineCoverage::DiamondExitMajorAxisV3;
    let x_major = super::expand::line_parameter(behavior, [2., 3.], [12., 8.], [7.5, 7.5]);
    let x_reverse = super::expand::line_parameter(behavior, [12., 8.], [2., 3.], [7.5, 7.5]);
    let y_major = super::expand::line_parameter(behavior, [3., 2.], [8., 12.], [7.5, 7.5]);
    let y_reverse = super::expand::line_parameter(behavior, [8., 12.], [3., 2.], [7.5, 7.5]);
    assert_eq!(
        (x_major, x_reverse, y_major, y_reverse),
        (0.55, 0.45, 0.55, 0.45)
    );
}

#[test]
fn varying_parameter_keeps_extrapolated_fragment_centers_unclamped() {
    let t = super::expand::line_parameter(
        RenderThinLineCoverage::DiamondExitMajorAxisV3,
        [0., 0.],
        [4., 2.],
        [-0.5, 0.5],
    );
    assert_eq!(t, -0.125);
}

#[test]
fn diamond_exit_v2_keeps_orthogonal_fragment_center_parameter() {
    let t = super::expand::line_parameter(
        RenderThinLineCoverage::DiamondExitV2,
        [0., 0.],
        [4., 2.],
        [1., 2.],
    );
    assert_eq!(t, 0.4);
}

#[test]
fn major_axis_v3_clip_samples_match_captured_world_plane_crossings() {
    let clip = crate::RenderClipping::new(
        vec![vec![[-1., 0., 0., 3.]], vec![[1., 0., 0., 3.]]],
        identity_clip(),
    )
    .unwrap();
    let source = line_draw([-10., 0., 0.], [10., 0., 0.], [[1.; 4]; 2], 0xffff).with_clipping(clip);
    assert_parameter_clip_pair(
        &source,
        [29.18777785, 305.7405886],
        [204.53140157, 406.97528069],
        [90.5, 341.5],
    );
    assert_parameter_clip_pair(
        &source,
        [204.46860366, 2.02474556],
        [379.81222738, 103.25943764],
        [318.5, 67.5],
    );
}

fn assert_parameter_clip_pair(
    draw: &PrimitiveDraw,
    start: [f64; 2],
    end: [f64; 2],
    sample: [f64; 2],
) {
    let point = |behavior| {
        let t = super::expand::line_parameter(behavior, start, end, sample);
        let x = -10. + 20. * t;
        super::user_clipping::accepts(draw, [x, 0., 0., 1.]).unwrap()
    };
    assert!(point(RenderThinLineCoverage::DiamondExitV2));
    assert!(!point(RenderThinLineCoverage::DiamondExitMajorAxisV3));
}

#[test]
fn start_endpoint_inside_diamond_emits_its_stippled_pixel() {
    let ndc = |pixel: f32| 2. * pixel / 32. - 1.;
    let source = line_draw(
        [ndc(8.25), ndc(16.5), 0.],
        [ndc(16.25), ndc(12.5), 0.],
        [[0., 1., 1., 1.]; 2],
        0xffff,
    );
    let selected = expand(&source).unwrap();
    assert_eq!(pixel_at(&selected, 8., 16.).len(), 4);
}

#[test]
fn far_plane_line_sample_clamps_extrapolated_depth_before_quad_clipping() {
    assert_endpoint_depth_clamp(1., 0., 1.);
}

#[test]
fn near_plane_line_sample_clamps_extrapolated_depth_before_quad_clipping() {
    assert_endpoint_depth_clamp(-1., 0., -1.);
}

fn assert_endpoint_depth_clamp(start_depth: f32, end_depth: f32, expected: f32) {
    assert_endpoint_depth_clamp_for(
        RenderThinLineCoverage::DiamondExitV2,
        start_depth,
        end_depth,
        expected,
    );
}

fn assert_endpoint_depth_clamp_for(
    behavior: RenderThinLineCoverage,
    start_depth: f32,
    end_depth: f32,
    expected: f32,
) {
    let ndc = |pixel: f32| 2. * pixel / 32. - 1.;
    let source = line_draw(
        [ndc(8.75), ndc(16.5), start_depth],
        [ndc(16.75), ndc(12.5), end_depth],
        [[0., 1., 1., 1.]; 2],
        0xffff,
    );
    let selected = expand_with_behavior(&source, behavior).unwrap();
    let pixel = pixel_at(&selected, 8., 16.);
    assert_eq!(pixel.len(), 4);
    for vertex in pixel {
        assert_eq!(vertex.position[2], expected);
    }
}

#[test]
fn semantic_line_polygon_offset_survives_private_quad_lowering() {
    let offset = crate::RenderPolygonOffset::new(1, 1.5, -2.).unwrap();
    let source =
        line_draw([-0.5, 0., 0.], [0.5, 0., 0.], [[1.; 4]; 2], 0xffff).with_polygon_offset(offset);
    let selected = expand(&source).unwrap();
    assert_eq!(source.topology(), crate::PrimitiveTopology::Lines);
    assert_eq!(selected.topology(), crate::PrimitiveTopology::Triangles);
    assert_eq!(selected.polygon_offset(), Some(offset));
}

fn pixel_at(draw: &PrimitiveDraw, x: f64, y: f64) -> &[RenderVertex] {
    draw.vertices
        .as_chunks::<4>()
        .0
        .iter()
        .find(|quad| {
            let screen = [0, 1].map(|axis| (f64::from(quad[0].position[axis]) + 1.) * 16.);
            (screen[0] - x).abs() < 0.0001 && (screen[1] - y).abs() < 0.0001
        })
        .unwrap()
}

fn assert_pixel_output(pixel: &[RenderVertex]) {
    let perspective = 3. / 26.;
    let expected_color = [1. - perspective, 0., perspective, 1.];
    let expected_depth = -7. / 32.;
    for vertex in pixel {
        for (actual, expected) in vertex.color.into_iter().zip(expected_color) {
            assert!((actual - expected).abs() < 1e-6);
        }
        assert!((f64::from(vertex.position[2]) - expected_depth).abs() < 1e-6);
    }
}

#[test]
fn clipping_and_zero_stipple_produce_the_semantic_empty_geometry() {
    let source = line_draw([-2., 0., 0.], [-1.5, 0., 0.], [[1.; 4]; 2], 0xffff);
    assert!(expand(&source).unwrap().indices.is_empty());
    let no_stipple = line_draw([-0.5, 0., 0.], [0.5, 0., 0.], [[1.; 4]; 2], 0);
    assert!(expand(&no_stipple).unwrap().indices.is_empty());
}

#[test]
fn user_clipping_matches_union_within_chains_and_intersection_between_chains() {
    let volume = crate::RenderClipping::new(
        vec![
            vec![[1., 0., 0., -0.5], [-1., 0., 0., -0.5]],
            vec![[0., 1., 0., 0.]],
        ],
        identity_clip(),
    )
    .unwrap();
    let draw = line_draw([-0.5, 0., 0.], [0.5, 0., 0.], [[1.; 4]; 2], 0xffff).with_clipping(volume);
    assert!(super::user_clipping::accepts(&draw, [0.75, 0., 0., 1.]).unwrap());
    assert!(super::user_clipping::accepts(&draw, [-0.75, 0., 0., 1.]).unwrap());
    assert!(!super::user_clipping::accepts(&draw, [0., -0.25, 0., 1.]).unwrap());
    assert!(!super::user_clipping::accepts(&draw, [0., 0.25, 0., 1.]).unwrap());
}

#[test]
fn user_clipping_filters_fragment_samples_without_mutating_source_geometry() {
    let source = line_draw([-0.5, 0., 0.], [0.5, 0., 0.], [[1.; 4]; 2], 0xffff);
    let clipped = source.clone().with_clipping(
        crate::RenderClipping::new(vec![vec![[1., 0., 0., 0.]]], identity_clip()).unwrap(),
    );
    let before = clipped.clone();
    let selected = expand(&clipped).unwrap();
    assert_eq!(clipped, before);
    assert!(selected.clipping.is_none());
    assert!(selected.vertices.as_chunks::<4>().0.iter().all(|quad| {
        let ndc_x = f64::from(quad[0].position[0]);
        ndc_x >= 0.
    }));
    assert!(!selected.indices.is_empty());
}

#[test]
fn unsupported_multisample_and_axis_composition_are_typed_refusals() {
    let source = line_draw([-0.5, 0., 0.], [0.5, 0., 0.], [[1.; 4]; 2], 0xffff);
    let result = selected_draw(
        RenderThinLineCoverage::DiamondExitV2,
        RenderThinAxisCoverage::FixedGridV2 {
            grid: crate::RenderLineGrid::Fractional12,
            origin: crate::RenderLineOrigin::BottomUp,
        },
        &source,
        vk::Extent2D {
            width: 32,
            height: 32,
        },
        false,
    );
    assert!(result.is_err());
}

#[test]
fn long_diagonal_candidate_work_grows_linearly_with_major_axis() {
    let extent = [32_768, 32_768];
    let work = super::expand::candidate_work([0.25, 0.75], [32_767.25, 32_767.75], extent);
    assert!(work <= 3 * (extent[0] as usize + 3));
}

#[test]
fn bottom_up_quad_coordinates_map_to_the_same_logical_pixel_under_both_viewports() {
    let extent = [32, 409];
    let ndc = super::expand::pixel_vertex([10., 127.5], extent, 0., [1.; 4]).position[1];
    let height = f64::from(extent[1]);
    let top_left_attachment = 1. + height - height * (f64::from(ndc) + 1.) / 2.;
    let top_left_logical = top_left_attachment.floor() as u32 - 1;
    let legacy_attachment = height * (f64::from(ndc) + 1.) / 2.;
    let legacy_logical = extent[1] - 1 - legacy_attachment.floor() as u32;
    assert_eq!(top_left_logical, 281);
    assert_eq!(legacy_logical, 281);
}

#[test]
fn endpoint_projection_quantization_is_ties_even_with_a_half_grid_bound() {
    let scale = 65_536.;
    assert_eq!(super::expand::fixed_coordinate(0.5 / scale).unwrap(), 0);
    assert_eq!(super::expand::fixed_coordinate(1.5 / scale).unwrap(), 2);
    for coordinate in [0.12345, 127.999_999, 409.125] {
        let quantized = super::expand::fixed_coordinate(coordinate).unwrap() as f64 / scale;
        assert!((quantized - coordinate).abs() <= 0.5 / scale);
    }
}

#[test]
fn fused_v4_preserves_perspective_attributes_and_clipped_depth_contract() {
    let behavior = RenderThinLineCoverage::DiamondExitFusedProjectionV4;
    let source = perspective_line_draw();
    let before = source.clone();
    let selected = expand_with_behavior(&source, behavior).unwrap();
    assert_pixel_output(pixel_at(&selected, 12., 16.));
    assert_eq!(source, before);
    assert_endpoint_depth_clamp_for(behavior, 1., 0., 1.);
    assert_endpoint_depth_clamp_for(behavior, -1., 0., -1.);
}

#[test]
fn fused_v4_retains_dominant_axis_for_signed_slopes_and_extrapolation() {
    let behavior = RenderThinLineCoverage::DiamondExitFusedProjectionV4;
    for (start, end, expected) in [
        ([2., 3.], [12., 8.], 0.55),
        ([12., 8.], [2., 3.], 0.45),
        ([3., 2.], [8., 12.], 0.55),
        ([8., 12.], [3., 2.], 0.45),
    ] {
        assert_eq!(
            super::expand::line_parameter(behavior, start, end, [7.5, 7.5]),
            expected
        );
    }
    assert_eq!(
        super::expand::line_parameter(behavior, [0., 0.], [4., 2.], [-0.5, 0.5]),
        -0.125
    );
}

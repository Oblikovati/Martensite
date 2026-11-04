//! Closed-form and generated controls do not depend on native graphics references.
use super::*;
use crate::{LineStyle, RenderBackend, RenderFrame, RenderVertex, identity_clip};
fn segment(a: [f32; 4], b: [f32; 4]) -> PrimitiveDraw {
    let vertices = [a, b].map(|p| RenderVertex::homogeneous(p, [1., 0., 0., 1.]));
    PrimitiveDraw::segments(
        vertices.to_vec(),
        vec![0, 1],
        identity_clip(),
        LineStyle::new(1., 0x5555, 1).unwrap(),
    )
    .unwrap()
}
fn finite(draw: &PrimitiveDraw) -> ProjectedSegments {
    prepare_projected_segments(
        draw,
        [64, 64],
        DashBehavior::FiniteProjectiveV2,
        1e-4,
        &[97],
    )
    .unwrap()
}
#[test]
fn exact_infinite_ray_retains_source_identity_and_half_interval() {
    let source = segment([0., 0., 0., 1.], [1., 0., 0., 0.]);
    let retained = source.clone();
    let prepared = finite(&source);
    assert_eq!(source, retained);
    assert_eq!(prepared.pixel_error_limit(), Some(1e-4));
    assert_eq!(prepared.source_identities(), &[97]);
    assert_eq!(prepared.intervals()[0].exact_parameters, ["0", "1/2"]);
    assert_eq!(prepared.intervals()[0].source_identity, 97);
    assert_eq!(prepared.draws()[0].vertices[1].position[0], 0.5);
    assert_eq!(prepared.draws()[0].vertices[1].weight, 0.5);
    assert_eq!(prepared.intervals()[0].pixel_error_bound, 0.);
}
#[test]
fn clipping_crossing_outside_and_projective_origin_have_distinct_outcomes() {
    let crossing = finite(&segment([-2., 0., 0., 1.], [2., 0., 0., 1.]));
    assert_eq!(crossing.intervals()[0].exact_parameters, ["1/4", "3/4"]);
    assert!(
        finite(&segment([2., 0., 0., 1.], [3., 0., 0., 1.]))
            .draws()
            .is_empty()
    );
    assert_projective_origin_refusal();
}
fn assert_projective_origin_refusal() {
    let origin = segment([0.; 4], [1., 0., 0., 1.]);
    let failure = prepare_projected_segments(
        &origin,
        [64, 64],
        DashBehavior::FiniteProjectiveV2,
        1e-4,
        &[5],
    )
    .unwrap_err();
    assert_eq!(failure.class, ProjectedFailureClass::Refused);
    assert_eq!(failure.segment, Some(0));
    assert_eq!(failure.source_identity, Some(5));
}

#[test]
fn depth_clamping_keeps_xy_clipping_and_original_depth_policy() {
    let mut source = segment([0., 0., 0., 1.], [1., 0., 2., 0.]);
    assert_eq!(finite(&source).intervals()[0].exact_parameters[1], "1/3");
    source.depth_clamp = true;
    let output = finite(&source);
    assert_eq!(output.intervals()[0].exact_parameters[1], "1/2");
    assert!(output.draws()[0].depth_clamp);
    assert_eq!(
        output.draws()[0].vertices[1].position[2] / output.draws()[0].vertices[1].weight,
        2.
    );
}
#[test]
fn legacy_selection_and_extent_guard_preserve_recorded_upload() {
    let source = segment([0., 0., 0., 1.], [1., 0., 0., 0.]);
    let output = prepare_projected_segments(
        &source,
        [64, 64],
        DashBehavior::LegacyDerivativesV1,
        1e-4,
        &[9],
    )
    .unwrap();
    assert_eq!(output.draws(), std::slice::from_ref(&source));
    let prepared = finite(&source);
    assert_prepared_cannot_replay_legacy(&prepared);
    assert!(RenderFrame::new(64, 64, [0.; 4], prepared.draws().to_vec()).is_ok());
    assert!(RenderFrame::new(65, 64, [0.; 4], prepared.draws().to_vec()).is_err());
}
fn assert_prepared_cannot_replay_legacy(prepared: &ProjectedSegments) {
    let result = prepare_projected_segments(
        &prepared.draws()[0],
        [64, 64],
        DashBehavior::LegacyDerivativesV1,
        1e-4,
        &[97],
    );
    let refusal = result.unwrap_err();
    assert_eq!(refusal.class, ProjectedFailureClass::Refused);
    assert!(refusal.cause.contains("original unprepared source"));
}

#[test]
fn generated_perspective_boundaries_preserve_positive_weights_and_isolation() {
    for exponent in -20..20 {
        let weight = martensite_cmath::powi(f64::from(2_f32), exponent) as f32;
        let source = segment([-2. * weight, 0., 0., weight], [2., 0.5, 0., 1.]);
        let retained = source.clone();
        let prepared = finite(&source);
        println!(
            "FINITE_INTERVAL {{\"exponent\":{exponent},\"start\":\"{}\",\"end\":\"{}\"}}",
            prepared.intervals()[0].exact_parameters[0],
            prepared.intervals()[0].exact_parameters[1]
        );
        assert_eq!(source, retained);
        assert!(prepared.draws()[0].vertices.iter().all(|p| p.weight > 0.));
        assert!(prepared.intervals()[0].pixel_error_bound <= 1e-4);
    }
}
#[test]
fn owned_axis_rule_has_exact_diagonal_threshold_and_extent_dependence() {
    for (end, expected) in [
        ([1., 0., 0., 1.], 1),
        ([0., 1., 0., 1.], 2),
        ([1., 1., 0., 1.], 2),
        ([1., 0.995, 0., 1.], 1),
        ([1., 0.9995, 0., 1.], 2),
    ] {
        let output = finite(&segment([0., 0., 0., 1.], end));
        assert_eq!(output.draws()[0].projected_dash.unwrap().axis, expected);
    }
}
#[test]
fn refusal_and_incomplete_leave_inputs_unchanged() {
    let source = segment([-0.9, 0.1, 0., 0.91], [0.8, 0.7, 0., 0.83]);
    let retained = source.clone();
    let error = prepare_projected_segments(
        &source,
        [997, 991],
        DashBehavior::FiniteProjectiveV2,
        1e-30,
        &[8],
    )
    .unwrap_err();
    assert_eq!(error.class, ProjectedFailureClass::Incomplete);
    assert_eq!(source, retained);
    assert_missing_identity_refusal(&source);
}
fn assert_missing_identity_refusal(source: &PrimitiveDraw) {
    let result = prepare_projected_segments(
        source,
        [64, 64],
        DashBehavior::FiniteProjectiveV2,
        1e-4,
        &[],
    );
    assert_eq!(result.unwrap_err().class, ProjectedFailureClass::Refused);
}

#[test]
fn published_projection_types_are_send_and_sync() {
    let _: &(dyn Send + Sync) = &std::marker::PhantomData::<ProjectedSegments>;
    let _: &(dyn Send + Sync) = &std::marker::PhantomData::<ProjectedFailure>;
    let _: &(dyn Send + Sync) = &std::marker::PhantomData::<SegmentInterval>;
    let _: &(dyn Send + Sync) = &std::marker::PhantomData::<DashBehavior>;
    let _: &(dyn Send + Sync) = &std::marker::PhantomData::<ProjectedFailureClass>;
}
#[test]
fn finite_gpu_phase_matches_independent_pixel_mask_and_preserves_depth() {
    let mut renderer = crate::VulkanRenderer::new(crate::VulkanOptions {
        validation: true,
        device_index: Some(0),
    })
    .unwrap();
    for (a, b, axis) in [
        ([-1., 0., 0., 1.], [1., 0., 0., 1.], 1),
        ([0., -1., 0., 1.], [0., 1., 0., 1.], 2),
        ([0., 0., 0., 1.], [1., 0., 0., 0.], 1),
        ([-0.5, -0.2, 0., 0.5], [0.6, 0.7, 0., 1.], 1),
    ] {
        for extent in [[64, 64], [97, 61]] {
            check_gpu_segment(&mut renderer, a, b, axis, extent);
        }
    }
    println!("FINITE_GPU_DRIVER {:?}", renderer.driver_identity());
    assert!(renderer.validation_errors().unwrap().is_empty());
}
fn render_prepared(
    renderer: &mut crate::VulkanRenderer,
    source: &PrimitiveDraw,
    extent: [u32; 2],
) -> crate::FrameCapture {
    let draws = prepared_for_extent(source, extent);
    let frame = RenderFrame::new(extent[0], extent[1], [0., 0., 0., 1.], draws).unwrap();
    renderer.render(&frame).unwrap()
}
fn prepared_for_extent(source: &PrimitiveDraw, extent: [u32; 2]) -> Vec<PrimitiveDraw> {
    let output = prepare_projected_segments(
        source,
        extent,
        DashBehavior::FiniteProjectiveV2,
        1e-4,
        &[97],
    )
    .unwrap();
    output.draws().to_vec()
}

fn check_gpu_segment(
    renderer: &mut crate::VulkanRenderer,
    a: [f32; 4],
    b: [f32; 4],
    axis: u32,
    extent: [u32; 2],
) {
    let mut source = segment(a, b);
    for (pattern, factor) in [(0x5555, 1), (0x8421, 2), (0xffff, 256), (0, 1)] {
        source.line_style = Some(LineStyle::new(1., pattern, factor).unwrap());
        let dashed = render_prepared(renderer, &source, extent);
        source.line_style = Some(LineStyle::new(1., 0xffff, factor).unwrap());
        let solid = render_prepared(renderer, &source, extent);
        let half = renderer.stored_depth(0.5);
        assert_gpu_phase(&solid, &dashed, (axis, half), pattern, factor);
        save_pair([a, b], pattern, factor, &solid, &dashed);
    }
}

fn assert_gpu_phase(
    solid: &crate::FrameCapture,
    dashed: &crate::FrameCapture,
    (axis, half): (u32, f32),
    pattern: u16,
    factor: u16,
) {
    let mut covered = 0;
    for index in 0..(solid.width * solid.height) as usize {
        let visible = solid.rgba[index * 4] == 255;
        let bit = phase_bit(index, solid, axis, factor);
        let accepted = visible && (pattern & (1 << bit)) != 0;
        assert_gpu_pixel(dashed, index, (axis, half), accepted);
        covered += usize::from(visible);
    }
    assert!(covered > 16);
}

fn phase_bit(index: usize, capture: &crate::FrameCapture, axis: u32, factor: u16) -> u32 {
    let coordinate = if axis == 1 {
        index % capture.width as usize + 1
    } else {
        capture.height as usize - index / capture.width as usize
    };
    ((coordinate as f64 - 0.5) / f64::from(factor) + 0.5).floor() as u32 & 15
}

fn assert_gpu_pixel(
    capture: &crate::FrameCapture,
    index: usize,
    (axis, half): (u32, f32),
    accepted: bool,
) {
    assert_eq!(
        capture.rgba[index * 4] == 255,
        accepted,
        "pixel {index}, axis {axis}"
    );
    assert_eq!(capture.depth[index], if accepted { half } else { 1. });
}
#[test]
fn nondefault_contract_refuses_attributes_and_degenerate_projection() {
    let mut source = segment([0., 0., 0., 1.], [1., 0., 0., 1.]);
    source.vertices[1].color = [0., 1., 0., 1.];
    let retained = source.clone();
    let failure = prepare_projected_segments(
        &source,
        [64, 64],
        DashBehavior::FiniteProjectiveV2,
        1e-4,
        &[44],
    )
    .unwrap_err();
    assert_eq!(failure.class, ProjectedFailureClass::Refused);
    assert_eq!(failure.source_identity, Some(44));
    assert_eq!(source, retained);
    assert_degenerate_refusal();
}
fn assert_degenerate_refusal() {
    let zero = segment([0., 0., 0., 1.], [0., 0., 0.5, 2.]);
    assert!(
        prepare_projected_segments(
            &zero,
            [64, 64],
            DashBehavior::FiniteProjectiveV2,
            1e-4,
            &[45]
        )
        .is_err()
    );
}

#[test]
fn caller_identities_remain_ordered_after_completed_empty_pairs() {
    let mut source = segment([2., 0., 0., 1.], [3., 0., 0., 1.]);
    source
        .vertices
        .extend(segment([-1., 0., 0., 1.], [1., 0., 0., 1.]).vertices);
    source.indices.extend([2, 3]);
    let output = prepare_projected_segments(
        &source,
        [64, 64],
        DashBehavior::FiniteProjectiveV2,
        1e-4,
        &[900, 12],
    )
    .unwrap();
    assert_eq!(output.source_identities(), &[900, 12]);
    assert_eq!(output.intervals().len(), 1);
    assert_eq!(output.intervals()[0].source_segment, 1);
    assert_eq!(output.intervals()[0].source_identity, 12);
}
#[test]
fn extreme_homogeneous_scale_is_incomplete_without_denominator_clamping() {
    let smallest = f32::from_bits(1);
    let mut source = segment([smallest, 0., 0., 0.], [0., 0., 0., 1.]);
    source.clip_transform[0] = smallest;
    source.clip_transform[3] = smallest;
    let retained = source.clone();
    let failure = prepare_projected_segments(
        &source,
        [64, 64],
        DashBehavior::FiniteProjectiveV2,
        1e-4,
        &[71],
    )
    .unwrap_err();
    assert_eq!(failure.class, ProjectedFailureClass::Incomplete);
    assert_eq!(failure.source_identity, Some(71));
    assert_eq!(source, retained);
}
#[test]
fn source_mutation_cannot_rewrite_published_prepared_geometry() {
    let mut source = segment([-1., 0., 0., 1.], [1., 0., 0., 1.]);
    let published = finite(&source);
    let expected = published.draws()[0].vertices.clone();
    source.vertices[0].position = [0.5, 0.5, 0.5];
    source.indices.clear();
    assert_eq!(published.draws()[0].vertices, expected);
    assert_eq!(published.draws()[0].indices, [0, 1]);
}
#[test]
fn generated_near_diagonal_axes_match_independent_binary_float_products() {
    let threshold = ((999_f64 / 1001.).sqrt() as f32).to_bits();
    for bits in threshold - 64..=threshold + 64 {
        let y = f32::from_bits(bits);
        let yy = f64::from(y) * f64::from(y);
        let expected = if 1000. * (1. - yy) > 1. + yy { 1 } else { 2 };
        let output = finite(&segment([0., 0., 0., 1.], [1., y, 0., 1.]));
        assert_eq!(output.draws()[0].projected_dash.unwrap().axis, expected);
    }
}

fn save_pair(
    ends: [[f32; 4]; 2],
    pattern: u16,
    factor: u16,
    solid: &crate::FrameCapture,
    dashed: &crate::FrameCapture,
) {
    let Some(directory) = super::capture_tests::DashCaptureDirectory::requested() else {
        return;
    };
    let bits = ends.map(|point| point.map(f32::to_bits));
    let name = format!(
        "finite-{bits:?}-{}x{}-{pattern}-{factor}",
        solid.width, solid.height
    );
    directory.save(&format!("{name}-solid"), &solid.rgba, &solid.depth);
    directory.save(&format!("{name}-dashed"), &dashed.rgba, &dashed.depth);
}
#[test]
fn column_major_transform_clips_against_exact_transformed_coordinates() {
    let mut source = segment([-1., 0., 0., 1.], [1., 0., 0., 1.]);
    source.clip_transform[0] = 2.;
    source.clip_transform[12] = 1.;
    source.clip_transform[15] = 2.;
    let output = finite(&source);
    assert_eq!(output.intervals()[0].exact_parameters, ["0", "3/4"]);
    assert_eq!(output.draws()[0].vertices[0].position, [-0.5, 0., 0.]);
    assert_eq!(output.draws()[0].vertices[1].position, [1., 0., 0.]);
    assert_eq!(output.draws()[0].vertices[0].weight, 1.);
}
#[test]
fn certified_bound_rounds_up_instead_of_accepting_a_lower_nearest_float() {
    let third = super::exact::scalar(1.) / super::exact::scalar(3.);
    let bound = super::exact::upward(&third).unwrap();
    assert_eq!(bound.to_bits(), 0x3fd5_5555_5555_5556);
    assert!(num_rational::BigRational::from_float(bound).unwrap() >= third);
}
#[test]
fn unsupported_world_clipping_refuses_before_and_after_preparation() {
    let source = segment([-1., 0., 0., 1.], [1., 0., 0., 1.]);
    let planes = crate::RenderClipping::new(vec![vec![[1., 0., 0., 0.]]], identity_clip()).unwrap();
    let clipped = source.clone().with_clipping(planes.clone());
    let refusal = prepare_projected_segments(
        &clipped,
        [64, 64],
        DashBehavior::FiniteProjectiveV2,
        1e-4,
        &[55],
    )
    .unwrap_err();
    assert_eq!(refusal.class, ProjectedFailureClass::Refused);
    assert!(refusal.cause.contains("world-clipping=true"));
    let prepared = finite(&source).draws()[0].clone().with_clipping(planes);
    assert!(RenderFrame::new(64, 64, [0.; 4], vec![prepared]).is_err());
    assert!(source.clipping().is_none());
}
#[test]
fn invalid_extent_or_error_limit_is_typed_refusal() {
    let source = segment([-1., 0., 0., 1.], [1., 0., 0., 1.]);
    for (extent, limit) in [
        ([0, 64], 1e-4),
        ([64, 0], 1e-4),
        ([64, 64], 0.),
        ([64, 64], -1.),
        ([64, 64], f64::NAN),
        ([64, 64], f64::INFINITY),
    ] {
        assert_invalid_request(&source, extent, limit);
    }
}

fn assert_invalid_request(source: &PrimitiveDraw, extent: [u32; 2], limit: f64) {
    let failure = prepare_projected_segments(
        source,
        extent,
        DashBehavior::FiniteProjectiveV2,
        limit,
        &[55],
    )
    .unwrap_err();
    assert_eq!(failure.class, ProjectedFailureClass::Refused);
    assert!(failure.segment.is_none());
}

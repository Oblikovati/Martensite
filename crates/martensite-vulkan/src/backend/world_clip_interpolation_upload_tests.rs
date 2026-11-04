use super::*;
use ash::vk;

fn clipped_phong_triangle() -> PrimitiveDraw {
    let mut draw = super::super::triangle_grid_coverage_upload::tests::phong_triangle([
        [-0.75, -0.75, 0.0],
        [0.75, -0.75, 0.0],
        [-0.75, 0.75, 0.0],
    ]);
    draw.clipping = Some(
        crate::RenderClipping::new(vec![vec![[0.0, 1.0, 0.0, -0.5]]], crate::identity_clip())
            .unwrap(),
    );
    draw
}

fn clipped_homogeneous_triangle(
    weights: [f32; 3],
    clipping: crate::RenderClipping,
) -> PrimitiveDraw {
    let mut draw = super::super::triangle_grid_coverage_upload::tests::phong_triangle([
        [-0.75, -0.75, 0.0],
        [0.75, -0.75, 0.0],
        [-0.75, 0.75, 0.0],
    ]);
    for (vertex, weight) in draw.vertices.iter_mut().zip(weights) {
        vertex.weight = weight;
        vertex
            .position
            .iter_mut()
            .for_each(|coordinate| *coordinate *= weight);
    }
    draw.clipping = Some(clipping);
    draw
}

fn coverage() -> crate::RenderTriangleCoverageBehavior {
    crate::RenderTriangleCoverageBehavior::FixedGrid12V2 {
        origin: crate::RenderTriangleGridOrigin::TopDown,
    }
}

fn empty_pruning_coverage() -> crate::RenderTriangleCoverageBehavior {
    crate::RenderTriangleCoverageBehavior::FixedGrid12V3 {
        origin: crate::RenderTriangleGridOrigin::TopDown,
    }
}

#[test]
fn selected_grid12_clip_policy_requires_its_exact_triangle_contract() {
    let draw = clipped_phong_triangle();
    let accepted = admit(
        Behavior::ProjectedGrid12V1,
        crate::RenderWorldClippingBehavior::FragmentV1,
        coverage(),
        crate::RenderPhongInterpolationBehavior::UnsnappedV2,
        &draw,
    )
    .unwrap();
    assert!(accepted);
    let triangles = super::super::triangle_grid_coverage_upload::prepare(
        coverage(),
        crate::RenderPhongInterpolationBehavior::UnsnappedV2,
        &draw,
        [0.0, 409.0, 409.0, -409.0],
        vk::SampleCountFlags::TYPE_1,
        8,
        true,
        false,
    )
    .unwrap();
    assert!(triangles.is_none());
}

#[test]
fn selected_grid12_clip_policy_refuses_missing_raster_and_interpolation_contracts() {
    let draw = clipped_phong_triangle();
    for (world, coverage, phong) in [
        (
            crate::RenderWorldClippingBehavior::RasterClipDistanceV2,
            coverage(),
            crate::RenderPhongInterpolationBehavior::UnsnappedV2,
        ),
        (
            crate::RenderWorldClippingBehavior::FragmentV1,
            crate::RenderTriangleCoverageBehavior::HardwareV1,
            crate::RenderPhongInterpolationBehavior::UnsnappedV2,
        ),
        (
            crate::RenderWorldClippingBehavior::FragmentV1,
            coverage(),
            crate::RenderPhongInterpolationBehavior::HardwareV1,
        ),
    ] {
        let error = admit(Behavior::ProjectedGrid12V1, world, coverage, phong, &draw).unwrap_err();
        assert!(error.world_clip_interpolation_failure().is_some());
    }
}

#[test]
fn grid12_clipping_composes_with_only_certified_empty_v3_carriers() {
    for clips in ZERO_AREA_WORLD_CLIP_INPUTS {
        assert_private_empty_carrier(clips);
        assert_projected_v3_private_empty_carrier(clips);
    }
}

const ZERO_AREA_WORLD_CLIP_INPUTS: [[[u32; 4]; 3]; 2] = [
    [
        [635_944_289, 1_059_254_569, 1_062_556_938, 1_065_353_216],
        [594_518_956, 1_059_254_569, 1_056_964_517, 1_065_353_216],
        [1_024_912_442, 1_059_254_569, 1_056_709_635, 1_065_353_216],
    ],
    [
        [3_197_059_925, 3_193_726_871, 3_212_836_794, 1_065_353_216],
        [3_197_059_925, 1_046_243_223, 1_065_353_144, 1_065_353_216],
        [3_197_059_925, 3_193_726_871, 1_065_353_144, 1_065_353_216],
    ],
];

fn assert_private_empty_carrier(clips: [[u32; 4]; 3]) {
    let original = triangle_from_clip_bits(clips);
    assert!(admit_grid12_world_clip(&original));
    let carrier = prepare_empty_carrier(&original);
    assert_eq!(carrier.indices, [0, 0, 0]);
    assert_eq!(original.indices, [0, 1, 2]);
    assert!(
        super::super::world_clip_interpolation_upload::selects_grid12_fragment(
            &original,
            Behavior::ProjectedGrid12V1,
        )
    );
}

fn admit_grid12_world_clip(draw: &PrimitiveDraw) -> bool {
    admit(
        Behavior::ProjectedGrid12V1,
        crate::RenderWorldClippingBehavior::FragmentV1,
        empty_pruning_coverage(),
        crate::RenderPhongInterpolationBehavior::UnsnappedV2,
        draw,
    )
    .unwrap()
}

fn prepare_empty_carrier(draw: &PrimitiveDraw) -> PrimitiveDraw {
    super::super::triangle_grid_coverage_upload::prepare(
        empty_pruning_coverage(),
        crate::RenderPhongInterpolationBehavior::UnsnappedV2,
        draw,
        [0.0, 410.0, 409.0, -409.0],
        vk::SampleCountFlags::TYPE_1,
        8,
        true,
        false,
    )
    .unwrap()
    .unwrap()
}

#[test]
fn grid12_clipping_does_not_prune_a_degenerate_line_through_sample_centers() {
    let mut draw = clipped_phong_triangle();
    for (vertex, y) in draw.vertices.iter_mut().zip([-0.5_f32, 0.0_f32, 0.5_f32]) {
        vertex.position = [0.0, y, 0.0];
    }
    let error = super::super::triangle_grid_coverage_upload::prepare(
        empty_pruning_coverage(),
        crate::RenderPhongInterpolationBehavior::UnsnappedV2,
        &draw,
        [0.0, 410.0, 409.0, -409.0],
        vk::SampleCountFlags::TYPE_1,
        8,
        true,
        false,
    )
    .unwrap_err();
    assert!(matches!(
        error.triangle_coverage_failure(),
        Some(crate::RenderTriangleCoverageFailure::RasterGridDegenerate { .. })
    ));
}

fn triangle_from_clip_bits(bits: [[u32; 4]; 3]) -> PrimitiveDraw {
    let mut draw = super::super::triangle_grid_coverage_upload::tests::phong_triangle([
        [0.0; 3], [0.0; 3], [0.0; 3],
    ]);
    for (vertex, clip) in draw.vertices.iter_mut().zip(bits) {
        vertex.position = [
            f32::from_bits(clip[0]),
            f32::from_bits(clip[1]),
            f32::from_bits(clip[2]),
        ];
        vertex.weight = f32::from_bits(clip[3]);
    }
    draw.clipping = Some(
        crate::RenderClipping::new(vec![vec![[0.0, 0.0, 1.0, -0.5]]], crate::identity_clip())
            .unwrap(),
    );
    draw
}

#[test]
fn legacy_fragment_interpolation_remains_a_noop_with_existing_inputs() {
    let draw = clipped_phong_triangle();
    assert!(
        !admit(
            Behavior::HardwareV1,
            crate::RenderWorldClippingBehavior::RasterClipDistanceV3,
            crate::RenderTriangleCoverageBehavior::HardwareV1,
            crate::RenderPhongInterpolationBehavior::HardwareV1,
            &draw,
        )
        .unwrap()
    );
}

#[test]
fn selected_grid12_clip_policy_refuses_unsupported_triangle_draws() {
    let mut draw = clipped_phong_triangle();
    draw.lighting = None;
    let error = admit(
        Behavior::ProjectedGrid12V1,
        crate::RenderWorldClippingBehavior::FragmentV1,
        coverage(),
        crate::RenderPhongInterpolationBehavior::UnsnappedV2,
        &draw,
    )
    .unwrap_err();
    assert!(matches!(
        error.world_clip_interpolation_failure(),
        Some(Failure::UnsupportedDraw { .. })
    ));
}

#[test]
fn selected_grid12_clip_policy_keeps_nontriangle_fragment_clipping_legacy() {
    let mut line = PrimitiveDraw::segments(
        vec![
            crate::RenderVertex::new([-0.5, 0.0, 0.0], [1.0; 4]),
            crate::RenderVertex::new([0.5, 0.0, 0.0], [1.0; 4]),
        ],
        vec![0, 1],
        crate::identity_clip(),
        crate::LineStyle::new(1.0, 0xffff, 1).unwrap(),
    )
    .unwrap();
    line.clipping = Some(
        crate::RenderClipping::new(vec![vec![[0.0, 1.0, 0.0, -0.5]]], crate::identity_clip())
            .unwrap(),
    );

    assert_eq!(line.topology(), crate::PrimitiveTopology::Lines);
    assert!(
        !admit(
            Behavior::ProjectedGrid12V1,
            crate::RenderWorldClippingBehavior::FragmentV1,
            coverage(),
            crate::RenderPhongInterpolationBehavior::UnsnappedV2,
            &line,
        )
        .unwrap()
    );
    assert!(!selects_grid12_fragment(&line, Behavior::ProjectedGrid12V1));
}

#[test]
fn selected_grid12_clip_policy_accepts_varying_positive_homogeneous_weights() {
    let clipping =
        crate::RenderClipping::new(vec![vec![[0.0, 1.0, 0.0, -0.5]]], crate::identity_clip())
            .unwrap();
    let draw = clipped_homogeneous_triangle([0.5, 1.0, 2.0], clipping);
    assert!(
        admit(
            Behavior::ProjectedGrid12V1,
            crate::RenderWorldClippingBehavior::FragmentV1,
            coverage(),
            crate::RenderPhongInterpolationBehavior::UnsnappedV2,
            &draw,
        )
        .unwrap()
    );
}

#[test]
fn selected_grid12_clip_policy_refuses_subnormal_homogeneous_weight() {
    let clipping =
        crate::RenderClipping::new(vec![vec![[0.0, 1.0, 0.0, -0.5]]], crate::identity_clip())
            .unwrap();
    let draw = clipped_homogeneous_triangle([f32::from_bits(1), 1.0, 1.0], clipping);
    let error = admit(
        Behavior::ProjectedGrid12V1,
        crate::RenderWorldClippingBehavior::FragmentV1,
        coverage(),
        crate::RenderPhongInterpolationBehavior::UnsnappedV2,
        &draw,
    )
    .unwrap_err();
    assert!(error.world_clip_interpolation_failure().is_some());
}

#[test]
fn selected_grid12_clip_policy_refuses_world_weight_quotients_that_can_flush_to_zero() {
    let clipping =
        crate::RenderClipping::new(vec![vec![[0.0, 1.0, 0.0, -0.5]]], crate::identity_clip())
            .unwrap();
    let mut draw = clipped_homogeneous_triangle([1.0e-20; 3], clipping);
    draw.clip_transform = [
        1.0e38, 0.0, 0.0, 0.0, 0.0, 1.0e38, 0.0, 0.0, 0.0, 0.0, 1.0e38, 0.0, 0.0, 0.0, 0.0, 1.0e38,
    ];
    let error = admit(
        Behavior::ProjectedGrid12V1,
        crate::RenderWorldClippingBehavior::FragmentV1,
        coverage(),
        crate::RenderPhongInterpolationBehavior::UnsnappedV2,
        &draw,
    )
    .unwrap_err();
    assert!(error.world_clip_interpolation_failure().is_some());
}

#[test]
fn clip_w_thresholds_use_the_full_transform_rounding_interval() {
    let mut draw = clipped_phong_triangle();
    draw.vertices[0].position = [1.0; 3];
    draw.clip_transform[7] = 16_777_216.0;
    draw.clip_transform[3] = 1.0;
    draw.clip_transform[11] = -16_777_216.0;
    draw.clip_transform[15] = 0.5;

    let error = require_normal_clip_w(&draw, 0, draw.vertices[0]).unwrap_err();

    assert!(error.world_clip_interpolation_failure().is_some());
}

#[test]
fn selected_grid12_clip_policy_refuses_nonfinite_divided_plane_domain() {
    let clipping = crate::RenderClipping::new(
        vec![vec![[f32::MAX, 0.0, 0.0, 0.0]]],
        crate::identity_clip(),
    )
    .unwrap();
    let draw = clipped_homogeneous_triangle([1.0, 1.0, 1.0], clipping);
    let error = admit(
        Behavior::ProjectedGrid12V1,
        crate::RenderWorldClippingBehavior::FragmentV1,
        coverage(),
        crate::RenderPhongInterpolationBehavior::UnsnappedV2,
        &draw,
    )
    .unwrap_err();
    assert!(error.world_clip_interpolation_failure().is_some());
}

#[test]
fn plane_domain_uses_absolute_transform_terms_and_f32_rounding_margin() {
    let model = [
        1.0e20, 0.0, 0.0, 0.0, 1.0e13, 1.0, 0.0, 0.0, -1.0e20, 0.0, 1.0, 0.0, -1.0e13, 0.0, 0.0,
        1.0,
    ];
    let clipping = crate::RenderClipping::new(vec![vec![[1.0e30, 0.0, 0.0, 0.0]]], model).unwrap();
    let mut draw = clipped_homogeneous_triangle([1.0; 3], clipping);
    draw.vertices[0].position = [1.0, 1.0, 1.0];
    let error = admit(
        Behavior::ProjectedGrid12V1,
        crate::RenderWorldClippingBehavior::FragmentV1,
        coverage(),
        crate::RenderPhongInterpolationBehavior::UnsnappedV2,
        &draw,
    )
    .unwrap_err();
    assert!(error.world_clip_interpolation_failure().is_some());
}

#[test]
fn q12_projective_weights_preserve_varying_clip_w_with_an_independent_rational_case() {
    let scale = 4096_i64;
    let a = [0, 0];
    let b = [4 * scale, 0];
    let c = [0, 4 * scale];
    let sample = [scale, 2 * scale];
    let edge = |start: [i64; 2], end: [i64; 2], point: [i64; 2]| {
        (end[0] - start[0]) * (point[1] - start[1]) - (end[1] - start[1]) * (point[0] - start[0])
    };
    let area = edge(a, b, c) as f32;
    let barycentric = [edge(b, c, sample), edge(c, a, sample), edge(a, b, sample)]
        .map(|value| value as f32 / area);
    let clip_w = [1.0_f32, 2.0, 4.0];
    let reciprocal_w = clip_w.map(|value| 1.0 / value);
    let total = (barycentric[0] * reciprocal_w[0] + barycentric[1] * reciprocal_w[1])
        + barycentric[2] * reciprocal_w[2];
    let world_over_clip_w = [1.0_f32 / clip_w[0], 0.0, 0.0];
    let world_y = ((barycentric[0] * world_over_clip_w[0] + barycentric[1] * world_over_clip_w[1])
        + barycentric[2] * world_over_clip_w[2])
        / total;
    assert_eq!(world_y, 0.5);
    let reversed_barycentric = [
        edge(c, b, sample) as f32 / edge(a, c, b) as f32,
        edge(b, a, sample) as f32 / edge(a, c, b) as f32,
        edge(a, c, sample) as f32 / edge(a, c, b) as f32,
    ];
    assert_eq!(
        reversed_barycentric,
        [barycentric[0], barycentric[2], barycentric[1]]
    );
}

include!("world_clip_interpolation_v3_tests.rs");

include!("world_clip_interpolation_v4_tests.rs");

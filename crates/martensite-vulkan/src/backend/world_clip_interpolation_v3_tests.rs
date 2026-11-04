#[test]
fn grid12_v2_composition_allows_unclipped_unsnapped_v3_phong_coverage() {
    let draw = super::super::triangle_grid_coverage_upload::tests::phong_triangle([
        [-0.75, -0.75, 0.0],
        [0.75, -0.75, 0.0],
        [-0.75, 0.75, 0.0],
    ]);
    assert!(permits_unclipped_grid12_v3_coverage(
        &draw,
        Behavior::ProjectedGrid12V2,
        crate::RenderWorldClippingBehavior::FragmentV1,
        empty_pruning_coverage(),
        crate::RenderPhongInterpolationBehavior::UnsnappedV3,
    ));
}

#[test]
fn grid12_v2_composition_keeps_unadmitted_clipped_draws_refused() {
    let draw = clipped_phong_triangle();
    assert!(!permits_unclipped_grid12_v3_coverage(
        &draw,
        Behavior::ProjectedGrid12V2,
        crate::RenderWorldClippingBehavior::FragmentV1,
        empty_pruning_coverage(),
        crate::RenderPhongInterpolationBehavior::UnsnappedV3,
    ));
}

#[test]
fn grid12_v3_coverage_prepares_unclipped_unsnapped_v3_triangles() {
    let draw = super::super::triangle_grid_coverage_upload::tests::phong_triangle([
        [-0.75, -0.75, 0.0],
        [0.75, -0.75, 0.0],
        [-0.75, 0.75, 0.0],
    ]);
    let allow_unclipped_v3 = permits_unclipped_grid12_v3_coverage(
        &draw,
        Behavior::ProjectedGrid12V2,
        crate::RenderWorldClippingBehavior::FragmentV1,
        empty_pruning_coverage(),
        crate::RenderPhongInterpolationBehavior::UnsnappedV3,
    );
    let prepared = super::super::triangle_grid_coverage_upload::prepare(
        empty_pruning_coverage(),
        crate::RenderPhongInterpolationBehavior::UnsnappedV3,
        &draw,
        [0.0, 0.0, 4.0, 4.0],
        vk::SampleCountFlags::TYPE_1,
        8,
        false,
        allow_unclipped_v3,
    )
    .unwrap();
    assert!(prepared.is_none());
}

#[test]
fn grid12_v3_coverage_accepts_an_empty_clip_state_without_world_planes() {
    let mut draw = super::super::triangle_grid_coverage_upload::tests::phong_triangle([
        [-0.75, -0.75, 0.0],
        [0.75, -0.75, 0.0],
        [-0.75, 0.75, 0.0],
    ]);
    draw.clipping = Some(crate::RenderClipping::new(Vec::new(), crate::identity_clip()).unwrap());
    let allow_unclipped_v3 = permits_unclipped_grid12_v3_coverage(
        &draw,
        Behavior::ProjectedGrid12V2,
        crate::RenderWorldClippingBehavior::FragmentV1,
        empty_pruning_coverage(),
        crate::RenderPhongInterpolationBehavior::UnsnappedV3,
    );
    assert!(allow_unclipped_v3);
    assert!(!super::has_world_planes(&draw));
    assert!(
        super::super::triangle_grid_coverage_upload::prepare(
            empty_pruning_coverage(),
            crate::RenderPhongInterpolationBehavior::UnsnappedV3,
            &draw,
            [0.0, 0.0, 4.0, 4.0],
            vk::SampleCountFlags::TYPE_1,
            8,
            false,
            allow_unclipped_v3,
        )
        .is_ok()
    );
}

#[test]
fn grid12_v3_coverage_still_refuses_unselected_clipped_triangles() {
    let draw = clipped_phong_triangle();
    let error = super::super::triangle_grid_coverage_upload::prepare(
        empty_pruning_coverage(),
        crate::RenderPhongInterpolationBehavior::UnsnappedV3,
        &draw,
        [0.0, 0.0, 4.0, 4.0],
        vk::SampleCountFlags::TYPE_1,
        8,
        false,
        false,
    )
    .unwrap_err();
    assert!(matches!(
        error.triangle_coverage_failure(),
        Some(crate::RenderTriangleCoverageFailure::UnsupportedInterface { .. })
    ));
}

#[test]
fn projected_v3_composition_refuses_mismatched_policy_versions() {
    let draw = triangle_from_clip_bits(ZERO_AREA_WORLD_CLIP_INPUTS[0]);
    for (behavior, coverage, phong) in [
        (
            Behavior::ProjectedGrid12V1,
            empty_pruning_coverage(),
            crate::RenderPhongInterpolationBehavior::UnsnappedV3,
        ),
        (
            Behavior::ProjectedGrid12V2,
            coverage(),
            crate::RenderPhongInterpolationBehavior::UnsnappedV3,
        ),
        (
            Behavior::ProjectedGrid12V2,
            empty_pruning_coverage(),
            crate::RenderPhongInterpolationBehavior::UnsnappedV2,
        ),
    ] {
        assert!(
            admit(
                behavior,
                crate::RenderWorldClippingBehavior::FragmentV1,
                coverage,
                phong,
                &draw,
            )
            .is_err()
        );
    }
}

fn assert_projected_v3_private_empty_carrier(clips: [[u32; 4]; 3]) {
    let original = triangle_from_clip_bits(clips);
    assert!(admit_projected_v3_world_clip(&original));
    let carrier = super::super::triangle_grid_coverage_upload::prepare(
        empty_pruning_coverage(),
        crate::RenderPhongInterpolationBehavior::UnsnappedV3,
        &original,
        [0.0, 410.0, 409.0, -409.0],
        vk::SampleCountFlags::TYPE_1,
        8,
        true,
        false,
    )
    .unwrap()
    .unwrap();
    assert_eq!(carrier.indices, [0, 0, 0]);
    assert_eq!(original.indices, [0, 1, 2]);
    assert!(selects_grid12_fragment(
        &original,
        Behavior::ProjectedGrid12V2
    ));
}

fn admit_projected_v3_world_clip(draw: &PrimitiveDraw) -> bool {
    admit(
        Behavior::ProjectedGrid12V2,
        crate::RenderWorldClippingBehavior::FragmentV1,
        empty_pruning_coverage(),
        crate::RenderPhongInterpolationBehavior::UnsnappedV3,
        draw,
    )
    .unwrap()
}

#[test]
fn projected_v3_keeps_degenerate_lines_through_sample_centers_refused() {
    let mut draw = clipped_phong_triangle();
    for (vertex, y) in draw.vertices.iter_mut().zip([-0.5_f32, 0.0_f32, 0.5_f32]) {
        vertex.position = [0.0, y, 0.0];
    }
    let error = super::super::triangle_grid_coverage_upload::prepare(
        empty_pruning_coverage(),
        crate::RenderPhongInterpolationBehavior::UnsnappedV3,
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

fn native_grid_v4() -> crate::RenderTriangleCoverageBehavior {
    crate::RenderTriangleCoverageBehavior::NativeGrid12V4 {
        origin: crate::RenderTriangleGridOrigin::TopDown,
    }
}

#[test]
fn projected_grid_v2_clipping_composes_with_native_grid_v4_only_on_unsnapped_v3() {
    let draw = clipped_phong_triangle();
    let selected = admit(
        Behavior::ProjectedGrid12V2,
        crate::RenderWorldClippingBehavior::FragmentV1,
        native_grid_v4(),
        crate::RenderPhongInterpolationBehavior::UnsnappedV3,
        &draw,
    )
    .unwrap();
    assert!(selected);

    let prepared = super::super::triangle_grid_coverage_upload::prepare(
        native_grid_v4(),
        crate::RenderPhongInterpolationBehavior::UnsnappedV3,
        &draw,
        [0.0, 410.0, 409.0, -409.0],
        vk::SampleCountFlags::TYPE_1,
        8,
        true,
        false,
    )
    .unwrap();
    assert!(prepared.is_none());
}

#[test]
fn world_clip_runtime_and_draw_admission_share_the_same_v4_composition_predicate() {
    let supported = super::supports_composition(
        Behavior::ProjectedGrid12V2,
        crate::RenderWorldClippingBehavior::FragmentV1,
        native_grid_v4(),
        crate::RenderPhongInterpolationBehavior::UnsnappedV3,
    );
    assert!(supported);
    assert!(!super::supports_composition(
        Behavior::ProjectedGrid12V2,
        crate::RenderWorldClippingBehavior::RasterClipDistanceV2,
        native_grid_v4(),
        crate::RenderPhongInterpolationBehavior::UnsnappedV3,
    ));
    assert!(!super::supports_composition(
        Behavior::ProjectedGrid12V2,
        crate::RenderWorldClippingBehavior::FragmentV1,
        native_grid_v4(),
        crate::RenderPhongInterpolationBehavior::UnsnappedV2,
    ));
}

#[test]
fn projected_grid_v2_clipping_refuses_other_native_grid_v4_compositions() {
    let draw = clipped_phong_triangle();
    for (coverage, phong) in [
        (
            native_grid_v4(),
            crate::RenderPhongInterpolationBehavior::UnsnappedV2,
        ),
        (
            crate::RenderTriangleCoverageBehavior::FixedGrid12V3 {
                origin: crate::RenderTriangleGridOrigin::TopDown,
            },
            crate::RenderPhongInterpolationBehavior::UnsnappedV2,
        ),
    ] {
        assert!(
            admit(
                Behavior::ProjectedGrid12V2,
                crate::RenderWorldClippingBehavior::FragmentV1,
                coverage,
                phong,
                &draw,
            )
            .is_err()
        );
    }
}

#[test]
fn native_grid_v4_keeps_empty_clip_hatch_pipeline_with_projected_axis_modules() {
    let mut draw = clipped_phong_triangle();
    draw.clipping = Some(crate::RenderClipping::new(vec![], crate::identity_clip()).unwrap());
    assert!(super::permits_unclipped_grid12_v3_coverage(
        &draw,
        Behavior::ProjectedGrid12V2,
        crate::RenderWorldClippingBehavior::FragmentV1,
        native_grid_v4(),
        crate::RenderPhongInterpolationBehavior::UnsnappedV3,
    ));
    let prepared = super::super::triangle_grid_coverage_upload::prepare(
        native_grid_v4(),
        crate::RenderPhongInterpolationBehavior::UnsnappedV3,
        &draw,
        [0.0, 410.0, 409.0, -409.0],
        vk::SampleCountFlags::TYPE_1,
        8,
        false,
        true,
    )
    .unwrap();
    assert!(prepared.is_none());
}

#[path = "triangle_grid_empty_fixtures.rs"]
mod fixtures;
use super::super::tests::phong_triangle;
use super::*;

fn behavior(origin: crate::RenderTriangleGridOrigin) -> Behavior {
    Behavior::FixedGrid12V3 { origin }
}
fn microtriangle() -> PrimitiveDraw {
    let bits = fixtures::EMPTY_CLIP_TRIANGLES[0];
    phong_triangle(bits.map(|row| row.map(f32::from_bits)))
}
fn v3_domain_empty_triangle() -> PrimitiveDraw {
    let bits = fixtures::V3_DOMAIN_EMPTY_TRIANGLE;
    phong_triangle(bits.map(|row| row.map(f32::from_bits)))
}
fn prepare_test(
    draw: &PrimitiveDraw,
    origin: crate::RenderTriangleGridOrigin,
) -> Result<Option<PrimitiveDraw>, RenderError> {
    prepare(
        behavior(origin),
        RenderPhongInterpolationBehavior::UnsnappedV2,
        draw,
        [0.0, 410.0, 409.0, -409.0],
        vk::SampleCountFlags::TYPE_1,
        8,
        false,
        false,
    )
}

#[test]
fn actual_1495_empty_refusal_gets_only_private_repeated_index_carrier() {
    let original = microtriangle();
    for origin in [
        crate::RenderTriangleGridOrigin::TopDown,
        crate::RenderTriangleGridOrigin::BottomUp,
    ] {
        let selected = prepare_test(&original, origin).unwrap().unwrap();
        assert_eq!(selected.indices, [0, 0, 0]);
        assert_eq!(selected.vertices, original.vertices);
        assert_eq!(original.indices, [0, 1, 2]);
        assert_eq!(selected.clip_transform, original.clip_transform);
    }
}

#[test]
fn certified_empty_carrier_precedes_v3_phong_domain_admission() {
    let original = v3_domain_empty_triangle();
    let viewport = [0.0, 410.0, 409.0, -409.0];
    assert!(
        super::super::super::phong_interpolation_upload::admit_with_world_clip(
            RenderPhongInterpolationBehavior::UnsnappedV3,
            &original,
            viewport,
            false,
        )
        .is_err()
    );
    let allow_unclipped_v3 =
        super::super::super::world_clip_interpolation_upload::permits_unclipped_grid12_v3_coverage(
            &original,
            crate::RenderWorldClipInterpolationBehavior::ProjectedGrid12V2,
            crate::RenderWorldClippingBehavior::FragmentV1,
            behavior(crate::RenderTriangleGridOrigin::TopDown),
            RenderPhongInterpolationBehavior::UnsnappedV3,
        );
    assert!(allow_unclipped_v3);
    let selected = prepare(
        behavior(crate::RenderTriangleGridOrigin::TopDown),
        RenderPhongInterpolationBehavior::UnsnappedV3,
        &original,
        viewport,
        vk::SampleCountFlags::TYPE_1,
        8,
        false,
        allow_unclipped_v3,
    )
    .unwrap()
    .unwrap();
    assert_eq!(selected.indices, [0, 0, 0]);
    assert_eq!(original.indices, [0, 1, 2]);
    assert_eq!(selected.vertices, original.vertices);
    assert!(
        super::super::super::phong_interpolation_upload::admit_with_world_clip(
            RenderPhongInterpolationBehavior::UnsnappedV3,
            &selected,
            viewport,
            false,
        )
        .is_ok()
    );
}

#[test]
fn prior_v2_refusal_and_following_visible_primitive_ordinal_are_preserved() {
    let mut draw = microtriangle();
    let following = phong_triangle([[-0.5, -0.5, 0.0], [0.5, -0.5, 0.0], [0.0, 0.5, 0.0]]);
    draw.vertices.extend(following.vertices);
    draw.lighting
        .as_mut()
        .unwrap()
        .normals
        .extend([[0.0, 0.0, 1.0]; 3]);
    draw.indices.extend([3, 4, 5]);
    assert!(
        admit(
            Behavior::FixedGrid12V2 {
                origin: crate::RenderTriangleGridOrigin::TopDown
            },
            RenderPhongInterpolationBehavior::UnsnappedV2,
            &draw,
            [0.0, 410.0, 409.0, -409.0],
            vk::SampleCountFlags::TYPE_1,
            8
        )
        .is_err()
    );
    let selected = prepare_test(&draw, crate::RenderTriangleGridOrigin::TopDown)
        .unwrap()
        .unwrap();
    assert_eq!(selected.indices, [0, 0, 0, 3, 4, 5]);
    assert_eq!(selected.vertices.len(), 6);
}

#[test]
fn domain_and_msaa_refusals_are_not_pruned() {
    let draw = microtriangle();
    let error = prepare(
        behavior(crate::RenderTriangleGridOrigin::TopDown),
        RenderPhongInterpolationBehavior::UnsnappedV2,
        &draw,
        [0.0, 410.0, 409.0, -409.0],
        vk::SampleCountFlags::TYPE_4,
        8,
        false,
        false,
    )
    .unwrap_err();
    assert!(matches!(
        error.triangle_coverage_failure(),
        Some(Failure::UnsupportedInterface { .. })
    ));
    let mut invalid = draw.clone();
    invalid.vertices[0].position[0] = f32::NAN;
    assert!(prepare_test(&invalid, crate::RenderTriangleGridOrigin::TopDown).is_err());
    invalid.vertices[0].position[0] = 2.0;
    assert!(prepare_test(&invalid, crate::RenderTriangleGridOrigin::TopDown).is_err());
}

#[test]
fn auxiliary_and_depth_passes_keep_refusals() {
    let mut draw = microtriangle();
    draw.stencil_rect = true;
    assert!(prepare_test(&draw, crate::RenderTriangleGridOrigin::TopDown).is_err());
    draw.stencil_rect = false;
    draw.depth_pass = crate::depth_prepass::RenderDepthPass::DepthOnly;
    // A depth-only pass is outside the coverage interface, so it stays on the device rasterizer.
    assert_eq!(
        prepare_test(&draw, crate::RenderTriangleGridOrigin::TopDown).unwrap(),
        None
    );
}

#[test]
fn coverage_discard_precedes_any_fragment_auxiliary_effect() {
    let source = include_str!("../../shaders/lighting.frag");
    let body = source.split("void main() {").nth(1).unwrap();
    let coverage = body.find("if (!triangleGridCoveredV2()) discard;").unwrap();
    let logical_cull = body.find("if (triangleGridCulledV4()) discard;").unwrap();
    assert!(logical_cull < coverage);
    for effect in [
        "captureFacetNormal();",
        "oitEarlyReturn()",
        "discardClippedFragment();",
    ] {
        assert!(coverage < body.find(effect).unwrap(), "{effect}");
    }
    assert!(
        coverage
            < body
                .find("gl_FragDepth = triangleGridLogicalDepthV4();")
                .unwrap()
    );
    assert!(!source.contains("early_fragment_tests"));
}

#[test]
fn potentially_covered_refusal_near_pixel_center_is_not_collapsed() {
    let mut draw = microtriangle();
    for vertex in &mut draw.vertices {
        vertex.position[0] += 0.45 / 204.5;
    }
    let error = prepare_test(&draw, crate::RenderTriangleGridOrigin::TopDown).unwrap_err();
    assert!(matches!(
        error.triangle_coverage_failure(),
        Some(Failure::RasterGridDegenerate { .. })
    ));
}

#[test]
fn all_three_actual_empty_refusals_keep_both_origin_contracts() {
    for bits in fixtures::EMPTY_CLIP_TRIANGLES {
        let draw = phong_triangle(bits.map(|p| p.map(f32::from_bits)));
        for origin in [
            crate::RenderTriangleGridOrigin::TopDown,
            crate::RenderTriangleGridOrigin::BottomUp,
        ] {
            assert!(prepare_test(&draw, origin).unwrap().is_some());
        }
    }
}

#[test]
fn downstream_quad_and_fixed_point_reencoding_refuse_private_carriers() {
    let mut draw = microtriangle();
    draw.quad_corners = vec![[0, 1, 2, 0]];
    assert!(prepare_test(&draw, crate::RenderTriangleGridOrigin::TopDown).is_err());
    draw.quad_corners.clear();
    draw.fixed_grid_point = true;
    assert!(prepare_test(&draw, crate::RenderTriangleGridOrigin::TopDown).is_err());
}

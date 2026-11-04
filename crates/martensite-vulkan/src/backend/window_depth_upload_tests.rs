use super::*;
use crate::{RenderVertex, identity_clip};
fn triangle() -> PrimitiveDraw {
    PrimitiveDraw::new(
        vec![RenderVertex::new([0.; 3], [1.; 4]); 3],
        vec![0, 1, 2],
        identity_clip(),
    )
    .unwrap()
}
fn parameters() -> Parameters {
    let behavior = Behavior::UnsnappedWindowDepthV2 {
        reference_grid: crate::RenderViewportGridPrecision::Fractional12,
        raster_grid: crate::RenderViewportGridPrecision::Fractional8,
    };
    Parameters::selected(
        behavior,
        vk::Viewport {
            x: 0.,
            y: 0.,
            width: 409.,
            height: 409.,
            min_depth: 0.,
            max_depth: 1.,
        },
        false,
    )
    .unwrap()
    .unwrap()
}
#[test]
fn semantic_lines_and_default_replay_do_not_select_private_triangle_depth() {
    let triangle = triangle();
    assert!(!enabled(Behavior::HardwareV1, &triangle));
    assert!(enabled(
        Behavior::UnsnappedWindowDepthV2 {
            reference_grid: crate::RenderViewportGridPrecision::Fractional12,
            raster_grid: crate::RenderViewportGridPrecision::Fractional8
        },
        &triangle
    ));
    let line = PrimitiveDraw::segments(
        triangle.vertices[..2].to_vec(),
        vec![0, 1],
        identity_clip(),
        crate::LineStyle::new(1., 0xffff, 1).unwrap(),
    )
    .unwrap();
    assert!(!enabled(
        Behavior::UnsnappedWindowDepthV2 {
            reference_grid: crate::RenderViewportGridPrecision::Fractional12,
            raster_grid: crate::RenderViewportGridPrecision::Fractional8
        },
        &line
    ));
    assert!(require_physical_source(None, &triangle, &triangle.clone()).is_ok());
    assert!(require_physical_source(Some(parameters()), &triangle, &triangle).is_ok());
    assert!(require_physical_source(Some(parameters()), &triangle, &triangle.clone()).is_err());
}

#[test]
fn native_grid_v4_empty_carrier_cannot_use_joint_physical_source_depth() {
    let mut semantic = triangle();
    semantic = semantic
        .with_lighting(
            crate::RenderLighting::new(
                vec![[0.0, 0.0, 1.0]; 3],
                crate::RenderMaterial::default(),
                crate::identity_clip(),
                crate::identity_clip(),
                false,
                crate::RenderShading::Phong,
            )
            .unwrap(),
        )
        .unwrap();
    for vertex in &mut semantic.vertices {
        vertex.position[0] = -0.1;
    }
    let carrier = crate::backend::triangle_grid_coverage_upload::prepare(
        crate::RenderTriangleCoverageBehavior::NativeGrid12V4 {
            origin: crate::RenderTriangleGridOrigin::TopDown,
        },
        crate::RenderPhongInterpolationBehavior::UnsnappedV3,
        &semantic,
        [0.0, 409.0, 409.0, 409.0],
        vk::SampleCountFlags::TYPE_1,
        8,
        false,
        true,
    )
    .unwrap()
    .unwrap();
    assert_eq!(carrier.indices, [0, 0, 0]);
    assert!(require_physical_source(Some(parameters()), &semantic, &carrier).is_err());
}
#[test]
fn actual_pipeline_cannot_drop_admitted_viewport_or_depth_selection() {
    let mut kind = PipelineVariant::from_index(0, false);
    assert!(require_pipeline(None, kind).is_ok());
    assert!(require_pipeline(Some(parameters()), kind).is_err());
    kind.window_depth = Some(parameters());
    assert!(require_pipeline(Some(parameters()), kind).is_ok());
    assert!(require_pipeline(None, kind).is_err());
}
#[test]
fn every_closure_boundary_is_preserved_as_a_typed_refusal() {
    for (actual, expected) in [
        (
            WindowDepthClosureFailure::OriginalClip,
            Boundary::OriginalClip,
        ),
        (
            WindowDepthClosureFailure::WindowRange,
            Boundary::WindowRange,
        ),
        (
            WindowDepthClosureFailure::ReferenceGrid,
            Boundary::ReferenceGrid,
        ),
        (
            WindowDepthClosureFailure::Arithmetic,
            Boundary::ShaderArithmetic,
        ),
        (
            WindowDepthClosureFailure::CorrectedClip,
            Boundary::CorrectedClip,
        ),
    ] {
        assert_eq!(boundary(actual), expected);
    }
}

#[test]
fn structural_empty_requires_bit_identical_position_and_weight_not_nearness_or_indices() {
    let mut draw = triangle();
    assert!(identical_source_pair(&draw, [0, 1, 2]));
    draw.vertices[0].position[0] = 1.;
    draw.vertices[1].position[0] = 1_f32.next_up();
    assert!(!identical_source_pair(&draw, [0, 1, 2]));
    draw.vertices[2] = draw.vertices[0];
    draw.vertices[2].weight = 2.;
    assert!(!identical_source_pair(&draw, [0, 1, 2]));
    draw.vertices[2].weight = draw.vertices[0].weight;
    assert!(identical_source_pair(&draw, [0, 1, 2]));
}

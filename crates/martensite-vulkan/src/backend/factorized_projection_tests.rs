use super::*;
fn triangle() -> PrimitiveDraw {
    let vertices = [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]]
        .map(|position| crate::RenderVertex::new(position, [1.; 4]))
        .to_vec();
    PrimitiveDraw::new(vertices, vec![0, 1, 2], crate::identity_clip()).unwrap()
}
fn factors() -> crate::RenderProjectionFactors {
    let mut model = crate::identity_clip();
    model[12] = 16_777_216.;
    let mut view = crate::identity_clip();
    view[12] = -16_777_216.;
    crate::RenderProjectionFactors::new(crate::identity_clip(), view, model).unwrap()
}
#[test]
fn triangle_without_factors_keeps_the_precomposed_route() {
    let source = triangle();
    assert!(admit(Behavior::PrecomposedV1, &source).is_ok());
    assert!(admit(Behavior::FactorizedTrianglesV2, &source).is_ok());
    assert!(!enabled(Behavior::FactorizedTrianglesV2, &source));
    let selected = source.clone().with_projection_factors(factors());
    assert!(admit(Behavior::FactorizedTrianglesV2, &selected).is_ok());
    assert_eq!(selected.clip_transform(), source.clip_transform());
    assert_eq!(selected.projection_factors(), Some(&factors()));
}
#[test]
fn transport_keeps_camera_and_model_rounding_boundaries_separate() {
    let draw = triangle().with_projection_factors(factors());
    let bytes = transform(Behavior::FactorizedTrianglesV2, &draw, vec![17; 64]);
    let expected: Vec<u8> = factors()
        .camera_product()
        .into_iter()
        .chain(factors().model())
        .flat_map(f32::to_ne_bytes)
        .collect();
    assert_eq!(bytes, expected);
    assert_eq!(bytes.len(), 128);
    assert_eq!(
        transform(Behavior::PrecomposedV1, &draw, vec![17; 64]),
        vec![17; 64]
    );
}
#[test]
fn nontriangle_widgets_preserve_prior_transport_and_shader() {
    let mut draw = triangle();
    draw.line_style = Some(crate::LineStyle::new(1., 0xffff, 1).unwrap());
    assert!(!enabled(Behavior::FactorizedTrianglesV2, &draw));
    assert!(admit(Behavior::FactorizedTrianglesV2, &draw).is_ok());
    assert_eq!(
        transform(Behavior::FactorizedTrianglesV2, &draw, vec![19; 80]),
        vec![19; 80]
    );
    let kind = super::super::pipeline_variants::special_variant(&draw, false);
    assert_eq!(
        vertex(Behavior::FactorizedTrianglesV2, kind, VERTICES[0].0).unwrap(),
        VERTICES[0].0
    );
}
#[test]
fn unsupported_triangle_routes_keep_the_precomposed_transform() {
    let mut draw = triangle().with_projection_factors(factors());
    draw.surface_cutoff = Some(0.5);
    assert!(admit(Behavior::FactorizedTrianglesV2, &draw).is_ok());
    assert!(!enabled(Behavior::FactorizedTrianglesV2, &draw));
    assert!(admit(Behavior::PrecomposedV1, &draw).is_ok());
}
#[test]
fn finite_operands_with_overflowing_camera_product_refuse_only_selected_version() {
    let mut projection = crate::identity_clip();
    projection[0] = f32::MAX;
    let mut view = crate::identity_clip();
    view[0] = 2.;
    let factors =
        crate::RenderProjectionFactors::new(projection, view, crate::identity_clip()).unwrap();
    let draw = triangle().with_projection_factors(factors);
    assert!(admit(Behavior::PrecomposedV1, &draw).is_ok());
    assert!(admit(Behavior::FactorizedTrianglesV2, &draw).is_err());
}
#[test]
fn ordinary_triangle_in_stencil_frame_uses_the_factorized_route() {
    let draw = triangle().with_projection_factors(factors());
    let mut frame = crate::RenderFrame::new(8, 8, [0.; 4], vec![draw]).unwrap();
    frame.stencil_clear_value = 1;
    let stencil =
        super::super::needs_stencil_pipeline(&frame, crate::RenderStencilState::default());
    assert!(stencil);
    let mut actual = super::super::pipeline_variants::special_variant(&frame.draws[0], stencil);
    actual.factorized_projection = true;
    assert!(admit_pipeline(Behavior::FactorizedTrianglesV2, &frame.draws[0], actual).is_ok());
    let mut legacy = actual;
    legacy.factorized_projection = false;
    assert!(admit_pipeline(Behavior::PrecomposedV1, &frame.draws[0], legacy).is_ok());
    let mut ordinary = super::super::pipeline_variants::special_variant(&frame.draws[0], false);
    ordinary.factorized_projection = true;
    assert!(admit_pipeline(Behavior::FactorizedTrianglesV2, &frame.draws[0], ordinary).is_ok());
}
#[test]
fn module_selection_and_push_range_are_closed_and_versioned() {
    let mut kind = super::super::pipeline_variants::special_variant(&triangle(), false);
    kind.factorized_projection = true;
    let original = vk::PushConstantRange::default()
        .size(64)
        .stage_flags(vk::ShaderStageFlags::VERTEX);
    assert_eq!(push_range(Behavior::PrecomposedV1, kind, original).size, 64);
    assert_eq!(
        push_range(Behavior::FactorizedTrianglesV2, kind, original).size,
        128
    );
    for &(stock, selected) in VERTICES {
        assert_eq!(vertex(Behavior::PrecomposedV1, kind, stock).unwrap(), stock);
        assert_eq!(
            vertex(Behavior::FactorizedTrianglesV2, kind, stock).unwrap(),
            selected
        );
    }
    assert!(vertex(Behavior::FactorizedTrianglesV2, kind, b"unknown module").is_err());
}
#[test]
fn private_line_triangles_keep_semantic_line_module_and_payload() {
    let mut semantic = triangle().with_projection_factors(factors());
    semantic.line_style = Some(crate::LineStyle::new(1., 0xffff, 1).unwrap());
    let mut physical =
        semantic.with_replacement_geometry(semantic.vertices.clone(), semantic.indices.clone());
    physical.line_style = None;
    physical.clip_transform = crate::identity_clip();
    assert_eq!(physical.topology(), crate::PrimitiveTopology::Triangles);
    let kind = super::super::pipeline_variants::special_variant(&physical, false);
    assert!(!enabled(Behavior::FactorizedTrianglesV2, &semantic));
    assert!(admit_pipeline(Behavior::FactorizedTrianglesV2, &semantic, kind).is_ok());
    assert_eq!(
        vertex(Behavior::FactorizedTrianglesV2, kind, VERTICES[0].0).unwrap(),
        VERTICES[0].0
    );
    assert_eq!(
        transform(Behavior::FactorizedTrianglesV2, &semantic, vec![23; 64]),
        vec![23; 64]
    );
}
#[test]
fn blended_triangles_take_the_factorized_route_as_the_native_chain_does() {
    let blended = triangle()
        .with_projection_factors(factors())
        .with_surface_blending(true)
        .unwrap();
    assert!(enabled(Behavior::FactorizedTrianglesV2, &blended));
}
#[test]
fn raster_clipping_does_not_borrow_the_factorized_interface() {
    let draw = triangle().with_projection_factors(factors());
    let args = (
        crate::RenderPhongInterpolationBehavior::HardwareV1,
        crate::RenderTriangleCoverageBehavior::HardwareV1,
    );
    assert!(
        admit_composition(
            Behavior::FactorizedTrianglesV2,
            &draw,
            crate::RenderWorldClippingBehavior::RasterClipDistanceV2,
            args.0,
            args.1
        )
        .is_err()
    );
    assert!(
        admit_composition(
            Behavior::PrecomposedV1,
            &draw,
            crate::RenderWorldClippingBehavior::RasterClipDistanceV2,
            args.0,
            args.1
        )
        .is_ok()
    );
    let mut kind = super::super::pipeline_variants::special_variant(&draw, false);
    kind.factorized_projection = true;
    kind.world_clip_v2 = true;
    assert!(admit_pipeline(Behavior::FactorizedTrianglesV2, &draw, kind).is_err());
}

#[test]
fn fixed_function_clip_version_still_composes_with_factorized_programmable_frames() {
    let draw = triangle().with_projection_factors(factors());
    let world = crate::RenderWorldClippingBehavior::RasterFixedFunctionV4;
    assert!(enabled(Behavior::FactorizedLinesV3, &draw));
    assert!(
        admit_composition(
            Behavior::FactorizedLinesV3,
            &draw,
            world,
            crate::RenderPhongInterpolationBehavior::HardwareV1,
            crate::RenderTriangleCoverageBehavior::HardwareV1,
        )
        .is_ok()
    );
}

#[test]
fn new_world_clip_modes_cannot_bypass_factorized_vertex_admission() {
    for mode in 0..3 {
        let mut kind = super::super::pipeline_variants::special_variant(&triangle(), false);
        kind.world_clip_v2 = mode == 0;
        kind.world_clip_v3_line = mode == 1;
        kind.world_clip_grid12 = mode == 2;
        assert!(!kind_supported(kind));
        assert!(admit_vertex_composition(kind).is_ok());
        kind.factorized_projection = true;
        assert!(admit_vertex_composition(kind).is_err());
    }
}

#[test]
fn new_world_clip_modes_refuse_window_depth_before_vertex_early_returns() {
    let parameters =
        super::super::window_depth_parameters::WindowDepthPipelineParameters::selected(
            crate::RenderWindowDepthBehavior::ReferenceWindowDepthV3 {
                reference_grid: crate::RenderViewportGridPrecision::Fractional12,
                raster_grid: crate::RenderViewportGridPrecision::Fractional8,
            },
            vk::Viewport {
                width: 409.,
                height: 409.,
                max_depth: 1.,
                ..Default::default()
            },
            false,
        )
        .unwrap()
        .unwrap();
    for mode in 0..3 {
        let mut kind = super::super::pipeline_variants::special_variant(&triangle(), false);
        kind.world_clip_v2 = mode == 0;
        kind.world_clip_v3_line = mode == 1;
        kind.world_clip_grid12 = mode == 2;
        kind.window_depth = Some(parameters);
        assert!(
            admit_vertex_composition(kind)
                .unwrap_err()
                .window_depth_failure()
                .is_some()
        );
    }
}

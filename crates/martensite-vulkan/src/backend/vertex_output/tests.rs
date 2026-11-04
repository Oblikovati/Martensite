use super::*;
use crate::{PrimitiveDraw, RenderFrame, RenderVertex, VertexOutputRequest};
use std::num::NonZeroU64;
fn token() -> NonZeroU64 {
    NonZeroU64::new(173).unwrap()
}
fn triangle() -> PrimitiveDraw {
    let draw = PrimitiveDraw::new(
        vec![RenderVertex::new([0.; 3], [1.; 4]); 3],
        vec![0, 1, 2],
        crate::identity_clip(),
    )
    .unwrap();
    let lighting = crate::RenderLighting::new(
        vec![[0., 0., 1.]; 3],
        Default::default(),
        crate::identity_clip(),
        crate::identity_clip(),
        false,
        crate::RenderShading::Phong,
    )
    .unwrap();
    draw.with_lighting(lighting).unwrap()
}
fn text_triangle() -> PrimitiveDraw {
    let vertices = vec![crate::RenderVertex::new([0.; 3], [1.; 4]); 3];
    let atlas = crate::RasterImage::new([1, 1], vec![[1.; 4]]).unwrap();
    let style = crate::TexturedStyle::new(atlas, vec![[0., 0.]; 3], 0.285).unwrap();
    PrimitiveDraw::textured(vertices, vec![0, 1, 2], crate::identity_clip(), style).unwrap()
}
fn request() -> VertexOutputRequest {
    VertexOutputRequest::new(vec![0, 1, 2], 96)
        .unwrap()
        .for_source_token(token())
}
fn frame(draws: Vec<PrimitiveDraw>) -> RenderFrame {
    RenderFrame::new(1, 1, [0.; 4], draws).unwrap()
}
#[test]
fn explicit_request_missing_present_and_ambiguous_tags() {
    let source = triangle().with_vertex_output_target(token());
    assert!(validate_request_frame(None, &frame(vec![source.clone()])).is_ok());
    assert!(validate_request_frame(Some(&request()), &frame(vec![source.clone()])).is_ok());
    assert!(validate_request_frame(Some(&request()), &frame(vec![triangle()])).is_ok());
    assert!(
        validate_request_frame(Some(&request()), &frame(vec![source.clone(), source])).is_err()
    );
    let no_token = VertexOutputRequest::new(vec![0, 1, 2], 96).unwrap();
    assert!(validate_request_frame(Some(&no_token), &frame(vec![triangle()])).is_err());
}
#[test]
fn target_identity_survives_existing_clone_and_geometry_preparation() {
    let source = triangle().with_vertex_output_target(token());
    let replaced =
        source.with_replacement_geometry(source.vertices.clone(), source.indices.clone());
    assert_eq!(source.clone().vertex_output_target, Some(token()));
    assert_eq!(replaced.vertex_output_target, Some(token()));
    assert!(validate_draw(&request(), &replaced).is_ok());
}
#[test]
fn changed_indices_and_unsupported_vertex_stages_refuse() {
    let mut source = triangle();
    source.indices.swap(0, 1);
    assert!(validate_draw(&request(), &source).is_err());
    source.indices.swap(0, 1);
    source.polygon_points = true;
    assert!(validate_draw(&request(), &source).is_err());
    source.polygon_points = false;
    source.lighting = None;
    assert!(validate_draw(&request(), &source).is_err());
}
#[test]
fn only_declared_frozen_vertex_variants_are_instrumented() {
    for bytes in [
        include_bytes!("../../../shaders/lighting.vert.spv").as_slice(),
        include_bytes!("../../../shaders/numeric-max-v2.lighting.vert.spv").as_slice(),
        include_bytes!("../../../shaders/factorized-v2.lighting.vert.spv").as_slice(),
        include_bytes!("../../../shaders/factorized-v2.numeric-max-v2.lighting.vert.spv")
            .as_slice(),
    ] {
        let observed = shader_bytes(bytes).unwrap();
        assert_ne!(observed, bytes);
        assert_eq!(&observed[..4], &[3, 2, 35, 7]);
    }
    assert!(shader_bytes(include_bytes!("../../../shaders/lighting.frag.spv")).is_none());
    assert!(shader_bytes(include_bytes!("../../../shaders/lighting.clipped.vert.spv")).is_none());
}
#[test]
fn text_format_selects_only_its_matching_vertex_stage_and_keeps_fragment_stage() {
    use crate::VertexOutputFormat as Format;
    let stock = include_bytes!("../../../shaders/text.vert.spv");
    let observed = selected_for_format(stock, Format::TextClipUvId28).unwrap();
    assert_eq!(
        observed,
        include_bytes!("../../../shaders/diagnostic.vertex-output.text.vert.spv")
    );
    assert!(
        selected_for_format(
            include_bytes!("../../../shaders/point.vert.spv"),
            Format::TextClipUvId28
        )
        .is_none()
    );
    let fragment = include_bytes!("../../../shaders/text.frag.spv");
    assert_eq!(
        fragment_for_format(fragment, Format::TextClipUvId28),
        Some(include_bytes!("../../../shaders/diagnostic.vertex-output.text.frag.spv").as_slice())
    );
    assert!(
        fragment_for_format(
            include_bytes!("../../../shaders/lighting.frag.spv"),
            Format::TextClipUvId28
        )
        .is_none()
    );
}
#[test]
fn text_request_admits_only_original_indexed_text_draws() {
    let request = VertexOutputRequest::text(vec![0, 1, 2], 84)
        .unwrap()
        .for_source_token(token());
    let text = text_triangle().with_vertex_output_target(token());
    assert!(validate_request_frame(Some(&request), &frame(vec![text.clone()])).is_ok());
    assert!(
        validate_request_frame(
            Some(&request),
            &frame(vec![triangle().with_vertex_output_target(token())])
        )
        .is_err()
    );
    assert!(validate_request_frame(Some(&request), &frame(vec![text.clone(), text])).is_err());
}
#[test]
fn published_contracts_and_capture_owners_are_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<crate::VertexOutputRequest>();
    assert_send_sync::<crate::VertexOutputRecord>();
    assert_send_sync::<VertexOutputResources>();
}

#[test]
fn vertex_output_retained_phong_flat_map_is_unused_and_admitted() {
    let source = crate::vertex_output_fixture::RetainedPhongFixture::new();
    let draw = source.draw();
    assert_eq!(draw.native_flat_sources, [2]);
    assert!(!draw.fixed_function_lighting);
    assert_eq!(
        draw.lighting.as_ref().unwrap().shading(),
        crate::RenderShading::Phong
    );
    assert!(validate_draw(&request(), &draw).is_ok());
}
#[test]
fn vertex_output_nonfixed_encoder_ignores_retained_flat_sources() {
    let source = crate::vertex_output_fixture::RetainedPhongFixture::new();
    let mut draw = source.draw();
    let sources = super::super::fixed_vertex_sources::fixed_vertex_sources(&draw, &draw.indices);
    let vertices = super::super::frame::encode_vertices(&draw);
    let indices = draw.indices.clone();
    draw.native_flat_sources = vec![0];
    assert_eq!(
        super::super::fixed_vertex_sources::fixed_vertex_sources(&draw, &draw.indices),
        sources
    );
    assert_eq!(super::super::frame::encode_vertices(&draw), vertices);
    assert_eq!(draw.indices, indices);
    assert!(
        super::super::fixed_vertex_sources::fixed_expanded_indices(&draw, sources.len()).is_none()
    );
}

#[test]
fn vertex_output_fixed_lighting_remains_refused_with_retained_flat_map() {
    let source = crate::vertex_output_fixture::RetainedPhongFixture::new();
    let mut draw = source.draw();
    draw.lighting
        .as_mut()
        .unwrap()
        .prepare_fixed_lighting()
        .unwrap();
    assert!(!draw.fixed_function_lighting);
    assert!(validate_draw(&request(), &draw).is_err());
}

#[test]
fn vertex_output_fragment_interface_selection_preserves_canonical_registry_keys() {
    let stocks: [&'static [u8]; 4] = [
        include_bytes!("../../../shaders/lighting.frag.spv"),
        include_bytes!("../../../shaders/lighting.coordinates-v2.frag.spv"),
        include_bytes!("../../../shaders/numeric-max-v2.lighting.frag.spv"),
        include_bytes!("../../../shaders/numeric-max-v2.lighting.coordinates-v2.frag.spv"),
    ];
    for stock in stocks {
        let observed = fragment_bytes(stock).unwrap();
        assert_ne!(observed, stock);
        assert_eq!(&observed[..4], &[3, 2, 35, 7]);
    }
    let stock = stocks[0];
    assert_eq!(
        super::super::coordinate_shader::coordinate_fragment(stock).unwrap(),
        stocks[1]
    );
    assert!(fragment_bytes(include_bytes!("../../../shaders/lighting.clipped.frag.spv")).is_none());
}

fn unlit_plain_point() -> PrimitiveDraw {
    let mut style = crate::PointStyle::new(1., [1, 1], vec![[1.; 4]]).unwrap();
    style.color_policy = crate::point_style::PointColorPolicy::Plain;
    PrimitiveDraw::points(
        vec![RenderVertex::new([0.; 3], [1.; 4]); 2],
        vec![1, 0],
        crate::identity_clip(),
        style,
    )
    .unwrap()
}
#[test]
fn explicit_point_request_admits_only_original_unexpanded_plain_unlit_points() {
    let points_request = VertexOutputRequest::points(vec![1, 0], 40).unwrap();
    let source = unlit_plain_point();
    assert!(validate_draw(&points_request, &source).is_ok());
    let mut expanded = source.clone();
    expanded.fixed_grid_point = true;
    assert!(validate_draw(&points_request, &expanded).is_err());
    let mut sprite = source.clone();
    sprite.point_style.as_mut().unwrap().color_policy =
        crate::point_style::PointColorPolicy::AlphaSprite;
    assert!(validate_draw(&points_request, &sprite).is_err());
    let mut polygon = source.clone();
    polygon.polygon_points = true;
    assert!(validate_draw(&points_request, &polygon).is_err());
    assert!(validate_draw(&points_request, &triangle()).is_err());
    assert!(validate_draw(&request(), &source).is_err());
}
#[test]
fn point_shader_pairs_are_explicit_and_never_select_lighting_or_fixed_grid() {
    use crate::VertexOutputFormat as Format;
    let stock = include_bytes!("../../../shaders/point.vert.spv");
    assert!(selected_for_format(stock, Format::PointsClipId20).is_some());
    assert!(selected_for_format(stock, Format::TrianglesClipNormalId32).is_none());
    for source in [
        include_bytes!("../../../shaders/lighting.vert.spv").as_slice(),
        include_bytes!("../../../shaders/fixed-grid-v2.point.vert.spv").as_slice(),
    ] {
        assert!(selected_for_format(source, Format::PointsClipId20).is_none());
    }
    for fragment in [
        include_bytes!("../../../shaders/point.frag.spv").as_slice(),
        include_bytes!("../../../shaders/point.coordinates-v2.frag.spv").as_slice(),
    ] {
        assert!(fragment_for_format(fragment, Format::PointsClipId20).is_some());
    }
}

#[test]
fn vertex_output_indexed_instance_count_preserves_direct_vertex_identity() {
    assert!(require_single_instance(1).is_ok());
    assert!(require_single_instance(0).is_err());
    assert!(require_single_instance(2).is_err());
}

#[test]
fn v3_capture_registry_keeps_unsnapped_vertex_and_guarded_fragment_identity() {
    let vertex = include_bytes!("../../../shaders/unsnapped-v2.lighting.vert.spv");
    let fragment = include_bytes!("../../../shaders/unsnapped-v3.lighting.frag.spv");
    assert_eq!(
        shader_bytes(vertex),
        Some(
            include_bytes!(
                "../../../shaders/diagnostic.vertex-output.unsnapped-v2.lighting.vert.spv"
            )
            .as_slice()
        )
    );
    assert_eq!(
        fragment_bytes(fragment),
        Some(
            include_bytes!(
                "../../../shaders/diagnostic.vertex-output.unsnapped-v3.lighting.frag.spv"
            )
            .as_slice()
        )
    );
    assert!(
        fragment_bytes(include_bytes!(
            "../../../shaders/unsnapped-v2.lighting.frag.spv"
        ))
        .is_none()
    );
}

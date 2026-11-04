use super::*;
use crate::{PrimitiveDraw, RenderVertex, identity_clip};

fn owned_draw() -> PrimitiveDraw {
    PrimitiveDraw::points(
        vec![RenderVertex::new([1., -0., 3.], [0., 1., 0., 1.])],
        vec![0],
        identity_clip(),
        crate::PointStyle::new(1., [1, 1], vec![[1.; 4]]).unwrap(),
    )
    .unwrap()
}
#[test]
fn upload_snapshot_owns_every_byte_after_source_mutation() {
    let mut vertices = vec![0, 255, 128, 17];
    let indices = vec![0, 0, 0, 0];
    let snapshot = GeometryArguments::new(&owned_draw(), &vertices, &indices, 1);
    vertices.fill(0);
    assert_eq!(snapshot.vertices, [0, 255, 128, 17]);
    assert_eq!(snapshot.indices, indices);
}
#[test]
fn command_buffer_pipeline_and_buffer_identity_are_all_retained() {
    let snapshot = GeometryArguments::new(&owned_draw(), &[0, 255], &[1, 0, 0, 0], 1);
    let json = snapshot.json([11, 12, 13, 14, 15, 16], 1, 3);
    assert!(
        json.contains("\"command_ordinal_pipeline_vertex_index_layout\":[11, 12, 13, 14, 15, 16]")
    );
    assert!(json.contains("\"index_count\":1"));
    assert!(json.contains("\"instance_count\":3"));
    assert!(json.contains("\"gpu_readback\":false"));
    assert!(json.contains("\"post_vertex_observed\":false"));
}
#[test]
fn absent_texture_and_vertex_color_are_reported_without_defaults() {
    let properties = properties(&owned_draw());
    assert!(properties.contains("\"surface_image\":false"));
    assert!(properties.contains("\"surface_unavailable\":false"));
    assert!(properties.contains("\"source_color_array\":false"));
    assert!(properties.contains("\"point_size_bits\":1065353216"));
}

#[test]
fn shader_creation_retains_actual_module_and_all_source_bytes() {
    let json = shader_json(17, &[3, 2, 35, 7, 0, 255, 128, 4]);
    assert!(json.contains("\"module\":17"));
    assert!(json.contains("\"spirv_bytes\":[3, 2, 35, 7, 0, 255, 128, 4]"));
    assert!(json.contains("\"gpu_readback\":false"));
}

#[test]
fn both_pipeline_ids_are_recorded_and_different_pipeline_is_incomplete() {
    let json = modules_json([11, 12, 13, 14, 15, 16], 17, false, false, &[21, 22]);
    assert!(json.contains("\"actual_pipeline_handle\":13"));
    assert!(json.contains("\"base_pipeline_handle\":17"));
    assert!(json.contains("Err(DifferentPipeline)"));
    assert!(json.contains("\"modules_vertex_fragment_optional_geometry\":[]"));
}
#[test]
fn equal_pipeline_is_usable_only_without_custom_or_oit_routes() {
    assert_eq!(require_base_pipeline(13, 13, false, false), Ok(()));
    assert_eq!(
        require_base_pipeline(13, 13, true, false),
        Err(PipelineJoinIncomplete::CustomProgram)
    );
    assert_eq!(
        require_base_pipeline(13, 13, false, true),
        Err(PipelineJoinIncomplete::OitRoute)
    );
    let json = modules_json([11, 12, 13, 14, 15, 16], 13, false, false, &[21, 22]);
    assert!(json.contains("\"base_pipeline_join\":\"Ok(())\""));
    assert!(json.contains("\"modules_vertex_fragment_optional_geometry\":[21, 22]"));
}

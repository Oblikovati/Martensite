use super::*;
use crate::{
    RenderBackend, RenderBehaviorConfiguration, RenderFrame, RenderLighting, RenderMaterial,
    RenderMaterialCommon, RenderShading, RenderVertex, VulkanOptions, VulkanRenderer,
    identity_clip,
};

#[test]
fn packed_and_exact_paths_capture_the_same_axis_aligned_fixture_in_both_light_pipelines() {
    let mut exact = mesh_renderer(RenderMeshVertexBehavior::Float32V1);
    let mut packed = mesh_renderer(RenderMeshVertexBehavior::Quantized16OctV2);
    for count in [4, super::super::directional_phong::MIN_VERTICES] {
        let draw = full_extent_axis_lit_quad(count, false);
        let frame = RenderFrame::new(32, 24, [0., 0., 0., 1.], vec![draw]).unwrap();
        let expected = exact.render(&frame).unwrap();
        let before = packed.work_ledger().totals();
        let actual = packed.render(&frame).unwrap();
        let uploaded = packed.work_ledger().totals().since(&before);
        let index_width = if count <= 65_536 { 2 } else { 4 };
        assert_eq!(
            uploaded.geometry_uploaded_bytes,
            count as u64 * u64::from(PACKED_STRIDE) + 6 * index_width
        );
        assert!(
            actual
                .rgba
                .as_chunks::<4>()
                .0
                .iter()
                .any(|pixel| pixel[..3] != [0, 0, 0])
        );
        assert_frame_bits(&actual, &expected);

        let before = packed.work_ledger().totals();
        let warm = packed.render(&frame).unwrap();
        assert_eq!(
            packed
                .work_ledger()
                .totals()
                .since(&before)
                .geometry_uploaded_bytes,
            0
        );
        assert_frame_bits(&warm, &expected);
    }
    assert!(exact.validation_errors().unwrap().is_empty());
    assert!(packed.validation_errors().unwrap().is_empty());
}

#[test]
fn a_weighted_position_selects_the_uncompressed_vertex_path() {
    let mut packed = mesh_renderer(RenderMeshVertexBehavior::Quantized16OctV2);
    let draw = full_extent_axis_lit_quad(4, true);
    let frame = RenderFrame::new(32, 24, [0., 0., 0., 1.], vec![draw]).unwrap();
    let before = packed.work_ledger().totals();
    packed.render(&frame).unwrap();
    let uploaded = packed.work_ledger().totals().since(&before);
    let wide = u64::from(super::super::pipeline_state::LIT_VERTEX_STRIDE);
    assert_eq!(uploaded.geometry_uploaded_bytes, 4 * wide + 12);
    assert!(packed.validation_errors().unwrap().is_empty());
}

#[test]
fn interior_position_quantization_matches_the_independently_decoded_depth_plane() {
    let mut exact = mesh_renderer(RenderMeshVertexBehavior::Float32V1);
    let mut packed = mesh_renderer(RenderMeshVertexBehavior::Quantized16OctV2);
    let authored = depth_plane_fixture(0.5);
    // The midpoint of [0, 1] falls between codes. Code 32768 at a 1/65535 step
    // rounds to this binary32 distance, independent of the implementation's encoder.
    let decoded = depth_plane_fixture(f32::from_bits(0x3f00_0080));
    let stock = exact.render(&authored).unwrap();
    let reference = exact.render(&decoded).unwrap();
    let selected = packed.render(&authored).unwrap();
    let center = 12 * 32 + 16;
    let displacement = selected.depth[center] - stock.depth[center];
    assert_eq!(stock.depth[center], 0.75);
    assert!(
        displacement > 0. && displacement <= 4.0e-6,
        "{displacement}"
    );
    assert_eq!(selected.rgba, stock.rgba);
    assert_frame_bits(&selected, &reference);
    assert_frame_bits(&exact.render(&authored).unwrap(), &stock);
    assert!(exact.validation_errors().unwrap().is_empty());
    assert!(packed.validation_errors().unwrap().is_empty());
}

fn depth_plane_fixture(depth: f32) -> RenderFrame {
    let draw = full_extent_axis_lit_quad(6, false);
    let mut vertices = draw.vertices.to_vec();
    for vertex in &mut vertices[..4] {
        vertex.position[2] = depth;
    }
    vertices[4].position[2] = 0.;
    vertices[5].position[2] = 1.;
    let draw = draw.with_replacement_geometry(vertices.into(), draw.indices.clone());
    RenderFrame::new(32, 24, [0., 0., 0., 1.], vec![draw]).unwrap()
}

fn mesh_renderer(behavior: RenderMeshVertexBehavior) -> VulkanRenderer {
    let configuration = RenderBehaviorConfiguration::default().with_mesh_vertices(behavior);
    let renderer = VulkanRenderer::new_with_render_behavior(
        VulkanOptions {
            validation: true,
            device_index: Some(0),
        },
        Default::default(),
        Default::default(),
        Default::default(),
        configuration,
    )
    .unwrap();
    assert_eq!(renderer.mesh_vertex_behavior(), behavior);
    renderer
}

fn full_extent_axis_lit_quad(count: usize, weighted: bool) -> PrimitiveDraw {
    let positions = [
        [-0.75, -0.75, 0.25],
        [0.75, -0.75, 0.25],
        [0.75, 0.75, 0.25],
        [-0.75, 0.75, 0.25],
    ];
    let mut vertices = positions
        .map(|position| RenderVertex::new(position, [1.; 4]))
        .to_vec();
    if weighted {
        vertices[0].weight = 2.;
    }
    vertices.resize(count, vertices[0]);
    let draw = PrimitiveDraw::new(vertices, vec![0, 1, 2, 0, 2, 3], identity_clip()).unwrap();
    draw.with_lighting(axis_lighting(count)).unwrap()
}

fn axis_lighting(count: usize) -> RenderLighting {
    let common = RenderMaterialCommon {
        emission: [0.2, 0.3, 0.6, 1.],
        ..Default::default()
    };
    let material = RenderMaterial {
        common: [common; 2],
        ..Default::default()
    };
    let mut world = identity_clip();
    world[0] = -2.;
    world[5] = 1.5;
    world[10] = 0.5;
    RenderLighting::new(
        vec![[1., 0., 0.]; count],
        material,
        world,
        identity_clip(),
        false,
        RenderShading::Phong,
    )
    .unwrap()
}

fn assert_frame_bits(actual: &crate::FrameCapture, expected: &crate::FrameCapture) {
    assert_eq!(actual.rgba, expected.rgba);
    assert!(
        actual
            .depth
            .iter()
            .map(|value| value.to_bits())
            .eq(expected.depth.iter().map(|value| value.to_bits()))
    );
}

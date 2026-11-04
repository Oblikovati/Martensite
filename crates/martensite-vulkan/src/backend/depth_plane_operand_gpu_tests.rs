//! Diagnostic operand isolation; these observations are not an image acceptance allowance.
use super::{VulkanOptions, VulkanRenderer};
use crate::{
    PrimitiveDraw, RenderBackend, RenderBehaviorConfiguration, RenderCoordinateConvention,
    RenderDepthBiasBehavior, RenderFrame, RenderPolygonOffset, RenderVertex, identity_clip,
};

// Exact input words from the independently matched GL/Vulkan post-vertex observation.
const CLIPS: [[f32; 3]; 3] = [
    [-0.089_045_93, 0.077_637_866, -0.525_740_4],
    [-0.098_782_94, 0.102_820_1, -0.536_539_1],
    [-0.088_632_76, 0.077_642_73, -0.525_968_5],
];

#[cfg(feature = "vertex-output-diagnostics")]
#[test]
fn captured_thin_triangle_observes_identity_projection_before_depth_controls() {
    let token = std::num::NonZeroU64::MIN;
    let request = crate::VertexOutputRequest::new(vec![0, 1, 2], 96)
        .unwrap()
        .for_source_token(token);
    let mut renderer = VulkanRenderer::new_with_vertex_output_behavior(
        VulkanOptions {
            validation: true,
            device_index: Some(0),
        },
        crate::RenderRasterPolicy::LastProvokingVertexV2,
        RenderCoordinateConvention::LegacyBottomUpV1,
        Default::default(),
        RenderBehaviorConfiguration::default(),
        request.clone(),
    )
    .unwrap();
    let mut frame = operand_frame(0., 0.);
    let lighting = crate::RenderLighting::new(
        vec![[0., 0., 1.]; 3],
        Default::default(),
        identity_clip(),
        identity_clip(),
        false,
        crate::RenderShading::Phong,
    )
    .unwrap();
    frame.draws[0] = frame.draws[0]
        .clone()
        .with_lighting(lighting)
        .unwrap()
        .with_vertex_output_target(token);
    renderer.render(&frame).unwrap();
    request.finish().unwrap();
    assert!(renderer.validation_errors().unwrap().is_empty());
}

#[test]
fn captured_thin_triangle_separates_unbiased_slope_and_unit_depth() {
    for behavior in [
        RenderDepthBiasBehavior::HardwareFormatV1,
        RenderDepthBiasBehavior::Native24UnitsV2,
    ] {
        let mut renderer = operand_renderer(behavior);
        for (factor, units) in [(0., 0.), (1., 0.), (0., 1.), (1., 1.)] {
            record_operand(&mut renderer, behavior, factor, units);
        }
        assert!(renderer.validation_errors().unwrap().is_empty());
    }
}

fn operand_renderer(behavior: RenderDepthBiasBehavior) -> VulkanRenderer {
    VulkanRenderer::new_with_render_behavior(
        VulkanOptions {
            validation: true,
            device_index: Some(0),
        },
        crate::RenderRasterPolicy::LastProvokingVertexV2,
        RenderCoordinateConvention::LegacyBottomUpV1,
        Default::default(),
        RenderBehaviorConfiguration::default().with_depth_bias(behavior),
    )
    .unwrap()
}

pub(super) fn operand_frame(factor: f32, units: f32) -> RenderFrame {
    let vertices = CLIPS.map(|position| RenderVertex::new(position, [1., 0., 0., 1.]));
    let draw = PrimitiveDraw::new(vertices.to_vec(), vec![0, 1, 2], identity_clip())
        .unwrap()
        .with_polygon_offset(RenderPolygonOffset::new(1, factor, units).unwrap());
    RenderFrame::new(409, 409, [0., 0., 0., 1.], vec![draw]).unwrap()
}

fn record_operand(
    renderer: &mut VulkanRenderer,
    behavior: RenderDepthBiasBehavior,
    factor: f32,
    units: f32,
) {
    let capture = renderer.render(&operand_frame(factor, units)).unwrap();
    let offset = 186 * 409 + 185;
    assert_eq!(&capture.rgba[offset * 4..offset * 4 + 4], &[255, 0, 0, 255]);
    println!(
        "{{\"kind\":\"depth-plane-operand\",\"behavior\":\"{behavior:?}\",\"format\":\"{:?}\",\"clip_control\":{},\"factor\":{factor},\"units\":{units},\"depth\":{},\"depth_bits\":{}}}",
        renderer.runtime.depth.formats.plain,
        renderer.runtime.depth.clip_control,
        capture.depth[offset],
        capture.depth[offset].to_bits()
    );
}

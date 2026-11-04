//! Co-enabled core text and exact depth-bias features must compose without a fallback.
use super::{VulkanOptions, VulkanRenderer};
use crate::{
    FrameCapture, RenderBehaviorConfiguration, RenderCoordinateConvention,
    RenderDepthBiasBehavior as Bias, RenderError, RenderPolygonOffset,
    RenderTextInterpolationBehavior as Text,
};

fn renderer() -> Result<VulkanRenderer, RenderError> {
    let config = RenderBehaviorConfiguration::default()
        .with_text_interpolation(Text::ProjectedGrid12FlatCornersV4)
        .with_depth_bias(Bias::Native24UnitsV2);
    VulkanRenderer::new_with_render_behavior(
        VulkanOptions {
            validation: true,
            device_index: Some(0),
        },
        Default::default(),
        RenderCoordinateConvention::TopLeftLogicalV2,
        Default::default(),
        config,
    )
}

fn capture(renderer: &mut VulkanRenderer, units: f32) -> FrameCapture {
    let draw = super::text_flat_corners_gpu_tests::quad(false)
        .with_polygon_offset(RenderPolygonOffset::new(1, 0., units).unwrap());
    super::text_flat_corners_gpu_tests::capture(renderer, draw)
}

fn assert_depth(renderer: &VulkanRenderer, frame: &FrameCapture, expected: f32) {
    let stored = super::depth_bias_gpu_tests::stored(renderer, expected);
    assert!(
        frame.depth.iter().all(|&value| value == stored),
        "depth={:?}; expected stored {stored:?}",
        frame.depth
    );
}

#[test]
fn flat_corners_and_native24_keep_color_and_apply_exact_bias_together() {
    // MissingExactFloatControl must remain an explicit typed refusal on unsupported profiles.
    let mut renderer = renderer().unwrap();
    assert_eq!(
        renderer.runtime.text_interpolation,
        Text::ProjectedGrid12FlatCornersV4
    );
    assert_eq!(renderer.runtime.depth_bias_behavior, Bias::Native24UnitsV2);
    assert!(renderer.runtime.depth_bias_command.is_some());
    let baseline = capture(&mut renderer, 0.);
    let biased = capture(&mut renderer, 1.);
    assert_eq!(baseline.rgba, biased.rgba);
    assert_depth(&renderer, &baseline, 0.5);
    assert_depth(&renderer, &biased, 0.5 + 1. / 8_388_608.);
}

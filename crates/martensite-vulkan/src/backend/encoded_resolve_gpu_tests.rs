//! Empty layer scopes must not reinterpret already-rendered multisample colors.
use super::{VulkanOptions, VulkanRenderer};
use crate::{FrameCapture, PrimitiveDraw, RenderBackend, RenderFrame, RenderLayer, RenderVertex};

#[test]
fn empty_upper_layer_preserves_encoded_multisample_color_and_depth() {
    let mut renderer = validated_renderer();
    for samples in [0, 4] {
        require_empty_layer_neutral(&mut renderer, samples);
    }
    assert!(renderer.validation_errors().unwrap().is_empty());
}
fn require_empty_layer_neutral(renderer: &mut VulkanRenderer, samples: i32) {
    let baseline = capture_encoded(renderer, samples, false);
    let empty = capture_encoded(renderer, samples, true);
    assert_capture_equal(&baseline, &empty);
}
fn capture_encoded(renderer: &mut VulkanRenderer, samples: i32, empty: bool) -> FrameCapture {
    let frame = encoded_frame(samples, empty);
    require_samples(renderer, &frame, samples);
    let capture = renderer.render(&frame).unwrap();
    if samples == 4 {
        assert!(
            capture
                .rgba
                .as_chunks::<4>()
                .0
                .iter()
                .any(|p| p[0] > 0 && p[0] < 255)
        );
    }
    capture
}
pub(super) fn validated_renderer() -> VulkanRenderer {
    VulkanRenderer::new(VulkanOptions {
        validation: true,
        device_index: Some(0),
    })
    .unwrap()
}
#[test]
fn empty_upper_layer_clears_depth_without_touching_color() {
    let mut renderer = validated_renderer();
    let baseline = capture_encoded(&mut renderer, 4, false);
    let mut frame = encoded_frame(4, true);
    frame.layers.last_mut().unwrap().clear_depth = true;
    let cleared = renderer.render(&frame).unwrap();
    assert_eq!(baseline.rgba, cleared.rgba);
    assert!(baseline.depth.iter().any(|depth| *depth < 1.));
    assert!(cleared.depth.iter().all(|depth| *depth == 1.));
    assert!(renderer.validation_errors().unwrap().is_empty());
}
#[test]
fn empty_stencil_restore_layer_preserves_following_draw() {
    let mut renderer = validated_renderer();
    let baseline = renderer.render(&stencil_frame(false)).unwrap();
    let empty = renderer.render(&stencil_frame(true)).unwrap();
    assert_capture_equal(&baseline, &empty);
    assert!(!renderer.stencil.is_enabled());
    assert!(renderer.validation_errors().unwrap().is_empty());
}
fn stencil_frame(split: bool) -> RenderFrame {
    let draw = encoded_frame(4, false).draws[0].clone();
    let mut program = crate::stencil_program::StencilProgram::default();
    let mut layers = Vec::new();
    if split {
        layers.push(RenderLayer::new(vec![draw.clone()], true, true, false));
    } else {
        program.geometry(vec![draw.clone()]);
    }
    append_stencil_restore(&mut program);
    layers.push(RenderLayer::from_stencil_program(
        program, true, true, false,
    ));
    layers.push(RenderLayer::new(vec![draw], true, true, false));
    let mut frame = RenderFrame::with_layers(8, 8, [0., 0., 0., 1.], layers).unwrap();
    frame.raster.requested_samples = 4;
    require_stencil_clear(&frame, usize::from(split));
    frame
}
fn append_stencil_restore(program: &mut crate::stencil_program::StencilProgram) {
    use crate::stencil_program::StencilCommand;
    program.command(StencilCommand::Options(true));
    program.command(StencilCommand::CappingRestore);
}
fn require_stencil_clear(frame: &RenderFrame, index: usize) {
    let plan = crate::stencil_plan::StencilPlan::new(frame, Default::default());
    assert!(plan.clear_after_layer[index]);
    assert!(!plan.after_layer[index].is_enabled());
}
pub(super) fn encoded_frame(samples: i32, empty: bool) -> RenderFrame {
    let vertices = [[-1., -1., 0.], [1., -1., 0.], [-1., 1., 0.]]
        .map(|position| RenderVertex::new(position, [1.; 4]));
    let mut draw =
        PrimitiveDraw::new(vertices.to_vec(), vec![0, 1, 2], crate::identity_clip()).unwrap();
    draw.compatibility_color_target = true;
    let mut layers = vec![RenderLayer::new(vec![draw], true, true, false)];
    if empty {
        layers.push(RenderLayer::new(vec![], false, false, false));
    }
    let mut frame = RenderFrame::with_layers(8, 8, [0., 0., 0., 1.], layers).unwrap();
    frame.raster.requested_samples = samples;
    frame
}
pub(super) fn assert_capture_equal(before: &FrameCapture, after: &FrameCapture) {
    assert_eq!(
        before.rgba, after.rgba,
        "colorless scope changed resolved color"
    );
    assert_eq!(
        before.depth, after.depth,
        "colorless scope changed resolved depth"
    );
}

fn require_samples(renderer: &VulkanRenderer, frame: &RenderFrame, requested: i32) {
    let plan = super::frame_raster_plan::frame_raster_plan(&renderer.runtime, frame, false);
    let expected = if requested == 0 { 1 } else { 4 };
    assert_eq!(
        plan.samples.as_raw(),
        expected,
        "actual samples must match the declared control"
    );
}

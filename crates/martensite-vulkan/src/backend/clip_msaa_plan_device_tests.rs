//! Device-side plan controls bind requested samples to the allocated attachment metadata.
use super::super::super::{
    pipeline_variants::RasterPipelines,
    retained_submission::RetirementStore,
    runtime::{VulkanOptions, VulkanRuntime},
};
use super::*;
use crate::{
    PrimitiveDraw, RenderBackend, RenderBehaviorConfiguration, RenderClipping,
    RenderCoordinateConvention, RenderFrame, RenderShading, RenderVertex,
    RenderWorldClippingBehavior, VulkanRenderer, identity_clip,
};
use ash::vk;
use std::sync::Arc;

#[test]
#[ignore = "bounded actual-plan diagnostic; requires the canonical GPU lock"]
fn clip_probe_allocates_the_requested_single_and_eight_sample_targets() {
    let runtime = Arc::new(
        VulkanRuntime::create(VulkanOptions {
            validation: true,
            device_index: Some(0),
        })
        .unwrap(),
    );
    for requested in [0, 8] {
        assert_allocated_sample_count(&runtime, requested);
    }
}

#[test]
#[ignore = "bounded world-clip V2 control; requires the canonical GPU lock"]
fn raster_world_clip_v2_matches_zero_sample_control_and_restores_eight_sample_pixels() {
    let mut legacy = clipping_renderer(RenderBehaviorConfiguration::default());
    let mut selected = clipping_renderer(
        RenderBehaviorConfiguration::default()
            .with_world_clipping(RenderWorldClippingBehavior::RasterClipDistanceV2),
    );
    for samples in [0, 8] {
        let frame = clipped_quad_frame(samples);
        let baseline = legacy.render(&frame).unwrap();
        let raster = selected.render(&frame).unwrap();
        assert_microcase_coverage(samples, &baseline, &raster);
    }
    assert!(legacy.validation_errors().unwrap().is_empty());
    assert!(selected.validation_errors().unwrap().is_empty());
}

#[test]
#[ignore = "bounded world-clip V3 crossing-line control; requires the canonical GPU lock"]
fn raster_world_clip_v3_keeps_only_line_samples_beyond_the_clipped_endpoint() {
    let mut renderer = clipping_renderer(
        RenderBehaviorConfiguration::default()
            .with_world_clipping(RenderWorldClippingBehavior::RasterClipDistanceV3),
    );
    for samples in [0, 8] {
        let capture = renderer.render(&crossing_clipped_line(samples)).unwrap();
        assert_red_region(&capture, 32..34, 28..36, true);
        assert_red_region(&capture, 30..32, 28..36, false);
    }
    assert!(renderer.validation_errors().unwrap().is_empty());
}

fn crossing_clipped_line(samples: i32) -> RenderFrame {
    let vertices = [[-0.75, 0., 0.], [0.75, 0., 0.]]
        .map(|point| RenderVertex::new(point, [1., 0., 0., 1.]))
        .to_vec();
    let clipping = RenderClipping::new(vec![vec![[1., 0., 0., 0.]]], identity_clip()).unwrap();
    let mut transform = identity_clip();
    transform[0] = 0.1;
    transform[5] = 0.1;
    let mut draw = PrimitiveDraw::segments(
        vertices,
        vec![0, 1],
        transform,
        crate::LineStyle::new(1., 0xffff, 1).unwrap(),
    )
    .unwrap();
    draw.clipping = Some(clipping);
    let mut frame = RenderFrame::new(64, 64, [0., 0., 1., 1.], vec![draw]).unwrap();
    frame.raster.requested_samples = samples;
    frame
}

fn assert_red_region(
    capture: &crate::FrameCapture,
    x: std::ops::Range<usize>,
    y: std::ops::Range<usize>,
    expected: bool,
) {
    let visible = y
        .into_iter()
        .any(|row| x.clone().any(|column| has_red(capture, column, row)));
    assert_eq!(visible, expected, "region x={x:?}, expected red={expected}");
}

fn has_red(capture: &crate::FrameCapture, x: usize, y: usize) -> bool {
    capture.rgba[(y * 64 + x) * 4] > 0
}

fn clipping_renderer(configuration: RenderBehaviorConfiguration) -> VulkanRenderer {
    VulkanRenderer::new_with_render_behavior(
        VulkanOptions {
            validation: true,
            device_index: Some(0),
        },
        Default::default(),
        RenderCoordinateConvention::TopLeftLogicalV2,
        Default::default(),
        configuration,
    )
    .unwrap()
}

fn assert_microcase_coverage(
    samples: i32,
    legacy: &crate::FrameCapture,
    selected: &crate::FrameCapture,
) {
    assert_pixel_visibility(selected, 33, 30, true);
    assert_pixel_visibility(selected, 33, 31, true);
    assert_pixel_visibility(selected, 32, 30, samples == 8);
    assert_pixel_visibility(selected, 32, 31, samples == 8);
    if samples == 0 {
        assert_eq!(selected.rgba, legacy.rgba);
        assert_eq!(selected.depth, legacy.depth);
    }
}

fn assert_pixel_visibility(capture: &crate::FrameCapture, x: usize, y: usize, visible: bool) {
    let offset = (y * 64 + x) * 4;
    let pixel = &capture.rgba[offset..offset + 4];
    assert_eq!(pixel != [0, 0, 255, 255], visible, "pixel ({x},{y})");
}

fn assert_allocated_sample_count(runtime: &Arc<VulkanRuntime>, requested: i32) {
    let frame = clipped_quad_frame(requested);
    let pipeline = RasterPipelines::new(runtime, false).unwrap();
    let retirements = RetirementStore::new().unwrap();
    let setup = FrameSetup {
        runtime,
        frame: &frame,
        pipeline: &pipeline,
        retirements: &retirements,
    };
    let plan = setup
        .prepare(Default::default(), Default::default())
        .unwrap();
    let expected = if requested == 0 {
        vk::SampleCountFlags::TYPE_1
    } else {
        vk::SampleCountFlags::TYPE_8
    };
    assert_eq!(
        plan.raster.samples, expected,
        "requested {requested}; effective {:?}",
        plan.raster.samples
    );
    let uploads = setup.upload(&plan).unwrap();
    let resources = setup.allocate(plan, uploads).unwrap();
    assert_actual_attachments(&resources, expected);
}

fn clipped_quad_frame(samples: i32) -> RenderFrame {
    let vertices = [
        [0.0625, 0.0625, 0.],
        [0.5, 0.0625, 0.],
        [0.5, 0.5, 0.],
        [0.0625, 0.5, 0.],
    ]
    .map(|position| RenderVertex::new(position, [1., 0., 0., 1.]))
    .to_vec();
    let clipping = RenderClipping::new(vec![vec![[1., 0., 0., -0.25]]], identity_clip()).unwrap();
    let mut transform = identity_clip();
    // Match the Native orthographic scale20 projection coefficient, 2 / 20.
    transform[0] = 0.1;
    transform[5] = 0.1;
    let mut draw = PrimitiveDraw::new(vertices, vec![0, 1, 2, 0, 2, 3], transform)
        .unwrap()
        .with_clipping(clipping);
    draw.lighting = Some(
        super::super::super::fixed_vertex_sources::fixed_test_lighting(4, RenderShading::Phong),
    );
    draw.fixed_function_lighting = true;
    let mut frame = RenderFrame::new(64, 64, [0., 0., 1., 1.], vec![draw]).unwrap();
    frame.raster.requested_samples = samples;
    frame
}

fn assert_actual_attachments(resources: &FrameResources, expected: vk::SampleCountFlags) {
    assert_eq!(resources.render_color().samples, expected);
    assert_eq!(resources.render_depth().samples, expected);
    assert_eq!(resources.render_color().format, vk::Format::R8G8B8A8_SRGB);
    assert_eq!(resources.color.format, vk::Format::R8G8B8A8_SRGB);
    println!(
        "{{\"event\":\"clip-msaa-attachments\",\"effective_samples\":{},\"render_color_format\":{},\"resolve_color_format\":{},\"depth_format\":{},\"render_color_samples\":{},\"resolve_color_samples\":{},\"depth_samples\":{}}}",
        expected.as_raw(),
        resources.render_color().format.as_raw(),
        resources.color.format.as_raw(),
        resources.depth.format.as_raw(),
        resources.render_color().samples.as_raw(),
        resources.color.samples.as_raw(),
        resources.render_depth().samples.as_raw()
    );
}

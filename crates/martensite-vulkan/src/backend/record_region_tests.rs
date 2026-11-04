//! Closed-form asymmetric regions exercise the same upload, draw and readback commands.
use super::super::render_target_record::set_raster_region;
use super::super::{VulkanOptions, VulkanRenderer, submission::CommandSubmission};
use super::*;
use crate::{FrameCapture, PrimitiveDraw, RenderFaceCulling, RenderVertex, identity_clip};

const fn rectangle(x: i32, y: i32, width: u32, height: u32) -> vk::Rect2D {
    vk::Rect2D {
        offset: vk::Offset2D { x, y },
        extent: vk::Extent2D { width, height },
    }
}
fn region_frame(culling: RenderFaceCulling) -> RenderFrame {
    let points = [[-1., -1., 0.], [1., -1., 0.], [1., 1., 0.], [-1., 1., 0.]];
    let vertices = points
        .map(|point| RenderVertex::new(point, [1., 0., 0., 1.]))
        .to_vec();
    let draw = PrimitiveDraw::new(vertices, vec![0, 1, 2, 0, 2, 3], identity_clip()).unwrap();
    RenderFrame::new(
        23,
        17,
        [0., 0., 1., 1.],
        vec![draw.with_face_culling(culling)],
    )
    .unwrap()
}
fn partial_capture(
    renderer: &mut VulkanRenderer,
    regions: [vk::Rect2D; 2],
    culling: RenderFaceCulling,
) -> FrameCapture {
    let frame = region_frame(culling);
    let resources = renderer.frame_resources(&frame).unwrap();
    let commands = CommandSubmission::new(&renderer.runtime).unwrap();
    let ctx = &renderer.runtime;
    let cmd = commands.command;
    transition_targets(ctx, cmd, &resources, false);
    begin_partial_layer(ctx, cmd, &resources, regions);
    draw_partial_layer(ctx, cmd, &resources, &frame.layers[0]);
    finish_partial_capture(ctx, &commands, &resources)
}
fn draw_partial_layer(
    ctx: &VulkanRuntime,
    cmd: vk::CommandBuffer,
    target: &FrameResources,
    layer: &crate::RenderLayerSpan,
) {
    set_depth_policy(ctx, cmd, layer);
    record_draws(ctx, cmd, target, layer);
}
fn begin_partial_layer(
    ctx: &VulkanRuntime,
    cmd: vk::CommandBuffer,
    target: &FrameResources,
    regions: [vk::Rect2D; 2],
) {
    begin_rendering(ctx, cmd, target, [0., 0., 1., 1.], true, true, false);
    set_raster_region(ctx, cmd, target.extent, regions[0], regions[1]);
}
fn finish_partial_capture(
    runtime: &VulkanRuntime,
    commands: &CommandSubmission,
    resources: &FrameResources,
) -> FrameCapture {
    finish_rendering(runtime, commands.command);
    transition_targets(runtime, commands.command, resources, true);
    read_attachments(runtime, commands.command, resources);
    commands.finish().unwrap();
    resources.capture().unwrap()
}
const RED: [u8; 4] = [255, 0, 0, 255];
const BLUE: [u8; 4] = [0, 0, 255, 255];
const FULL: vk::Rect2D = rectangle(0, 0, 23, 17);
const PARTIAL_VIEW: vk::Rect2D = rectangle(3, 2, 12, 10);
const REGIONS: [([vk::Rect2D; 2], vk::Rect2D); 3] = [
    ([PARTIAL_VIEW, FULL], PARTIAL_VIEW),
    ([FULL, rectangle(5, 4, 7, 5)], rectangle(5, 4, 7, 5)),
    ([PARTIAL_VIEW, rectangle(5, 1, 7, 5)], rectangle(5, 2, 7, 4)),
];
fn assert_region_pixels(capture: &FrameCapture, expected: vk::Rect2D, half: f32) {
    for (index, pixel) in capture.rgba.as_chunks::<4>().0.iter().enumerate() {
        assert_region_pixel(capture.depth[index], *pixel, index, (expected, half));
    }
}
fn assert_region_pixel(
    depth: f32,
    pixel: [u8; 4],
    index: usize,
    (expected, half): (vk::Rect2D, f32),
) {
    let x = (index % 23) as i32;
    let y = (index / 23) as i32;
    let inside = x >= expected.offset.x
        && x < expected.offset.x + expected.extent.width as i32
        && y >= expected.offset.y
        && y < expected.offset.y + expected.extent.height as i32;
    let color = if inside { RED } else { BLUE };
    let expected_depth = if inside { half } else { 1. };
    assert_eq!(pixel, color, "pixel {x},{y}");
    assert_eq!(depth, expected_depth, "depth {x},{y}");
}
fn region_renderer() -> VulkanRenderer {
    VulkanRenderer::new(VulkanOptions {
        validation: true,
        device_index: Some(0),
    })
    .unwrap()
}
#[test]
fn offset_viewport_and_scissor_preserve_top_left_color_and_depth() {
    let mut renderer = region_renderer();
    for (regions, expected) in REGIONS {
        let capture = partial_capture(&mut renderer, regions, RenderFaceCulling::None);
        assert_region_pixels(&capture, expected, renderer.stored_depth(0.5));
    }
    assert!(renderer.validation_errors().unwrap().is_empty());
}
#[test]
fn front_and_back_culling_keep_the_same_geometric_side_after_row_conversion() {
    let mut renderer = region_renderer();
    for (culling, expected) in [
        (RenderFaceCulling::Back, FULL),
        (RenderFaceCulling::Front, rectangle(0, 0, 0, 0)),
    ] {
        let capture = partial_capture(&mut renderer, [FULL, FULL], culling);
        assert_region_pixels(&capture, expected, renderer.stored_depth(0.5));
    }
    assert!(renderer.validation_errors().unwrap().is_empty());
}

#[test]
fn candidate_partial_regions_and_culling_use_the_same_physical_convention() {
    for raster in [
        crate::RenderRasterPolicy::LegacyFirstV1,
        crate::RenderRasterPolicy::LastProvokingVertexV2,
    ] {
        let mut renderer = candidate_region_renderer(raster);
        for (regions, expected) in REGIONS {
            let capture = partial_capture(&mut renderer, regions, RenderFaceCulling::None);
            assert_region_pixels(&capture, expected, renderer.stored_depth(0.5));
        }
        assert_candidate_culling(&mut renderer);
        assert!(
            renderer.validation_errors().unwrap().is_empty(),
            "{:?}",
            renderer.validation_errors().unwrap()
        );
    }
}

fn candidate_region_renderer(raster: crate::RenderRasterPolicy) -> VulkanRenderer {
    let coordinates = crate::RenderCoordinateConvention::TopLeftLogicalV2;
    let options = VulkanOptions {
        validation: true,
        device_index: Some(0),
    };
    VulkanRenderer::new_with_conventions(options, raster, coordinates).unwrap()
}

fn assert_candidate_culling(renderer: &mut VulkanRenderer) {
    for (culling, expected) in [
        (RenderFaceCulling::Back, FULL),
        (RenderFaceCulling::Front, rectangle(0, 0, 0, 0)),
    ] {
        let capture = partial_capture(renderer, [FULL, FULL], culling);
        assert_region_pixels(&capture, expected, renderer.stored_depth(0.5));
    }
}

#[path = "fixed_grid_region_tests.rs"]
mod fixed_grid_tests;

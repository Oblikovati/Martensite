//! Existing partial-region recording exercises point expansion and ordinary draw restoration.
use super::*;
use crate::fixed_grid_point_gpu_tests::{face_emission, fixed_grid_renderer_with, lit_front_point};
use crate::{
    RenderCoordinateConvention, RenderLighting, RenderMaterial, RenderShading, identity_clip,
};

// Neither sample may be a depth midpoint: that would hide a lost viewport range.
const ORDINARY_CLIP_Z: f32 = 0.25;
const NONFULL_DEPTH_RANGE: [f32; 2] = [0.125, 0.75];

fn ordinary_lit_rectangle() -> PrimitiveDraw {
    let vertices = [
        [-0.75, -0.75, ORDINARY_CLIP_Z],
        [-0.5, -0.75, ORDINARY_CLIP_Z],
        [-0.5, -0.5, ORDINARY_CLIP_Z],
        [-0.75, -0.5, ORDINARY_CLIP_Z],
    ]
    .map(|p| RenderVertex::new(p, [1.; 4]))
    .to_vec();
    let material = RenderMaterial {
        common: [
            face_emission([1., 0., 0., 1.]),
            face_emission([0., 0., 1., 1.]),
        ],
        ..Default::default()
    };
    let lighting = RenderLighting::new(
        vec![[0., 0., 1.]; 4],
        material,
        identity_clip(),
        identity_clip(),
        false,
        RenderShading::Phong,
    )
    .unwrap()
    .with_lights(crate::RenderLightSet {
        ambient: [0.; 4],
        sources: vec![],
    });
    PrimitiveDraw::new(vertices, vec![0, 1, 2, 0, 2, 3], identity_clip())
        .unwrap()
        .with_lighting(lighting)
        .unwrap()
        .with_face_culling(RenderFaceCulling::Back)
}
fn nonfull_frame() -> RenderFrame {
    let mut point = lit_front_point(RenderShading::Phong, false);
    point.point_style.as_mut().unwrap().size = 7.;
    point.vertices[0].position = [0.984375, -0.3125, 0.5];
    RenderFrame::new(
        64,
        64,
        [0., 0., 0., 1.],
        vec![point, ordinary_lit_rectangle()],
    )
    .unwrap()
}
fn update_point_projection(resources: &mut FrameResources, original: vk::Viewport) {
    let viewports = crate::backend::fixed_grid_viewport::expanded_viewports(
        original,
        resources.attachment_extent,
    );
    let point = &mut resources.draws[0];
    assert!(point.point_viewports.is_some());
    point.point_viewports = Some(viewports);
    crate::backend::fixed_grid_viewport::append_viewports(&mut point.transform, viewports);
}
fn capture_nonfull(renderer: &mut VulkanRenderer) -> FrameCapture {
    let frame = nonfull_frame();
    let mut resources = renderer.frame_resources(&frame).unwrap();
    let regions = [rectangle(16, 8, 32, 24), rectangle(0, 0, 64, 64)];
    let ctx = &renderer.runtime;
    let mut original = ctx
        .attachment_coordinates
        .raster_regions(resources.extent, regions[0], regions[1])
        .viewport;
    [original.min_depth, original.max_depth] = NONFULL_DEPTH_RANGE;
    update_point_projection(&mut resources, original);
    record_nonfull(ctx, &resources, &frame, regions, original)
}
fn record_nonfull(
    ctx: &std::sync::Arc<VulkanRuntime>,
    resources: &FrameResources,
    frame: &RenderFrame,
    regions: [vk::Rect2D; 2],
    original: vk::Viewport,
) -> FrameCapture {
    let commands = CommandSubmission::new(ctx).unwrap();
    let cmd = commands.command;
    transition_targets(ctx, cmd, resources, false);
    begin_rendering(ctx, cmd, resources, [0., 0., 0., 1.], true, true, false);
    set_raster_region(ctx, cmd, resources.extent, regions[0], regions[1]);
    // SAFETY: the command buffer is recording on this thread and every handle it names outlives the submission.
    unsafe {
        ctx.device().cmd_set_viewport(cmd, 0, &[original]);
    }
    draw_partial_layer(ctx, cmd, resources, &frame.layers[0]);
    finish_partial_capture(ctx, &commands, resources)
}
fn require_nonfull_capture(capture: &FrameCapture) {
    for y in 0..64 {
        for x in 0..64 {
            let point = (44..=50).contains(&x) && (20..=26).contains(&y);
            let ordinary = (20..=23).contains(&x) && (26..=28).contains(&y);
            let color = if point {
                [0, 255, 0, 255]
            } else if ordinary {
                [255, 0, 0, 255]
            } else {
                [0, 0, 0, 255]
            };
            let depth = if point {
                0.59375
            } else if ordinary {
                0.515625
            } else {
                1.
            };
            let i = y * 64 + x;
            assert_eq!(&capture.rgba[i * 4..i * 4 + 4], color, "({x},{y})");
            assert_eq!(capture.depth[i], depth, "depth({x},{y})");
        }
    }
}
#[test]
#[ignore = "requires explicitly authorized GPU lease and pinned validation environment"]
fn fixed_grid_owned_nonfull_mirrored_y_depth_range_and_ordinary_restore() {
    for coordinates in [
        RenderCoordinateConvention::LegacyBottomUpV1,
        RenderCoordinateConvention::TopLeftLogicalV2,
    ] {
        let mut renderer = fixed_grid_renderer_with(coordinates);
        let capture = capture_nonfull(&mut renderer);
        crate::fixed_grid_point_gpu_tests::record_point_capture(
            &format!("nonfull-coords-{}", coordinates.behavior_version()),
            &capture,
        );
        require_nonfull_capture(&capture);
        assert!(renderer.validation_errors().unwrap().is_empty());
    }
}

#[test]
fn fixed_grid_nonfull_depth_reference_distinguishes_both_samples_from_default_range() {
    let [near, far] = NONFULL_DEPTH_RANGE;
    let expected = [0.5, ORDINARY_CLIP_Z].map(|clip_z| near + (far - near) * (clip_z + 1.) * 0.5);
    assert_eq!(expected, [0.59375, 0.515625]);
    assert_ne!(expected[0], 0.75);
    assert_ne!(expected[1], 0.625);
    assert!(expected.into_iter().all(|depth| depth != 0.5));
}

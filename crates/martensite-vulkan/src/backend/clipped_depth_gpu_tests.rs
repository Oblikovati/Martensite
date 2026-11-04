//! Same-draw flat fan transport and sample-rate depth must preserve fixed pixel-center color.
use super::{VulkanOptions, VulkanRenderer};
use crate::{PrimitiveDraw, RenderBackend, RenderBehaviorConfiguration, RenderFrame, RenderVertex};
use crate::{RenderViewportGridPrecision as Grid, RenderWindowDepthBehavior as Depth};
use ash::vk;
fn renderer(depth: Depth) -> VulkanRenderer {
    let clipping = crate::RenderWorldClippingBehavior::RasterClipDistanceV2;
    renderer_with_clip(depth, clipping)
}
fn renderer_with_clip(
    depth: Depth,
    clipping: crate::RenderWorldClippingBehavior,
) -> VulkanRenderer {
    VulkanRenderer::new_with_render_behavior(
        VulkanOptions {
            validation: true,
            device_index: Some(0),
        },
        crate::RenderRasterPolicy::LastProvokingVertexV2,
        crate::RenderCoordinateConvention::LegacyBottomUpV1,
        Default::default(),
        RenderBehaviorConfiguration::default()
            .with_world_clipping(clipping)
            .with_depth_bias(crate::RenderDepthBiasBehavior::Native24UnitsV2)
            .with_window_depth(depth),
    )
    .unwrap()
}
fn camera_factors() -> crate::RenderProjectionFactors {
    use martensite_tkmath::gp::Point3;
    use martensite_tkservice::graphic3d::Graphic3dCamera;
    let camera = Graphic3dCamera::default();
    camera
        .set_eye_and_center(&Point3::new(0., 0., 0.), &Point3::new(0., 0., -1.))
        .unwrap();
    let view = camera.orientation_matrix_f().data();
    assert_eq!(view, crate::identity_clip());
    crate::RenderProjectionFactors::retained_camera(
        crate::identity_clip(),
        view,
        crate::identity_clip(),
        &camera,
    )
    .unwrap()
}
fn row_vertices(row: usize) -> [RenderVertex; 3] {
    let bottom = -0.9375 + row as f32 / 16.;
    let top = bottom + 0.046875;
    let z = -0.25 + row as f32 / 64.;
    [
        RenderVertex::new([-0.875, bottom, z], [0.125, 0.25, 0.5, 1.]),
        RenderVertex::new([0.875, bottom, z], [0.75, 0.5, 0.25, 1.]),
        RenderVertex::new([0.875, top, z], [0.5, 0.75, 0.125, 1.]),
    ]
}
pub(super) fn source_draw() -> PrimitiveDraw {
    let vertices: Vec<_> = (0..30).flat_map(row_vertices).collect();
    let mut draw = PrimitiveDraw::new(vertices, (0..90).collect(), crate::identity_clip()).unwrap();
    let mut lighting =
        super::fixed_vertex_sources::fixed_test_lighting(90, crate::RenderShading::Phong);
    lighting.vertex_colors = true;
    lighting.lights.ambient = [1.; 4];
    for material in &mut lighting.material.common {
        material.emission = [0.; 4];
    }
    draw.lighting = Some(lighting);
    draw.fixed_function_lighting = true;
    draw.projection_factors = Some(camera_factors());
    draw.clipping = Some(
        crate::RenderClipping::new(vec![vec![[1., 0., 0., 0.]]], crate::identity_clip()).unwrap(),
    );
    draw
}
fn frame() -> RenderFrame {
    let mut frame = RenderFrame::new(256, 256, [0., 0., 0., 1.], vec![source_draw()])
        .unwrap()
        .with_stencil_clear_value(1);
    frame.raster.requested_samples = 8;
    frame.raster.fixed_function = true;
    frame
}
fn require_actual_eight(renderer: &VulkanRenderer, frame: &RenderFrame) {
    assert_eq!(
        renderer.runtime.depth.formats.stencil,
        vk::Format::D24_UNORM_S8_UINT
    );
    let plan = super::frame_raster_plan::frame_raster_plan(&renderer.runtime, frame, true);
    assert_eq!(plan.samples, vk::SampleCountFlags::TYPE_8);
}
#[test]
#[ignore = "requires pinned profile and canonical GPU lock; explicit V4 conformance"]
fn same_draw_thirty_clipped_fans_preserve_full_color_and_constant_depth() {
    let source = frame();
    let mut old = renderer(Depth::HardwareV1);
    let mut selected = renderer(Depth::ClippedFanFramebufferV4 {
        fan_grid: Grid::Fractional8,
        raster_grid: Grid::Fractional8,
    });
    compare_complete_frames(&mut old, &mut selected, &source);
}

fn require_nonblank(reference: &crate::FrameCapture) {
    let lit = reference
        .rgba
        .as_chunks::<4>()
        .0
        .iter()
        .filter(|pixel| pixel[..3] != [0, 0, 0])
        .count();
    assert!(
        lit > 3000,
        "expected non-vacuous clipped color ramps; got {lit} lit pixels"
    );
}

fn require_no_validation(renderer: &VulkanRenderer) {
    let errors = renderer.validation_errors().unwrap();
    assert!(
        errors.is_empty(),
        "expected no validation errors, received {errors:?}"
    );
}

fn compare_complete_frames(
    old: &mut VulkanRenderer,
    selected: &mut VulkanRenderer,
    source: &RenderFrame,
) {
    require_actual_eight(old, source);
    require_actual_eight(selected, source);
    let reference = old.render(source).unwrap();
    let actual = selected.render(source).unwrap();
    require_nonblank(&reference);
    assert_eq!(actual.rgba, reference.rgba);
    assert_eq!(actual.depth, reference.depth);
    require_no_validation(old);
    require_no_validation(selected);
}

fn unlit_companion() -> PrimitiveDraw {
    let vertices = [
        [-0.75, -0.875, 0.75],
        [0.75, -0.875, 0.75],
        [0.75, 0.875, 0.75],
    ]
    .map(|point| RenderVertex::new(point, [0.125, 0.5, 0.25, 1.]));
    let mut draw =
        PrimitiveDraw::new(vertices.to_vec(), vec![0, 1, 2], crate::identity_clip()).unwrap();
    draw.fixed_function_lighting = true;
    draw.clipping = Some(
        crate::RenderClipping::new(vec![vec![[1., 0., 0., 0.]]], crate::identity_clip()).unwrap(),
    );
    draw
}
fn mixed_frame() -> RenderFrame {
    let mut source = RenderFrame::new(
        256,
        256,
        [0., 0., 0., 1.],
        vec![source_draw(), unlit_companion()],
    )
    .unwrap()
    .with_stencil_clear_value(1);
    source.raster.requested_samples = 8;
    source.raster.fixed_function = true;
    assert_eq!(source.layers()[0].draw_range(), 0..2);
    assert_eq!(source.draws.len(), 2);
    source
}
#[test]
#[ignore = "requires pinned profile/canonical lock; mixed-family V4 composition conformance"]
fn mixed_unlit_and_thirty_fixed_fans_keep_fragment_policy_on_unselected_draws() {
    let source = mixed_frame();
    let clipping = crate::RenderWorldClippingBehavior::FragmentV1;
    let mut baseline = renderer_with_clip(Depth::HardwareV1, clipping);
    let mut selected = renderer_with_clip(
        Depth::ClippedFanFramebufferV4 {
            fan_grid: Grid::Fractional8,
            raster_grid: Grid::Fractional8,
        },
        clipping,
    );
    compare_complete_frames(&mut baseline, &mut selected, &source);
}

#[test]
#[ignore = "requires pinned profile and canonical GPU lock; explicit V6 conformance"]
fn shared_incidence_q12_thirty_fans_preserve_full_color_and_constant_depth() {
    let source = frame();
    let clipping = crate::RenderWorldClippingBehavior::FragmentV1;
    let mut old = renderer_with_clip(
        Depth::HardwareV1,
        crate::RenderWorldClippingBehavior::RasterClipDistanceV2,
    );
    let mut selected = renderer_with_clip(
        Depth::ClippedIncidenceFramebufferV6 {
            fan_grid: Grid::Fractional12,
            raster_grid: Grid::Fractional8,
        },
        clipping,
    );
    compare_complete_frames(&mut old, &mut selected, &source);
}
fn v6_renderer() -> VulkanRenderer {
    renderer(Depth::ClippedIncidenceFramebufferV6 {
        fan_grid: Grid::Fractional12,
        raster_grid: Grid::Fractional8,
    })
}
fn reference_empty_draw(points: [[f32; 3]; 3]) -> PrimitiveDraw {
    let vertices = points.map(|point| RenderVertex::new(point, [0.25, 0.5, 0.75, 1.]));
    let mut draw =
        PrimitiveDraw::new(vertices.to_vec(), vec![0, 1, 2], crate::identity_clip()).unwrap();
    draw.lighting = Some(super::fixed_vertex_sources::fixed_test_lighting(
        3,
        crate::RenderShading::Phong,
    ));
    draw.fixed_function_lighting = true;
    draw.face_culling = crate::RenderFaceCulling::None;
    draw.projection_factors = Some(camera_factors());
    draw.clipping = Some(
        crate::RenderClipping::new(vec![vec![[1., 0., 0., 0.]]], crate::identity_clip()).unwrap(),
    );
    draw
}
#[test]
#[ignore = "requires pinned profile and canonical GPU lock; actual empty-reference completion"]
fn certified_reference_empty_triangle_keeps_original_submission_and_neutral_frame() {
    let empty = reference_empty_draw([[0.5, 0.5, 0.], [0.625, 0.625, 0.], [0.75, 0.75, 0.]]);
    let mut source =
        RenderFrame::new(256, 256, [0., 0., 0., 1.], vec![source_draw(), empty]).unwrap();
    source.raster.requested_samples = 8;
    source.raster.fixed_function = true;
    assert_eq!(source.draws[1].indices, [0, 1, 2]);
    assert_eq!(source.layers()[0].draw_range(), 0..2);
    let mut baseline = renderer(Depth::HardwareV1);
    let mut selected = v6_renderer();
    compare_complete_frames(&mut baseline, &mut selected, &source);
}
fn covered_empty_reference_frame() -> RenderFrame {
    let points = [
        [0., -1. / 512., 0.],
        [1. / 128., 3. / 512., 0.],
        [129. / 8388608., -1. / 512. + 127. / 8388608., 0.],
    ];
    let mut source = RenderFrame::new(
        256,
        256,
        [0., 0., 0., 1.],
        vec![reference_empty_draw(points)],
    )
    .unwrap();
    source.raster.fixed_function = true;
    source.raster.requested_samples = 8;
    source
}
fn require_a_physical_sample(reference: &crate::FrameCapture) {
    assert!(
        reference
            .rgba
            .as_chunks::<4>()
            .0
            .iter()
            .any(|pixel| pixel[..3] != [0, 0, 0])
    );
}
fn require_missing_reference_plane(error: crate::RenderError) {
    assert!(matches!(error.window_depth_failure(),
        Some(crate::RenderWindowDepthFailure::GpuDraw { cause,.. })
        if matches!(cause.as_ref(),crate::RenderWindowDepthFailure::GpuCompletion {
            observed_header,.. } if observed_header[3]&512!=0)));
}
#[test]
#[ignore = "requires pinned profile and canonical GPU lock; physical sample still requires a plane"]
fn physical_covered_sample_without_reference_plane_still_refuses_publication() {
    let source = covered_empty_reference_frame();
    let mut baseline = renderer(Depth::HardwareV1);
    let mut selected = v6_renderer();
    require_actual_eight(&baseline, &source);
    require_actual_eight(&selected, &source);
    require_a_physical_sample(&baseline.render(&source).unwrap());
    require_missing_reference_plane(selected.render(&source).unwrap_err());
    require_no_validation(&baseline);
    require_no_validation(&selected);
}

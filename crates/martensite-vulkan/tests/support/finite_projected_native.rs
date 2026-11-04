//! Existing independently captured finite lines qualify the opt-in rule without changing fixtures.
#[path = "line_alpha_arrays.rs"]
mod line_alpha_arrays;
#[path = "view_setup.rs"]
mod view_setup;
#[path = "viewer_scene.rs"]
mod viewer_scene;
use crate::triangle_edge_replay::native_golden;
use martensite_tkservice::graphic3d::*;
use martensite_vulkan::*;
#[test]
fn owned_finite_segments_match_recorded_constant_color_native_lines() {
    let driver = Graphic3dRenderDriver::new(native_options()).unwrap();
    let scene = viewer_scene::ViewerScene::new(&driver);
    configure_corpus_scene(&scene);
    let mut renderer = VulkanRenderer::new(native_options()).unwrap();
    for mode in [
        "opaque", "auto", "blend", "mask", "forced", "zero", "wide", "dash",
    ] {
        compare_native(&scene, &mut renderer, mode);
    }
    assert!(renderer.validation_errors().unwrap().is_empty());
    scene.view.remove();
}
fn native_options() -> VulkanOptions {
    VulkanOptions {
        validation: true,
        device_index: Some(0),
    }
}

fn configure_corpus_scene(scene: &viewer_scene::ViewerScene) {
    // These retained parents must stay valid while a separate backend consumes lowered snapshots.
    assert!(!scene.viewer.structure_manager().is_null());
    assert!(scene.structure.is_visible());
    assert_eq!(scene.vertices.vertex_number(), 3);
    scene.window.set_size(64, 64);
    scene
        .cview()
        .set_z_layer_target(GRAPHIC3D_ZLAYER_ID_DEFAULT);
    scene.cview().set_z_layer_redraw_mode(true);
}

fn compare_native(scene: &viewer_scene::ViewerScene, renderer: &mut VulkanRenderer, mode: &str) {
    let mode = format!("line-alpha-{mode}");
    line_alpha_arrays::replace_alpha_lines(&*scene.group, &mode);
    scene.render_view().try_redraw().unwrap();
    assert_native_bytes(
        renderer.device_name(),
        &mode,
        &scene.render_view().capture().unwrap(),
        "legacy",
    );
    let group = scene.group.cast::<Graphic3dDrawGroup>();
    let draws = group
        .primitive_draws_for_view(&scene.view.camera(), [64, 64])
        .unwrap();
    let prepared = prepare_native_draws(&draws);
    let frame = RenderFrame::new(64, 64, [0., 0., 1., 1.], prepared).unwrap();
    let capture = renderer.render(&frame).unwrap();
    assert_native_bytes(renderer.device_name(), &mode, &capture, "finite");
}
fn prepare_native_draws(draws: &[PrimitiveDraw]) -> Vec<PrimitiveDraw> {
    let mut prepared = Vec::new();
    for draw in draws {
        let identities: Vec<u64> = (0..draw.indices().len() as u64 / 2).collect();
        let result = prepare_projected_segments(
            draw,
            [64, 64],
            DashBehavior::FiniteProjectiveV2,
            1e-4,
            &identities,
        )
        .unwrap();
        prepared.extend_from_slice(result.draws());
    }
    prepared
}
fn assert_native_bytes(device: &str, mode: &str, capture: &FrameCapture, stage: &str) {
    let profile = if device.contains("llvmpipe") {
        "lavapipe"
    } else {
        "host"
    };
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/line-alpha");
    let rgba = std::fs::read(root.join(profile).join(format!("{mode}.rgba"))).unwrap();
    let depth = std::fs::read(root.join(profile).join(format!("{mode}.depth"))).unwrap();
    native_golden::assert_capture(capture, &rgba, &depth, mode);
    record_native_capture(profile, mode, stage, capture, &depth);
    println!("FINITE_NATIVE_MATCH {profile}/{mode}/{stage}");
}

fn record_native_capture(
    profile: &str,
    mode: &str,
    stage: &str,
    capture: &FrameCapture,
    depth: &[u8],
) {
    let maximum = maximum_native_depth_error(capture, depth);
    println!("FINITE_NATIVE_DEPTH {profile}/{mode}/{stage} max_error={maximum:?}");
    if let Some(directory) = capture_tests::DashCaptureDirectory::requested() {
        directory.save(
            &format!("native-{profile}-{mode}-{stage}"),
            &capture.rgba,
            &capture.depth,
        );
    }
}
#[path = "../../src/projected_segments/capture_tests.rs"]
mod capture_tests;

fn maximum_native_depth_error(capture: &FrameCapture, depth: &[u8]) -> f32 {
    capture
        .depth
        .iter()
        .zip(depth.as_chunks::<4>().0)
        .map(|(actual, bytes)| (actual - f32::from_le_bytes(*bytes)).abs())
        .fold(0_f32, f32::max)
}

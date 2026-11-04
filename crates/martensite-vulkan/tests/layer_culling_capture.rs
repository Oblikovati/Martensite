//! Actual device captures retain the same immutable layer scenarios as the native viewer.
use super::super::scene::ViewerScene;
use super::{layer_limits, selectable_scene};
use martensite_tkernel::standard::Handle;
use martensite_tkmath::gp::Point3;
use martensite_tkservice::graphic3d::*;
use martensite_vulkan::{FrameCapture, Graphic3dRenderDriver, VulkanOptions};

fn captured_scene(driver: &Handle<Graphic3dRenderDriver>) -> ViewerScene {
    let scene = ViewerScene::new(driver);
    scene
        .cview()
        .set_z_layer_target(GRAPHIC3D_ZLAYER_ID_DEFAULT);
    scene.cview().set_z_layer_redraw_mode(true);
    scene
}

fn camera_mode(scene: &ViewerScene, perspective: bool) {
    let camera = scene.view.camera();
    camera.set_projection_type(if perspective {
        Graphic3dCameraProjection::Perspective
    } else {
        Graphic3dCameraProjection::Orthographic
    });
    camera.set_scale(20.);
    camera.set_eye(&Point3::new(0., 0., 20.)).unwrap();
    camera.set_center(&Point3::default()).unwrap();
}

fn retained_capture(capture: &FrameCapture, label: &str) {
    let Some(folder) = std::env::var_os("MARTENSITE_GEOMETRY_ARTIFACT_DIR") else {
        return;
    };
    let folder = std::path::PathBuf::from(folder);
    std::fs::create_dir_all(&folder).unwrap();
    std::fs::write(folder.join(format!("frame-{label}.rgba")), &capture.rgba).unwrap();
    let depth: Vec<u8> = capture
        .depth
        .iter()
        .flat_map(|value| value.to_le_bytes())
        .collect();
    std::fs::write(folder.join(format!("frame-{label}.depth")), depth).unwrap();
}

fn check_capture(capture: &FrameCapture, visible: bool, perspective: bool) {
    let expected = if perspective { 95. / 99. } else { 19. / 99. };
    let mut foreground = 0;
    let (colors, remainder) = capture.rgba.as_chunks::<4>();
    assert!(remainder.is_empty());
    for (color, depth) in colors.iter().zip(&capture.depth) {
        assert!(depth.is_finite());
        if *color == [255, 0, 0, 255] {
            foreground += 1;
            assert!((*depth - expected).abs() <= 1.0e-6);
        } else {
            assert_eq!(*color, [0, 0, 255, 255]);
            assert_eq!(*depth, 1.);
        }
    }
    assert_eq!(foreground > 0, visible);
}

fn stage(scene: &ViewerScene, label: &str, visible: bool, perspective: bool) {
    scene.render_view().try_redraw().unwrap();
    let capture = scene.render_view().capture().unwrap();
    check_capture(&capture, visible, perspective);
    retained_capture(&capture, label);
}

fn size_stages(scene: &ViewerScene) {
    use Graphic3dRenderingParamsFrustumCulling::{NoUpdate, Off, On};
    camera_mode(scene, false);
    for (label, size, mode, visible) in [
        ("size-visible", 16., On, true),
        ("size-hidden", 128., On, false),
        ("size-no-update-hidden", 0., NoUpdate, false),
        ("size-off-visible", 128., Off, true),
        ("size-no-update-visible", 128., NoUpdate, true),
        ("size-on-hidden", 128., On, false),
    ] {
        layer_limits(&scene.viewer, 0., size);
        scene
            .view
            .change_rendering_params(&mut |params| params.ray_tracing.frustum_culling_state = mode);
        stage(scene, label, visible, false);
    }
}

fn distance_stages(scene: &ViewerScene) {
    camera_mode(scene, true);
    layer_limits(&scene.viewer, 11., 0.);
    stage(scene, "distance-hidden", false, true);
    layer_limits(&scene.viewer, 12., 0.);
    stage(scene, "distance-touch-visible", true, true);
    camera_mode(scene, false);
    layer_limits(&scene.viewer, 11., 0.);
    stage(scene, "ortho-distance-visible", true, false);
    let pers =
        Graphic3dTransformPers::with_point(Graphic3dTransModeFlags::ZOOM_PERS, &Point3::default())
            .unwrap();
    scene
        .structure
        .set_transform_persistence(&Handle::new(pers).cast());
    layer_limits(&scene.viewer, 0., 128.);
    stage(scene, "persistent-size", false, false);
}

fn selection_stages(driver: &Handle<Graphic3dRenderDriver>) {
    let scene = selectable_scene::AisViewerScene::new(driver);
    let view = scene.render_view();
    view.set_z_layer_target(GRAPHIC3D_ZLAYER_ID_DEFAULT);
    view.set_z_layer_redraw_mode(true);
    for (label, size, visible) in [
        ("selection-visible", 0., true),
        ("selection-hidden", 128., false),
    ] {
        layer_limits(&scene.viewer, 0., size);
        view.try_redraw().unwrap();
        let capture = view.capture().unwrap();
        check_capture(&capture, visible, false);
        retained_capture(&capture, label);
        scene.context.move_to(32, 32, &scene.view, false).unwrap();
        assert!(scene.context.has_detected());
        assert!(scene.context.is_displayed(&scene.object.cast()));
        scene.context.clear_detected(false);
    }
}

#[test]
fn actual_layer_visibility_capture_preserves_depth_and_selection() {
    let driver = Graphic3dRenderDriver::new(VulkanOptions {
        validation: true,
        device_index: Some(0),
    })
    .unwrap();
    let scene = captured_scene(&driver);
    size_stages(&scene);
    distance_stages(&scene);
    scene.structure.erase();
    scene.view.remove();
    selection_stages(&driver);
    assert!(driver.validation_errors().unwrap().is_empty());
    println!(
        "{{\"device\":{:?},\"validation_errors\":{},\"states\":12}}",
        driver.device_name(),
        driver.validation_errors().unwrap().len()
    );
}

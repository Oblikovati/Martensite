#[allow(dead_code)]
#[path = "../support/recording_device.rs"]
pub(super) mod recording_device;
#[allow(dead_code)]
#[path = "../support/viewer_scene.rs"]
mod scene;
#[path = "../support/view_setup.rs"]
mod view_setup;
use martensite_tkernel::standard::Handle;
use martensite_tkservice::graphic3d::Graphic3dCViewMembers as _;
use martensite_vulkan::Graphic3dRenderDriver;
use std::sync::atomic::Ordering;

pub(super) fn recorded_scene() -> (
    scene::ViewerScene,
    recording_device::RecordingDevice,
    Handle<Graphic3dRenderDriver>,
) {
    let device = recording_device::RecordingDevice::default();
    let driver = Graphic3dRenderDriver::with_backend(device.clone(), 512).unwrap();
    (scene::ViewerScene::new(&driver), device, driver)
}

#[test]
fn case_stats_defaults_and_capture_bytes_are_owned() {
    let (scene, _, _driver) = recorded_scene();
    let render = scene.render_view();
    let stats = render.rendering_params().stats;
    assert_eq!(stats.collected_stats.0, 15);
    assert_eq!(stats.stats_update_interval, 1.0);
    assert!(!stats.to_show_stats);
    render.try_redraw().unwrap();
    let capture = render.capture().unwrap();
    assert_eq!([capture.width, capture.height], [64, 64]);
    assert_eq!(&capture.rgba[..8], &[0, 0, 91, 255, 1, 0, 91, 255]);
    assert!(capture.depth.iter().all(|&value| value == 0.25));
}

#[test]
fn actual_backend_failure_is_not_a_cached_capture_and_recovery_keeps_input() {
    let (scene, device, _driver) = recorded_scene();
    let render = scene.render_view();
    render.try_redraw().unwrap();
    let before = render.capture().unwrap();
    let vertex_owner = scene.vertices.object_address();
    device.fail.store(true, Ordering::Relaxed);
    assert!(render.try_redraw().is_err());
    assert!(render.capture().is_err());
    assert_eq!(scene.vertices.object_address(), vertex_owner);
    device.fail.store(false, Ordering::Relaxed);
    render.try_redraw().unwrap();
    let after = render.capture().unwrap();
    assert_eq!(before.rgba, after.rgba);
    assert_eq!(before.depth, after.depth);
}

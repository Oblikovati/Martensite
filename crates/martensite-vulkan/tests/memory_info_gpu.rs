//! Exercise the ordinary driver API and an accepted live scene on selected Vulkan profiles.
#[path = "../examples/support/device_options.rs"]
mod device_options;
#[path = "memory_info_gpu/observation.rs"]
mod observation;
#[allow(dead_code)]
#[path = "support/viewer_scene.rs"]
mod scene;
#[path = "support/view_setup.rs"]
mod view_setup;
use martensite_tkernel::tcollection::AsciiString;
use martensite_tkservice::graphic3d::Graphic3dGraphicDriver;
use martensite_vulkan::{FrameCapture, Graphic3dRenderDriver, RenderDeviceMemoryReport};

#[test]
fn real_driver_reports_memory_without_mutating_accepted_frames() {
    let driver = Graphic3dRenderDriver::new(device_options::validation_options()).unwrap();
    let scene = scene::ViewerScene::new(&driver);
    let before = capture_scene(&scene);
    let sample = driver.memory_report().unwrap();
    assert_legacy_success(&driver);
    let after = capture_scene(&scene);
    assert_same_frame(&before, &after);
    assert!(
        driver.validation_errors().unwrap().is_empty(),
        "{:?}",
        driver.validation_errors().unwrap()
    );
    drop(scene);
    drop(driver);
    observation::record_sample(&sample);
    assert_owned_sample(sample);
}
fn capture_scene(scene: &scene::ViewerScene) -> FrameCapture {
    let view = scene.render_view();
    view.try_redraw().unwrap();
    view.capture().unwrap()
}
fn assert_legacy_success(driver: &Graphic3dRenderDriver) {
    let (mut free, mut text) = (0xA55, AsciiString::from("sentinel"));
    assert!(driver.memory_info(&mut free, &mut text));
    assert_eq!(
        free, 0,
        "global free memory is unknown, not budget minus usage"
    );
    assert!(
        std::str::from_utf8(text.as_bytes())
            .unwrap()
            .contains("global free memory: unknown")
    );
}
fn assert_same_frame(before: &FrameCapture, after: &FrameCapture) {
    assert_eq!((before.width, before.height), (64, 64));
    assert_eq!(before.rgba, after.rgba);
    assert_eq!(before.depth, after.depth);
}
fn assert_owned_sample(sample: RenderDeviceMemoryReport) {
    let owned = std::thread::spawn(move || sample).join().unwrap();
    assert!(owned.device().is_some());
    assert!(!owned.heaps().is_empty());
    assert_eq!(owned.global_free_bytes(), None);
}

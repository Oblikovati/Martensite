//! Device work counters of redraws on a real Vulkan device with validation: what a frame
//! created, moved and waited for (phase 0 of docs/features/vulkan-occt-parity/PLAN.md).
#[path = "../examples/support/device_options.rs"]
mod device_options;
#[allow(dead_code)]
#[path = "support/viewer_scene.rs"]
mod scene;
#[allow(dead_code)] // Each caller uses a subset of the shared scene support.
#[path = "support/frame_statistics_scene.rs"]
mod statistics_scene;
#[path = "support/view_setup.rs"]
mod view_setup;
use martensite_tkernel::standard::Handle;
use martensite_vulkan::{Graphic3dRenderDriver, RenderDeviceWork, RenderWorkTimes};
use statistics_scene::StatisticsScene;
use std::time::Duration;

/// The 64x64 test window: colour and depth are read back with four bytes per pixel each.
const READBACK_BYTES: u64 = 2 * 64 * 64 * 4;

fn device_scene() -> (StatisticsScene, Handle<Graphic3dRenderDriver>) {
    let driver = Graphic3dRenderDriver::new(device_options::validation_options()).unwrap();
    (StatisticsScene::mixed(&driver), driver)
}

fn assert_clean_validation(driver: &Graphic3dRenderDriver) {
    let messages = driver.validation_errors().unwrap();
    assert!(messages.is_empty(), "validation messages {messages:?}");
}

/// The counts of `work` without its times, which differ on every redraw.
fn counts(work: RenderDeviceWork) -> RenderDeviceWork {
    RenderDeviceWork {
        times: RenderWorkTimes::default(),
        ..work
    }
}

#[test]
fn a_redraw_reports_the_resources_and_bytes_it_used() {
    let (scene, driver) = device_scene();
    scene.render_view().try_redraw().unwrap();
    let work = scene.render_view().last_frame_work();
    assert_eq!(work.redraws, 1, "{work:?}");
    assert!(work.buffers_created > 0, "{work:?}");
    assert!(work.uploaded_bytes > 0, "{work:?}");
    assert!(work.submissions >= 1, "{work:?}");
    assert!(work.read_back_bytes >= READBACK_BYTES, "{work:?}");
    assert_clean_validation(&driver);
}

#[test]
fn the_view_share_is_the_device_difference_over_the_redraw() {
    let (scene, driver) = device_scene();
    let before = driver.device_work();
    scene.render_view().try_redraw().unwrap();
    let device = driver.device_work().since(&before);
    assert_eq!(device, scene.render_view().last_frame_work());
}

#[test]
fn phase_times_fit_inside_the_redraw() {
    let (scene, _driver) = device_scene();
    scene.render_view().try_redraw().unwrap();
    let times = scene.render_view().last_frame_work().times;
    let phases = times.scene
        + times.upload
        + times.recording
        + times.submission
        + times.waiting
        + times.readback;
    assert!(phases <= times.redraw, "{times:?}");
    assert!(times.scene > Duration::ZERO, "{times:?}");
    assert!(times.upload > Duration::ZERO, "{times:?}");
}

#[test]
fn counts_belong_to_one_redraw_and_do_not_accumulate() {
    let (scene, _driver) = device_scene();
    let view = scene.render_view();
    // The first redraw also creates the pipelines and glyph images later frames reuse.
    view.try_redraw().unwrap();
    view.try_redraw().unwrap();
    let second = counts(view.last_frame_work());
    view.try_redraw().unwrap();
    assert_eq!(counts(view.last_frame_work()), second);
}

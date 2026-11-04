//! Frame statistics on a real Vulkan device with validation: the native-equal counters of
//! scene_mixed, and the Vulkan image-memory counters bounded by their attachment formats.
#[path = "../examples/support/device_options.rs"]
mod device_options;
#[allow(dead_code)]
#[path = "support/viewer_scene.rs"]
mod scene;
#[allow(dead_code)] // Each caller uses a subset of the shared scene support.
#[path = "support/frame_statistics_scene.rs"]
mod statistics_scene;
#[allow(dead_code)]
#[path = "support/surface_texture_scene.rs"]
mod texture_scene;
#[path = "support/view_setup.rs"]
mod view_setup;
use martensite_tkservice::graphic3d::{
    Graphic3dFrameStatsCounter as Counter, Graphic3dFrameStatsMembers as _,
    Graphic3dRenderingParamsPerfCounters as Perf,
};
use martensite_vulkan::Graphic3dRenderDriver;
use statistics_scene::{StatisticsScene, collect_every_frame};

/// The 64x64 test window: sRGB RGBA8 color plus at least a 32-bit depth attachment.
const MINIMUM_ATTACHMENT_BYTES: usize = 64 * 64 * (4 + 4);

fn device_scene() -> (
    StatisticsScene,
    martensite_tkernel::standard::Handle<Graphic3dRenderDriver>,
) {
    let driver = Graphic3dRenderDriver::new(device_options::validation_options()).unwrap();
    let scene = StatisticsScene::mixed(&driver);
    collect_every_frame(&scene.view, Perf::ALL);
    (scene, driver)
}

fn assert_clean_validation(driver: &Graphic3dRenderDriver) {
    let messages = driver.validation_errors().unwrap();
    assert!(messages.is_empty(), "validation messages {messages:?}");
}

#[test]
fn device_frames_report_the_native_scene_counters() {
    let (scene, driver) = device_scene();
    scene.render_view().try_redraw().unwrap();
    let stats = scene.render_view().frame_stats();
    assert_eq!(stats.counter_value(Counter::NbStructsNotCulled), 10);
    assert_eq!(stats.counter_value(Counter::NbTrianglesNotCulled), 30);
    assert_eq!(stats.counter_value(Counter::NbLinesNotCulled), 5);
    assert_eq!(stats.counter_value(Counter::NbPointsNotCulled), 5);
    assert_eq!(stats.counter_value(Counter::EstimatedBytesGeom), 1272);
    assert_clean_validation(&driver);
}

#[test]
fn attachment_memory_is_the_device_allocation_of_each_frame() {
    let (scene, driver) = device_scene();
    let view = scene.render_view();
    view.try_redraw().unwrap();
    let first = view
        .frame_stats()
        .counter_value(Counter::EstimatedBytesFbos);
    view.try_redraw().unwrap();
    let second = view
        .frame_stats()
        .counter_value(Counter::EstimatedBytesFbos);
    assert!(first >= MINIMUM_ATTACHMENT_BYTES, "attachments {first}");
    assert_eq!(
        first, second,
        "repeated frames must not accumulate attachments"
    );
    assert_eq!(second, view.image_memory().frame_attachment_bytes);
    assert_clean_validation(&driver);
}

#[test]
fn attachment_memory_grows_with_the_view_extent() {
    let (scene, driver) = device_scene();
    let view = scene.render_view();
    view.try_redraw().unwrap();
    let small = view
        .frame_stats()
        .counter_value(Counter::EstimatedBytesFbos);
    scene.scene.window.set_size(128, 128);
    scene.view.must_be_resized();
    view.try_redraw().unwrap();
    let large = view
        .frame_stats()
        .counter_value(Counter::EstimatedBytesFbos);
    assert!(large >= 4 * MINIMUM_ATTACHMENT_BYTES, "attachments {large}");
    assert!(large > small, "128 px {large} <= 64 px {small}");
    assert_clean_validation(&driver);
}

#[test]
fn texture_memory_matches_the_live_sampled_images() {
    let (scene, driver) = device_scene();
    let view = scene.render_view();
    view.try_redraw().unwrap();
    let counted = view
        .frame_stats()
        .counter_value(Counter::EstimatedBytesTextures);
    assert_eq!(counted, view.image_memory().sampled_image_bytes);
    assert_clean_validation(&driver);
}

#[test]
fn sampled_texture_images_count_as_texture_memory() {
    let driver = Graphic3dRenderDriver::new(device_options::validation_options()).unwrap();
    let fixture = texture_scene::SurfaceTextureScene::with_mode(&driver, "nearest");
    collect_every_frame(&fixture.scene.view, Perf::ALL);
    let view = fixture.scene.render_view();
    view.try_redraw().unwrap();
    let counted = view
        .frame_stats()
        .counter_value(Counter::EstimatedBytesTextures);
    // The 2x2 RGBA8 fixture texture is the only sampled image of this device.
    assert!(counted >= 2 * 2 * 4, "texture bytes {counted}");
    assert_eq!(counted, view.image_memory().sampled_image_bytes);
    assert_clean_validation(&driver);
}

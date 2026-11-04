//! Actual injected submissions prove explicit extents without changing the physical window.
use crate::case_view::viewer_session::ViewerSession;
use crate::hatch_surface::scene::SourceViewOutput as _;
use crate::source_output::{self, SourceCompletion};
#[path = "../support/recording_device.rs"]
mod recording_device;
use martensite_tkservice::graphic3d::{
    Graphic3dCViewMembers, Graphic3dGraphicDriver, Graphic3dTypeOfLimit,
};
use martensite_tkservice::image::ImagePixMap;
use martensite_vulkan::Graphic3dRenderDriver;

fn recorded_capture(
    name: &'static str,
) -> (
    SourceCompletion<ViewerSession>,
    recording_device::RecordingDevice,
) {
    let (driver, session, device) = recorded_owner();
    let completion =
        source_output::sized_capture::capture_case_sized(driver, session, name, (137, 100))
            .unwrap();
    (completion, device)
}

fn recorded_owner() -> (
    martensite_tkernel::standard::Handle<Graphic3dRenderDriver>,
    ViewerSession,
    recording_device::RecordingDevice,
) {
    let device = recording_device::RecordingDevice::default();
    let driver = Graphic3dRenderDriver::with_backend(device.clone(), 4096).unwrap();
    let session = ViewerSession::with_view_size(&driver, 409, 409).unwrap();
    (driver, session, device)
}

#[test]
fn both_explicit_dump_dimensions_keep_the_case_window_until_end() {
    let (mut completion, device) = recorded_capture("sized_control_width.png");
    assert_eq!(device.last_frame().extent(), [137, 100]);
    completion
        .capture_additional_sized("sized_control_height.png", (100, 137))
        .unwrap();
    assert_eq!(device.last_frame().extent(), [100, 137]);
    assert_eq!(completion.scene.source_window_size(), (409, 409));
    source_output::sized_capture::recorded_cpu_publication(&completion).unwrap();
    completion.scene.close();
}

#[test]
fn explicit_extent_preserves_the_same_actual_borrowed_view() {
    let (completion, _) = recorded_capture("sized_control_identity.png");
    let view = completion.scene.view().clone();
    let image = ImagePixMap::default();
    assert!(completion.scene.source_dump(&image).unwrap());
    assert_eq!((image.width(), image.height()), (409, 409));
    assert_eq!(completion.scene.view(), &view);
    source_output::sized_capture::recorded_cpu_publication(&completion).unwrap();
    completion.scene.close();
}

#[test]
fn actual_unavailable_texture_capacity_is_not_replaced_with_a_constant() {
    let device = recording_device::RecordingDevice::default();
    let driver = Graphic3dRenderDriver::with_backend(device, 4096).unwrap();
    assert_eq!(
        driver.inquire_limit(Graphic3dTypeOfLimit::MaxTextureSize),
        0
    );
}

#[test]
fn later_injected_dump_failure_does_not_become_a_successful_capture() {
    let (mut completion, device) = recorded_capture("sized_control_before_failure.png");
    device
        .fail
        .store(true, std::sync::atomic::Ordering::Relaxed);
    let result = completion.capture_additional_sized("sized_control_failure.png", (100, 137));
    if result.is_ok() {
        assert!(source_output::sized_capture::recorded_cpu_publication(&completion).is_err());
        completion.scene.close();
        return;
    }
    assert_eq!(completion.scene.source_window_size(), (409, 409));
}

#[test]
fn injected_cpu_publication_never_claims_available_gpu_validation() {
    let device = recording_device::RecordingDevice::default();
    let driver = Graphic3dRenderDriver::with_backend(device, 4096).unwrap();
    let failure = driver
        .validation_errors()
        .expect_err("CPU recorder has no GPU validation observer");
    assert_eq!(
        failure.cause(),
        martensite_vulkan::RenderValidationObservationCause::Unavailable
    );
    assert!(!failure.was_enabled());
}

fn recorded_options(
    name: &'static str,
) -> (
    SourceCompletion<ViewerSession>,
    recording_device::RecordingDevice,
) {
    let (driver, scene, device) = recorded_owner();
    let options = explicit_options();
    let capture =
        source_output::options_capture::capture_case_options(driver, scene, name, &options)
            .unwrap();
    (capture, device)
}

fn explicit_options() -> martensite_tkv3d::v3d::V3dImageDumpOptions<'static> {
    martensite_tkv3d::v3d::V3dImageDumpOptions {
        width: 37,
        height: 19,
        target_z_layer_id: 0,
        is_single_layer: true,
        ..Default::default()
    }
}

fn require_options_restored(scene: &ViewerSession) {
    use martensite_tkservice::graphic3d::GRAPHIC3D_ZLAYER_ID_BOT_OSD;
    assert_eq!(scene.source_window_size(), (409, 409));
    let view = scene.view().get().unwrap().view();
    assert_eq!(view.z_layer_target(), GRAPHIC3D_ZLAYER_ID_BOT_OSD);
    assert!(!view.z_layer_redraw_mode());
}

#[test]
fn explicit_buffer_and_layer_captures_keep_the_actual_view_and_required_media() {
    let (mut completion, device) = recorded_options("options_control_rgb.png");
    let view = completion.scene.view().clone();
    assert_eq!(device.last_frame().extent(), [37, 19]);
    let mut options = explicit_options();
    options.buffer_type = martensite_tkservice::graphic3d::Graphic3dBufferType::Depth;
    completion
        .capture_additional_options("options_control_depth.png", &options)
        .unwrap();
    assert_eq!(completion.scene.view(), &view);
    require_options_restored(&completion.scene);
    source_output::sized_capture::recorded_cpu_publication(&completion).unwrap();
    completion.scene.close();
}

#[test]
fn later_explicit_option_failure_is_never_reported_as_complete_publication() {
    let (mut completion, device) = recorded_options("options_control_before_failure.png");
    device
        .fail
        .store(true, std::sync::atomic::Ordering::Relaxed);
    let result =
        completion.capture_additional_options("options_control_failure.png", &explicit_options());
    if result.is_ok() {
        assert!(source_output::sized_capture::recorded_cpu_publication(&completion).is_err());
    }
    assert_eq!(completion.scene.source_window_size(), (409, 409));
    completion.scene.close();
}

#[test]
fn returned_capture_error_releases_the_actual_installed_frame_hook() {
    let (_driver, session, _device) = recorded_owner();
    let view = crate::session_run::render_view(&session).unwrap();
    let record = crate::tie_edges::SubmittedDraws::install(&view);
    assert_eq!(std::sync::Arc::strong_count(&record), 2);
    let result: Result<(), Box<dyn std::error::Error>> =
        crate::session_run::capture_then_clear(&view, || Err("injected capture refusal".into()));
    assert_eq!(result.unwrap_err().to_string(), "injected capture refusal");
    assert_eq!(std::sync::Arc::strong_count(&record), 1);
    session.close();
}

#[test]
fn successful_capture_releases_the_actual_installed_frame_hook() {
    let (_driver, session, _device) = recorded_owner();
    let view = crate::session_run::render_view(&session).unwrap();
    let record = crate::tie_edges::SubmittedDraws::install(&view);
    assert_eq!(std::sync::Arc::strong_count(&record), 2);
    assert_eq!(
        crate::session_run::capture_then_clear(&view, || Ok(17)).unwrap(),
        17
    );
    assert_eq!(std::sync::Arc::strong_count(&record), 1);
    session.close();
}

#[test]
fn actual_explicit_412_and_500_dumps_preserve_the_409_window() {
    for (size, name) in [(412, "options_412.png"), (500, "options_500.png")] {
        let (driver, scene, device) = recorded_owner();
        let options = martensite_tkv3d::v3d::V3dImageDumpOptions {
            width: size,
            height: size,
            ..Default::default()
        };
        let capture =
            source_output::options_capture::capture_case_options(driver, scene, name, &options)
                .unwrap();
        assert_eq!(device.last_frame().extent(), [size as u32, size as u32]);
        assert_eq!(capture.scene.source_window_size(), (409, 409));
        source_output::sized_capture::recorded_cpu_publication(&capture).unwrap();
        capture.scene.close();
    }
}

#[test]
fn actual_submitted_camera_tile_marks_last_pass_geometry_incomplete() {
    use martensite_tkernel::ncollection::Vec2;
    use martensite_tkservice::graphic3d::{Graphic3dCameraTile, Graphic3dDataStructureManager};
    let (_driver, scene, _device) = recorded_owner();
    let view = crate::session_run::render_view(&scene).unwrap();
    let camera = view.camera();
    let record = crate::tie_edges::SubmittedDraws::install(&view);
    assert!(!record.observed_tile());
    camera.set_tile(&Graphic3dCameraTile {
        total_size: Vec2::new(500, 500),
        tile_size: Vec2::new(300, 300),
        ..Default::default()
    });
    scene.view().redraw();
    view.set_frame_hooks(None);
    assert!(record.observed_tile());
    scene.close();
}

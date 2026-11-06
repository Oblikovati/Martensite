//! Real injected backend controls of source measurement and deferred command updates.

#[test]
fn actual_injected_source_measurement_executes_one_hundred_and_one_redraws() {
    use super::culling_cpu_baseline::recording_device::RecordingDevice;
    use crate::case_view::viewer_session::ViewerSession;
    let device = RecordingDevice::default();
    let driver =
        martensite_vulkan::Graphic3dRenderDriver::with_backend(device.clone(), 512).unwrap();
    let session = ViewerSession::with_graphic_driver(&driver.cast(), 16, 16).unwrap();
    let before = device.frames.read().len();
    let measured = session.measure_fps().unwrap();
    assert_eq!(measured.completed_frames(), 100);
    assert_eq!(device.frames.read().len() - before, 101);
    assert!(measured.command_text().starts_with("FPS: "));
}

#[test]
fn actual_no_update_scope_does_not_schedule_an_end_of_scope_frame() {
    use super::culling_cpu_baseline::recording_device::RecordingDevice;
    use crate::case_view::viewer_session::ViewerSession;
    let device = RecordingDevice::default();
    let driver =
        martensite_vulkan::Graphic3dRenderDriver::with_backend(device.clone(), 512).unwrap();
    let mut session = ViewerSession::with_graphic_driver(&driver.cast(), 16, 16).unwrap();
    let before = device.frames.read().len();
    session
        .without_updates(|current| current.set_perf_update_interval(0.))
        .unwrap();
    assert_eq!(device.frames.read().len(), before);
    session.view().redraw();
    assert_eq!(device.frames.read().len(), before + 1);
}

#[test]
fn startup_refuses_a_removed_source_view_instead_of_printing_fps() {
    use super::culling_cpu_baseline::recording_device::RecordingDevice;
    use crate::case_view::viewer_session::{PerformanceFailure, ViewerSession};
    let device = RecordingDevice::default();
    let driver = martensite_vulkan::Graphic3dRenderDriver::with_backend(device, 512).unwrap();
    let session = ViewerSession::with_graphic_driver(&driver.cast(), 16, 16).unwrap();
    session.view().remove();
    assert!(matches!(
        session.measure_fps(),
        Err(PerformanceFailure::Redraw {
            timed_frame: None,
            ..
        })
    ));
}

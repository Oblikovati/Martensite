//! Live server raster capture and allocation diagnostics.
#![cfg(all(target_os = "linux", feature = "x11-integration-tests"))]
use martensite_raster_core::{
    PixelSegment, RasterFailureContext, RasterWindowSystem, VirtualWindowSystem,
    WindowFailureContext,
};
use martensite_tkernel::standard::Failure;
struct CaptureFailureContext;
impl WindowFailureContext for CaptureFailureContext {
    fn failure(value: &str) -> Failure {
        Failure::with_message("CaptureFailure", value)
    }
}
#[test]
fn virtual_window_saves_actual_palette_pixels() {
    let system = VirtualWindowSystem::<RasterFailureContext>::connect_virtual().unwrap();
    let mut window = system
        .try_create_window("Capture", (5, 8), (16, 16), 0)
        .unwrap();
    assert_eq!(window.position(), (5, 8));
    assert_eq!((window.width(), window.height()), (16, 16));
    assert!(!window.is_mapped());
    window.clear();
    window.set_color(1);
    window.draw_segments(&[PixelSegment::new(1, 2, 12, 2)]);
    window.redraw();
    assert_saved_pixels(window.as_ref());
    drop(window);
    assert_eq!(system.next_event().kind, 0);
}
fn assert_saved_pixels(window: &dyn martensite_raster_core::RasterWindow) {
    let path = std::env::temp_dir().join(format!("raster-core-{}.ppm", std::process::id()));
    assert!(window.save(path.to_str().unwrap()));
    let bytes = std::fs::read(&path).unwrap();
    std::fs::remove_file(path).unwrap();
    assert!(bytes.starts_with(b"P6\n16 16\n255\n"));
    assert_eq!(bytes.len(), 13 + 16 * 16 * 3);
    assert!(bytes[13..].as_chunks::<3>().0.contains(&[255, 0, 0]));
}

#[test]
fn allocation_failure_keeps_injected_diagnostic_context() {
    let system = VirtualWindowSystem::<CaptureFailureContext>::connect_virtual().unwrap();
    let error = match system.try_create_window("Capture", (0, 0), (16, 16), 7) {
        Ok(_) => panic!("existing window 7: expected allocation refusal"),
        Err(error) => error,
    };
    assert_eq!(error.message(), "existing window=7");
    assert!(format!("{error:?}").contains("CaptureFailure"));
    assert!(!system.define_color(15, "Red"));
}

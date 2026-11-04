//! Actual retained 2D presentation admission needs no Vulkan device.
use super::*;
#[path = "../../tests/support/recording_device.rs"]
mod recording_device;
use crate::hatch_surface::scene::viewer_session::ViewerSession;
use recording_device::RecordingDevice;
fn retained_scale_frame() -> crate::RenderFrame {
    let recorder = RecordingDevice::default();
    let driver = crate::Graphic3dRenderDriver::with_backend(recorder.clone(), 4096).unwrap();
    let mut session = ViewerSession::with_view_size(&driver, 409, 409).unwrap();
    let scale = session.color_scale("actual-scale").unwrap();
    session.show_color_scale("actual-scale", &scale).unwrap();
    session.view().redraw();
    recorder.last_frame()
}
#[test]
fn actual_retained_color_scale_line_admission() {
    let frame = retained_scale_frame();
    let lines: Vec<_> = frame
        .draws()
        .iter()
        .filter(|d| d.topology() == PrimitiveTopology::Lines)
        .collect();
    assert_eq!(lines.len(), 1);
    let choice = RenderThinAxisCoverage::FixedGridV2 {
        grid: crate::RenderLineGrid::Fractional12,
        origin: crate::RenderLineOrigin::BottomUp,
    };
    let output = selected_draw(choice, lines[0], frame.extent(), true)
        .unwrap()
        .unwrap();
    assert_eq!(output.vertices.len(), 16);
    assert_eq!(output.polygon_offset, lines[0].polygon_offset);
    assert_eq!(output.depth_clamp, lines[0].depth_clamp);
}

//! The real anonymous presentation must reach the injected device, not merely store aspects.
use super::*;
#[path = "../support/recording_device.rs"]
pub(super) mod recording_device;
use martensite_vulkan::{Graphic3dRenderDriver, Graphic3dRenderView, PrimitiveDraw, RenderFrame};
use recording_device::RecordingDevice;

fn recorded_point_scene() -> (ViewerSession, RecordingDevice) {
    let device = RecordingDevice::default();
    let driver = Graphic3dRenderDriver::with_backend(device.clone(), 4096).unwrap();
    let mut session = ViewerSession::with_graphic_driver(&driver.cast(), 409, 409).unwrap();
    point_frame::display_point_frame(&mut session).unwrap();
    (session, device)
}
fn render_view(session: &ViewerSession) -> Handle<Graphic3dRenderView> {
    let view = session.view().get().unwrap();
    view.view().cast::<Graphic3dRenderView>()
}
fn assert_point_face(draw: &PrimitiveDraw) {
    assert_eq!(
        draw.topology(),
        martensite_vulkan::PrimitiveTopology::Triangles
    );
    assert!(draw.point_style().is_none());
    assert!(!draw.indices().is_empty());
}
fn assert_separate_polygon_point_and_text(frame: &RenderFrame) {
    let point_faces: Vec<_> = frame
        .draws()
        .iter()
        .filter(|draw| draw.rasterizes_polygon_points())
        .collect();
    assert_eq!(point_faces.len(), 1);
    assert_point_face(point_faces[0]);
    assert!(
        frame
            .draws()
            .iter()
            .any(|draw| !draw.rasterizes_polygon_points())
    );
}
#[test]
fn anonymous_point_frame_reaches_the_actual_renderer() {
    let (session, device) = recorded_point_scene();
    render_view(&session)
        .try_redraw()
        .expect("actual point interior and separate text must render");
    assert_separate_polygon_point_and_text(&device.last_frame());
}
fn filled_triangle_count(draws: &[PrimitiveDraw]) -> usize {
    draws
        .iter()
        .filter(|d| !d.rasterizes_polygon_points())
        .map(|d| d.indices().len() / 3)
        .sum()
}
#[test]
fn actual_polygon_point_draws_do_not_supply_filled_triangle_coverage() {
    let (session, _) = recorded_point_scene();
    let view = render_view(&session);
    let record = crate::tie_edges::SubmittedDraws::install(&view);
    view.try_redraw().unwrap();
    let draws = record.draws();
    assert!(draws.iter().any(|d| d.rasterizes_polygon_points()));
    assert_eq!(
        record.triangles(409., 409.).len(),
        filled_triangle_count(&draws)
    );
    view.set_frame_hooks(None);
}

#[test]
fn point_frame_cpu_diagnostic_retains_actual_draw_and_lighting_inputs() {
    let (session, device) = recorded_point_scene();
    render_view(&session).try_redraw().unwrap();
    let frame = device.last_frame();
    let faces: Vec<_> = frame
        .draws()
        .iter()
        .filter(|d| d.rasterizes_polygon_points())
        .collect();
    assert_eq!(faces.len(), 1);
    assert_point_face(faces[0]);
    assert_eq!(faces[0].vertices().len(), 4);
    assert_eq!(faces[0].indices(), &[0, 1, 2, 1, 2, 3]);
    report_point_cpu(faces[0]);
    report_point_layers_cpu(&frame);
}

fn report_point_cpu(draw: &PrimitiveDraw) {
    // Debug is retained as a JSON string: CPU submission inputs carry no GPU coverage claim.
    println!(
        "{{\"event\":\"point_frame_cpu_inputs\",\"draw_debug\":{:?}}}",
        format!("{draw:#?}")
    );
}

fn report_point_layers_cpu(frame: &RenderFrame) {
    println!(
        "{{\"event\":\"point_frame_cpu_layers\",\"layers_debug\":{:?}}}",
        format!("{:#?}", frame.layers())
    );
}

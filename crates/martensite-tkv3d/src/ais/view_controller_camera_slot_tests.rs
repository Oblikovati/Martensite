//! Frame and camera-action overrides, installed as both hook owners, match the native trace.
use super::view_controller_event_slot_fixture::EventSlotMode;
use super::view_controller_frame_slot_fixture::{CAMERA_SLOTS, FrameSlotRecorder};
use super::view_controller_slot_trace::{SlotTrace, attempt_outcome, bits, slot_trace_case};
use super::*;
use crate::v3d::{V3dViewSource, V3dViewer};
use martensite_tkernel::ncollection::Vec2;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkmath::gp::{Direction3, Point3};
use martensite_tkservice::aspect::AspectNeutralWindow;
use martensite_tkservice::aspect::{
    ASPECT_VKEY_FLAGS_CTRL, AspectScrollDelta, AspectVKey, AspectVKeyBasic,
};
use martensite_tkservice::graphic3d::{Graphic3dCameraFocusType, Graphic3dCameraProjection};
use martensite_tkservice::null_driver::NullGraphicDriver;
use std::sync::Arc;

const CAMERA_SLOT_BASELINE: &str =
    include_str!("../../tests/fixtures/controller-camera-slots-baseline.jsonl");
const FORWARD: AspectVKey = AspectVKeyBasic::NavForward as AspectVKey;

/// The witness scene, in its order: a mapped 640x480 view whose camera becomes stereo first,
/// then eye (0,0,200), Y up, scale 200 (which moves a perspective eye), aspect 4:3, focus 1.
fn camera_slot_scene() -> (
    Handle<V3dViewer>,
    Handle<AisInteractiveContext>,
    Handle<dyn V3dViewSource>,
) {
    let viewer = V3dViewer::new(&NullGraphicDriver::new_handle()).unwrap();
    let view = viewer.create_view().unwrap();
    let context = AisInteractiveContext::new(&viewer).unwrap();
    let window = Handle::new(AspectNeutralWindow::default());
    window.set_size(640, 480);
    view.set_window(&window.cast(), 0).unwrap();
    view.set_immediate_update(false);
    let camera = view.camera();
    camera.set_projection_type(Graphic3dCameraProjection::Stereo);
    camera
        .set_eye_and_center(&Point3::new(0.0, 0.0, 200.0), &Point3::default())
        .unwrap();
    camera.set_up(&Direction3::Y);
    camera.set_scale(200.0);
    camera.set_aspect(4.0 / 3.0);
    camera.set_z_focus(Graphic3dCameraFocusType::Relative, 1.0);
    (viewer, context, view)
}
/// Queue a CTRL scroll and a plain scroll, pan and orbit starts and a forward key; handle a frame.
fn run_camera_slot(r: &FrameSlotRecorder, c: &mut AisViewController) -> Result<(), Failure> {
    c.input_base_mut().event_timer.stop();
    c.input_base_mut().event_timer.reset_to(1.0);
    c.update_mouse_scroll(&AspectScrollDelta::new(
        Vec2::new(320, 240),
        1.0,
        ASPECT_VKEY_FLAGS_CTRL,
    ))?;
    c.update_mouse_scroll(&AspectScrollDelta::new(Vec2::new(300, 200), 2.0, 0))?;
    let ui = c.change_input_buffer(AisViewInputBufferType::Ui);
    ui.panning.to_start = true;
    ui.panning.point_start = Vec2::new(100, 100);
    ui.panning.to_pan = true;
    ui.panning.delta = Vec2::new(5, -3);
    ui.orbit_rotation.to_start = true;
    ui.orbit_rotation.to_rotate = true;
    ui.orbit_rotation.point_start = Vec2::new(300.0, 200.0);
    ui.orbit_rotation.point_to = Vec2::new(320.0, 210.0);
    c.key_down(FORWARD, 1.0, 1.0)?;
    c.flush_view_events(&r.context, &r.view, true)
}
fn camera_queue(b: &AisViewInputBuffer) -> String {
    let (pan, orbit) = (&b.panning, &b.orbit_rotation);
    format!(
        "{}/{}{}/{}{}",
        b.zoom_actions.len(),
        pan.to_start as i32,
        pan.to_pan as i32,
        orbit.to_start as i32,
        orbit.to_rotate as i32
    )
}
fn camera_triple(x: f64, y: f64, z: f64) -> String {
    format!("{},{},{}", bits(x), bits(y), bits(z))
}
fn camera_slot_state(r: &FrameSlotRecorder, c: &AisViewController) -> String {
    let camera = r.view.camera();
    let (eye, center, up) = (camera.eye(), camera.center(), camera.up());
    format!(
        "{{\"case\":\"{}\",\"phase\":\"state\",\"ui\":\"{}\",\"gl\":\"{}\",\"eye\":\"{}\",\"center\":\"{}\",\"up\":\"{}\",\"zfocus\":\"{}\",\"forward\":{}}}",
        r.trace.case.name,
        camera_queue(&c.ui_input),
        camera_queue(&c.render_input),
        camera_triple(eye.x(), eye.y(), eye.z()),
        camera_triple(center.x(), center.y(), center.z()),
        camera_triple(up.x(), up.y(), up.z()),
        bits(camera.z_focus()),
        c.keys().is_key_down(FORWARD).unwrap() as i32
    )
}
fn attempt_camera_slot(r: &FrameSlotRecorder, c: &mut AisViewController) {
    let failure = attempt_outcome(|| run_camera_slot(r, c));
    r.trace.finish_attempt(failure, camera_slot_state(r, c));
}
fn assert_camera_slot_case(name: &'static str) {
    let case = slot_trace_case(name, CAMERA_SLOTS, "camera");
    let (_viewer, context, view) = camera_slot_scene();
    let recorder = Arc::new(FrameSlotRecorder::new(SlotTrace::new(case), context, view));
    let mut c = AisViewController::new().unwrap();
    // One shared owner overrides both hook traits, as the native derived class does.
    c.set_frame_hooks(recorder.clone());
    c.set_camera_hooks(recorder.clone());
    attempt_camera_slot(&recorder, &mut c);
    if matches!(case.mode, EventSlotMode::Typed | EventSlotMode::Unrelated) {
        attempt_camera_slot(&recorder, &mut c);
    }
    recorder.trace.assert_native(CAMERA_SLOT_BASELINE);
}
macro_rules! camera_slot_modes {
    ($($slot:ident => $name:literal),* $(,)?) => {
        $(mod $slot {
            use super::assert_camera_slot_case;
            #[test]
            fn direct() { assert_camera_slot_case(concat!($name, "-direct")); }
            #[test]
            fn delegate() { assert_camera_slot_case(concat!($name, "-delegate")); }
            #[test]
            fn reenter() { assert_camera_slot_case(concat!($name, "-reenter")); }
            #[test]
            fn typed() { assert_camera_slot_case(concat!($name, "-typed")); }
            #[test]
            fn unrelated() { assert_camera_slot_case(concat!($name, "-unrelated")); }
        })*
    };
}
camera_slot_modes!(
    cameraactions => "cameraactions",
    handlezoom => "handlezoom",
    focus => "focus",
    pickpoint => "pickpoint",
    gravity => "gravity",
    navigation => "navigation",
    handleevents => "handleevents",
    flushevents => "flushevents",
);

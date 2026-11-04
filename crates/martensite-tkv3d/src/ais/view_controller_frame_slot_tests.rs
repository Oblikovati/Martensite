//! Queueing, transfer and per-frame camera overrides match the native derived-controller trace.
use super::view_controller_action_scene_fixture::action_mapped_view;
use super::view_controller_event_slot_fixture::EventSlotMode;
use super::view_controller_frame_slot_fixture::{FRAME_SLOTS, FrameSlotRecorder};
use super::view_controller_slot_trace::{
    SlotTrace, attempt_outcome, bits, dir_bits, point_bits, slot_trace_case,
};
use super::*;
use crate::v3d::{V3dTypeOfOrientation, V3dViewSource, V3dViewer};
use martensite_tkernel::ncollection::Vec2;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkmath::gp::{Axis1, Direction3, Point3};
use martensite_tkservice::aspect::{
    AspectScrollDelta, AspectVKey, AspectVKeyBasic, AspectWindowInputListener,
};
use martensite_tkservice::graphic3d::{Graphic3dCameraFocusType, Graphic3dCameraProjection};
use martensite_tkservice::null_driver::NullGraphicDriver;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::Arc;

const FRAME_SLOT_BASELINE: &str =
    include_str!("../../tests/fixtures/controller-frame-slots-baseline.jsonl");
const FIT_ALL_KEY: AspectVKey = AspectVKeyBasic::ViewFitAll as AspectVKey;

/// The witness scene: orthographic camera at (0,0,200), Y up, scale 200, aspect 4:3.
fn frame_slot_scene() -> (
    Handle<V3dViewer>,
    Handle<AisInteractiveContext>,
    Handle<dyn V3dViewSource>,
) {
    let viewer = V3dViewer::new(&NullGraphicDriver::new_handle()).unwrap();
    let view = action_mapped_view(&viewer);
    let context = AisInteractiveContext::new(&viewer).unwrap();
    let camera = view.camera();
    camera
        .set_eye_and_center(&Point3::new(0.0, 0.0, 200.0), &Point3::default())
        .unwrap();
    camera.set_up(&Direction3::Y);
    camera.set_scale(200.0);
    camera.set_aspect(4.0 / 3.0);
    camera.set_projection_type(Graphic3dCameraProjection::Orthographic);
    camera.set_z_focus(Graphic3dCameraFocusType::Relative, 1.0);
    (viewer, context, view)
}
/// Queue pan, orbit and view rotation in the UI buffer, as a mouse drag would.
fn queue_frame_rotations(c: &mut AisViewController) {
    let ui = c.change_input_buffer(AisViewInputBufferType::Ui);
    ui.panning.to_start = true;
    ui.panning.point_start = Vec2::new(100, 100);
    ui.panning.to_pan = true;
    ui.panning.delta = Vec2::new(5, -3);
    ui.orbit_rotation.to_start = true;
    ui.orbit_rotation.to_rotate = true;
    ui.orbit_rotation.point_start = Vec2::new(300.0, 200.0);
    ui.orbit_rotation.point_to = Vec2::new(320.0, 210.0);
    ui.view_rotation.to_start = true;
    ui.view_rotation.to_rotate = true;
    ui.view_rotation.point_start = Vec2::new(300.0, 200.0);
    ui.view_rotation.point_to = Vec2::new(310.0, 190.0);
}
/// Queue input through the public virtuals, flush and handle one frame, then lose focus.
fn run_frame_slot(r: &FrameSlotRecorder, c: &mut AisViewController) -> Result<(), Failure> {
    r.trace.results.lock().unwrap().clear();
    let keep = |value: bool| r.trace.keep(value);
    keep(c.update_z_rotation(0.25)?);
    keep(c.update_mouse_scroll(&AspectScrollDelta::new(Vec2::new(320, 240), 2.0, 0))?);
    queue_frame_rotations(c);
    c.key_down(FIT_ALL_KEY, 0.0, 1.0)?;
    c.update_view_orientation(V3dTypeOfOrientation::Xpos, false)?;
    let axis = Axis1::new(Point3::default(), Direction3::Z);
    let mut point = *r.axis_point.lock().unwrap();
    let hit = c.pick_axis(&mut point, &r.context, r.view.view_base(), axis);
    *r.axis_point.lock().unwrap() = point;
    keep(hit?);
    c.flush_view_events(&r.context, &r.view, true)?;
    c.process_focus(false)
}
fn attempt_frame_slot(r: &FrameSlotRecorder, c: &mut AisViewController) {
    let failure = attempt_outcome(|| run_frame_slot(r, c));
    r.trace.finish_attempt(failure, frame_slot_state(r, c));
}
/// Transferable queue flags of one buffer, in the witness `Queue()` layout.
fn frame_queue(b: &AisViewInputBuffer) -> String {
    let zoom = b
        .zoom_actions
        .last()
        .map(|z| format!("{},{},{}", z.point.x, z.point.y, bits(z.delta)))
        .unwrap_or_default();
    let (pan, orbit, rotation) = (&b.panning, &b.orbit_rotation, &b.view_rotation);
    let (z, orient) = (&b.z_rotate, &b.orientation);
    format!(
        "{}:{zoom}/{}{},{},{}/{}{}/{}{}/{},{}/{}{},{}/{}{}",
        b.zoom_actions.len(),
        pan.to_start as i32,
        pan.to_pan as i32,
        pan.delta.x,
        pan.delta.y,
        orbit.to_start as i32,
        orbit.to_rotate as i32,
        rotation.to_start as i32,
        rotation.to_rotate as i32,
        z.to_rotate as i32,
        bits(z.angle),
        orient.to_fit_all as i32,
        orient.to_set_view_orient as i32,
        orient.view_orient as i32,
        b.dragging.to_abort as i32,
        b.is_new_gesture as i32
    )
}
fn frame_slot_state(r: &FrameSlotRecorder, c: &AisViewController) -> String {
    let results: Vec<String> = r
        .trace
        .results
        .lock()
        .unwrap()
        .iter()
        .map(i32::to_string)
        .collect();
    let camera = r.view.camera();
    let fit_key = c.keys().is_key_down(FIT_ALL_KEY).unwrap();
    format!(
        "{{\"case\":\"{}\",\"phase\":\"state\",\"results\":\"{}\",\"ui\":\"{}\",\"gl\":\"{}\",\"eye\":\"{}\",\"center\":\"{}\",\"up\":\"{}\",\"scale\":\"{}\",\"fit_key\":{},\"axis_point\":\"{}\"}}",
        r.trace.case.name,
        results.join(","),
        frame_queue(&c.ui_input),
        frame_queue(&c.render_input),
        point_bits(&camera.eye()),
        point_bits(&camera.center()),
        dir_bits(&camera.up()),
        bits(camera.scale()),
        fit_key as i32,
        point_bits(&r.axis_point.lock().unwrap())
    )
}
fn assert_frame_slot_case(name: &'static str) {
    let case = slot_trace_case(name, FRAME_SLOTS, "frame");
    let (_viewer, context, view) = frame_slot_scene();
    let recorder = Arc::new(FrameSlotRecorder::new(SlotTrace::new(case), context, view));
    let mut c = AisViewController::new().unwrap();
    c.set_allow_touch_z_rotation(true);
    c.set_frame_hooks(recorder.clone());
    attempt_frame_slot(&recorder, &mut c);
    if matches!(case.mode, EventSlotMode::Typed | EventSlotMode::Unrelated) {
        attempt_frame_slot(&recorder, &mut c);
    }
    recorder.trace.assert_native(FRAME_SLOT_BASELINE);
}
macro_rules! frame_slot_modes {
    ($($slot:ident => $name:literal),* $(,)?) => {
        $(mod $slot {
            use super::assert_frame_slot_case;
            #[test]
            fn direct() { assert_frame_slot_case(concat!($name, "-direct")); }
            #[test]
            fn delegate() { assert_frame_slot_case(concat!($name, "-delegate")); }
            #[test]
            fn reenter() { assert_frame_slot_case(concat!($name, "-reenter")); }
            #[test]
            fn typed() { assert_frame_slot_case(concat!($name, "-typed")); }
            #[test]
            fn unrelated() { assert_frame_slot_case(concat!($name, "-unrelated")); }
        })*
    };
}
frame_slot_modes!(
    zoom => "zoom",
    zrotation => "zrotation",
    reset => "reset",
    orientation => "orientation",
    buffers => "buffers",
    gestures => "gestures",
    fitauto => "fitauto",
    axis => "axis",
    panning => "panning",
    zrotate => "zrotate",
    orbit => "orbit",
    viewrotation => "viewrotation",
    orientkeys => "orientkeys",
    redraw => "redraw",
    xrinput => "xrinput",
    xrpresentations => "xrpresentations",
);

/// Fails `UpdateZoom` and `ResetViewInput`, so the infallible listener entrypoints must unwind.
struct FailingFrameHooks;
impl AisViewControllerFrameHooks for FailingFrameHooks {
    fn update_zoom(
        &self,
        _: &mut AisViewController,
        _: &AspectScrollDelta,
    ) -> Result<bool, Failure> {
        Err(Failure::with_type(
            "Standard_ProgramError",
            "zoom refused",
            "",
        ))
    }
    fn reset_view_input(&self, _: &mut AisViewController) -> Result<(), Failure> {
        Err(Failure::with_type(
            "Standard_ProgramError",
            "reset refused",
            "",
        ))
    }
}
fn listener_failure(call: impl FnOnce(&mut dyn AspectWindowInputListener)) -> (String, String) {
    let mut c = AisViewController::new().unwrap();
    c.set_frame_hooks(Arc::new(FailingFrameHooks));
    let payload = catch_unwind(AssertUnwindSafe(|| call(&mut c)))
        .expect_err("listener call; expected unwind");
    let failure = payload
        .downcast::<Failure>()
        .unwrap_or_else(|_| panic!("unwind payload; expected the override Failure"));
    (
        failure.exception_type().to_string(),
        failure.message().to_string(),
    )
}
#[test]
fn listener_scroll_and_focus_carry_the_override_failure() {
    let scroll = listener_failure(|l| {
        l.update_mouse_scroll(&AspectScrollDelta::with_delta(1.0, 0));
    });
    assert_eq!(
        scroll,
        (
            "Standard_ProgramError".to_string(),
            "zoom refused".to_string()
        )
    );
    let focus = listener_failure(|l| l.process_focus(false));
    assert_eq!(
        focus,
        (
            "Standard_ProgramError".to_string(),
            "reset refused".to_string()
        )
    );
}

//! XR handler overrides match the native derived-controller trace per case.
use super::view_controller_action_scene_fixture::action_mapped_view;
use super::view_controller_event_slot_fixture::EventSlotMode;
use super::view_controller_frame_slot_fixture::{FrameSlotRecorder, XR_SLOTS};
use super::view_controller_navigation_xr_fixture::NavigationOpenSession;
use super::view_controller_slot_trace::{SlotTrace, attempt_outcome, bits, slot_trace_case};
use super::*;
use crate::v3d::{V3dView, V3dViewSource, V3dViewer};
use martensite_tkernel::standard::Handle;
use martensite_tkmath::gp::{Direction3, Point3};
use martensite_tkservice::aspect::AspectXRSession;
use martensite_tkservice::graphic3d::{
    Graphic3dCViewXr, Graphic3dCameraFocusType, Graphic3dCameraProjection,
};
use martensite_tkservice::null_driver::NullGraphicDriver;
use std::sync::Arc;

/// A direct `handleXRMoveTo` override reporting a hit makes teleport read `PickedData(1)` from an
/// empty selector: both raise `Standard_OutOfRange`; the Rust message names the index and range.
const PICKED_DATA_MESSAGE: &[(&str, &str)] = &[(
    "SelectMgrViewerSelector::PickedData() out of range index",
    "SelectMgrViewerSelector::PickedData() index 1; expected 1..=0",
)];
const XR_SLOT_BASELINE: &str =
    include_str!("../../tests/fixtures/controller-xr-slots-baseline.jsonl");

/// The witness scene: orthographic camera at (0,0,200), Y up, scale 200, aspect 4:3, with the
/// scripted two-hand session and its base and posed XR cameras.
fn xr_slot_scene() -> (
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
    configure_xr_hands(&view);
    (viewer, context, view)
}
fn configure_xr_hands(view: &V3dView) {
    let session: Handle<dyn AspectXRSession> =
        Handle::new(NavigationOpenSession::with_hands()).cast();
    let cview = view.view();
    cview.set_xr_session(&session);
    cview.set_base_xr_camera(&Handle::new(view.camera().copy_members()));
    cview.set_posed_xr_camera(&Handle::new(view.camera().copy_members()));
    cview.setup_xr_posed_camera();
}
fn xr_triple(x: f64, y: f64, z: f64) -> String {
    format!("{},{},{}", bits(x), bits(y), bits(z))
}
fn xr_slot_state(r: &FrameSlotRecorder, c: &AisViewController) -> String {
    let xr = &c.xr;
    let camera = r.view.view_base().view().base_xr_camera();
    let (eye, up) = match camera.get() {
        Some(camera) => {
            let (eye, up) = (camera.eye(), camera.up());
            (
                xr_triple(eye.x(), eye.y(), eye.z()),
                xr_triple(up.x(), up.y(), up.z()),
            )
        }
        None => (String::new(), String::new()),
    };
    format!(
        "{{\"case\":\"{}\",\"phase\":\"state\",\"teleport_hand\":{},\"picking_hand\":{},\"depth_left\":\"{}\",\"depth_right\":\"{}\",\"detected\":{},\"base_camera\":{},\"eye\":\"{eye}\",\"up\":\"{up}\"}}",
        r.trace.case.name,
        xr.last_teleport_hand as i32,
        xr.last_picking_hand as i32,
        bits(xr.last_pick_depth_left),
        bits(xr.last_pick_depth_right),
        (!r.context.detected_owner().is_null()) as i32,
        (!camera.is_null()) as i32
    )
}
fn attempt_xr_slot(r: &FrameSlotRecorder, c: &mut AisViewController) {
    let failure = attempt_outcome(|| c.flush_view_events(&r.context, &r.view, true));
    r.trace.finish_attempt(failure, xr_slot_state(r, c));
}
fn assert_xr_slot_case(name: &'static str) {
    let case = slot_trace_case(name, XR_SLOTS, "xr");
    let (_viewer, context, view) = xr_slot_scene();
    let recorder = Arc::new(FrameSlotRecorder::new(SlotTrace::new(case), context, view));
    let mut c = AisViewController::new().unwrap();
    c.set_frame_hooks(recorder.clone());
    attempt_xr_slot(&recorder, &mut c);
    if matches!(case.mode, EventSlotMode::Typed | EventSlotMode::Unrelated) {
        attempt_xr_slot(&recorder, &mut c);
    }
    recorder
        .trace
        .assert_native_relabelled(XR_SLOT_BASELINE, PICKED_DATA_MESSAGE);
}
macro_rules! xr_slot_modes {
    ($($slot:ident => $name:literal),* $(,)?) => {
        $(mod $slot {
            use super::assert_xr_slot_case;
            #[test]
            fn direct() { assert_xr_slot_case(concat!($name, "-direct")); }
            #[test]
            fn delegate() { assert_xr_slot_case(concat!($name, "-delegate")); }
            #[test]
            fn reenter() { assert_xr_slot_case(concat!($name, "-reenter")); }
            #[test]
            fn typed() { assert_xr_slot_case(concat!($name, "-typed")); }
            #[test]
            fn unrelated() { assert_xr_slot_case(concat!($name, "-unrelated")); }
        })*
    };
}
xr_slot_modes!(
    xrturnpad => "xrturnpad",
    xrteleport => "xrteleport",
    xrpicking => "xrpicking",
    xrhighlight => "xrhighlight",
    xrmoveto => "xrmoveto",
);

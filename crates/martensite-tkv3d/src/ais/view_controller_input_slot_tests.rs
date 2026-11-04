//! Keyboard, touch and 3D-mouse overrides match the native derived-controller trace per case.
use super::view_controller_event_slot_fixture::EventSlotMode;
use super::view_controller_frame_slot_fixture::{FrameSlotRecorder, INPUT_SLOTS};
use super::view_controller_slot_trace::{SlotTrace, attempt_outcome, bits, slot_trace_case};
use super::*;
use martensite_tkernel::ncollection::Vec2;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkservice::aspect::{ASPECT_VKEY_UPPER, AspectVKey, AspectVKeyBasic as K};
use martensite_tkservice::wnt::WntHidSpaceMouse;
use std::sync::Arc;

const INPUT_SLOT_BASELINE: &str =
    include_str!("../../tests/fixtures/controller-input-slots-baseline.jsonl");
const PRODUCT: u64 = 0xC62B;

/// The witness `AxisReport`: six little-endian 16-bit axes behind the report type byte.
fn axis_report(kind: u8, first: i32, second: i32, third: i32) -> [u8; 13] {
    let mut bytes = [0u8; 13];
    bytes[0] = kind;
    let values = [first, second, third, -third, -first, second];
    for (axis, value) in values.iter().enumerate() {
        bytes[1 + axis * 2] = (value & 255) as u8;
        bytes[2 + axis * 2] = ((*value as u32 >> 8) & 255) as u8;
    }
    bytes
}
/// The witness `ButtonReport`: report type 3 and a 32-bit button mask.
fn button_report(state: u32) -> [u8; 13] {
    let mut bytes = [0u8; 13];
    bytes[0] = 3;
    bytes[1..5].copy_from_slice(&state.to_le_bytes());
    bytes
}
fn feed_3d_mouse(
    r: &FrameSlotRecorder,
    c: &mut AisViewController,
    bytes: &[u8],
) -> Result<(), Failure> {
    let report = WntHidSpaceMouse::new(PRODUCT, bytes);
    r.trace.keep(c.update_3d_mouse(&report)?);
    Ok(())
}
/// Feed 3D-mouse reports, direct keys and touches through the public virtuals.
fn run_input_slot(r: &FrameSlotRecorder, c: &mut AisViewController) -> Result<(), Failure> {
    r.trace.results.lock().unwrap().clear();
    c.input_base_mut().event_timer.stop();
    c.input_base_mut().event_timer.reset_to(1.0);
    feed_3d_mouse(r, c, &axis_report(1, 350, -175, 1000)[..7])?;
    feed_3d_mouse(r, c, &axis_report(2, -350, 175, 0)[..7])?;
    feed_3d_mouse(r, c, &button_report(2)[..5])?;
    feed_3d_mouse(r, c, &button_report(0)[..5])?;
    c.key_down(K::A as AspectVKey, 1.5, 0.75)?;
    c.key_up(K::A as AspectVKey, 2.0)?;
    c.key_from_axis(
        K::NavSlideLeft as AspectVKey,
        K::NavSlideRight as AspectVKey,
        2.5,
        -0.5,
    )?;
    c.add_touch_point(1, Vec2::new(10.0, 20.0), false)?;
    c.add_touch_point(2, Vec2::new(30.0, 40.0), true)?;
    c.update_touch_point(3, Vec2::new(50.0, 60.0))?;
    c.update_touch_point(3, Vec2::new(55.0, 65.0))?;
    r.trace.keep(c.remove_touch_point(2, false)?);
    Ok(())
}
fn input_touches(c: &AisViewController) -> String {
    let touches = &c.input_base().touch_points;
    (1..=touches.extent())
        .map(|index| {
            let (id, touch) = (
                touches.find_key(index).unwrap(),
                touches.find_from_index(index).unwrap(),
            );
            let (from, to) = (touch.from, touch.to);
            format!(
                "{id}={},{},{},{};",
                bits(from.x),
                bits(from.y),
                bits(to.x),
                bits(to.y)
            )
        })
        .collect()
}
fn input_slot_state(r: &FrameSlotRecorder, c: &AisViewController) -> String {
    let results: Vec<String> = r
        .trace
        .results
        .lock()
        .unwrap()
        .iter()
        .map(i32::to_string)
        .collect();
    let keys: String = (0..=ASPECT_VKEY_UPPER)
        .filter(|key| c.keys().is_key_down(*key).unwrap())
        .map(|key| format!("{key};"))
        .collect();
    let buttons: String = c
        .input_base()
        .mouse3d_button_state
        .iter()
        .map(|b| (*b as i32).to_string())
        .collect();
    let ui = &c.ui_input;
    format!(
        "{{\"case\":\"{}\",\"phase\":\"state\",\"results\":\"{}\",\"keys\":\"{keys}\",\"touches\":\"{}\",\"buttons3d\":\"{buttons}\",\"ui\":\"{}{},{},{},{},{}\"}}",
        r.trace.case.name,
        results.join(","),
        input_touches(c),
        ui.panning.to_start as i32,
        ui.panning.to_pan as i32,
        ui.z_rotate.to_rotate as i32,
        ui.zoom_actions.len(),
        ui.is_new_gesture as i32,
        ui.selection.points.len()
    )
}
fn attempt_input_slot(r: &FrameSlotRecorder, c: &mut AisViewController) {
    let failure = attempt_outcome(|| run_input_slot(r, c));
    r.trace.finish_attempt(failure, input_slot_state(r, c));
}
fn assert_input_slot_case(name: &'static str) {
    let case = slot_trace_case(name, INPUT_SLOTS, "input");
    let recorder = Arc::new(FrameSlotRecorder::new(
        SlotTrace::new(case),
        Handle::default(),
        Handle::default(),
    ));
    let mut c = AisViewController::new().unwrap();
    c.set_frame_hooks(recorder.clone());
    attempt_input_slot(&recorder, &mut c);
    if matches!(case.mode, EventSlotMode::Typed | EventSlotMode::Unrelated) {
        attempt_input_slot(&recorder, &mut c);
    }
    recorder.trace.assert_native(INPUT_SLOT_BASELINE);
}
macro_rules! input_slot_modes {
    ($($slot:ident => $name:literal),* $(,)?) => {
        $(mod $slot {
            use super::assert_input_slot_case;
            #[test]
            fn direct() { assert_input_slot_case(concat!($name, "-direct")); }
            #[test]
            fn delegate() { assert_input_slot_case(concat!($name, "-delegate")); }
            #[test]
            fn reenter() { assert_input_slot_case(concat!($name, "-reenter")); }
            #[test]
            fn typed() { assert_input_slot_case(concat!($name, "-typed")); }
            #[test]
            fn unrelated() { assert_input_slot_case(concat!($name, "-unrelated")); }
        })*
    };
}
input_slot_modes!(
    keydown => "keydown",
    keyup => "keyup",
    keyaxis => "keyaxis",
    addtouch => "addtouch",
    removetouch => "removetouch",
    updatetouch => "updatetouch",
    mouse3d => "mouse3d",
    translation => "translation",
    rotation => "rotation",
    keys3d => "keys3d",
);

//! Mouse and frame-selection overrides match the native derived-controller trace per case.
use super::view_controller_event_slot_fixture::*;
use super::view_controller_slot_fixture::controller_slot_scene;
use super::*;
use martensite_tkernel::ncollection::{Sequence, Vec2};
use martensite_tkernel::standard::Failure;
use martensite_tkservice::aspect::{
    ASPECT_VKEY_FLAGS_SHIFT as SHIFT, ASPECT_VKEY_MOUSE_LEFT_BUTTON as LEFT,
    AspectWindowInputListener,
};
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::Arc;

const EVENT_SLOT_BASELINE: &str =
    include_str!("../../tests/fixtures/controller-event-slots-baseline.jsonl");

/// Bind the native witness maps: left click picks with XOR, left drag rectangles, shift-left lassos.
fn event_slot_controller() -> AisViewController {
    let mut c = AisViewController::new().unwrap();
    c.change_mouse_selection_schemes().clear(true);
    c.change_mouse_selection_schemes()
        .bind(LEFT, AisSelectionScheme::Xor);
    c.change_mouse_gesture_map().clear(true);
    c.change_mouse_gesture_map()
        .bind(LEFT, AisMouseGesture::SelectRectangle);
    c.change_mouse_gesture_map()
        .bind(LEFT | SHIFT, AisMouseGesture::SelectLasso);
    c
}
/// Drive the public entrypoint that reaches the case slot, keeping each returned flag.
fn run_event_slot(r: &EventSlotRecorder, c: &mut AisViewController) -> Result<(), Failure> {
    r.results.lock().unwrap().clear();
    let keep = |value: bool| r.results.lock().unwrap().push(value as i32);
    match r.case.slot {
        EventSlot::Click => {
            keep(c.update_mouse_buttons(Vec2::new(10, 20), LEFT, 0, false)?);
            keep(c.update_mouse_buttons(Vec2::new(10, 20), 0, 0, false)?);
        }
        EventSlot::Buttons => {
            keep(c.press_mouse_button(Vec2::new(5, 6), LEFT, 0, false)?);
            keep(c.update_mouse_position(Vec2::new(25, 36), LEFT, SHIFT, false)?);
        }
        EventSlot::Position => {
            let listener: &mut dyn AspectWindowInputListener = c;
            keep(listener.update_mouse_position(Vec2::new(12, 14), 0, 0, false)?);
        }
        _ => run_event_frame(r, c, &keep)?,
    }
    Ok(())
}
fn run_event_frame(
    r: &EventSlotRecorder,
    c: &mut AisViewController,
    keep: &dyn Fn(bool),
) -> Result<(), Failure> {
    keep(c.update_mouse_position(Vec2::new(12, 14), 0, 0, false)?);
    if r.case.slot == EventSlot::Poly {
        let mut band = Sequence::default();
        band.append(Vec2::new(1, 2));
        band.append(Vec2::new(30, 40));
        c.select_points_in_viewer(&band, AisSelectionScheme::Replace)?;
    } else {
        c.select_in_viewer(Vec2::new(10, 20), AisSelectionScheme::Replace)?;
    }
    c.flush_view_events(&r.context, &r.view, true)
}
fn attempt_event_slot(r: &EventSlotRecorder, c: &mut AisViewController) {
    let outcome = catch_unwind(AssertUnwindSafe(|| run_event_slot(r, c)));
    let failure = match outcome {
        Ok(Ok(())) => None,
        Ok(Err(failure)) => Some((
            failure.exception_type().to_string(),
            failure.message().to_string(),
        )),
        Err(payload) => Some(("rust-panic".to_string(), event_panic_message(payload))),
    };
    if let Some((kind, message)) = failure {
        let name = r.case.name;
        r.push(format!(
            "{{\"case\":\"{name}\",\"phase\":\"failure\",\"kind\":\"{kind}\",\"message\":\"{message}\"}}"
        ));
    }
    r.push(event_slot_state(r, c));
}
fn event_panic_message(payload: Box<dyn std::any::Any + Send>) -> String {
    match payload.downcast::<&'static str>() {
        Ok(text) => text.to_string(),
        Err(payload) => *payload
            .downcast::<String>()
            .unwrap_or_else(|_| panic!("panic payload; expected a text message")),
    }
}
fn event_buffer(buffer: &AisViewInputBuffer) -> String {
    let selection = &buffer.selection;
    let points: String = selection
        .points
        .iter()
        .map(|p| format!("{},{};", p.x, p.y))
        .collect();
    format!(
        "{}/{}/{}/{}/{}/{},{}",
        selection.tool as i32,
        selection.scheme as i32,
        selection.to_apply_tool as i32,
        points,
        buffer.move_to.to_hilight as i32,
        buffer.move_to.point.x,
        buffer.move_to.point.y
    )
}
fn event_slot_state(r: &EventSlotRecorder, c: &AisViewController) -> String {
    let results: Vec<String> = r
        .results
        .lock()
        .unwrap()
        .iter()
        .map(i32::to_string)
        .collect();
    let (press, previous) = (c.mouse_dispatch.press_point, *c.previous_move_to());
    format!(
        "{{\"case\":\"{}\",\"phase\":\"state\",\"results\":\"{}\",\"ui\":\"{}\",\"gl\":\"{}\",\"gesture\":{},\"pressed\":{},\"modifiers\":{},\"press\":\"{},{}\",\"prev_move_to\":\"{},{}\",\"selected\":{}}}",
        r.case.name,
        results.join(","),
        event_buffer(&c.ui_input),
        event_buffer(&c.render_input),
        c.click_state.active_gesture as i32,
        c.input_base.mouse_pressed,
        c.input_base.mouse_modifiers,
        press.x,
        press.y,
        previous.x,
        previous.y,
        r.context.nb_selected()
    )
}
fn assert_event_slot_case(name: &'static str) {
    let case = event_slot_case(name);
    let scene = controller_slot_scene();
    let recorder = Arc::new(EventSlotRecorder::new(
        case,
        scene.context.clone(),
        scene.view.clone(),
    ));
    let mut c = event_slot_controller();
    c.set_camera_hooks(recorder.clone());
    attempt_event_slot(&recorder, &mut c);
    if matches!(case.mode, EventSlotMode::Typed | EventSlotMode::Unrelated) {
        attempt_event_slot(&recorder, &mut c);
    }
    let actual = recorder.records.lock().unwrap().join("\n") + "\n";
    println!("EVENT_SLOT_BEGIN {name}\n{actual}EVENT_SLOT_END {name}");
    let prefix = format!("{{\"case\":\"{name}\",");
    let expected: String = EVENT_SLOT_BASELINE
        .lines()
        .filter(|line| line.starts_with(&prefix))
        .map(|line| line.replace("\"kind\":\"std::logic_error\"", "\"kind\":\"rust-panic\"") + "\n")
        .collect();
    assert!(!expected.is_empty(), "case={name}; expected a native trace");
    assert_eq!(
        actual, expected,
        "case={name}; expected the native override trace and state"
    );
}
macro_rules! event_slot_controls {
    ($($test:ident => $case:expr),* $(,)?) => {
        $(#[test]
        fn $test() {
            assert_event_slot_case($case);
        })*
    };
}
macro_rules! event_slot_modes {
    ($slot:ident, $name:literal) => {
        mod $slot {
            use super::assert_event_slot_case;
            event_slot_controls!(
                direct => concat!($name, "-direct"),
                delegate => concat!($name, "-delegate"),
                reenter => concat!($name, "-reenter"),
                typed => concat!($name, "-typed"),
                unrelated => concat!($name, "-unrelated"),
            );
        }
    };
}
event_slot_modes!(click, "click");
event_slot_modes!(buttons, "buttons");
event_slot_modes!(position, "position");
event_slot_modes!(moveto, "moveto");
event_slot_modes!(pick, "pick");
event_slot_modes!(highlight, "highlight");
event_slot_modes!(poly, "poly");
event_slot_modes!(lazy, "lazy");

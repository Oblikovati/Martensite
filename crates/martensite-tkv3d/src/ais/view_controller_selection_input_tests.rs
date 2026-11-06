//! Selection-input overrides match the native derived-controller trace per case.
use super::view_controller_selection_input_fixture::*;
use super::*;
use martensite_tkernel::ncollection::{Sequence, Vec2};
use martensite_tkernel::standard::Failure;
use martensite_tkservice::aspect::ASPECT_VKEY_MOUSE_LEFT_BUTTON as LEFT;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::Arc;

const SELECTION_INPUT_BASELINE: &str =
    include_str!("../../tests/fixtures/controller-selection-input-baseline.jsonl");

/// Drive the public entrypoint that reaches the slot, keeping each returned flag.
fn run_selection_input(
    r: &SelectionInputRecorder,
    c: &mut AisViewController,
) -> Result<(), Failure> {
    r.results.lock().unwrap().clear();
    let keep = |value: bool| r.results.lock().unwrap().push(value as i32);
    match r.case.slot {
        SelectionInputSlot::Pick => {
            keep(c.update_mouse_click(Vec2::new(10, 20), LEFT, 0, false)?)
        }
        SelectionInputSlot::Points => {
            let mut points = Sequence::default();
            for (x, y) in [(1, 2), (30, 4), (15, 40)] {
                points.append(Vec2::new(x, y));
            }
            c.select_points_in_viewer(&points, AisSelectionScheme::Add)?;
        }
        SelectionInputSlot::Rubber | SelectionInputSlot::Lasso => {
            keep(c.update_mouse_buttons(Vec2::new(5, 6), LEFT, 0, false)?);
            keep(c.update_mouse_position(Vec2::new(25, 36), LEFT, 0, false)?);
        }
    }
    Ok(())
}
fn attempt_selection_input(r: &SelectionInputRecorder, c: &mut AisViewController) {
    let outcome = catch_unwind(AssertUnwindSafe(|| run_selection_input(r, c)));
    let failure = match outcome {
        Ok(Ok(())) => None,
        Ok(Err(failure)) => Some((
            failure.exception_type().to_string(),
            failure.message().to_string(),
        )),
        Err(payload) => Some(("rust-panic".to_string(), panic_message(payload))),
    };
    if let Some((kind, message)) = failure {
        let name = r.case.name;
        r.push(format!(
            "{{\"case\":\"{name}\",\"phase\":\"failure\",\"kind\":\"{kind}\",\"message\":\"{message}\"}}"
        ));
    }
    r.push(selection_input_state(r, c));
}
fn panic_message(payload: Box<dyn std::any::Any + Send>) -> String {
    match payload.downcast::<&'static str>() {
        Ok(text) => text.to_string(),
        Err(payload) => *payload
            .downcast::<String>()
            .unwrap_or_else(|_| panic!("panic payload; expected a text message")),
    }
}
fn selection_input_state(r: &SelectionInputRecorder, c: &AisViewController) -> String {
    let results: Vec<String> = r
        .results
        .lock()
        .unwrap()
        .iter()
        .map(i32::to_string)
        .collect();
    let selection = &c.ui_input.selection;
    let points: Vec<String> = selection
        .points
        .iter()
        .map(|p| format!("{},{}", p.x, p.y))
        .collect();
    let press = c.mouse_dispatch.press_point;
    format!(
        "{{\"case\":\"{}\",\"phase\":\"state\",\"results\":\"{}\",\"tool\":{},\"scheme\":{},\"apply\":{},\"points\":\"{}\",\"gesture\":{},\"press\":\"{},{}\"}}",
        r.case.name,
        results.join(","),
        selection.tool as i32,
        selection.scheme as i32,
        selection.to_apply_tool as i32,
        points.join(";"),
        c.click_state.active_gesture as i32,
        press.x,
        press.y
    )
}
fn assert_selection_input_case(name: &'static str) {
    let case = selection_input_case(name);
    let recorder = Arc::new(SelectionInputRecorder::new(case));
    let mut c = selection_input_controller(case);
    c.set_camera_hooks(recorder.clone());
    attempt_selection_input(&recorder, &mut c);
    if matches!(
        case.mode,
        SelectionInputMode::Typed | SelectionInputMode::Unrelated
    ) {
        attempt_selection_input(&recorder, &mut c);
    }
    let actual = recorder.records.lock().unwrap().join("\n") + "\n";
    println!("SELECTION_INPUT_BEGIN {name}\n{actual}SELECTION_INPUT_END {name}");
    let prefix = format!("{{\"case\":\"{name}\",");
    let expected: String = SELECTION_INPUT_BASELINE
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
macro_rules! selection_input_control {
    ($test:ident, $case:literal) => {
        #[test]
        fn $test() {
            assert_selection_input_case($case);
        }
    };
}
selection_input_control!(pick_direct, "pick-direct");
selection_input_control!(pick_delegate, "pick-delegate");
selection_input_control!(pick_reenter, "pick-reenter");
selection_input_control!(pick_typed, "pick-typed");
selection_input_control!(pick_unrelated, "pick-unrelated");
selection_input_control!(points_direct, "points-direct");
selection_input_control!(points_delegate, "points-delegate");
selection_input_control!(points_reenter, "points-reenter");
selection_input_control!(points_typed, "points-typed");
selection_input_control!(points_unrelated, "points-unrelated");
selection_input_control!(rubber_direct, "rubber-direct");
selection_input_control!(rubber_delegate, "rubber-delegate");
selection_input_control!(rubber_reenter, "rubber-reenter");
selection_input_control!(rubber_typed, "rubber-typed");
selection_input_control!(rubber_unrelated, "rubber-unrelated");
selection_input_control!(lasso_direct, "lasso-direct");
selection_input_control!(lasso_delegate, "lasso-delegate");
selection_input_control!(lasso_reenter, "lasso-reenter");
selection_input_control!(lasso_typed, "lasso-typed");
selection_input_control!(lasso_unrelated, "lasso-unrelated");

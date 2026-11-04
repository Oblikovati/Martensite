//! Full observations pin callback behavior while retaining distinct language failure kinds.
use super::view_controller_slot_fixture::controller_slot_case;
use super::view_controller_slot_replay::record_controller_slot_case;
const SLOT_BASELINE: &str =
    include_str!("../../tests/fixtures/controller-virtual-slots-baseline.jsonl");

fn assert_controller_slot_case(name: &str) {
    let case = controller_slot_case(name);
    let actual = record_controller_slot_case(case);
    let expected = expected_controller_slot_case(name);
    println!("SLOT_TRACE_BEGIN {name}\n{actual}SLOT_TRACE_END {name}");
    assert_eq!(
        actual, expected,
        "controller slot={name}; expected complete witnessed callback/state trace"
    );
}
fn expected_controller_slot_case(name: &str) -> String {
    let prefix = format!("{{\"case\":\"{name}\",");
    let rows: Vec<&str> = SLOT_BASELINE
        .lines()
        .filter(|line| line.starts_with(&prefix))
        .collect();
    assert!(
        !rows.is_empty(),
        "slot case={name}; expected known nonempty trace"
    );
    // An unrelated Rust panic is not a C++ standard-library exception; retain that distinction.
    rows.join("\n")
        .replace("\"kind\":\"std::logic_error\"", "\"kind\":\"rust-panic\"")
        + "\n"
}
macro_rules! controller_slot_control {
    ($test:ident, $case:literal) => {
        #[test]
        fn $test() {
            let case_name: &str = $case;
            assert_controller_slot_case(case_name);
        }
    };
}
controller_slot_control!(gravity_direct, "gravity-direct");
controller_slot_control!(gravity_orbit, "gravity-orbit");
controller_slot_control!(navigation_direct, "navigation-direct");
controller_slot_control!(navigation_frame, "navigation-frame");
controller_slot_control!(frame_direct, "frame-direct");
controller_slot_control!(frame_from_flush, "frame-from-flush");
controller_slot_control!(frame_not_from_flush_false, "frame-not-from-flush-false");
controller_slot_control!(flush_direct, "flush-direct");
controller_slot_control!(gravity_delegate, "gravity-delegate");
controller_slot_control!(navigation_delegate, "navigation-delegate");
controller_slot_control!(frame_delegate, "frame-delegate");
controller_slot_control!(flush_delegate, "flush-delegate");
controller_slot_control!(gravity_reenter, "gravity-reenter");
controller_slot_control!(navigation_reenter, "navigation-reenter");
controller_slot_control!(frame_reenter, "frame-reenter");
controller_slot_control!(flush_reenter, "flush-reenter");
controller_slot_control!(gravity_typed, "gravity-typed");
controller_slot_control!(navigation_typed, "navigation-typed");
controller_slot_control!(frame_typed, "frame-typed");
controller_slot_control!(flush_typed, "flush-typed");
controller_slot_control!(gravity_unrelated, "gravity-unrelated");
controller_slot_control!(navigation_unrelated, "navigation-unrelated");
controller_slot_control!(frame_unrelated, "frame-unrelated");
controller_slot_control!(flush_unrelated, "flush-unrelated");
controller_slot_control!(flush_direct_false, "flush-direct-false");

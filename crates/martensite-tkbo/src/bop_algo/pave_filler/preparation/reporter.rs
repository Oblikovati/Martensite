//! Execute the exact fourteen frozen Native scenarios without changing their operations.
#[path = "reporter/journal.rs"]
mod journal;
#[path = "reporter/output.rs"]
mod output;
#[path = "reporter/reference.rs"]
mod reference;
#[path = "reporter/scenarios.rs"]
mod scenarios;
#[path = "reporter/state.rs"]
mod state;

#[test]
fn fourteen_front_receivers_match_native_states_polls_and_serial_scopes() {
    for kind in 0..4 {
        scenarios::record_initialization(kind);
    }
    scenarios::record_checker_initialization();
    scenarios::record_non_destructive_bypass();
    scenarios::record_no_face_preparation();
    for missing in [false, true] {
        for parallel in [false, true] {
            scenarios::record_planar_preparation(parallel, missing, 0);
        }
    }
    // Inputs from the actual eight-task Native census in proof36bcc, not Source-derived targets.
    for target in [1, 9, 10] {
        scenarios::record_planar_preparation(false, true, target);
    }
}

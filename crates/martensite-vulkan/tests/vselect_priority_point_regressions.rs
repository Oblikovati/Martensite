//! Isolate the complete priority-point original while retaining every shared control.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
#[path = "support/case_v3d_policy_controls.rs"]
mod case_v3d_policy_controls;
#[allow(dead_code, unused_imports)]
// Retain the shared controls under their existing registry names.
#[path = "vselect_bugs_cases/persistent_priority/mod.rs"]
mod persistent_priority;
#[allow(dead_code, unused_imports)]
// Keep quadric controls even though their original bodies belong to the broad host.
mod vselect_quadric_cases;

#[test]
fn bug30484_3() {
    case_v3d_process_case::run_case_within("bug30484_3", 300, persistent_priority::priority_point);
}

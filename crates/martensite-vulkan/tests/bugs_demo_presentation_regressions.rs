//! Complete bugs/demo presentation programs keep the bugs CPU1000 stream policy.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
mod bugs_demo_presentation_cases;

#[test]
fn bug14673_1() {
    case_bugs_process_case::run_bugs_case(
        "bug14673_1",
        bugs_demo_presentation_cases::unicode_scale,
    );
}

#[test]
fn bug24873_1() {
    case_bugs_process_case::run_bugs_case(
        "bug24873_1",
        bugs_demo_presentation_cases::shown_sphere_bounds,
    );
}

#[test]
fn bug24873_2() {
    case_bugs_process_case::run_bugs_case(
        "bug24873_2",
        bugs_demo_presentation_cases::printed_sphere_bounds,
    );
}

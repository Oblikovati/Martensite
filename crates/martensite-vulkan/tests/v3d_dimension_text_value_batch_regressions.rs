//! Full text/value dimension cases retain picking, pixel checks and owner images.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
#[path = "support/case_v3d_policy_controls.rs"]
mod case_v3d_policy_controls;
#[allow(dead_code, unused_imports)]
// Keep all canonical dimension programs unchanged while selecting these five cases.
mod v3d_dimension_text_cases;
use v3d_dimension_text_cases as cases;

#[test]
fn bug24389() {
    case_v3d_process_case::run_case("bug24389", cases::local_part_highlight);
}

#[test]
fn bug24412_1() {
    case_v3d_process_case::run_case("bug24412_1", cases::moved_diameter_text);
}

#[test]
fn bug24412_2() {
    case_v3d_process_case::run_case("bug24412_2", cases::moved_angle_text);
}

#[test]
fn bug27688() {
    case_v3d_process_case::run_case("bug27688", cases::custom_text_values);
}

#[test]
fn bug27919() {
    case_v3d_process_case::run_case("bug27919", cases::fixed_size_arrows);
}

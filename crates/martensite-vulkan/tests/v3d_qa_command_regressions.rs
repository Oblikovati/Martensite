//! Full QA-command cases with their anonymous presentations and command updates.
#![allow(clippy::duplicate_mod)] // Shared case modules keep one identity per host path.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
mod v3d_qa_command_cases;
use v3d_qa_command_cases::*;

#[test]
fn bug30949() {
    case_v3d_process_case::run_case("bug30949", default_dump);
}
#[test]
fn bug12584() {
    case_v3d_process_case::run_case("bug12584", anonymous_color_scale);
}
#[test]
fn bug204_2() {
    case_v3d_process_case::run_case("bug204_2", selected_removal);
}
#[test]
fn bug301() {
    case_v3d_process_case::run_case("bug301", angle_arrows);
}

//! Whole picked-point original with actual ranked state and an isolated selection-group worker.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
#[path = "vselect_picked_point/commands.rs"]
mod picked_point_commands;
#[path = "vselect_picked_point/observations.rs"]
mod picked_point_observations;
#[path = "vselect_picked_point/point_text.rs"]
mod picked_point_text;

#[test]
fn bug29938() {
    case_v3d_process_case::run_case_within(
        "bug29938",
        300,
        picked_point_commands::complete_picked_point,
    );
}

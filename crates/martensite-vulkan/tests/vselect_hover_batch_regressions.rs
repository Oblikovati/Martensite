//! Three complete hover cases share their existing bodies and the CPU300 selection lifecycle.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
#[path = "vselect_bugs_cases/hover_picking/mod.rs"]
mod hover_picking;

#[test]
fn bug27530() {
    case_v3d_process_case::run_case_within("bug27530", 300, hover_picking::overlapping_box_cycle);
}

#[test]
fn bug31412() {
    case_v3d_process_case::run_case_within(
        "bug31412",
        300,
        hover_picking::occluded_box_projections,
    );
}

#[test]
fn bug31425() {
    case_v3d_process_case::run_case_within("bug31425", 300, hover_picking::free_edge_camera_pick);
}

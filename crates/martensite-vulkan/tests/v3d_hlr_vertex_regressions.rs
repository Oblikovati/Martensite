//! Complete assigned HLR and vertex-mode cases, each in the source CPU1000 worker.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
#[path = "v3d_object_hlr_cases/commands.rs"]
mod object_hlr_commands;
mod v3d_hlr_vertex_cases;
#[test]
fn bug24388_1() {
    case_v3d_process_case::run_case_within(
        "bug24388_1",
        1000,
        v3d_hlr_vertex_cases::dotted_box_highlight,
    );
}
#[test]
fn bug23709_4() {
    case_v3d_process_case::run_case_within(
        "bug23709_4",
        1000,
        v3d_hlr_vertex_cases::restored_exact_surface,
    );
}
#[test]
fn bug25099() {
    case_v3d_process_case::run_case_within(
        "bug25099",
        1000,
        v3d_hlr_vertex_cases::vertex_draw_modes,
    );
}

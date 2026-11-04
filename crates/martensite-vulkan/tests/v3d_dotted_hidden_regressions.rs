//! The second dotted-line original retains the source parser's decimal-comma prefix.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
#[allow(dead_code)]
#[path = "v3d_object_hlr_cases/commands.rs"]
mod object_hlr_commands;
mod v3d_dotted_hidden;
#[test]
fn bug24388_2() {
    case_v3d_process_case::run_case_within("bug24388_2", 1000, v3d_dotted_hidden::dotted_cone);
}

mod v3d_perspective_boxes;
#[test]
fn bug26401() {
    case_v3d_process_case::run_case_within(
        "bug26401",
        1000,
        v3d_perspective_boxes::perspective_boxes,
    );
}

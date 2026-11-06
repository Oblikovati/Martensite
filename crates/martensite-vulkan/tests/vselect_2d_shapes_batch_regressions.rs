//! Complete persistent arbitrary-shape original without a command interpreter.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
mod vselect_2d_shapes;

#[test]
fn bug27796() {
    case_v3d_process_case::run_case_within("bug27796", 300, vselect_2d_shapes::persistent_shapes);
}

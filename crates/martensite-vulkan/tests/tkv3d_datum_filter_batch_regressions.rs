//! Complete shape-filter and graduated-trihedron cases reuse canonical bodies.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
#[path = "support/case_v3d_policy_controls.rs"]
mod case_v3d_policy_controls;
#[path = "case_host/context_mode_transition.rs"]
mod context_mode_transition;
#[allow(dead_code, unused_imports)]
// Reuse the complete canonical tree without pruning its other cases.
mod v3d_trihedron_cases;

#[test]
fn bug25340() {
    case_v3d_process_case::run_case("bug25340", v3d_trihedron_cases::shape_type_filter);
}

#[test]
fn bug25611_1() {
    case_v3d_process_case::run_case("bug25611_1", v3d_trihedron_cases::graduated_sphere);
}

#[test]
fn bug25611_2() {
    case_v3d_process_case::run_case("bug25611_2", v3d_trihedron_cases::graduated_options);
}

//! Two complete trihedron cases retain their bodies and the V3d CPU1000 lifecycle.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
#[path = "case_host/context_mode_transition.rs"]
mod context_mode_transition;
// The existing module exports sibling bodies; only these two wrappers register cases.
#[allow(dead_code, unused_imports)]
mod v3d_trihedron_cases;

#[test]
fn bug23484_1() {
    case_v3d_process_case::run_case("bug23484_1", v3d_trihedron_cases::wireframe_view_trihedron);
}

#[test]
fn bug10() {
    case_v3d_process_case::run_case("bug10", v3d_trihedron_cases::face_plane_trihedron);
}

#[test]
fn occ10_wrong_length_report_fails_the_actual_case_stream_policy() {
    let valid = "Length = 5\ngetLength = 5\nOCC10: OK\nTEST COMPLETED\n";
    let mutated = "Length = 5\ngetLength = 4\nOCC10: ERROR\nTEST COMPLETED\n";
    assert!(case_v3d_policy::accepted_report(valid));
    assert!(!case_v3d_policy::accepted_report(mutated));
}

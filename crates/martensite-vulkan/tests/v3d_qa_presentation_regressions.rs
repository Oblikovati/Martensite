//! Authored presentation ordering and primitive-bound colours.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
mod v3d_qa_presentation_cases;

#[test]
fn bug128() {
    case_v3d_process_case::run_case("bug128", v3d_qa_presentation_cases::ordered_shading);
}

#[test]
fn bug23883() {
    case_v3d_process_case::run_case("bug23883", v3d_qa_presentation_cases::bound_colour);
}

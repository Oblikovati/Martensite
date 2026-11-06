//! One complete lit-highlight original shares the unchanged procedural controls.
#![allow(clippy::duplicate_mod)] // Shared case modules keep one identity per host path.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
// The canonical sibling bodies stay unchanged; only this original is registered here.
#[allow(dead_code, unused_imports)]
mod v3d_procedural_presentation_cases;

#[test]
fn bug27617() {
    case_v3d_process_case::run_case_within(
        "bug27617",
        1000,
        v3d_procedural_presentation_cases::lit_dynamic_highlight,
    );
}

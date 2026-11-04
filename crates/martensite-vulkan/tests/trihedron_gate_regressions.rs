//! A bounded qualification-gate witness; original case bodies and assertions stay shared.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
#[allow(dead_code, unused_imports)] // This host runs a part of the shared cases.
#[path = "v3d_trihedron_cases/mod.rs"]
mod cases;
#[path = "case_host/context_mode_transition.rs"]
mod context_mode_transition;

#[test]
fn bug24989() {
    case_v3d_process_case::run_case("bug24989", cases::default_trihedron);
}

#[test]
fn bug24396() {
    case_v3d_process_case::run_case("bug24396", cases::point_highlight);
}

//! Context and object highlight properties retain full original programs and CPU1000.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
mod v3d_highlight_properties_cases;

#[test]
fn bug27818_1() {
    case_v3d_process_case::run_case(
        "bug27818_1",
        v3d_highlight_properties_cases::context_highlight,
    );
}

#[test]
fn bug27818_2() {
    case_v3d_process_case::run_case(
        "bug27818_2",
        v3d_highlight_properties_cases::object_highlights,
    );
}

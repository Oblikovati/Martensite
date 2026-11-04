//! Complete procedural view bodies retain the group's CPU1000 and image/END lifecycle.
#![allow(clippy::duplicate_mod)] // Shared case modules keep one identity per host path.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
#[path = "case_host/sized_dump_controls.rs"]
mod sized_dump_controls;
mod v3d_view_lifecycle_cases;

#[test]
fn bug28093() {
    case_v3d_process_case::run_case_within(
        "bug28093",
        1000,
        v3d_view_lifecycle_cases::highlight_after_raytrace,
    );
}

#[test]
fn bug29020() {
    case_v3d_process_case::run_case_within(
        "bug29020",
        1000,
        v3d_view_lifecycle_cases::maximum_texture_dumps,
    );
}

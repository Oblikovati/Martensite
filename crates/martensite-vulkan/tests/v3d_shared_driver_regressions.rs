//! Two genuine viewer owners share the same driver and source name map.
#![allow(clippy::duplicate_mod)] // Shared case modules keep one identity per host path.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
mod v3d_shared_driver;
#[allow(dead_code)]
#[path = "v3d_view_lifecycle_cases/mod.rs"]
mod view_lifecycle_commands;
#[test]
fn bug28890() {
    case_v3d_process_case::run_case_within(
        "bug28890",
        1000,
        v3d_shared_driver::shared_driver_views,
    );
}

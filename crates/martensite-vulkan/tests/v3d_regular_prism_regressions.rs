//! Authored periodic-profile prism and complete regular surface tessellation.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
#[allow(dead_code)]
#[path = "support/regular_surface_tessellation/mod.rs"]
mod regular_surface_tessellation;
mod v3d_regular_prism;
#[test]
fn bug32422() {
    case_bugs_process_case::run_bugs_case("bug32422", v3d_regular_prism::regular_prism_view);
}

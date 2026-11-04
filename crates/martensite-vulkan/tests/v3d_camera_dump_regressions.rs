//! Complete camera dump source reuses the real named-view lifecycle and capture banks.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
// The shared tree also retains the separately registered view-naming original.
#[allow(dead_code)]
mod v3d_named_lifecycle;
#[test]
fn bug24001() {
    case_v3d_process_case::run_case_within("bug24001", 1000, v3d_named_lifecycle::camera_dumps);
}

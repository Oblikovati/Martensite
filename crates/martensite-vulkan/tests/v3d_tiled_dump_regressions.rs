//! Exact dump tiles reuse the shared explicit-options capture path.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
mod v3d_tiled_dump;
#[test]
fn bug29127() {
    case_v3d_process_case::run_case_within("bug29127", 1000, v3d_tiled_dump::tiled_rgb_dump);
}

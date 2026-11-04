//! The complete frame-time user-draw regression.
#![allow(clippy::duplicate_mod)] // Shared case modules keep one identity per host path.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
mod v3d_user_draw;

#[test]
fn bug23067() {
    case_v3d_process_case::run_case("bug23067", v3d_user_draw::workspace_user_draw);
}

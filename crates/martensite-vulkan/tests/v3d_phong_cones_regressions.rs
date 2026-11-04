//! Actual fill-program assignment is shared with the earlier shader fixtures.
#![allow(clippy::duplicate_mod)] // Shared case modules keep one identity per host path.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
#[path = "support/object_fill_program.rs"]
mod object_fill_program;
mod v3d_phong_cones;

#[test]
fn bug21753() {
    case_v3d_process_case::run_case_within("bug21753", 1000, v3d_phong_cones::phong_cones);
}

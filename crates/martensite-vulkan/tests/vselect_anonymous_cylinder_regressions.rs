//! Anonymous cylinder highlighting, including the complete CPU300 selection-group lifecycle.
#![allow(clippy::duplicate_mod)] // Shared case modules keep one identity per host path.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
mod vselect_anonymous_cylinder;

#[test]
fn buc60876() {
    case_v3d_process_case::run_case_within(
        "buc60876",
        300,
        vselect_anonymous_cylinder::complete_anonymous_cylinder,
    );
}

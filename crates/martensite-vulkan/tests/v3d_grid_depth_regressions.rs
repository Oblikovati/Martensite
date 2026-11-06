//! The authored grid-depth TODO retains an actual pixel assertion and outcome policy.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
mod v3d_grid_depth;
#[test]
fn bug26154() {
    case_todo::run_case_expecting(
        "bug26154",
        1000,
        &["ERROR: OCC26154 is reproduced."],
        v3d_grid_depth::automatic_grid_depth,
    );
}

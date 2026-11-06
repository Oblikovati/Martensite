//! Three complete view-cube cases retain all pixel checks and authored captures.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
#[allow(dead_code, unused_imports)]
mod v3d_viewcube_cases;

#[test]
fn default() {
    case_v3d_process_case::run_case_within(
        "default",
        1000,
        v3d_viewcube_cases::default_cube_clicks,
    );
}

#[test]
fn orthopers() {
    case_v3d_process_case::run_case_within(
        "orthopers",
        1000,
        v3d_viewcube_cases::orthographic_cube_clicks,
    );
}

#[test]
fn style() {
    case_v3d_process_case::run_case_within("style", 1000, v3d_viewcube_cases::styled_cubes);
}

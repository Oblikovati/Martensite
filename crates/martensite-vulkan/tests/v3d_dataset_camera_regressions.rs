//! The exact restored shape and original camera/color checks share the standard viewer.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
mod v3d_dataset_camera;
#[test]
fn bug22701() {
    case_v3d_process_case::run_case_within(
        "bug22701",
        1000,
        v3d_dataset_camera::restored_hole_colors,
    );
}

//! Both source frustum modes retain the complete far-eye pixel-check loop.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
mod v3d_far_planes;
#[test]
fn bug25760_1() {
    case_v3d_process_case::run_case_within(
        "bug25760_1",
        1000,
        v3d_far_planes::far_plane_projection,
    );
}

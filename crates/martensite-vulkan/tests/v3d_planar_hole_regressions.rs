//! All512 authored inner-wire points remain literal native Rust construction inputs.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
mod v3d_planar_hole;
#[test]
fn bug23219() {
    case_v3d_process_case::run_case_within(
        "bug23219",
        1000,
        v3d_planar_hole::planar_face_triangulation,
    );
}

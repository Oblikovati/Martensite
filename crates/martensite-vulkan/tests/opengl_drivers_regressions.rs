//! The driver group keeps its CPU300 policy and actual dedicated-backend requirements.
#![allow(clippy::duplicate_mod)] // Shared case modules keep one identity per host path.
include!("case_host/session_modules.rs");
mod opengl_driver_cases;
#[test]
#[ignore = "User-authorized backend exclusion: the recorded notes; no Rust pass claimed"]
fn bug24728() {
    case_opengl_process_case::run_case("bug24728", opengl_driver_cases::bug24728);
}
#[test]
#[ignore = "User-authorized backend exclusion: the recorded notes; no Rust pass claimed"]
fn d3dhost() {
    case_opengl_process_case::run_case("d3dhost", opengl_driver_cases::d3dhost);
}
#[test]
#[ignore = "User-authorized backend exclusion: the recorded notes; no Rust pass claimed"]
fn glinfo() {
    case_opengl_process_case::run_case("glinfo", opengl_driver_cases::glinfo);
}
#[test]
#[ignore = "User-authorized backend exclusion: the recorded notes; no Rust pass claimed"]
fn opengles() {
    case_opengl_process_case::run_case("opengles", opengl_driver_cases::opengles);
}

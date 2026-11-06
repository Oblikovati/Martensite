//! The complete infinite face includes both inherited view initialization and final capture.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
#[path = "vselect_cases/infinite_face.rs"]
mod infinite_face;
#[test]
fn face_g1() {
    case_v3d_process_case::run_case_within("face_g1", 300, infinite_face::infinite_face_isolines);
}

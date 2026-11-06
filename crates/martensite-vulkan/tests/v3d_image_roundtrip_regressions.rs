//! Complete file/memory/stream image QA regression uses actual public image and viewer APIs.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
#[path = "case_host/dump_diff.rs"]
mod dump_diff;
mod v3d_image_roundtrip;

#[test]
fn bug31956() {
    case_v3d_process_case::run_case_within("bug31956", 1000, v3d_image_roundtrip::image_roundtrips);
}

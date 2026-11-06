//! Two complete animation programs reuse the authored timelines and source callbacks.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
#[allow(dead_code, unused_imports)]
mod v3d_anim_cases;

#[test]
fn translate() {
    case_v3d_process_case::run_case_within("translate", 1000, v3d_anim_cases::camera_translation);
}

#[test]
fn propeller() {
    case_v3d_process_case::run_case_within("propeller", 1000, v3d_anim_cases::propeller_tour);
}

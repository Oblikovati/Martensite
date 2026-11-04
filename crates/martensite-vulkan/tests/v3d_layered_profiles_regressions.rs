//! Five exact font/wire placements share only their genuinely identical authored geometry.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
#[path = "support/brep_text_shape.rs"]
mod brep_text_shape;
#[allow(dead_code)]
#[path = "support/dimension_input.rs"]
mod dimension_input;
mod v3d_layered_profiles;
#[test]
fn bug25783_1() {
    case_v3d_process_case::run_case_within(
        "bug25783_1",
        1000,
        v3d_layered_profiles::overlay_profiles,
    );
}
#[test]
fn bug26149() {
    case_v3d_process_case::run_case_within(
        "bug26149",
        1000,
        v3d_layered_profiles::underlay_profiles,
    );
}

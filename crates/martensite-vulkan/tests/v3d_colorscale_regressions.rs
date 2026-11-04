//! Colour-scale regressions of the V3d group, each in an isolated worker under the V3d policy.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
mod v3d_colorscale_cases;
use std::error::Error;
use v3d_colorscale_cases as cases;

fn run(identity: &str, body: fn() -> Result<(), Box<dyn Error>>) {
    case_v3d_process_case::run_case(identity, body);
}

#[test]
fn bug22632() {
    run("bug22632", cases::logarithmic_ranges);
}

#[test]
fn bug22819() {
    run("bug22819", cases::transparent_sphere);
}

#[test]
fn bug25027() {
    run("bug25027", cases::wide_view_overlay);
}

#[test]
fn bug25136() {
    run("bug25136", cases::customized_scales);
}

#[test]
fn bug26312() {
    run("bug26312", cases::multiple_views);
}

#[test]
fn bug27573() {
    run("bug27573", cases::find_color_custom);
}

#[test]
fn bug28004() {
    run("bug28004", cases::hue_and_free_labels);
}

#[test]
fn bug28813() {
    run("bug28813", cases::position_and_height);
}

#[test]
fn bug31454() {
    run("bug31454", cases::uniform_lightness);
}

#[test]
fn colorscale_faceculling() {
    run("colorscale_faceculling", cases::face_culled_scale);
}

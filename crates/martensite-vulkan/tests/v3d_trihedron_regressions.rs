//! Trihedron regressions of the V3d group, each in an isolated worker.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
mod v3d_trihedron_cases;
use std::error::Error;
use v3d_trihedron_cases as cases;

fn run(identity: &str, body: fn() -> Result<(), Box<dyn Error>>) {
    case_v3d_process_case::run_case(identity, body);
}

#[test]
fn buc60814() {
    run("buc60814", cases::selected_circle_and_trihedron);
}

#[test]
fn bug10() {
    run("bug10", cases::face_plane_trihedron);
}

#[test]
fn bug218() {
    run("bug218", cases::labelled_plane_trihedron);
}

#[test]
fn bug22368() {
    run("bug22368", cases::single_mode_trihedron);
}

#[test]
fn bug22483() {
    run("bug22483", cases::view_trihedron_positions);
}

#[test]
fn bug23422() {
    run("bug23422", cases::moved_box_detection);
}

#[test]
fn bug23484_1() {
    run("bug23484_1", cases::wireframe_view_trihedron);
}

#[test]
fn bug24396() {
    run("bug24396", cases::point_highlight);
}

#[test]
fn bug24989() {
    run("bug24989", cases::default_trihedron);
}

#[test]
fn bug25340() {
    run("bug25340", cases::shape_type_filter);
}

#[test]
fn bug25611_1() {
    run("bug25611_1", cases::graduated_sphere);
}

#[test]
fn bug25611_2() {
    run("bug25611_2", cases::graduated_options);
}

#[test]
fn bug26754() {
    run("bug26754", cases::hidden_label_selection);
}

#[test]
fn bug27832() {
    run("bug27832", cases::scaled_view_trihedron);
}

#[test]
fn bug27958() {
    run("bug27958", cases::trihedron_options);
}

#[path = "case_host/context_mode_transition.rs"]
mod context_mode_transition;

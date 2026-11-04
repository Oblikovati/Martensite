//! Manipulator regressions of the V3d group, each in an isolated worker.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
#[allow(dead_code, unused_imports)] // This host runs a part of the shared cases.
mod v3d_manipulator_cases;
use std::error::Error;
use v3d_manipulator_cases as cases;

fn run(identity: &str, body: fn() -> Result<(), Box<dyn Error>>) {
    case_v3d_process_case::run_case(identity, body);
}

#[test]
fn disable_parts() {
    run("disable_parts", cases::disabled_parts);
}

#[test]
fn shape_location() {
    run("shape_location", cases::located_shape);
}

#[test]
fn view_affinity() {
    run("view_affinity", cases::view_affinity);
}

#[test]
fn bug33514() {
    run("bug33514", cases::held_trihedron);
}

#[test]
fn bug30669() {
    run("bug30669", cases::child_drag);
}

#[test]
fn rotate2() {
    run("rotate2", cases::stepped_rotation);
}

#[test]
fn drag_in_2d_view() {
    run("drag_in_2d_view", cases::drag_in_2d);
}

#[test]
fn drag_pan_zoom() {
    run("drag_pan_zoom", cases::gesture_drags);
}

#[test]
fn dragg() {
    run("dragg", cases::dragged_object);
}

#[test]
fn rotate() {
    run("rotate", cases::rotated_object);
}

#[test]
fn scale() {
    run("scale", cases::scaled_object);
}

#[test]
fn translate() {
    run("translate", cases::translated_object);
}

#[test]
fn translate2() {
    run("translate2", cases::turned_camera_translation);
}

#[test]
fn flat() {
    run("flat", cases::flat_skin);
}

#[test]
fn zoom_persistence() {
    run("zoom_persistence", cases::zoom_persistence);
}

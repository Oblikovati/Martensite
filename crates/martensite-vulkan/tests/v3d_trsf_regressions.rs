//! Transformation and persistence regressions of the V3d group, each in an isolated worker.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
mod v3d_trsf_cases;
use std::error::Error;
use v3d_trsf_cases as cases;

fn run(identity: &str, body: fn() -> Result<(), Box<dyn Error>>) {
    case_v3d_process_case::run_case(identity, body);
}

#[test]
fn bug23120() {
    run("bug23120", cases::centered_trihedron_box);
}

#[test]
fn bug24358() {
    run("bug24358", cases::assembly_after_reopened_view);
}

#[test]
fn bug24756() {
    run("bug24756", cases::moved_instance_chain);
}

#[test]
fn bug27285() {
    run("bug27285", cases::selected_assembly);
}

#[test]
fn bug27817() {
    run("bug27817", cases::trihedron_face_highlight);
}

/// AMD: the one coverage pixel (197,209) is snap-dependent (host reference `snap_coverage`).
#[test]
fn bug27793() {
    run("bug27793", cases::stereo_corner_trihedrons);
}

#[test]
fn bug25276() {
    run("bug25276", cases::transformed_assemblies);
}

#[test]
fn bug23652() {
    run("bug23652", cases::aligned_zoomable_labels);
}

#[test]
fn bug_flipped_selection() {
    run("bug_flipped_selection", cases::flipped_label_selection);
}

#[test]
fn bug27359() {
    run("bug27359", cases::flipped_plane_labels);
}

#[test]
fn bug24837_1() {
    run("bug24837_1", cases::nested_assembly_edits);
}

#[test]
fn bug27869() {
    run("bug27869", cases::instance_exact_hidden_lines);
}

#[test]
fn bug32091() {
    run("bug32091", cases::plane_minimum_size);
}

#[test]
fn bug27739() {
    run("bug27739", cases::screen_boxes_detection);
}

#[test]
fn bug26344() {
    run("bug26344", cases::persistent_box_selection);
}

#[test]
fn bug26719_2() {
    run("bug26719_2", cases::zoom_persistent_pick_depth);
}

#[test]
fn bug24837_2() {
    run("bug24837_2", cases::labelled_assembly);
}

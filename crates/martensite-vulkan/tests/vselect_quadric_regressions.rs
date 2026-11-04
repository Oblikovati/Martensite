//! Quadric selection regressions of the selection group, each in an isolated worker.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
mod vselect_quadric_cases;
use std::error::Error;
use vselect_quadric_cases as cases;

/// The selection group runs every regression under a 300 s CPU limit.
fn run(identity: &str, body: fn() -> Result<(), Box<dyn Error>>) {
    case_v3d_process_case::run_case_within(identity, 300, body);
}

#[test]
fn cone_cylinder_check_depth() {
    run("cone_cylinder_check_depth", cases::cone_cylinder_depth);
}

#[test]
fn cone_cylinder_detecting() {
    run("cone_cylinder_detecting", cases::cone_cylinder_detecting);
}

#[test]
fn single_click_selection_cone() {
    run("single_click_selection_cone", cases::cone_single_clicks);
}

#[test]
fn single_click_selection_cylinder() {
    run(
        "single_click_selection_cylinder",
        cases::cylinder_single_clicks,
    );
}

#[test]
fn single_click_selection_trunc_cone() {
    run(
        "single_click_selection_trunc_cone",
        cases::truncated_cone_single_clicks,
    );
}

#[test]
fn cone_cylinder_polygon_selection() {
    run(
        "cone_cylinder_polygon_selection",
        cases::cone_cylinder_polygons,
    );
}

#[test]
fn cone_cylinder_rectangle_selection() {
    run(
        "cone_cylinder_rectangle_selection",
        cases::cone_cylinder_rectangles,
    );
}

#[test]
fn sphere_check_depth() {
    run("sphere_check_depth", cases::sphere_depth);
}

#[test]
fn sphere_detecting() {
    run("sphere_detecting", cases::sphere_detecting);
}

#[test]
fn sphere_single_click_selection() {
    run("sphere_single_click_selection", cases::sphere_single_clicks);
}

#[test]
fn sphere_polygon_selection() {
    run("sphere_polygon_selection", cases::sphere_polygons);
}

#[test]
fn sphere_rectangle_selection() {
    run("sphere_rectangle_selection", cases::sphere_rectangles);
}

#[test]
fn filled_circle() {
    run("filled_circle", cases::filled_circle_picks);
}

#[test]
fn circle_wire() {
    run("circle_wire", cases::circle_wire_picks);
}

#[test]
fn circle_sector() {
    run("circle_sector", cases::circle_sector_picks);
}

#[test]
fn transformed_circle() {
    run("transformed_circle", cases::transformed_circle_images);
}

#[test]
fn cone_cylinder_generate_images() {
    run(
        "cone_cylinder_generate_images",
        cases::cone_cylinder_normal_images,
    );
}

#[test]
fn sphere_generate_images() {
    run("sphere_generate_images", cases::sphere_normal_images);
}

#[test]
fn axis_a1() {
    run("axis_a1", cases::axis_a1);
}

#[test]
fn axis_a2() {
    run("axis_a2", cases::axis_a2);
}

#[test]
fn axis_a3() {
    run("axis_a3", cases::axis_a3);
}

#[test]
fn hollow_cone_cyl() {
    run("hollow_cone_cyl", cases::hollow_cone_cyl);
}

#[test]
fn trsf_cone() {
    run("trsf_cone", cases::transformed_cone);
}

#[test]
fn trsf_cyl() {
    run("trsf_cyl", cases::transformed_cylinder);
}

#[test]
fn sphere_trsf() {
    run("sphere_trsf", cases::sphere_trsf);
}

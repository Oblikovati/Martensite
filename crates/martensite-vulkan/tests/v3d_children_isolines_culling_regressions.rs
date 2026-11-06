//! Three complete scene bodies reuse their canonical tree and original V3d worker policy.
#![allow(clippy::duplicate_mod)] // Shared case modules keep one identity per host path.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
#[path = "case_host/dump_diff.rs"]
mod dump_diff;
#[allow(dead_code)] // The canonical sibling bodies retain the existing line fixture.
#[path = "viewer_fixtures/support/ais_line_fixture.rs"]
mod line_fixture;
#[allow(dead_code, unused_imports)] // Keep the full canonical tree unchanged in this bounded host.
mod v3d_bugs_scene_cases;

#[test]
fn bug30561() {
    case_v3d_process_case::run_case_within(
        "bug30561",
        1000,
        v3d_bugs_scene_cases::children_keep_placement,
    );
}

#[test]
fn bug25300_2() {
    case_v3d_process_case::run_case_within(
        "bug25300_2",
        1000,
        v3d_bugs_scene_cases::isolines_on_triangulation,
    );
}

#[test]
fn bug31341() {
    case_v3d_process_case::run_case_within(
        "bug31341",
        1000,
        v3d_bugs_scene_cases::culled_by_presentation_range,
    );
}

#[test]
fn bug24867() {
    case_v3d_process_case::run_case_within(
        "bug24867",
        1000,
        v3d_bugs_scene_cases::wide_outline_highlight,
    );
}

#[test]
fn bug25695() {
    case_v3d_process_case::run_case_within(
        "bug25695",
        1000,
        v3d_bugs_scene_cases::plastic_dynamic_highlight,
    );
}

//! Complete moved-object and anchored multi-object manipulator programs.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
// This bounded host shares complete authored bodies with the wide legacy host.
#[allow(dead_code, unused_imports)]
mod v3d_manipulator_cases;

#[test]
fn bug32116() {
    case_v3d_process_case::run_case_within(
        "bug32116",
        1000,
        v3d_manipulator_cases::persistent_pair_drag,
    );
}

#[test]
fn bug32750() {
    case_v3d_process_case::run_case_within(
        "bug32750",
        300,
        v3d_manipulator_cases::moved_box_selection,
    );
}

#[test]
fn shape_location() {
    case_v3d_process_case::run_case_within(
        "shape_location",
        1000,
        v3d_manipulator_cases::located_shape,
    );
}
#[test]
fn disable_parts() {
    case_v3d_process_case::run_case_within(
        "disable_parts",
        1000,
        v3d_manipulator_cases::disabled_parts,
    );
}

#[test]
fn bug27618() {
    case_v3d_process_case::run_case_within(
        "bug27618",
        300,
        v3d_manipulator_cases::overlapped_border_selection,
    );
}

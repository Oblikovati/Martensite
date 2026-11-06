//! Complete numeric grid and mutable selection-iterator cases.
#![allow(clippy::duplicate_mod)] // Shared case modules keep one identity per host path.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
mod v3d_grid_selection_cases;

#[test]
fn bug25672() {
    case_v3d_process_case::run_case("bug25672", v3d_grid_selection_cases::shifted_grid);
}

#[test]
fn bug26435() {
    case_v3d_process_case::run_case("bug26435", v3d_grid_selection_cases::pixel_grid);
}

#[test]
fn bug138_1() {
    case_v3d_process_case::run_case("bug138_1", || {
        v3d_grid_selection_cases::selected_walk(false)
    });
}

#[test]
fn bug138_2() {
    case_v3d_process_case::run_case("bug138_2", || v3d_grid_selection_cases::selected_walk(true));
}

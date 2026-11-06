//! Full object-specific polygonal/exact hidden-line command cases.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
mod v3d_object_hlr_cases;

#[test]
fn bug23709_1() {
    case_v3d_process_case::run_case("bug23709_1", v3d_object_hlr_cases::polygonal_box_algorithm);
}
#[test]
fn bug23709_2() {
    case_v3d_process_case::run_case("bug23709_2", v3d_object_hlr_cases::exact_cone_algorithm);
}

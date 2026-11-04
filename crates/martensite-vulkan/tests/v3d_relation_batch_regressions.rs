//! Complete relation cases retain dynamic picked operands and source transitions.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
#[path = "support/case_v3d_policy_controls.rs"]
mod case_v3d_policy_controls;
#[path = "support/relation_fresh_session_contract.rs"]
mod relation_fresh_session_contract;
#[allow(dead_code, unused_imports)]
// Keep the complete canonical tree unchanged while selecting these nine cases.
mod v3d_dimension_text_cases;
use v3d_dimension_text_cases as cases;

#[test]
fn concentric() {
    case_v3d_process_case::run_case("concentric", cases::concentric_circles);
}

#[test]
fn equaldistance() {
    case_v3d_process_case::run_case("equaldistance", cases::equal_distances);
}

#[test]
fn equalradius() {
    case_v3d_process_case::run_case("equalradius", cases::equal_radii);
}

#[test]
fn fix() {
    case_v3d_process_case::run_case("fix", cases::fixed_edge);
}

#[test]
fn identic() {
    case_v3d_process_case::run_case("identic", cases::identic_shapes);
}

#[test]
fn offset() {
    case_v3d_process_case::run_case("offset", cases::offset_faces);
}

#[test]
fn parallel() {
    case_v3d_process_case::run_case("parallel", cases::parallel_edges);
}

#[test]
fn perpendicular() {
    case_v3d_process_case::run_case("perpendicular", cases::perpendicular_edges);
}

#[test]
fn symmetric() {
    case_v3d_process_case::run_case("symmetric", cases::symmetric_shapes);
}

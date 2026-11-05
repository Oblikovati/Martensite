//! Complete original input/section/mesh-quality assertions retain their source failure policy.
use super::*;

#[test]
fn bug21415() {
    case_bugs_process_case::run_bugs_case_expecting(
        "v3d_section_quality_regressions::bug21415",
        &["Error : The area of result shape is"],
        v3d_section_quality_cases::large_tolerance_presentation,
    );
}

#[test]
fn bug24097() {
    case_bugs_process_case::run_bugs_case(
        "v3d_section_quality_regressions::bug24097",
        v3d_section_quality_cases::checked_short_section,
    );
}

#[test]
fn bug27239() {
    case_bugs_process_case::run_bugs_case(
        "v3d_section_quality_regressions::bug27239",
        v3d_section_quality_cases::checked_polygon_nodes,
    );
}

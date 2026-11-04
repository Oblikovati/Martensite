//! Complete label-layout cases reuse the canonical two-view dimension programs.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
#[path = "support/case_v3d_policy_controls.rs"]
mod case_v3d_policy_controls;
#[allow(dead_code, unused_imports)]
// Reuse the complete dimension tree without pruning other cases from shared modules.
mod v3d_dimension_text_cases;
use v3d_dimension_text_cases::{DimensionKind, angle_layouts, circle_layouts, length_layouts};

#[test]
fn bug24351_1() {
    case_v3d_process_case::run_case("bug24351_1", length_layouts);
}

#[test]
fn bug24351_2() {
    case_v3d_process_case::run_case("bug24351_2", || circle_layouts(DimensionKind::Diameter));
}

#[test]
fn bug24351_3() {
    case_v3d_process_case::run_case("bug24351_3", || circle_layouts(DimensionKind::Radius));
}

#[test]
fn bug24351_4() {
    case_v3d_process_case::run_case("bug24351_4", angle_layouts);
}

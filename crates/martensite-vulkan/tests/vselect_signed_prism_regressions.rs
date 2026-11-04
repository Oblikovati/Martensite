//! Signed prism cylinder detection with the complete CPU300 selection-group lifecycle.
#![allow(clippy::duplicate_mod)] // Shared case modules keep one identity per host path.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
#[path = "report_rules/circular_profile.rs"]
mod circular_profile;
#[allow(dead_code)] // This original consumes the existing entity-name predicate only.
#[path = "vselect_quadric_cases/source_checks.rs"]
mod quadric_source_checks;
mod vselect_signed_prism;

#[test]
fn bug33084() {
    case_v3d_process_case::run_case_within(
        "bug33084",
        300,
        vselect_signed_prism::complete_signed_prism,
    );
}

//! Complete anonymous-removal and shared-dimension QA cases.
#![allow(clippy::duplicate_mod)] // Shared case modules keep one identity per host path.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
// Reuse the original selected-removal engine without registering its sibling cases.
#[allow(dead_code, unused_imports)]
mod v3d_qa_command_cases;
mod v3d_qa_removal_dimension_cases;

#[test]
fn bug136() {
    case_todo::run_case_expecting(
        "bug136",
        1000,
        &["Error : Colors are not equal in default coordinate and in the near coordinates too"],
        v3d_qa_removal_dimension_cases::remove_four_primitives,
    );
}
#[test]
fn bug204_1() {
    case_todo::run_case_expecting(
        "bug204_1",
        1000,
        &[
            "Error : 3 boxes missing in the viewer!",
            "Error : Colors are not equal in default coordinate and in the near coordinates too",
        ],
        v3d_qa_removal_dimension_cases::deferred_selected_removal,
    );
}
#[test]
fn buc60915() {
    case_v3d_process_case::run_case(
        "buc60915",
        v3d_qa_removal_dimension_cases::shared_dimension_arrows,
    );
}

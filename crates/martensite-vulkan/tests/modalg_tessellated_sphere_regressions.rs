//! Complete CPU-only sphere geometry and triangulation property regression.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
mod modalg_tessellated_sphere;
#[test]
fn bug29731() {
    case_bugs_process_case::run_bugs_case(
        "bug29731",
        modalg_tessellated_sphere::tessellated_sphere_properties,
    );
}

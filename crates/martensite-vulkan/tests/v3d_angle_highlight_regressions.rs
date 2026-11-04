//! One complete angle-text original owns the opt-in causal observer request.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
// Keep the canonical helpers and sibling bodies shared without registering their cases.
#[allow(dead_code, unused_imports)]
mod v3d_dimension_text_cases;

#[test]
fn bug24133_2() {
    case_v3d_process_case::run_case("bug24133_2", v3d_dimension_text_cases::angle_text_highlight);
}

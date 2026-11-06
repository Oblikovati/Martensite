//! Complete Y/Z radius-text flips reuse their canonical programs and CPU1000 lifecycle.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
// The canonical module owns shared circle/check/banner helpers and unrelated sibling exports.
#[allow(dead_code, unused_imports)]
mod v3d_dimension_text_cases;
use v3d_dimension_text_cases::{FlipAxis, flipped_radius};

#[test]
fn bug24288_2() {
    case_v3d_process_case::run_case("bug24288_2", || flipped_radius(FlipAxis::Y));
}

#[test]
fn bug24288_3() {
    case_v3d_process_case::run_case("bug24288_3", || flipped_radius(FlipAxis::Z));
}

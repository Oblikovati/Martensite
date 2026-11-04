//! Highlighted angle, radius and diameter text in two independent viewers.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
#[allow(dead_code, unused_imports)]
// Reuse the complete dimension tree; this host selects its three highlight bodies.
mod v3d_dimension_text_cases;
use v3d_dimension_text_cases::{DimensionKind, angle_text_highlight, circle_text_highlight};

#[test]
fn bug24133_2() {
    case_v3d_process_case::run_case("bug24133_2", angle_text_highlight);
}

#[test]
fn bug24133_3() {
    case_v3d_process_case::run_case("bug24133_3", || {
        circle_text_highlight(DimensionKind::Radius)
    });
}

#[test]
fn bug24133_4() {
    case_v3d_process_case::run_case("bug24133_4", || {
        circle_text_highlight(DimensionKind::Diameter)
    });
}

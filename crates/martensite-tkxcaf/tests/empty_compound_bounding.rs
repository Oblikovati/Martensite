//! Complete empty nested compound case with its mandatory real view capture.
#![cfg(target_os = "linux")]
#[path = "empty_compound_views/mod.rs"]
mod empty_compound_views;
#[test]
fn bug16950_4_case_case_with_saved_capture() {
    empty_compound_views::run_case_case_with_policy("bug16950_4", 2);
}

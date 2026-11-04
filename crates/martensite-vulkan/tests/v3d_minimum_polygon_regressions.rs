//! Actual minimum-size edge meshes are compared through their complete source dump words.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
#[path = "support/shape_dump_text.rs"]
mod shape_dump;
mod v3d_minimum_polygon;

#[test]
fn bug26532() {
    case_bugs_process_case::run_bugs_case(
        "bug26532",
        v3d_minimum_polygon::minimum_size_polygon_comparison,
    );
}

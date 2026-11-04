//! Surface-intersection bindings preserve authored names and their non-shape display branch.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
mod v3d_surface_intersections;
#[test]
fn bug22720() {
    case_v3d_process_case::run_case_within(
        "bug22720",
        60,
        v3d_surface_intersections::trimmed_intersection_view,
    );
}
#[test]
fn bug604() {
    v3d_surface_intersections::run_torus();
}

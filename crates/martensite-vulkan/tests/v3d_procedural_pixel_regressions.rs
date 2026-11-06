//! Complete procedural pixel regressions, each running in its original isolated worker.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
mod v3d_procedural_pixel_cases;

#[test]
fn bug29262() {
    case_v3d_process_case::run_case(
        "bug29262",
        v3d_procedural_pixel_cases::erased_two_view_affinity,
    );
}

#[test]
fn bug23186() {
    case_v3d_process_case::run_case("bug23186", v3d_procedural_pixel_cases::four_point_pixels);
}

#[test]
fn bug30076() {
    case_v3d_process_case::run_case("bug30076", v3d_procedural_pixel_cases::patch_vertex_colors);
}

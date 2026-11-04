//! The two complete culling programs share their unchanged source command implementations.
#![allow(clippy::duplicate_mod)] // Shared case modules keep one identity per host path.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
#[expect(
    dead_code,
    reason = "The shared culling module also implements other scene programs."
)]
#[path = "v3d_bugs_scene_cases/culling.rs"]
mod culling;
#[path = "v3d_bugs_scene_cases/culling_cpu_baseline.rs"]
mod culling_cpu_baseline;
#[path = "v3d_bugs_scene_cases/culling_performance_contract.rs"]
mod culling_performance_contract;
#[path = "v3d_bugs_scene_cases/culling_phase.rs"]
mod culling_phase;
#[path = "v3d_bugs_scene_cases/culling_redraw_contract.rs"]
mod culling_redraw_contract;
#[path = "case_host/dump_diff.rs"]
mod dump_diff;
#[expect(
    unused_imports,
    dead_code,
    reason = "The shared prelude also serves other scene programs."
)]
#[path = "v3d_bugs_scene_cases/prelude.rs"]
mod prelude;

#[test]
fn bug24307_1() {
    case_v3d_process_case::run_case("bug24307_1", culling::culled_spheres);
}

#[test]
fn bug24307_2() {
    case_v3d_process_case::run_case("bug24307_2", culling::culled_boxes);
}

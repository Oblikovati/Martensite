//! Complete original wire-location regression uses shared topology and actual mesh bounds.
#![allow(clippy::duplicate_mod)] // Shared case modules keep one identity per host path.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
mod v3d_located_wire;

#[test]
fn bug31131() {
    case_bugs_process_case::run_bugs_case("bug31131", v3d_located_wire::located_wire_polygons);
}

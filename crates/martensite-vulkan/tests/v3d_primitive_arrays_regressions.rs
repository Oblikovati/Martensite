//! All actual authored primitive arrays reuse the viewer command's owned object implementation.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
mod v3d_primitive_arrays;
#[test]
fn bug23226() {
    case_v3d_process_case::run_case_within(
        "bug23226",
        1000,
        v3d_primitive_arrays::primitive_layouts,
    );
}

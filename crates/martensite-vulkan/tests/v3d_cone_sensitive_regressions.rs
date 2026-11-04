//! Four cone faces retain direct untyped explode names and sensitive debug display.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
mod v3d_cone_sensitive;
#[test]
fn bug33664_2() {
    case_v3d_process_case::run_case_within(
        "bug33664_2",
        1000,
        v3d_cone_sensitive::cone_sensitive_faces,
    );
}

#[test]
fn bug33664_1() {
    case_v3d_process_case::run_case_within(
        "bug33664_1",
        1000,
        v3d_cone_sensitive::cylinder_sensitive_area,
    );
}

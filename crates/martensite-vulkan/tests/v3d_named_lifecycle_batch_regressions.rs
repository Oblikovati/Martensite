//! Complete named multi-driver/view regression has no command interpreter dependency.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
// The shared tree also retains the separately registered camera-dump original.
#[allow(dead_code)]
mod v3d_named_lifecycle;

#[test]
fn bug23670_2() {
    case_v3d_process_case::run_case_within("bug23670_2", 1000, v3d_named_lifecycle::named_views);
}

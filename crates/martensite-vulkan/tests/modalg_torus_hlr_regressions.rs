//! The horn-torus normal-status regression executes the existing exact HLR pipeline.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
mod modalg_torus_hlr;
#[test]
fn bug29405() {
    case_bugs_process_case::run_bugs_case("bug29405", modalg_torus_hlr::torus_normal_status);
}

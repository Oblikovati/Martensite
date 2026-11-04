//! Complete text-aspect and Unicode image-path regressions share the actual viewer engine.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
mod v3d_text_cases;

#[test]
fn bug25369() {
    case_v3d_process_case::run_case("bug25369", v3d_text_cases::unicode_texture_path);
}

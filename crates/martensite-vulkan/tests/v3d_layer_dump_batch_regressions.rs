//! Full original layer/depth/shadow-map requests use the actual renderer and public image options.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
mod v3d_layer_dump;
#[test]
fn bug32752() {
    case_v3d_process_case::run_case_within("bug32752", 1000, v3d_layer_dump::layer_options);
}

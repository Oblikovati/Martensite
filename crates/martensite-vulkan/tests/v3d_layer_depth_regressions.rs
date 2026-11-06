//! Source layer settings and per-object inherited offsets use the ordinary session.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
#[path = "vselect_zlayer/layer.rs"]
#[allow(dead_code)]
mod case_layer_commands;
mod v3d_layer_depth;
#[test]
fn bug24785() {
    case_v3d_process_case::run_case_within("bug24785", 1000, v3d_layer_depth::overlapping_layers);
}

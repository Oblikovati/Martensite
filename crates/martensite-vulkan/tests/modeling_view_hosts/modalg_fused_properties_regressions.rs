//! CPU-only actual Boolean geometry versus triangulation properties.
use super::*;

#[test]
fn bug29734() {
    case_bugs_process_case::run_bugs_case(
        "modalg_fused_properties_regressions::bug29734",
        modalg_fused_properties::fused_mesh_properties,
    );
}

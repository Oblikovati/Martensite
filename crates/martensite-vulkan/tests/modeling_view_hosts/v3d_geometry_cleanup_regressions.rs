//! Actual multi-tool fuse/mesh/geometry removal with no invented visualization.
use super::*;

#[test]
fn bug33049() {
    case_bugs_process_case::run_bugs_case(
        "v3d_geometry_cleanup_regressions::bug33049",
        v3d_geometry_cleanup::geometry_removal_after_fuse,
    );
}

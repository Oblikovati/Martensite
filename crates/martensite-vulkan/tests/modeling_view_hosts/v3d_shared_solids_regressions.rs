//! Actual shared-solid mesh lifecycle retains both full argument-analyzer commands.
use super::*;

#[test]
fn bug25142() {
    case_bugs_process_case::run_bugs_case(
        "v3d_shared_solids_regressions::bug25142",
        v3d_shared_solids::shared_solids_mesh_view,
    );
}

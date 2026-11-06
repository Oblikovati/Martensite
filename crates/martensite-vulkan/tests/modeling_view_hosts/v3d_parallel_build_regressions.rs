//! Actual parallel prepared general build of every restored direct member.
use super::*;

#[test]
fn bug26701() {
    case_bugs_process_case::run_bugs_case(
        "v3d_parallel_build_regressions::bug26701",
        v3d_parallel_build::parallel_general_build_view,
    );
}

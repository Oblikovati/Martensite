//! Anonymous QA presentations and sewn property bindings remain separate objects.
use super::*;

#[test]
fn buc60857() {
    case_v3d_process_case::run_case_within(
        "v3d_trimmed_cones_regressions::buc60857",
        1000,
        v3d_trimmed_cones::trimmed_cones,
    );
}

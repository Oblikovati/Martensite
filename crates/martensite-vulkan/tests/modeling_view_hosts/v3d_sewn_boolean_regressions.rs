//! Complete sewn-solid Boolean original runs genuine construction and mesh checks.
use super::*;

#[test]
fn bug31926() {
    case_bugs_process_case::run_bugs_case(
        "v3d_sewn_boolean_regressions::bug31926",
        v3d_sewn_boolean::sewn_solid_cut_mesh,
    );
}

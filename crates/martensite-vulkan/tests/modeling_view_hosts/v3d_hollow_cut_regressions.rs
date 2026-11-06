//! Actual rotated sphere/cylinder cut and authored hollow mesh-edge presentation.
use super::*;

#[test]
fn bug29751() {
    case_bugs_process_case::run_bugs_case(
        "v3d_hollow_cut_regressions::bug29751",
        mesh_view_cases::hollow_cut_sphere,
    );
}

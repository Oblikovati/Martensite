//! Original planar/parabolic mesh checks and a shape-healing regression.
use super::*;

#[test]
fn bug32471() {
    case_bugs_process_case::run_bugs_case(
        "v3d_mesh_healing_regressions::bug32471",
        model_view_cases::planar_face_zero_deflection,
    );
}

#[test]
fn bug26359() {
    case_bugs_process_case::run_bugs_case(
        "v3d_mesh_healing_regressions::bug26359",
        model_view_cases::parabola_face_shaded,
    );
}

#[test]
fn bug1360() {
    case_bugs_process_case::run_bugs_case(
        "v3d_mesh_healing_regressions::bug1360",
        v3d_mesh_healing_cases::healed_face_presentation,
    );
}

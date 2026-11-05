//! Complete foundational batch02 mesh programs, without wide-host failure wrappers.
use super::*;

#[test]
fn bug25044_22() {
    case_bugs_process_case::run_bugs_case("v3d_mesh_foundation_regressions::bug25044_22", || {
        bugs_mesh_cases::meshed_face_view(
            &bugs_mesh_cases::MESH_TWEAKS,
            "bug25044_face22.brep",
            "bug25044_22.png",
        )
    });
}

#[test]
fn bug25044_58() {
    case_bugs_process_case::run_bugs_case("v3d_mesh_foundation_regressions::bug25044_58", || {
        bugs_mesh_cases::meshed_face_view(
            &bugs_mesh_cases::MESH_TWEAKS,
            "bug25044_face58.brep",
            "bug25044_58.png",
        )
    });
}

#[test]
fn bug23170() {
    case_bugs_process_case::run_bugs_case(
        "v3d_mesh_foundation_regressions::bug23170",
        modalg_display_cases::full_sphere_face_view,
    );
}

#[test]
fn bug28500() {
    case_bugs_process_case::run_bugs_case(
        "v3d_mesh_foundation_regressions::bug28500",
        mesh_view_cases::mesh_faulties,
    );
}

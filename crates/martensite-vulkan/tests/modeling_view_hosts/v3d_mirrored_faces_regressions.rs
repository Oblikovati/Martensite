//! Actual mirrored fourth-face fuse keeps all authored topology, area and validity checks.
use super::*;

#[test]
fn bug25982() {
    case_bugs_process_case::run_bugs_case(
        "v3d_mirrored_faces_regressions::bug25982",
        v3d_mirrored_faces_cases::dataset::mirrored_face_fuse_view,
    );
}

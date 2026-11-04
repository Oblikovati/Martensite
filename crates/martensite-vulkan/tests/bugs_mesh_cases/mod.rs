//! Meshing regressions of the bugs group: a dataset face is cleaned, meshed and shown shaded
//! from its stored triangulation.
use crate::case_view::viewer_session::DisplayRequest;
use crate::dataset_shape::read_dataset_shape;
use crate::session_run::{SessionRun, banner};
use crate::shape_mesh::{
    CountExpectation, MeshRequest, TriangulationExpectation, check_triangulation, mesh_shape,
    remove_triangulation,
};
use std::error::Error;

/// Heading of the BRepMesh tweaks regressions (OCC25044).
pub(crate) const MESH_TWEAKS: [&str; 4] = ["=======", "0025044: BRepMesh tweaks", "=======", ""];

/// Heading of the 2D point coincidence regressions (OCC25588).
pub(crate) const UV_COINCIDENCE: [&str; 4] = [
    "=======",
    "0025588: BRepMesh_ShapeTool::FindUV check for 2d points to be the same is inconsistent with ShapeAnalysis_Wire::CheckLacking",
    "=======",
    "",
];

/// Mesh dataset face `input` at 0.1 linear and 5 degree angular deflection, require triangles,
/// then show its own triangulation shaded and fitted as image `image`.
/// Example: `meshed_face_view(&MESH_TWEAKS, "bug25044_face1.brep", "bug25044_1.png")?`.
pub(crate) fn meshed_face_view(
    heading: &[&str],
    input: &str,
    image: &'static str,
) -> Result<(), Box<dyn Error>> {
    banner(heading);
    let result = read_dataset_shape(input)?;
    remove_triangulation(&result)?;
    mesh_shape(&result, &MeshRequest::new(0.1).with_angle_degrees(5.))?;
    let expected = TriangulationExpectation {
        triangles: CountExpectation::Present,
        ..TriangulationExpectation::default()
    };
    check_triangulation(&result, &expected)?;
    let mut run = SessionRun::new()?;
    run.session().set_display_mode(1)?;
    run.session().set_auto_triangulation(false);
    run.session()
        .display("result", &result, &DisplayRequest::default())?;
    run.session().fit()?;
    run.dump(image)?;
    run.finish()
}

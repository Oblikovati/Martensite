use super::*;
use crate::shape_mesh::{CountExpectation, TriangulationExpectation, check_triangulation};

const REUSED_HEADING: &[&str] = &[
    "=========",
    "OCC32692: Crash when the input shape has some invalid topology",
    "=========",
    "",
];

/// Retain the actual reused status required by the invalid-topology regression.
/// Example: `existing_invalid_mesh()?`.
pub(crate) fn existing_invalid_mesh() -> Outcome {
    banner(REUSED_HEADING);
    println!("REQUIRED ALL: Meshing statuses: Reused");
    let shape = crate::dataset_shape::read_dataset_shape("bug32692_broken_mesh.brep")?;
    let request = MeshRequest {
        parallel: true,
        ..MeshRequest::new(0.01)
    };
    meshing::mesh_with_strategy(&shape, request, MeshAlgoType::Default)?;
    let expected = TriangulationExpectation {
        nodes: CountExpectation::from_checktrinfo_reference("3", 0., 0.)?,
        triangles: CountExpectation::from_checktrinfo_reference("1", 0., 0.)?,
        faces: CountExpectation::from_checktrinfo_reference("1", 0., 0.)?,
        ..Default::default()
    };
    check_triangulation(&shape, &expected)?;
    crate::shape_view_checks::finish_without_view()
}

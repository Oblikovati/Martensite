//! Explicit algorithm selectors and statuses come from the actual incremental mesher.
use crate::shape_mesh::MeshRequest;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::standard::Failure;
use martensite_tkmesh::{
    brep_mesh::BRepMeshIncrementalMesh,
    imesh_tools::{MeshAlgoType, MeshParameters},
};

pub(crate) struct MeshObservation {
    pub status: i32,
    pub effective: MeshParameters,
}

/// Execute the requested strategy and retain actual status/effective parameters.
/// Example: `mesh_with_strategy(&shape, request, MeshAlgoType::Watson)?`.
pub(crate) fn mesh_with_strategy(
    shape: &Shape,
    request: MeshRequest,
    algorithm: MeshAlgoType,
) -> Result<MeshObservation, Failure> {
    let mut parameters = request.parameters();
    parameters.mesh_algo = algorithm;
    let mesher =
        BRepMeshIncrementalMesh::with_parameters(shape, &parameters, &ProgressRange::default())?;
    let status = mesher.get_status_flags();
    println!("Meshing statuses: {}", status_words(status));
    Ok(MeshObservation {
        status,
        effective: mesher.parameters(),
    })
}

const LABELS: [&str; 9] = [
    "OpenWire",
    "SelfIntersectingWire",
    "Failure",
    "ReMesh",
    "UnorientedWire",
    "TooFewPoints",
    "Outdated",
    "Reused",
    "UserBreak",
];

/// Format the original nine status bits without manufacturing success for unknown bits.
/// Example: `assert_eq!(status_words(128), "Reused ")`.
pub(crate) fn status_words(status: i32) -> String {
    if status == 0 {
        return "NoError".to_owned();
    }
    let mut labels = LABELS
        .into_iter()
        .enumerate()
        .filter_map(|(bit, label)| (status & (1 << bit) != 0).then_some(label))
        .collect::<Vec<_>>()
        .join(" ");
    if !labels.is_empty() {
        labels.push(' ');
    }
    labels
}

//! Watson/Delabella comparisons and preserved invalid-input triangulation exercise real meshes.
mod comparison;
#[cfg(test)]
mod cpu;
#[path = "../support/shape_mesh_execution.rs"]
mod meshing;
mod reused;
use crate::session_run::{SessionRun, banner};
use crate::shape_mesh::MeshRequest;
pub(crate) use comparison::{primitive_algorithms, terrain_algorithms};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::Failure;
use martensite_tkmesh::imesh_tools::MeshAlgoType;
pub(crate) use reused::existing_invalid_mesh;
type Outcome = Result<(), Box<dyn std::error::Error>>;

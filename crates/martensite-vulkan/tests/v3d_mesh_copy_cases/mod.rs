//! Mesh-copy checks compare actual independently remeshed shapes with the authored baseline.
mod copy_pair;
#[cfg(test)]
mod cpu;
mod retained;
use crate::session_run::{SessionRun, banner};
use crate::shape_mesh::{
    CountExpectation, MeshRequest, TriangulationExpectation, TriangulationSummary,
    check_triangulation, mesh_shape, remove_triangulation,
};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::Failure;
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPICopy, BRepBuilderAPIMakeShape};
#[path = "../v3d_state_curve_cases/mesh_information.rs"]
mod mesh_information;
pub(crate) use copy_pair::independently_remeshed_copy;
pub(crate) use retained::retained_conical_mesh;
type Outcome = Result<(), Box<dyn std::error::Error>>;

fn copied_shape(shape: &Shape, copy_mesh: bool) -> Result<Shape, Failure> {
    let mut copier = BRepBuilderAPICopy::with_shape(shape, true, copy_mesh)?;
    Ok(copier.shape()?.clone())
}

fn count_expectation(baseline: &TriangulationSummary) -> TriangulationExpectation {
    TriangulationExpectation {
        triangles: CountExpectation::exactly(f64::from(baseline.triangles)),
        nodes: CountExpectation::exactly(f64::from(baseline.nodes)),
        ..Default::default()
    }
}

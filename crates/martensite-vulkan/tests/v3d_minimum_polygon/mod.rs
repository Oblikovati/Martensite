mod comparison;
#[cfg(test)]
mod cpu;
mod dump_nodes;
use crate::case_view::viewer_session::DisplayRequest;
use crate::dataset_shape::read_dataset_shape;
use crate::session_run::{SessionRun, banner};
use crate::shape_mesh::MeshRequest;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::message::ProgressRange;
use martensite_tkmesh::brep_mesh::BRepMeshIncrementalMesh;
type BoxError = Box<dyn std::error::Error>;
type Outcome = Result<(), BoxError>;
const HEADING: &[&str] = &["========", "OCC26532", "========", ""];

fn mesh_polygon_source_edge(shape: &Shape, minimum: Option<f64>) -> Outcome {
    let mut parameters = MeshRequest::new(0.3).parameters();
    if let Some(minimum) = minimum {
        parameters.min_size = minimum;
    }
    BRepMeshIncrementalMesh::with_parameters(shape, &parameters, &ProgressRange::default())?;
    Ok(())
}

fn read_edge_polygon_dump_nodes(shape: &Shape) -> Result<Vec<dump_nodes::DumpNode>, BoxError> {
    dump_nodes::parse_polygon_dump_nodes(&crate::shape_dump::complete_shape_dump_text(shape)?)
}

fn capture_minimum_polygon_branch(
    run: &mut SessionRun,
    shape: &Shape,
) -> Result<Vec<dump_nodes::DumpNode>, BoxError> {
    run.session().clear()?;
    mesh_polygon_source_edge(shape, Some(0.06))?;
    run.session()
        .display("a", shape, &DisplayRequest::default())?;
    run.dump("bug26532_2.png")?;
    read_edge_polygon_dump_nodes(shape)
}

fn capture_default_polygon_branch(
    run: &mut SessionRun,
) -> Result<Vec<dump_nodes::DumpNode>, BoxError> {
    let shape = read_dataset_shape("bug26533_aal2.brep")?;
    run.session()
        .display("a2", &shape, &DisplayRequest::default())?;
    run.session().clear()?;
    mesh_polygon_source_edge(&shape, None)?;
    run.session()
        .display("a2", &shape, &DisplayRequest::default())?;
    run.dump("bug26532_3.png")?;
    read_edge_polygon_dump_nodes(&shape)
}

/// Preserve three views, two fresh meshes and the authored all-three-coordinate predicate.
/// Example: `minimum_size_polygon_comparison()?`.
pub(crate) fn minimum_size_polygon_comparison() -> Outcome {
    banner(HEADING);
    let shape = read_dataset_shape("bug26533_aal2.brep")?;
    let mut run = SessionRun::new()?;
    run.session()
        .display("a", &shape, &DisplayRequest::default())?;
    run.session().fit()?;
    run.dump("bug26532_1.png")?;
    let first = capture_minimum_polygon_branch(&mut run, &shape)?;
    let second = capture_default_polygon_branch(&mut run)?;
    comparison::report_dump_polygon_nodes(&first, &second);
    run.finish()
}

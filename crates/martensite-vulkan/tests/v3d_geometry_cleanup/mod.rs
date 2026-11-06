#[cfg(test)]
mod cpu;
mod dump_counts;
mod fusion;
use crate::shape_mesh::{MeshRequest, remove_geometry_only};
use martensite_tkbrep::topods::Shape;
use martensite_tkmesh::imesh_tools::MeshAlgoType;
type BoxError = Box<dyn std::error::Error>;
type Outcome = Result<(), BoxError>;
const HEADING: &[&str] = &[
    "=================================================================",
    "0033049: Command 'tclean -geom' does not remove all geometric objects from the shape",
    "=================================================================",
    "",
];

fn actual_geometry_counts(shape: &Shape) -> Result<dump_counts::GeometryDumpCounts, BoxError> {
    dump_counts::geometry_dump_counts(&crate::shape_dump::complete_shape_dump_text(shape)?)
}

/// Preserve genuine prepared fuse, mesh, geometry-only clean and all three dump counts.
/// Example: `geometry_removal_after_fuse()?`.
pub(crate) fn geometry_removal_after_fuse() -> Outcome {
    crate::session_run::banner(HEADING);
    let result = fusion::actual_prepared_multi_tool_fuse()?;
    crate::mesh_execution::mesh_with_strategy(
        &result,
        MeshRequest::new(0.1),
        MeshAlgoType::Default,
    )?;
    remove_geometry_only(&result)?;
    if actual_geometry_counts(&result)?.all_removed() {
        println!("OK: shape has no geometry");
    } else {
        println!("Error: geometry is not fully removed");
    }
    crate::shape_view_checks::finish_without_view()
}

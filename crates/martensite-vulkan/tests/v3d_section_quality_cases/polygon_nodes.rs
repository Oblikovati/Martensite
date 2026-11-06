use super::*;
use crate::boolean_typed_explode::all_distinct;
use crate::shape_mesh::MeshRequest;
use martensite_tkg3d::top_abs::ShapeEnum;

pub(super) fn fourth_edge_dump(shape: &Shape) -> Result<String, Box<dyn std::error::Error>> {
    let edges = all_distinct(shape, ShapeEnum::Edge);
    let edge = edges.get(3).ok_or_else(|| {
        martensite_tkernel::standard::exceptions::program_error(&format!(
            "exploded edges={}; expected at least4 for result_4",
            edges.len()
        ))
    })?;
    crate::shape_dump::complete_shape_dump_text(edge)
}

pub(super) fn checked_mesh() -> Result<Shape, Failure> {
    let shape = read_dataset_shape("bug27239_f1.brep")?;
    let observed = crate::mesh_execution::mesh_with_strategy(
        &shape,
        MeshRequest::new(0.1),
        martensite_tkmesh::imesh_tools::MeshAlgoType::Default,
    )?;
    if !crate::mesh_execution::status_words(observed.status).contains("NoError") {
        println!("Error: failure during triangulation");
    }
    if !crate::mesh_topology_check::check_mesh_topology(&shape)?.is_clean() {
        println!("Error : Some faces has failed to mesh");
    }
    crate::shape_report_checks::check_shape_structure(&shape)?;
    Ok(shape)
}

/// Apply the original literal dump predicate after actual meshing/topology/shape checks.
/// Example: `checked_polygon_nodes()?`.
pub(crate) fn checked_polygon_nodes() -> Outcome {
    banner(&["=========", "OCC27239", "=========", ""]);
    let shape = checked_mesh()?;
    if fourth_edge_dump(&shape)?.contains("5 5") {
        println!("Error: there are duplicating nodes in polygon on triangulation");
    }
    crate::shape_view_checks::checkview_3d(&shape, "bug27239.png")
}

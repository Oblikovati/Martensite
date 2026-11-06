//! The original tetrahedron is sewn and subtracted by the genuine Boolean engine.
#[cfg(test)]
mod cpu;
mod geometry;
mod shell_solid;
use crate::session_run::{SessionRun, banner};
use crate::shape_mesh::{MeshRequest, mesh_shape};
use martensite_tkbo::bop_algo::Operation;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::Failure;
type Outcome = Result<(), Box<dyn std::error::Error>>;
const HEADING: &[&str] = &[
    "========",
    "0031926: Shape Healing - ShapeAnalysis::OuterWire() considers next iteration element always to be a wire causing skipping of primal one",
    "========",
    "",
];

pub(super) fn sphere_minus_tetrahedron() -> Result<Shape, Failure> {
    let faces = geometry::source_faces()?;
    let sphere = crate::primitive_shapes::sphere(1.)?;
    let solid = geometry::sew_faces(&faces)?;
    Ok(crate::common_operation::direct(
        &sphere,
        &solid,
        Operation::Cut,
    ))
}

/// Execute all four planar-face/sewing/solid/Boolean commands before the mesh check.
/// Example: `sewn_solid_cut_mesh()?`.
pub(crate) fn sewn_solid_cut_mesh() -> Outcome {
    banner(HEADING);
    let result = sphere_minus_tetrahedron()?;
    mesh_shape(&result, &MeshRequest::new(1.))?;
    let mut run = SessionRun::new()?;
    crate::shape_view_checks::checkview_in(&mut run, &result, "bug31926.png")?;
    if crate::mesh_topology_check::check_mesh_topology(&result)?.is_clean() {
        println!("Mesh is OK");
    } else {
        println!("Error : Invalid mesh");
    }
    run.finish()
}

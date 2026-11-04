//! Mesh termination and object-owned exact hidden-line computation.
use crate::case_view::viewer_session::DisplayRequest;
use crate::dataset_shape::read_dataset_shape;
use crate::session_run::{SessionRun, banner};
use crate::shape_mesh::{MeshRequest, mesh_shape};
use martensite_tkv3d::{prs3d::Prs3dTypeOfHlr, v3d::V3dTypeOfOrientation};
#[cfg(test)]
mod cpu;
use martensite_tkernel::standard::Failure;
#[path = "../v3d_state_curve_cases/mesh_information.rs"]
mod mesh_information;
type Outcome = Result<(), Box<dyn std::error::Error>>;

/// Mesh the original valid solid within its worker's ten-second CPU budget.
/// Example: `outer_wire_termination()?`.
pub(crate) fn outer_wire_termination() -> Outcome {
    banner(&[
        "=======",
        "0031144: Shape Healing - ShapeAnalysis::OuterWire() infinite loop on solid obtained from IFC",
        "=======",
        "",
    ]);
    let shape = read_dataset_shape("bug31144.brep")?;
    crate::shape_report_checks::check_shape_structure(&shape)?;
    crate::shape_report_checks::print_tolerances(&shape)?;
    println!(
        "Note: incmesh is called because this is the only known way to call problematic method"
    );
    mesh_shape(&shape, &MeshRequest::new(1.))?;
    mesh_information::print_information(&shape)?;
    crate::shape_view_checks::finish_without_view()
}

/// Show the original filleted shape from behind, with its own exact HLR algorithm.
/// Example: `filleted_exact_hidden_lines()?`.
pub(crate) fn filleted_exact_hidden_lines() -> Outcome {
    banner(&["============", "OCC27782", "============", ""]);
    let result = read_dataset_shape("bug27782_hlr.brep")?;
    let mut run = SessionRun::new()?;
    run.session()
        .display("result", &result, &DisplayRequest::default())?;
    run.session()
        .set_standard_projection(V3dTypeOfOrientation::ZUP_BACK)?;
    run.session().fit()?;
    run.session().set_hidden_line_removal(true, None)?;
    crate::hlr_commands::set_object_algorithm(run.session(), "result", Prs3dTypeOfHlr::Algo)?;
    run.dump("bug27782.png")?;
    run.finish()
}

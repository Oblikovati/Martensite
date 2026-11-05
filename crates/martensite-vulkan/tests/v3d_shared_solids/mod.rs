#[cfg(test)]
mod cpu;
mod geometry;
use crate::case_view::viewer_session::{DisplayRequest, ViewerSession};
use crate::session_run::{SessionRun, banner};
use crate::shape_mesh::{MeshRequest, mesh_shape, remove_triangulation};
use martensite_tkbrep::topods::Shape;
use std::collections::BTreeMap;
type BoxError = Box<dyn std::error::Error>;

type FullReport = crate::full_argument_response::RetainedFullArgumentReport;
struct RetainedSourceDiagnostics {
    reports: [FullReport; 2],
    bindings: BTreeMap<String, Shape>,
}

fn captured_full_report(
    first: &Shape,
    second: Option<&Shape>,
    bindings: &mut BTreeMap<String, Shape>,
) -> Result<FullReport, BoxError> {
    let report = crate::full_argument_response::full_argument_report(first, second)?;
    print!("{}", report.output);
    bindings.extend(report.bindings.iter().cloned());
    Ok(report)
}

fn full_source_diagnostics(
    model: &geometry::SharedSolidsSource,
) -> Result<RetainedSourceDiagnostics, BoxError> {
    crate::shape_report_checks::print_tolerances(&model.result)?;
    crate::shape_report_checks::check_shape_structure(&model.result)?;
    let mut bindings = BTreeMap::new();
    let first = captured_full_report(&model.result, None, &mut bindings)?;
    let parts = &model.parts;
    let second = captured_full_report(&parts[0], Some(&parts[1]), &mut bindings)?;
    Ok(RetainedSourceDiagnostics {
        reports: [first, second],
        bindings,
    })
}

pub(super) fn show_source_parts(
    session: &mut ViewerSession,
    parts: &[Shape; 2],
) -> Result<(), BoxError> {
    session.set_display_mode(1)?;
    session.display("r_1", &parts[0], &DisplayRequest::default())?;
    session.display("r_2", &parts[1], &DisplayRequest::default())?;
    crate::mesh_topology_check::report_mesh_validity(&parts[0])?;
    Ok(())
}

/// Preserve shared-face meshing, both full analyzers and automatic shaded remeshing.
/// Example: `shared_solids_mesh_view()?`.
pub(crate) fn shared_solids_mesh_view() -> Result<(), BoxError> {
    banner(&["============", "CR25142", "============", ""]);
    let model = geometry::actual_shared_solids()?;
    let bindings = full_source_diagnostics(&model)?;
    geometry::sequential_mesh_checks(&model.parts)?;
    remove_triangulation(&model.result)?;
    let mut run = SessionRun::new()?;
    show_source_parts(run.session(), &model.parts)?;
    run.session().fit()?;
    run.dump("bug25142.png")?;
    let result = run.finish();
    drop(bindings);
    drop(model);
    result
}

#[cfg(test)]
mod cpu;
mod drawings;
mod geometry;
use crate::case_view::viewer_session::ViewerSession;
use crate::session_run::{SessionRun, banner};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg2d::{
    geom2d::{Geom2dCurve, Geom2dLine},
    geom2d_adaptor::Geom2dAdaptorCurve,
};
use martensite_tkg3d::geom::GeomCurve;
use martensite_tkgeomalgo::{
    gcc_ent::Position,
    geom_api,
    geom2d_gcc::{Lin2d2Tan, QualifiedCurve},
};
type Outcome = Result<(), Box<dyn std::error::Error>>;

pub(super) fn authored_solution_indices(count: i32) -> std::ops::Range<i32> {
    // RegressionCommands deliberately starts at0; preserve current behavior, never repair this loop.
    0..count
}

fn bind_and_display(
    session: &ViewerSession,
    bindings: &mut drawings::NamedPlanarCurves,
    name: &str,
    curve: &Handle<dyn Geom2dCurve>,
    source: &geometry::SpatialConics,
) -> Result<(), Failure> {
    drawings::bind_planar_curve(bindings, name, curve);
    drawings::display_reembedded_curve(session, curve, source)
}

pub(super) fn execute_case_command(
    session: &ViewerSession,
    bindings: &mut drawings::NamedPlanarCurves,
) -> Result<String, Failure> {
    let source = geometry::spatial_conics()?;
    let ellipse = geom_api::to_2d(&source.ellipse, &source.plane.pln())?;
    bind_and_display(session, bindings, "ell", &ellipse, &source)?;
    let circle = geom_api::to_2d(&source.circle, &source.plane.pln())?;
    bind_and_display(session, bindings, "cil", &circle, &source)?;
    let solver = display_case_tangents(session, bindings, &source, &ellipse, &circle)?;
    session.context().update_current_viewer();
    Ok(format!(" Is Done = \n{}", i32::from(solver.is_done())))
}

fn display_case_tangents(
    session: &ViewerSession,
    bindings: &mut drawings::NamedPlanarCurves,
    source: &geometry::SpatialConics,
    ellipse: &Handle<dyn Geom2dCurve>,
    circle: &Handle<dyn Geom2dCurve>,
) -> Result<Lin2d2Tan, Failure> {
    let solver = geometry::case_tangent_solver(ellipse, circle)?;
    for index in authored_solution_indices(solver.nb_solutions()) {
        let curve: Handle<dyn Geom2dCurve> =
            Handle::new(Geom2dLine::from_lin2d(solver.this_solution(index)?)).cast();
        let name = format!("lintan_{index}");
        bind_and_display(session, bindings, &name, &curve, source)?;
    }
    Ok(solver)
}

/// Keep the view, the hidden command and both authored catch branches before the screenshot.
/// Example: `conic_tangent_catch_regression()?`.
pub(crate) fn conic_tangent_catch_regression() -> Outcome {
    banner(&["========", "BUC60842", "========"]);
    let mut run = SessionRun::new()?;
    let mut bindings = Vec::new();
    let catch_result = execute_case_command(run.session(), &mut bindings);
    if catch_result.is_err() {
        println!("Faulty BUC60842");
    } else {
        println!("BUC60842 OK");
    }
    run.dump("buc60842.png")?;
    let completion = run.finish();
    drop(bindings);
    completion
}

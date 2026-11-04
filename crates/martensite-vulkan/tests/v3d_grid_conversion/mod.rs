//! The two authored conversion frames follow the synthetic grid detection phase.
mod checks;
#[cfg(test)]
mod cpu;
mod frames;
mod observations;
use crate::{
    case_view::viewer_session::ViewerSession,
    session_run::{SessionRun, banner},
};
use martensite_tkernel::standard::Failure;
type Outcome = Result<(), Box<dyn std::error::Error>>;

/// Run the complete detection, two conversion matrices and mandatory dump; `grid_conversion()?`.
pub(crate) fn grid_conversion() -> Outcome {
    banner(&[
        "============",
        "CR25671: V3dView::Convert doesn't work as expected in GRID active mode.",
        "============",
        "",
    ]);
    let mut run = SessionRun::with_view_size(400, 400)?;
    synthetic_detection(run.session())?;
    let _first = first_frame(run.session())?;
    let _second = second_frame(run.session())?;
    run.dump("bug25671.png")?;
    run.finish()
}
fn synthetic_detection(session: &mut ViewerSession) -> Result<(), Failure> {
    frames::detection_scene(session)?;
    session.move_cursor(197, 229)?;
    if !session.check_color(197, 229, [0., 1., 1.])? {
        println!("Error : Detection does not work correctly with activated grid.");
    }
    session.hide_grid()?;
    session.clear()
}
pub(super) fn first_frame(session: &ViewerSession) -> Result<bool, Failure> {
    frames::first(session)?;
    let observed = observations::observe(session, [0.5, 100., 0.5], false)?;
    Ok(checks::first(&observed))
}
pub(super) fn second_frame(session: &ViewerSession) -> Result<bool, Failure> {
    frames::second(session)?;
    let observed = observations::observe(session, [25., 4.94, 35.], true)?;
    Ok(checks::second(&observed))
}

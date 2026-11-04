//! Preserve genuine opposite extrusions and the weak entity-type diagnostic after the image.
#[cfg(test)]
mod controls;
mod geometry;

use crate::case_view::viewer_session::{DisplayRequest, ViewerSession};
use crate::quadric_source_checks::has_cylinder;
use crate::session_run::{SessionRun, banner};
use martensite_tkernel::standard::Failure;
use std::error::Error;

fn source_scene(
    session: &mut ViewerSession,
    inputs: &geometry::PrismInputs,
) -> Result<(), Failure> {
    session.clear()?;
    session.set_axonometric()?;
    let shaded = DisplayRequest::default().with_display_mode(1);
    session.display_many(
        &[("pr1", &inputs.prisms[0]), ("pr2", &inputs.prisms[1])],
        &shaded,
    )?;
    session.fit()?;
    session.display_active_sensitive()
}

fn cylinder_detected(session: &mut ViewerSession, x: i32, y: i32) -> Result<bool, Failure> {
    session.move_cursor(x, y)?;
    Ok(has_cylinder(&session.picked_entity_state_types()?))
}

fn source_probes(session: &mut ViewerSession) -> Result<(), Failure> {
    for (x, y) in [(130, 330), (270, 130)] {
        if !cylinder_detected(session, x, y)? {
            println!("Error: cylinder should be detected");
        }
    }
    Ok(())
}

/// Retain the mandatory pre-probe sensitive presentation; `complete_signed_prism()?`.
pub(crate) fn complete_signed_prism() -> Result<(), Box<dyn Error>> {
    banner(&[
        "=============",
        "0033084: Visualization - Cylindrical prism is selectable only by its base when extruded in some directions",
        "=============",
    ]);
    let inputs = geometry::source_inputs()?;
    let mut run = SessionRun::new()?;
    source_scene(run.session(), &inputs)?;
    run.dump("bug33084_prism_sensitive_prs.png")?;
    source_probes(run.session())?;
    run.finish()
}

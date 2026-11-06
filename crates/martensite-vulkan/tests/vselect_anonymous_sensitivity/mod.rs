//! Direct QA context calls retain anonymous boxes, no redraw on MoveTo, and command failures.
#[cfg(test)]
mod controls;
mod scene;

use crate::case_view::viewer_session::ViewerSession;
use crate::session_run::{SessionRun, banner};
use martensite_tkernel::standard::{Failure, exceptions::program_error};
use martensite_tkv3d::ais::AisSelectionScheme;
use std::error::Error;

const SECOND_BOX_DIAGNOSTIC: &str = "ERROR: b2 is selected after b1's tolerance increased";

fn move_then_toggle(session: &ViewerSession, x: i32) -> Result<(), Failure> {
    session.context().move_to(x, 322, session.view(), false)?;
    session.context().select_detected(AisSelectionScheme::Xor)?;
    Ok(())
}

fn require_selected(
    session: &ViewerSession,
    expected: i32,
    diagnostic: &str,
) -> Result<(), Failure> {
    let count = session.context().nb_selected();
    if count == expected {
        return Ok(());
    }
    println!("{diagnostic}");
    Err(program_error(&format!(
        "selected owner count {count}; expected {expected}"
    )))
}

fn selection_command(session: &ViewerSession) -> Result<(), Failure> {
    let objects = scene::display_source_boxes(session)?;
    move_then_toggle(session, 305)?;
    move_then_toggle(session, 103)?;
    require_selected(session, 0, "ERROR: no boxes must be selected!")?;
    session
        .context()
        .set_selection_sensitivity(&objects[0], 2, 5)?;
    move_then_toggle(session, 305)?;
    require_selected(session, 1, "ERROR: b1 was not selected")?;
    move_then_toggle(session, 103)?;
    require_selected(session, 1, SECOND_BOX_DIAGNOSTIC)
}

/// Run the original command and dump only after it succeeds; `complete_anonymous_sensitivity()?`.
pub(crate) fn complete_anonymous_sensitivity() -> Result<(), Box<dyn Error>> {
    banner(&[
        "============",
        "CR26462: Visualization - selection does not adapt to line width change",
    ]);
    banner(&["============", ""]);
    let mut run = SessionRun::new()?;
    selection_command(run.session())?;
    run.dump("bug26462_2.png")?;
    run.finish()
}

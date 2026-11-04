//! Literal z-layer depth-clear selection original, including its soft pixel diagnostic.
use super::layer::source_layer;
use super::scene::{source_boxes, source_inputs, source_osd_vertex};
use crate::session_run::{SessionRun, banner};
use std::error::Error;

const SOURCE_BANNER: [&str; 3] = [
    "========",
    "Consider ZLayer properties while sorting list of picked entities",
    "========",
];

/// Run the complete original without substituting the picked owner; `complete_zlayer()?`.
pub(crate) fn complete_zlayer() -> Result<(), Box<dyn Error>> {
    banner(&SOURCE_BANNER);
    let inputs = source_inputs()?;
    let mut run = SessionRun::new()?;
    let layer = source_layer(run.session())?;
    run.session().set_axonometric()?;
    source_boxes(run.session(), &inputs, layer)?;
    run.session().fit()?;
    run.session().rotate(0.1, 0., 0.)?;
    source_selection_commands(run.session())?;
    source_osd_vertex(run.session(), &inputs[2])?;
    run.dump("bug27797.png")?;
    run.finish()
}

fn source_selection_commands(
    session: &mut crate::case_view::viewer_session::ViewerSession,
) -> Result<(), martensite_tkernel::standard::Failure> {
    session.move_cursor(100, 300)?;
    let actual_state = crate::selection_queries::state_text(session, &["b1", "b2"], false)?;
    print!("{actual_state}");
    session.select_point(100, 300)?;
    session.move_cursor(0, 0)?;
    let actual = session.pixel_color_name(100, 300)?;
    if let Some(diagnostic) = selection_color_error(&actual) {
        println!("{diagnostic}");
    }
    Ok(())
}

/// Preserve the exact source string predicate and soft error; `selection_color_error("GRAY76") == None`.
pub(super) fn selection_color_error(actual: &str) -> Option<&'static str> {
    let mismatch = actual != "GRAY76";
    mismatch.then_some("Error: wrong object has been selected")
}

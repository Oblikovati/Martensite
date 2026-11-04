//! The callback and selection belong to the same real interactive object.
mod command;
#[cfg(test)]
mod controls;
mod element;
mod object;
use crate::session_run::{SessionRun, banner};
type Outcome = Result<(), Box<dyn std::error::Error>>;
const BANNER: &[&str] = &[
    "===========",
    "OCC23067: OpenGl package API needed by UserDraw feature is not exported",
    "===========",
    "",
];

/// Run the unchanged authored checks; `workspace_user_draw()?`.
pub(crate) fn workspace_user_draw() -> Outcome {
    banner(BANNER);
    let mut run = SessionRun::new()?;
    command::display_user_draw(run.session(), "a")?;
    run.session().fit()?;
    run.session().check_color(321, 159, [1., 1., 0.])?;
    run.dump("bug23067.png")?;
    run.finish()
}

//! The tutorial's actual object/owner callbacks share the live session identity and animation.
#[cfg(test)]
mod controls;
mod geometry;
mod object;
mod owner;
mod owner_click;
mod owner_highlight;
use crate::session_run::SessionRun;
type Outcome = Result<(), Box<dyn std::error::Error>>;

/// Execute every authored command; register only with the inherited budget; `tutorial_object()?`.
pub(crate) fn tutorial_object() -> Outcome {
    crate::session_run::banner(&[
        "==============================",
        "0032668: Documentation - add tutorial for creating a custom AIS Interactive Object",
        "==============================",
    ]);
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    let object = object::TutorialObject::new(run.session().objects_animation())?;
    run.session().display_object("p", object.cast())?;
    run.session().fit()?;
    run.session().move_cursor(300, 300)?;
    run.dump("aisobject.png")?;
    run.finish()
}

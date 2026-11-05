//! Source display keeps the three anonymous originals even after each named BRep is sewn.
#[cfg(test)]
mod cpu;
mod geometry;
mod measurements;
use crate::{
    case_view::viewer_session::ViewerSession,
    session_run::{SessionRun, banner},
};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::{
    quantity::{Color, NamedColor},
    standard::{Failure, Handle},
};
use martensite_tkv3d::ais::{AisInteractiveObject, AisShape};
type Outcome = Result<(), Box<dyn std::error::Error>>;
/// Execute the anonymous trimmed-cone QA construction and all original property checks; `trimmed_cones()?`.
pub(crate) fn trimmed_cones() -> Outcome {
    banner(&["========", "BUC60857", "========"]);
    let mut run = SessionRun::new()?;
    let shapes = geometry::faces()?;
    show(run.session(), &shapes)?;
    run.session().fit()?;
    let sewn = geometry::sewn_bindings(&shapes)?;
    measurements::report(&sewn)?;
    run.dump("buc60857.png")?;
    run.finish()
}
pub(super) fn show(session: &ViewerSession, shapes: &[Shape; 3]) -> Result<(), Failure> {
    for (index, color) in [NamedColor::BLUE1, NamedColor::RED, NamedColor::GREEN]
        .into_iter()
        .enumerate()
    {
        let object: Handle<dyn AisInteractiveObject> = AisShape::new(&shapes[index]).cast();
        session.context().display(&object, false)?;
        session
            .context()
            .set_color(&object, Color::named(color), index == 2)?;
    }
    Ok(())
}

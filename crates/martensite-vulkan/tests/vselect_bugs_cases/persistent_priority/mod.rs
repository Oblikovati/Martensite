//! Whole 2D-persistent selection-priority cases; the source's primitive objects stay real.
mod arrays;
mod boxes;
mod commands;
#[cfg(test)]
mod controls;
mod objects;
mod point;
mod quad;

pub(crate) use arrays::priority_arrays;
pub(crate) use boxes::priority_boxes;
pub(crate) use point::priority_point;
pub(crate) use quad::priority_quad;

use crate::case_view::viewer_session::{FaceBoundary, ViewerSession};
use crate::session_run::{SessionRun, banner};
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkernel::standard::{Failure, Handle, exceptions::program_error};
use martensite_tkv3d::ais::AisInteractiveObject;
use std::error::Error;
type Outcome = Result<(), Box<dyn Error>>;

fn priority_begin() -> Result<SessionRun, Box<dyn Error>> {
    banner(&[
        "=============",
        "0030484: Visualization - 2d persistent: order of detection doesn't coincide with order of objects creation",
        "=============",
    ]);
    // VClear before VInit is a no-op: the isolated worker has no current view yet.
    let mut run = SessionRun::new()?;
    commands::white_dynamic(run.session())?;
    Ok(run)
}

fn named(session: &ViewerSession, name: &str) -> Result<Handle<dyn AisInteractiveObject>, Failure> {
    session.object(name).cloned().ok_or_else(|| {
        program_error(&format!(
            "object {name:?}; expected a displayed original operand"
        ))
    })
}

fn material_color(session: &ViewerSession, name: &str, color: NamedColor) -> Result<(), Failure> {
    session.set_material(name, commands::PLASTIC)?;
    session.set_color(name, Color::named(color))?;
    Ok(())
}

fn face_boundaries(session: &ViewerSession, name: &str) -> Result<(), Failure> {
    let style = FaceBoundary::default();
    session.draw_face_boundaries(name, &style)?;
    Ok(())
}

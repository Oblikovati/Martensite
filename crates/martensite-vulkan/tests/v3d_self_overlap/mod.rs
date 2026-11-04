#[cfg(test)]
mod cpu;
mod geometry;
use crate::case_view::viewer_session::DisplayRequest;
use crate::session_run::{SessionRun, banner};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::{
    quantity::{Color, NamedColor},
    standard::Failure,
};
use martensite_tktopalgo::brep_extrema::BRepExtremaSelfIntersection;
type BoxError = Box<dyn std::error::Error>;

fn show_source_compound(shape: &Shape) -> Result<SessionRun, BoxError> {
    let mut run = SessionRun::new()?;
    run.session().set_display_mode(1)?;
    run.session()
        .display("C", shape, &DisplayRequest::default())?;
    run.session().set_transparency("C", 0.8)?;
    run.dump("bug26180_1.png")?;
    Ok(run)
}

fn show_overlapped_faces(run: &mut SessionRun, faces: &Shape) -> Result<(), BoxError> {
    run.session()
        .display("C_overlapped", faces, &DisplayRequest::default())?;
    run.session()
        .set_color("C_overlapped", Color::named(NamedColor::RED))?;
    run.session().set_transparency("C_overlapped", 0.5)?;
    run.session().fit()?;
    run.dump("bug26180_2.png")?;
    Ok(())
}

/// Preserve all 900 distinct boxes, actual meshed sphere and actual overlap-face output.
/// Example: `self_overlap_grid_view()?`.
pub(crate) fn self_overlap_grid_view() -> Result<(), BoxError> {
    banner(&["========", "OCC26180", "========", ""]);
    let compound = geometry::source_scene_compound()?;
    let mut run = show_source_compound(&compound)?;
    let faces = geometry::actual_self_overlap_faces(&compound)?;
    show_overlapped_faces(&mut run, &faces)?;
    run.finish()
}

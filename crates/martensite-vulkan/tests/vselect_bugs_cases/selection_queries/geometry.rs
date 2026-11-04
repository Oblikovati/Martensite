//! The source finite line uses the same injected units fixture as existing line regressions.
use super::*;
use martensite_tkg3d::geom::{GeomCartesianPoint, GeomPoint};
use martensite_tkv3d::ais::AisLine;
#[allow(dead_code)] // Shared fixture additionally supplies unrelated existing infinite-line helpers.
#[path = "../../viewer_fixtures/support/ais_line_fixture.rs"]
mod line_fixture;

/// Construct the actual finite datum with isolated existing units; `finite_line()`.
pub(super) fn finite_line() -> Handle<AisLine> {
    let start: Handle<dyn GeomPoint> = Handle::new(GeomCartesianPoint::new(0., 0., 0.)).cast();
    let end: Handle<dyn GeomPoint> = Handle::new(GeomCartesianPoint::new(10., 10., 0.)).cast();
    AisLine::from_points(&start, &end, line_fixture::units())
}

/// Keep both line displays and the fit-affecting datum point; `display_line_scene(session)?`.
pub(super) fn display_line_scene(session: &mut ViewerSession) -> Result<Handle<AisLine>, Failure> {
    let shape = crate::primitive_shapes::box_sized([10., 10., 0.1])?;
    session.display("b", &shape, &DisplayRequest::default().with_display_mode(1))?;
    let line = finite_line();
    session.display_object("l", line.cast())?;
    // VLine already displayed it; the following VDisplay remains a real second operation.
    let object = query_named_object(session, "l")?;
    session.update_context(|context| context.display(&object, false))?;
    session.display_object("p", crate::viewer_objects::point_object([20., 20., 0.])?)?;
    Ok(line)
}

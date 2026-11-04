//! The actual dynamic highlight remains lit after the authored rotation.
use super::*;
use crate::primitive_shapes::box_sized;
use martensite_tkbrep::topods::Shape;
use martensite_tkv3d::v3d::V3dTypeOfOrientation;
const BANNER: &[&str] = &[
    "========",
    "OCC27617: Visualization, Renderer - apply highlighting color without disabling lighting",
    "========",
];

/// Apply the literal shaded/highlighted box and camera operations; `highlight_geometry(session, &shape)?`.
pub(super) fn highlight_geometry(
    session: &mut ViewerSession,
    shape: &Shape,
) -> Result<(), Failure> {
    session.clear()?;
    let request = DisplayRequest {
        display_mode: Some(1),
        highlight_mode: Some(1),
        ..Default::default()
    };
    session.display("b", shape, &request)?;
    session.set_standard_projection(V3dTypeOfOrientation::ZUP_AXO_RIGHT)?;
    session.fit()?;
    session.rotate(1., 0., 0.)?;
    session.fit()?;
    session.move_cursor(250, 150)
}

/// Preserve the exact case-sensitive source predicate; `highlight_colour_error("TURQUOISE3")`.
pub(super) fn highlight_colour_error(actual: &str) -> Option<&'static str> {
    (actual != "TURQUOISE3").then_some("Error: Wrong dynamic highlight color")
}

/// Complete the exact named-RGB check and image; `lit_dynamic_highlight()?`.
pub(crate) fn lit_dynamic_highlight() -> Outcome {
    banner(BANNER);
    let shape = box_sized([1., 2., 3.])?;
    let mut run = SessionRun::new()?;
    highlight_geometry(run.session(), &shape)?;
    let colour = run.session().pixel_color_name(200, 150)?;
    if let Some(error) = highlight_colour_error(&colour) {
        println!("{error}");
    }
    run.dump("bug27617.png")?;
    run.finish()
}

use super::named_pixel;
use crate::{
    case_view::viewer_session::{DisplayRequest, HighlightStyle, ViewerSession},
    primitive_shapes::box_sized,
    session_run::{SessionRun, banner},
};
use martensite_tkernel::{
    quantity::{Color, NamedColor},
    standard::Failure,
};
use martensite_tkv3d::prs3d::Prs3dTypeOfHighlight;
use std::error::Error;

/// Pass the exact configured context style to global highlighting; `explicit_highlight(session)?`.
pub(super) fn explicit_highlight(session: &ViewerSession) -> Result<(), Failure> {
    let style = (*session
        .context()
        .highlight_style(Prs3dTypeOfHighlight::Dynamic)?)
    .clone();
    session
        .context()
        .hilight_with_color(session.object("b").unwrap(), &style, true)
}

/// Replay the complete authored regression; `highlight()?`.
pub(crate) fn highlight() -> Result<(), Box<dyn Error>> {
    banner(&[
        "============",
        "0031965: Visualization - AisInteractiveContext::HilightWithColor() ignores passed highlight style",
        "============",
        "",
    ]);
    let mut run = SessionRun::new()?;
    let session = run.session();
    shaded_box(session)?;
    red_dynamic_style(session)?;
    explicit_highlight(session)?;
    named_pixel(session, 200, 200, "RED2", "Error: wrong highlight color")?;
    run.dump("bug31965.png")?;
    run.finish()
}

fn shaded_box(session: &mut ViewerSession) -> Result<(), Failure> {
    session.display(
        "b",
        &box_sized([1., 2., 3.])?,
        &DisplayRequest::default().with_display_mode(1),
    )?;
    session.fit()?;
    Ok(())
}

/// Apply the authored dynamic selection properties; `red_dynamic_style(session)?`.
pub(super) fn red_dynamic_style(session: &ViewerSession) -> Result<(), Failure> {
    session.set_highlight_style(
        Prs3dTypeOfHighlight::Dynamic,
        &HighlightStyle {
            display_mode: Some(1),
            color: Some(Color::named(NamedColor::RED)),
            ..Default::default()
        },
    )
}

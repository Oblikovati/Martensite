//! The sample uses screen-sized Arial labels at exact model-space anchors.
use super::*;
use crate::case_view::viewer_session::TextLabelRequest;
use martensite_tkservice::{
    font::FontFontAspect,
    graphic3d::{
        Graphic3dHorizontalTextAlignment as Horizontal, Graphic3dVerticalTextAlignment as Vertical,
    },
};

pub(super) fn draw_labels(session: &mut ViewerSession) -> Result<(), Failure> {
    for (index, material) in MATERIALS.iter().enumerate() {
        draw_label(
            session,
            material,
            [20., 15. + index as f64 * 10., 0.],
            "GRAY10",
            Horizontal::Right,
        )?;
    }
    draw_headers(session)
}
fn draw_headers(session: &mut ViewerSession) -> Result<(), Failure> {
    for (index, color) in COLUMN_COLORS.iter().enumerate() {
        draw_label(
            session,
            color,
            [40. + index as f64 * 35., 5., 0.],
            if index == 0 { "BLACK" } else { color },
            Horizontal::Center,
        )?;
    }
    Ok(())
}
fn draw_label(
    session: &mut ViewerSession,
    text: &str,
    position: [f64; 3],
    color: &str,
    horizontal: Horizontal,
) -> Result<(), Failure> {
    session.draw_text(
        text,
        &source_label_request(text, (position, named_color(color)?, horizontal)),
    )
}
pub(super) fn source_label_request(
    text: &str,
    (position, color, horizontal): ([f64; 3], Color, Horizontal),
) -> TextLabelRequest<'_> {
    TextLabelRequest {
        text,
        position,
        color: Some(color),
        horizontal: Some(horizontal),
        vertical: Some(Vertical::Center),
        angle_degrees: Some(0.),
        zoomable: Some(false),
        height: Some(14.),
        aspect: Some(FontFontAspect::Regular),
        font: Some("Arial"),
        ..TextLabelRequest::new(text)
    }
}

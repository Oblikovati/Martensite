//! An assembly of boxes and their text labels, and its sensitive entities.
use super::labels::DEJAVU_SANS;
use super::prelude::*;
use crate::case_view::viewer_session::{TextLabelRequest, register_font_file};
use martensite_tkernel::quantity::{Color, ColorRgba, NamedColor};
use martensite_tkservice::font::FontFontAspect;
use martensite_tkservice::graphic3d::{
    Graphic3dHorizontalTextAlignment as H, Graphic3dNameOfMaterial,
    Graphic3dVerticalTextAlignment as V,
};
use martensite_tkv3d::v3d::V3dTypeOfOrientation as Orientation;

/// A label aligned right and bottom at `position`, unrotated, not zoomable, 14 px high
/// in regular SansFont; the hex colour is read as sRGB.
fn label<'a>(text: &'a str, position: [f64; 3], hex: &str) -> TextLabelRequest<'a> {
    let color = ColorRgba::from_hex(hex, true).expect("a six-digit sRGB hex colour");
    TextLabelRequest {
        position,
        color: Some(*color.rgb()),
        horizontal: Some(H::Right),
        vertical: Some(V::Bottom),
        angle_degrees: Some(0.),
        zoomable: Some(false),
        height: Some(14.),
        aspect: Some(FontFontAspect::Regular),
        font: Some("SansFont"),
        ..TextLabelRequest::new(text)
    }
}

/// `bug24837_2`: boxes and labels shaded, recoloured, connected into one assembly, and the
/// assembly's sensitive entities shown.
pub(crate) fn labelled_assembly() -> Outcome {
    banner(&[
        "============",
        "CR24837_2: Visualization - revise design and implementation of connected Interactive Objects",
        "============",
        "",
    ]);
    register_font_file(DEJAVU_SANS, "SansFont")?;
    let mut run = SessionRun::new()?;
    let s = run.session();
    s.clear()?;
    s.set_standard_projection(Orientation::ZUP_TOP)?;
    s.set_display_mode(0)?;
    let request = DisplayRequest::default();
    s.display("o1", &box_at([-10., -10., -5.], [1., 1., 10.])?, &request)?;
    s.display("o2", &box_at([10., 10., -5.], [1., 1., 10.])?, &request)?;
    s.display("b1", &box_at([0., 0., 0.], [1., 2., 3.])?, &request)?;
    s.display("b2", &box_at([3., 0., 0.], [3., 2., 1.])?, &request)?;
    s.fit()?;
    s.draw_text("tb1", &label("b1", [0., -3., 3.], "FF1919"))?;
    s.draw_text("tb2", &label("b2", [3., 0., 3.], "19FF19"))?;
    s.draw_text("bc_1", &label("bc_1", [0., -9., 3.], "FFE1E1"))?;
    s.draw_text("bc_2", &label("bc_2", [3., -6., 3.], "FFE1E1"))?;
    s.set_display_mode(1)?;
    run.dump("bug24837_2_2_1.png")?;
    let s = run.session();
    s.set_material("b1", Graphic3dNameOfMaterial::Plastified)?;
    s.set_material("b2", Graphic3dNameOfMaterial::Plastified)?;
    for (name, color) in [
        ("b1", NamedColor::RED),
        ("tb1", NamedColor::RED),
        ("b2", NamedColor::GREEN),
        ("tb2", NamedColor::GREEN),
    ] {
        s.set_color(name, Color::named(color))?;
    }
    run.dump("bug24837_2_2_2.png")?;
    let s = run.session();
    s.set_location("b1", [0., -3., 0.])?;
    let parts = ["b1", "b2", "tb1", "tb2"].map(ConnectedPart::object);
    s.connect_assembly("bc", [0., -6., 0.], &parts)?;
    run.dump("bug24837_2_2_3.png")?;
    let s = run.session();
    s.update_view(|view| s.context().display_active_sensitive(view))?;
    run.dump("bug24837_2_sensitive.png")?;
    run.finish()
}

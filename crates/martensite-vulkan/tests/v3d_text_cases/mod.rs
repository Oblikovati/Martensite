//! Full original sequences keep the text label, image writer and texture reader connected.
use crate::{
    case_view::viewer_session::{DisplayRequest, TextLabelRequest, TextureImage},
    session_run::SessionRun,
};
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkservice::{
    font::FontFontAspect,
    graphic3d::{
        Graphic3dHorizontalTextAlignment as HAlign, Graphic3dVerticalTextAlignment as VAlign,
    },
};
type Outcome = Result<(), Box<dyn std::error::Error>>;

fn register_font() -> Outcome {
    crate::case_view::viewer_session::register_font_file(
        "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
        "SansFont",
    )?;
    Ok(())
}

fn label(
    text: &str,
    position: [f64; 3],
    color: NamedColor,
    zoomable: bool,
) -> TextLabelRequest<'_> {
    TextLabelRequest {
        position,
        color: Some(Color::named(color)),
        horizontal: Some(HAlign::Left),
        vertical: Some(VAlign::Bottom),
        angle_degrees: Some(0.),
        zoomable: Some(zoomable),
        height: Some(50.),
        font: Some("SansFont"),
        ..TextLabelRequest::new(text)
    }
}

/// Preserve the red label followed by an independently styled yellow trihedron; `isolated_text_aspect()?`.
#[allow(dead_code)] // Authored demo CPU policy is unset; register only after the SDK inherit gate.
pub(crate) fn isolated_text_aspect() -> Outcome {
    crate::session_run::banner(&[
        "============",
        "0023745: Test Commands, ViewerText - vdrawtext command should not modify global text aspect",
        "============",
        "",
    ]);
    register_font()?;
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    run.session().set_axonometric()?;
    run.session().draw_text(
        "text0",
        &label("ANOTHERBUG", [100.; 3], NamedColor::RED, true),
    )?;
    run.session().display_trihedron("trihedron")?;
    run.session().check_color(239, 216, [1., 1., 0.])?;
    run.dump("bug23745.png")?;
    run.finish()
}

fn unicode_filename() -> String {
    // The file name, decoded from its literal little-endian UTF-16 bytes.
    String::from_utf16(&[0x043a, 0x0430, 0x0440, 0x002e, 0x0070, 0x006e, 0x0067]).unwrap()
}

/// Save, reload as a texture, then delete the original Unicode-named image; `unicode_texture_path()?`.
pub(crate) fn unicode_texture_path() -> Outcome {
    crate::session_run::banner(&[
        "========",
        "OCC25369: Visualization, Image_AlienPixMap - handle UTF-8 names in image read/save operations on Windows",
        "========",
        "",
    ]);
    register_font()?;
    let shape = crate::primitive_shapes::box_sized([1., 2., 3.])?;
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    run.session().set_display_mode(0)?;
    run.session()
        .display("b", &shape, &DisplayRequest::default())?;
    run.session().fit()?;
    unicode_image(&mut run)?;
    run.dump("bug25369.png")?;
    run.finish()
}

fn unicode_image(run: &mut SessionRun) -> Outcome {
    let filename = unicode_filename();
    let mut options = label(&filename, [0.; 3], NamedColor::WHITE, false);
    options.aspect = Some(FontFontAspect::Regular);
    run.session().draw_text("text0", &options)?;
    run.dump("кар.png")?;
    let path = crate::source_output::capture_path(&filename)?;
    run.session().set_texture("b", TextureImage::File(&path))?;
    std::fs::remove_file(path)?;
    Ok(())
}

#[test]
fn utf16_filename_matches_case_conversion() {
    assert_eq!(unicode_filename(), "кар.png");
}

#[test]
fn text_label_options_preserve_explicit_and_inherited_aspects() {
    let options = label("ANOTHERBUG", [100.; 3], NamedColor::RED, true);
    assert_eq!(options.position, [100.; 3]);
    assert_eq!(options.zoomable, Some(true));
    assert_eq!(options.height, Some(50.));
    assert!(options.aspect.is_none());
}

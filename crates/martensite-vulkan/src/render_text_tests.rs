use super::*;
use martensite_tkernel::tcollection::HAsciiString;

fn bold_style() -> Graphic3dAspects {
    let style = Graphic3dAspects::default();
    style.set_text_font(Handle::new(HAsciiString::new("DejaVu Sans")));
    style.set_text_font_aspect(FontFontAspect::Bold);
    style
}

#[test]
fn font_keys_name_font_aspect_resolution_height_and_hinting_like_native() {
    // the recorded render_text capture prints these two keys.
    let default = Graphic3dAspects::default();
    assert_eq!(
        RenderText::font_key(&default, 16, 72, FontHinting::OFF),
        "Courier:0:72:16:0"
    );
    assert_eq!(
        RenderText::font_key(&bold_style(), 20, 96, FontHinting::NORMAL),
        "DejaVu Sans:1:96:20:1"
    );
}

#[test]
fn a_new_size_releases_the_glyphs_and_the_same_size_keeps_them() {
    let label = RenderText::new(&Handle::new(Graphic3dText::new(16.)));
    label.upload.record_text(12, false, Default::default());
    label.set_font_size(16);
    assert_eq!(label.estimated_data_size(), 192);
    label.set_font_size(20);
    assert_eq!(label.estimated_data_size(), 0);
    assert_eq!(label.text().height(), 20.);
}

#[test]
fn default_text_is_empty_with_height_ten_and_draws_in_3d() {
    let label = RenderText::default();
    assert_eq!(label.text().height(), 10.);
    assert!(!label.is_2d());
    label.set_2d(true);
    assert_eq!(label.placement(), TextPlacement::Pixel);
}

#[test]
fn string_size_below_height_two_measures_nothing() {
    let style = Graphic3dAspects::default();
    assert_eq!(
        RenderText::string_size("Ag", &style, 1.9, 72, FontHinting::OFF),
        [0.; 3]
    );
}

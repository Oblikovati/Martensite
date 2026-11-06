//! Command-compatible font records, without opening or registering a font file.
use martensite_tkernel::{
    standard::{Failure, Handle},
    tcollection::AsciiString,
};
use martensite_tkservice::font::{FONT_FONT_ASPECT_NB, FontFontAspect, FontSystemFont};

pub(super) fn aliased_font_record(
    found: &FontSystemFont,
    alias: &str,
) -> Result<FontSystemFont, Failure> {
    let font = FontSystemFont::new(&AsciiString::new(alias))?;
    let aspects = [
        FontFontAspect::Regular,
        FontFontAspect::Bold,
        FontFontAspect::Italic,
        FontFontAspect::BoldItalic,
    ];
    debug_assert_eq!(aspects.len() as i32, FONT_FONT_ASPECT_NB);
    for aspect in aspects {
        font.set_font_path(aspect, &found.font_path(aspect), found.font_face_id(aspect))?;
    }
    Ok(font)
}

/// Keep detected properties unless the command aliases or explicitly overrides them.
/// Example: `command_font_record(&checked, None, Some(true))?`.
pub(super) fn command_font_record(
    checked: &Handle<FontSystemFont>,
    alias: Option<&str>,
    single_stroke: Option<bool>,
) -> Result<Handle<FontSystemFont>, Failure> {
    let found = checked.get().ok_or_else(|| {
        martensite_tkernel::standard::exceptions::program_error(
            "checked font is null; expected a readable font record",
        )
    })?;
    let font = match alias {
        Some(name) => Handle::new(aliased_font_record(found, name)?),
        None => checked.clone(),
    };
    if let Some(enabled) = single_stroke {
        font.set_single_stroke_font(enabled);
    }
    Ok(font)
}

#[cfg(test)]
#[path = "font_registration_tests.rs"]
mod tests;

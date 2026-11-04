use super::*;

fn checked_font_fixture() -> FontSystemFont {
    let font = FontSystemFont::new(&AsciiString::new("OLF Owned Fixture")).unwrap();
    font.set_single_stroke_font(true);
    font.set_font_path(
        FontFontAspect::Regular,
        &AsciiString::new("/owned/regular.ttf"),
        7,
    )
    .unwrap();
    font.set_font_path(
        FontFontAspect::Italic,
        &AsciiString::new("/owned/italic.ttc"),
        3,
    )
    .unwrap();
    font
}

#[test]
fn old_alias_preserves_paths_face_ids_empty_aspects_and_resets_detected_flag() {
    let found = checked_font_fixture();
    let alias = aliased_font_record(&found, "OwnedAlias").unwrap();
    assert_eq!(alias.font_name(), &AsciiString::new("OwnedAlias"));
    assert_eq!(alias.font_key(), &AsciiString::new("ownedalias"));
    assert_owned_paths(&alias);
    assert!(!alias.is_single_stroke_font());
    assert!(found.is_single_stroke_font());
}

fn assert_owned_paths(font: &FontSystemFont) {
    assert_eq!(
        font.font_path(FontFontAspect::Regular),
        AsciiString::new("/owned/regular.ttf")
    );
    assert_eq!(font.font_face_id(FontFontAspect::Regular), 7);
    assert_eq!(
        font.font_path(FontFontAspect::Italic),
        AsciiString::new("/owned/italic.ttc")
    );
    assert_eq!(font.font_face_id(FontFontAspect::Italic), 3);
    assert!(font.font_path(FontFontAspect::Bold).is_empty());
    assert_eq!(font.font_face_id(FontFontAspect::Bold), 0);
    assert!(font.font_path(FontFontAspect::BoldItalic).is_empty());
}

#[test]
fn unaliased_command_retains_detected_family_and_single_stroke() {
    let checked = Handle::new(checked_font_fixture());
    let font = command_font_record(&checked, None, None).unwrap();
    assert_eq!(font.font_name(), &AsciiString::new("OLF Owned Fixture"));
    assert!(font.is_single_stroke_font());
    assert_owned_paths(font.get().unwrap());
}

#[test]
fn explicit_single_stroke_is_applied_after_alias_creation() {
    let checked = Handle::new(checked_font_fixture());
    let font = command_font_record(&checked, Some("machtgth"), Some(true)).unwrap();
    assert_eq!(font.font_name(), &AsciiString::new("machtgth"));
    assert!(font.is_single_stroke_font());
    assert_owned_paths(font.get().unwrap());
}

#[test]
fn explicit_false_overrides_detected_flag_without_an_alias() {
    let checked = Handle::new(checked_font_fixture());
    let font = command_font_record(&checked, None, Some(false)).unwrap();
    assert!(!font.is_single_stroke_font());
    assert!(!checked.is_single_stroke_font());
}

#[test]
fn default_alias_command_matches_the_pinned_old_record() {
    let checked = Handle::new(checked_font_fixture());
    let font = command_font_record(&checked, Some("OwnedAlias"), None).unwrap();
    assert_eq!(font.font_name(), &AsciiString::new("OwnedAlias"));
    assert!(!font.is_single_stroke_font());
    assert_owned_paths(font.get().unwrap());
    assert!(checked.is_single_stroke_font());
}

#[test]
fn null_checked_record_cannot_be_registered() {
    assert!(command_font_record(&Handle::default(), None, None).is_err());
}

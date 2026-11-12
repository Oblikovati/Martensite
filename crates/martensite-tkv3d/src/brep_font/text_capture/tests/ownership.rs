use super::*;
#[test]
fn each_public_capture_type_is_send_and_sync() {
    fn check<T: Send + Sync>() {}
    check::<FontTextAssetId>();
    check::<FontTextOccurrenceId>();
    check::<OwnedFontTextAsset>();
    check::<FontTextSource>();
    check::<FontTextCaptureLimits>();
    check::<FontTextCaptureOptions>();
    check::<FontTextCaptureRequest>();
    check::<FontTextCaptureWork>();
    check::<FontTextOccurrenceKind>();
    check::<OwnedFontTextGlyph>();
    check::<OwnedFontTextOccurrence>();
    check::<OwnedFontTextCapture>();
    check::<FontTextCaptureFailure>();
    check::<FontTextCaptureError>();
    check::<FontTextCaptureDiagnostics>();
}
#[test]
fn exact_content_binding_rejects_same_id_with_changed_bytes_or_face() {
    let original = asset();
    let mut changed = original.bytes().to_vec();
    changed[0] ^= 1;
    let changed = OwnedFontTextAsset::new(original.id(), changed.into(), original.face());
    let face = OwnedFontTextAsset::new(original.id(), original.bytes.clone(), 1);
    assert!(!original.matches_binding(&changed));
    assert!(!original.matches_binding(&face));
    assert!(original.matches_binding(&original.clone()));
}
#[test]
fn source_mutation_and_font_drop_do_not_change_published_geometry() {
    let mut input = request("B");
    let original = asset();
    let result = capture_owned_font_text(&original, input.clone(), || false).unwrap();
    let glyph = result.occurrences()[0].glyph().unwrap();
    let before = glyph.outline().clone();
    assert_eq!(glyph.asset(), &original);
    assert_eq!(glyph.options(), input.options);
    input.sources[0].id = FontTextOccurrenceId(999);
    input.text = Arc::from("Z");
    drop(original);
    assert_eq!(result.occurrences()[0].glyph().unwrap().outline(), &before);
    assert_eq!(result.request().text.as_ref(), "B");
    assert_eq!(
        result.occurrences()[0].source().id,
        FontTextOccurrenceId(1000)
    );
    assert!(!crate::brep_font::BRepFontBuilder::build_text(result.plan()).is_null());
}

use super::*;
use martensite_tkservice::graphic3d::{
    Graphic3dHorizontalTextAlignment as H, Graphic3dVerticalTextAlignment as V,
};
use std::sync::Arc;
mod failures;
mod ownership;
mod stress;

fn limits() -> FontTextCaptureLimits {
    FontTextCaptureLimits {
        font_bytes: 1_000_000,
        text_bytes: 4096,
        sources: 4096,
        glyphs: 128,
        segments: 32768,
        contours: 4096,
        points: 131072,
        processed_entities: 131072,
        checkpoints: 262144,
    }
}
fn request(text: &str) -> FontTextCaptureRequest {
    let sources = text
        .char_indices()
        .enumerate()
        .map(|(ordinal, (start, scalar))| FontTextSource {
            id: FontTextOccurrenceId(1000 + ordinal as u64),
            utf8_span: start..start + scalar.len_utf8(),
        })
        .collect();
    FontTextCaptureRequest {
        text: Arc::from(text),
        sources,
        options: options(),
    }
}
fn options() -> FontTextCaptureOptions {
    FontTextCaptureOptions {
        size: 10.0,
        tolerance: 1e-7,
        outline_tolerance: 1e-7,
        width_scaling: 1.0,
        horizontal: H::Left,
        vertical: V::Bottom,
        limits: limits(),
    }
}

fn asset() -> OwnedFontTextAsset {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../external/martensite-test-data/fonts/dejavu-2.37/DejaVuSans.ttf");
    let bytes = std::fs::read(path).expect("required pinned DejaVu 2.37 fixture");
    assert_eq!(bytes.len(), 759720);
    OwnedFontTextAsset::new(FontTextAssetId(425), bytes.into(), 0)
}
fn capture(text: &str) -> OwnedFontTextCapture {
    capture_owned_font_text(&asset(), request(text), || false)
        .expect("pinned font capture must succeed")
}
#[test]
fn screenshot_text_preserves_all_authored_occurrences() {
    let result = capture("This thing\nworks!");
    assert_eq!(result.occurrences().len(), 17);
    assert_eq!(result.plan().items().len(), 15);
    assert_eq!(
        result.occurrences()[4].kind(),
        FontTextOccurrenceKind::Space
    );
    assert_eq!(
        result.occurrences()[10].kind(),
        FontTextOccurrenceKind::LineBreak
    );
    let input = request("This thing\nworks!");
    for (expected, actual) in input.sources.iter().zip(result.occurrences()) {
        assert_eq!(expected, actual.source());
    }
    assert!(result.occurrences()[11].position().y() < result.occurrences()[0].position().y());
}
#[test]
fn frozen_plan_has_real_existing_builder_caller() {
    let result = capture("B8i");
    let shape = crate::brep_font::BRepFontBuilder::build_text(result.plan());
    assert!(!shape.is_null());
    let upper_b = result.occurrences()[0].glyph().unwrap();
    let lower_i = result.occurrences()[2].glyph().unwrap();
    assert!(upper_b.outline().contours().len() >= 3);
    assert!(upper_b.region().hole_loops().len() >= 2);
    assert!(lower_i.region().regions().len() >= 2);
}

#[test]
fn repeated_glyphs_share_prototypes_without_sharing_authored_ids() {
    let result = capture("hh");
    let (left, right) = (&result.occurrences[0], &result.occurrences[1]);
    assert_ne!(left.source.id, right.source.id);
    assert!(Arc::ptr_eq(
        left.glyph.as_ref().unwrap(),
        right.glyph.as_ref().unwrap()
    ));
    assert_eq!(result.work().glyphs, 1);
}
#[test]
fn analytic_quadratic_outlines_are_preserved_before_regularization() {
    let result = capture("S");
    let raw = result.occurrences()[0].glyph().unwrap().outline();
    assert!(raw.segments().iter().any(|segment| segment.kind()
        == martensite_tkservice::font::FontGlyphOutlineSegmentKind::QuadraticBezier));
    assert!(!raw.is_single_stroke());
    assert_eq!(result.recipe_version(), 1);
}
#[test]
fn valid_empty_glyph_is_distinct_from_missing_glyph() {
    let result = capture("\u{a0}");
    assert_eq!(
        result.occurrences()[0].kind(),
        FontTextOccurrenceKind::EmptyGlyph
    );
    assert!(result.occurrences()[0].glyph().unwrap().index() > 0);
    assert!(result.plan().items().is_empty());
}
#[test]
fn whitespace_and_empty_sources_complete_without_ink() {
    for text in ["", " ", " \n "] {
        let result = capture(text);
        assert!(result.plan().items().is_empty());
        assert_eq!(result.occurrences().len(), text.chars().count());
        assert_eq!(result.work().glyphs, 0);
    }
}

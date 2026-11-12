use super::*;
fn refusal(request: FontTextCaptureRequest) -> FontTextCaptureFailure {
    capture_owned_font_text(&asset(), request, || false)
        .unwrap_err()
        .failure
}
#[test]
fn missing_primary_glyph_never_publishes_notdef() {
    assert!(matches!(
        refusal(request("\u{10ffff}")),
        FontTextCaptureFailure::MissingGlyph {
            scalar: '\u{10ffff}',
            ..
        }
    ));
}
#[test]
fn malformed_scalar_sources_are_rejected_before_backend_calls() {
    let mut duplicate = request("éa");
    duplicate.sources[1].id = duplicate.sources[0].id;
    assert!(matches!(
        refusal(duplicate),
        FontTextCaptureFailure::Source { ordinal: 1, .. }
    ));
    let mut split = request("é");
    split.sources[0].utf8_span = 0..1;
    assert!(matches!(
        refusal(split),
        FontTextCaptureFailure::Source { ordinal: 0, .. }
    ));
    let mut gap = request("ab");
    gap.sources.pop();
    assert!(matches!(
        refusal(gap),
        FontTextCaptureFailure::Source { ordinal: 1, .. }
    ));
}
#[test]
fn unsupported_controls_and_invalid_settings_are_explicit() {
    for text in ["a\0b", "a\tb", "a\rb"] {
        assert!(matches!(
            refusal(request(text)),
            FontTextCaptureFailure::UnsupportedScalar { .. }
        ));
    }
    let mut invalid = request("A");
    invalid.options.outline_tolerance = f64::NAN;
    assert!(matches!(
        refusal(invalid),
        FontTextCaptureFailure::InvalidInput {
            field: "outline_tolerance",
            ..
        }
    ));
}
#[test]
fn input_and_raw_storage_limits_refuse_without_partial_success() {
    for resource in [
        "font_bytes",
        "text_bytes",
        "sources",
        "glyphs",
        "segments",
        "contours",
        "points",
        "processed_entities",
    ] {
        let mut input = request("B");
        set_limit(&mut input.options.limits, resource);
        assert!(
            matches!(refusal(input), FontTextCaptureFailure::Limit { resource: actual, .. } if actual == resource)
        );
    }
}
fn set_limit(limits: &mut FontTextCaptureLimits, resource: &str) {
    match resource {
        "font_bytes" => limits.font_bytes = 0,
        "text_bytes" => limits.text_bytes = 0,
        "sources" => limits.sources = 0,
        "glyphs" => limits.glyphs = 0,
        "segments" => limits.segments = 0,
        "contours" => limits.contours = 0,
        "points" => limits.points = 0,
        "processed_entities" => limits.processed_entities = 0,
        _ => panic!("unknown fixture resource {resource}"),
    }
}
#[test]
fn cancellation_and_checkpoint_exhaustion_are_distinct() {
    let error = capture_owned_font_text(&asset(), request("B"), || true).unwrap_err();
    assert!(matches!(error.failure, FontTextCaptureFailure::Cancelled));
    assert_eq!(error.diagnostics.work.checkpoints, 0);
    let mut limited = request("B");
    limited.options.limits.checkpoints = 1;
    let error = capture_owned_font_text(&asset(), limited, || false).unwrap_err();
    assert!(matches!(
        error.failure,
        FontTextCaptureFailure::Limit {
            resource: "checkpoints",
            ..
        }
    ));
    assert_eq!(error.diagnostics.work.checkpoints, 1);
}
#[test]
fn malformed_font_rejection_preserves_exact_binding_context() {
    let bad = OwnedFontTextAsset::new(FontTextAssetId(1), Arc::from([1u8, 2, 3]), 0);
    let error = capture_owned_font_text(&bad, request("A"), || false).unwrap_err();
    assert!(matches!(
        error.failure,
        FontTextCaptureFailure::FaceRejected { face: 0, bytes: 3 }
    ));
}

#[test]
fn failed_atomic_call_still_checks_cancellation_before_returning_its_cause() {
    let mut cancelled = || true;
    let mut budget = super::super::budget::CaptureBudget::new(limits(), &mut cancelled);
    let failed: super::super::budget::CaptureResult<()> =
        Err(FontTextCaptureFailure::BackendUnavailable);
    assert!(matches!(
        budget.after_call(failed),
        Err(FontTextCaptureFailure::Cancelled)
    ));
    assert_eq!(budget.work.checkpoints, 0);
}

#[test]
fn repeated_controls_retain_exact_authored_utf8_source_context() {
    for (text, ordinal, id, span, scalar) in [
        ("é🙂\0\0", 2, 471, 6..7, '\0'),
        ("é🙂\n\0", 3, 83, 7..8, '\0'),
        ("é\t\t", 1, 925, 2..3, '\t'),
        ("é\n\r", 2, 61, 3..4, '\r'),
    ] {
        let mut input = request(text);
        let source = FontTextSource {
            id: FontTextOccurrenceId(id),
            utf8_span: span,
        };
        input.sources[ordinal] = source.clone();
        assert_control_source(refusal(input), source, scalar);
    }
}

fn assert_control_source(failure: FontTextCaptureFailure, authored: FontTextSource, scalar: char) {
    let FontTextCaptureFailure::UnsupportedScalar {
        source,
        scalar: actual,
        expected,
    } = failure
    else {
        panic!("expected bound unsupported scalar, observed {failure:?}");
    };
    assert_eq!(source, authored);
    assert_eq!(actual, scalar);
    assert!(expected.contains("no NUL"));
}

#[test]
fn unsupported_scalar_requires_a_valid_binding_before_control_policy() {
    let mut input = request("é\0");
    input.sources[1].utf8_span = 1..3;
    assert!(matches!(
        refusal(input),
        FontTextCaptureFailure::Source { ordinal: 1, .. }
    ));
}

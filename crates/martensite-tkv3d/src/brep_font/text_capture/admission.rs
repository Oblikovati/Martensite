use super::{
    budget::{CaptureBudget, CaptureResult},
    types::*,
};
use std::collections::BTreeSet;

pub(super) fn admit(
    asset: &OwnedFontTextAsset,
    request: &FontTextCaptureRequest,
    budget: &mut CaptureBudget<'_>,
) -> CaptureResult<()> {
    budget.checkpoint()?;
    budget.check("font_bytes", asset.bytes.len(), budget.limits.font_bytes)?;
    budget.check("text_bytes", request.text.len(), budget.limits.text_bytes)?;
    budget.check("sources", request.sources.len(), budget.limits.sources)?;
    valid_settings(asset, &request.options)?;
    if i32::try_from(request.sources.len()).is_err() {
        return Err(invalid(
            "sources",
            request.sources.len(),
            "formatter count fitting i32",
        ));
    }
    valid_text_length(request)?;
    valid_sources(request, budget)
}
fn invalid(
    field: &'static str,
    observed: impl ToString,
    expected: &'static str,
) -> FontTextCaptureFailure {
    FontTextCaptureFailure::InvalidInput {
        field,
        observed: observed.to_string(),
        expected,
    }
}
fn valid_settings(
    asset: &OwnedFontTextAsset,
    options: &FontTextCaptureOptions,
) -> CaptureResult<()> {
    if asset.bytes.is_empty() || std::ffi::c_long::try_from(asset.bytes.len()).is_err() {
        return Err(invalid(
            "font_bytes",
            asset.bytes.len(),
            "nonempty content fitting native c_long",
        ));
    }
    if asset.face < 0 {
        return Err(invalid(
            "face",
            asset.face,
            "nonnegative collection face selector",
        ));
    }
    valid_metrics(options)
}
fn valid_metrics(options: &FontTextCaptureOptions) -> CaptureResult<()> {
    let values = [
        ("size", options.size),
        ("tolerance", options.tolerance),
        ("outline_tolerance", options.outline_tolerance),
        ("width_scaling", f64::from(options.width_scaling)),
    ];
    for (name, value) in values {
        if !super::super::regularizer::is_valid_real(value) || value <= 0.0 {
            return Err(invalid(
                name,
                value,
                "finite positive value below the kernel infinity threshold",
            ));
        }
    }
    Ok(())
}

fn source_error(
    request: &FontTextCaptureRequest,
    ordinal: usize,
    expected: &'static str,
) -> FontTextCaptureFailure {
    FontTextCaptureFailure::Source {
        ordinal,
        observed: request.sources.get(ordinal).cloned(),
        expected,
    }
}
fn valid_sources(
    request: &FontTextCaptureRequest,
    budget: &mut CaptureBudget<'_>,
) -> CaptureResult<()> {
    let mut ids = BTreeSet::new();
    let mut count = 0;
    for (ordinal, (start, scalar)) in request.text.char_indices().enumerate() {
        budget.checkpoint()?;
        let source = valid_source(request, ordinal, start..start + scalar.len_utf8(), &mut ids)?;
        valid_scalar(source, scalar)?;
        count += 1;
    }
    valid_source_count(request, count)
}

fn valid_source<'a>(
    request: &'a FontTextCaptureRequest,
    ordinal: usize,
    span: std::ops::Range<usize>,
    ids: &mut BTreeSet<FontTextOccurrenceId>,
) -> CaptureResult<&'a FontTextSource> {
    let source = request
        .sources
        .get(ordinal)
        .ok_or_else(|| source_error(request, ordinal, "one source per UTF-8 scalar"))?;
    if source.utf8_span != span || !ids.insert(source.id) {
        return Err(source_error(
            request,
            ordinal,
            "unique caller ID and exact scalar byte span in complete source order",
        ));
    }
    Ok(source)
}

fn valid_scalar(source: &FontTextSource, scalar: char) -> CaptureResult<()> {
    if scalar != '\n' && scalar.is_control() {
        return Err(FontTextCaptureFailure::UnsupportedScalar {
            source: source.clone(),
            scalar,
            expected: "printable scalar, space or LF; no NUL or other controls",
        });
    }
    Ok(())
}

fn valid_source_count(request: &FontTextCaptureRequest, count: usize) -> CaptureResult<()> {
    if count != request.sources.len() {
        return Err(source_error(
            request,
            count,
            "no extra scalar source records",
        ));
    }
    Ok(())
}

fn valid_text_length(request: &FontTextCaptureRequest) -> CaptureResult<()> {
    if i32::try_from(request.text.len()).is_err() {
        return Err(invalid(
            "text_bytes",
            request.text.len(),
            "native UTF-8 string length fitting i32",
        ));
    }
    Ok(())
}

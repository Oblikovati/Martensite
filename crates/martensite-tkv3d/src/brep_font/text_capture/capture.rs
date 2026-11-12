use super::super::{BRepFontBuilder, BRepFontBuilderTextPlan, BRepFontBuilderTextPlanItem};
use super::{
    admission::admit,
    backend::{format_text, open_font},
    budget::{CaptureBudget, CaptureResult, native_failure},
    glyphs::GlyphPool,
    types::*,
};
use martensite_tkmath::gp::Xy;
use martensite_tkservice::font::{FontFTFont, FontRect, FontTextFormatter};

/// Capture strict primary-face text with caller-owned scalar identities and exact font content.
/// Backend calls are atomic; cancellation and storage limits do not bound their temporary cost.
/// Example: `capture_owned_font_text(&asset, request, || false)`.
pub fn capture_owned_font_text(
    asset: &OwnedFontTextAsset,
    request: FontTextCaptureRequest,
    mut cancelled: impl FnMut() -> bool,
) -> Result<OwnedFontTextCapture, FontTextCaptureError> {
    let mut budget = CaptureBudget::new(request.options.limits, &mut cancelled);
    let result = capture(asset, &request, &mut budget);
    match result {
        Ok((occurrences, plan)) => Ok(publish(asset, request, occurrences, plan, budget.work)),
        Err(failure) => Err(refusal(asset, &request, failure, &budget)),
    }
}
fn publish(
    asset: &OwnedFontTextAsset,
    request: FontTextCaptureRequest,
    occurrences: Vec<OwnedFontTextOccurrence>,
    plan: BRepFontBuilderTextPlan,
    work: FontTextCaptureWork,
) -> OwnedFontTextCapture {
    OwnedFontTextCapture {
        asset: asset.clone(),
        request,
        occurrences,
        plan,
        work,
    }
}

fn capture(
    asset: &OwnedFontTextAsset,
    request: &FontTextCaptureRequest,
    budget: &mut CaptureBudget<'_>,
) -> CaptureResult<(Vec<OwnedFontTextOccurrence>, BRepFontBuilderTextPlan)> {
    admit(asset, request, budget)?;
    let font = open_font(asset, &request.options, budget)?;
    let formatter = format_text(&font, request, budget)?;
    let occurrences = occurrences(asset, &font, &formatter, request, budget)?;
    let plan = make_plan(&formatter, request, &occurrences)?;
    budget.checkpoint()?;
    Ok((occurrences, plan))
}
fn occurrences(
    asset: &OwnedFontTextAsset,
    font: &FontFTFont,
    formatter: &FontTextFormatter,
    request: &FontTextCaptureRequest,
    budget: &mut CaptureBudget<'_>,
) -> CaptureResult<Vec<OwnedFontTextOccurrence>> {
    let context = super::glyphs::GlyphCapture::new(asset, font, &request.options);
    let mut pool = GlyphPool::new();
    let source_scalars = request.sources.iter().zip(request.text.chars()).enumerate();
    source_scalars
        .map(|(ordinal, (source, scalar))| {
            budget.checkpoint()?;
            let position = position(formatter, ordinal, request.options.size)?;
            let glyph = context.optional_glyph(source, scalar, &mut pool, budget)?;
            Ok(publish_occurrence(source, scalar, position, glyph))
        })
        .collect()
}

fn publish_occurrence(
    source: &FontTextSource,
    scalar: char,
    position: Xy,
    glyph: Option<std::sync::Arc<OwnedFontTextGlyph>>,
) -> OwnedFontTextOccurrence {
    let kind = occurrence_kind(scalar, glyph.as_deref());
    OwnedFontTextOccurrence {
        source: source.clone(),
        scalar,
        position,
        kind,
        glyph,
    }
}

fn position(formatter: &FontTextFormatter, ordinal: usize, size: f64) -> CaptureResult<Xy> {
    let ordinal = i32::try_from(ordinal).map_err(|_| FontTextCaptureFailure::InvalidInput {
        field: "sources",
        observed: ordinal.to_string(),
        expected: "formatter index fitting i32",
    })?;
    let corner = formatter
        .bottom_left(ordinal)
        .map_err(|cause| native_failure("layout scalar position", cause))?;
    Ok(Xy::new(
        f64::from(corner.x()) * size / 4800.0,
        f64::from(corner.y()) * size / 4800.0,
    ))
}
fn occurrence_kind(scalar: char, glyph: Option<&OwnedFontTextGlyph>) -> FontTextOccurrenceKind {
    match scalar {
        ' ' => FontTextOccurrenceKind::Space,
        '\n' => FontTextOccurrenceKind::LineBreak,
        _ if glyph.is_some_and(|glyph| glyph.outline.is_empty()) => {
            FontTextOccurrenceKind::EmptyGlyph
        }
        _ => FontTextOccurrenceKind::Outlined,
    }
}
fn make_plan(
    formatter: &FontTextFormatter,
    request: &FontTextCaptureRequest,
    occurrences: &[OwnedFontTextOccurrence],
) -> CaptureResult<BRepFontBuilderTextPlan> {
    let (regions, items) = plan_pools(occurrences);
    let (lower, upper) = bounds(formatter, request.options.size);
    BRepFontBuilder::create_text_plan(
        request.options.size,
        request.options.tolerance,
        lower,
        upper,
        regions,
        items,
    )
    .ok_or(FontTextCaptureFailure::PlanRejected)
}
type PlanPools = (
    Vec<std::sync::Arc<super::super::BRepFontPlanarRegion>>,
    Vec<BRepFontBuilderTextPlanItem>,
);
fn plan_pools(occurrences: &[OwnedFontTextOccurrence]) -> PlanPools {
    let mut regions = Vec::new();
    let mut items = Vec::new();
    let mut indices = std::collections::BTreeMap::new();
    for occurrence in occurrences
        .iter()
        .filter(|item| item.kind == FontTextOccurrenceKind::Outlined)
    {
        if let Some(glyph) = &occurrence.glyph {
            let index = *indices.entry(glyph.index).or_insert_with(|| {
                regions.push(glyph.region.clone());
                regions.len() - 1
            });
            items.push(BRepFontBuilderTextPlanItem::new(index, occurrence.position));
        }
    }
    (regions, items)
}

fn bounds(formatter: &FontTextFormatter, size: f64) -> (Xy, Xy) {
    let mut rect = FontRect::default();
    formatter.bnd_box(&mut rect);
    let scale = size / 4800.0;
    let (left, right) = (f64::from(rect.left) * scale, f64::from(rect.right) * scale);
    let (bottom, top) = (f64::from(rect.bottom) * scale, f64::from(rect.top) * scale);
    (
        Xy::new(left.min(right), bottom.min(top)),
        Xy::new(left.max(right), bottom.max(top)),
    )
}

fn refusal(
    asset: &OwnedFontTextAsset,
    request: &FontTextCaptureRequest,
    failure: FontTextCaptureFailure,
    budget: &CaptureBudget<'_>,
) -> FontTextCaptureError {
    let diagnostics = FontTextCaptureDiagnostics {
        asset_id: asset.id,
        face: asset.face,
        font_bytes: asset.bytes.len(),
        text_bytes: request.text.len(),
        sources: request.sources.len(),
        limits: budget.limits,
        work: budget.work,
    };
    FontTextCaptureError {
        failure,
        diagnostics: Box::new(diagnostics),
    }
}

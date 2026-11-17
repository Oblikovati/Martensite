use super::super::{BRepFontPlanarRegion, BRepFontRegularizer, BRepFontRegularizerOptions};
use super::{
    budget::{CaptureBudget, CaptureResult},
    types::*,
};
use martensite_tkservice::font::{
    FontFTFont, FontGlyphOutline, FontGlyphOutlineGlyph, FontGlyphOutlineSegmentKind,
};
use std::{collections::BTreeMap, sync::Arc};
pub(super) type GlyphPool = BTreeMap<u32, Arc<OwnedFontTextGlyph>>;
pub(super) struct GlyphCapture<'a> {
    pub asset: &'a OwnedFontTextAsset,
    pub font: &'a FontFTFont,
    pub options: &'a FontTextCaptureOptions,
}
impl<'a> GlyphCapture<'a> {
    pub fn new(
        asset: &'a OwnedFontTextAsset,
        font: &'a FontFTFont,
        options: &'a FontTextCaptureOptions,
    ) -> Self {
        Self {
            asset,
            font,
            options,
        }
    }
    pub fn optional_glyph(
        &self,
        source: &FontTextSource,
        scalar: char,
        pool: &mut GlyphPool,
        budget: &mut CaptureBudget<'_>,
    ) -> CaptureResult<Option<Arc<OwnedFontTextGlyph>>> {
        if scalar == ' ' || scalar == '\n' {
            return Ok(None);
        }
        let glyph = select_glyph(self.font, source, scalar, budget)?;
        if let Some(cached) = pool.get(&glyph.index()) {
            return Ok(Some(cached.clone()));
        }
        let captured = Arc::new(self.capture_glyph(source, &glyph, budget)?);
        pool.insert(glyph.index(), captured.clone());
        Ok(Some(captured))
    }
    fn capture_glyph(
        &self,
        source: &FontTextSource,
        glyph: &FontGlyphOutlineGlyph,
        budget: &mut CaptureBudget<'_>,
    ) -> CaptureResult<OwnedFontTextGlyph> {
        budget.check_glyph_count()?;
        let outline = load_outline(self.font, source, glyph, budget)?;
        let region = regularize(
            source,
            glyph.index(),
            &outline,
            self.options.outline_tolerance,
            budget,
        )?;
        Ok(self.publish(glyph.index(), outline, region))
    }

    fn publish(
        &self,
        index: u32,
        outline: FontGlyphOutline,
        region: BRepFontPlanarRegion,
    ) -> OwnedFontTextGlyph {
        OwnedFontTextGlyph {
            asset: self.asset.clone(),
            options: *self.options,
            index,
            outline,
            region: Arc::new(region),
        }
    }
}
fn select_glyph(
    font: &FontFTFont,
    source: &FontTextSource,
    scalar: char,
    budget: &mut CaptureBudget<'_>,
) -> CaptureResult<FontGlyphOutlineGlyph> {
    budget.checkpoint()?;
    let outcome = font
        .resolve_glyph(u32::from(scalar))
        .filter(|glyph| glyph.index() != 0 && glyph.face() == 0)
        .ok_or(FontTextCaptureFailure::MissingGlyph {
            source: source.id,
            scalar,
        });
    budget.after_call(outcome)
}
fn load_outline(
    font: &FontFTFont,
    source: &FontTextSource,
    glyph: &FontGlyphOutlineGlyph,
    budget: &mut CaptureBudget<'_>,
) -> CaptureResult<FontGlyphOutline> {
    budget.checkpoint()?;
    let outcome = font
        .load_glyph_outline(glyph)
        .ok_or(FontTextCaptureFailure::OutlineRejected {
            source: source.id,
            glyph: glyph.index(),
        });
    let outline = budget.after_call(outcome)?;
    charge_outline(&outline, budget)?;
    Ok(outline)
}
fn charge_outline(outline: &FontGlyphOutline, budget: &mut CaptureBudget<'_>) -> CaptureResult<()> {
    budget.check(
        "segments",
        budget
            .work
            .segments
            .saturating_add(outline.segments().len()),
        budget.limits.segments,
    )?;
    budget.check(
        "contours",
        budget
            .work
            .contours
            .saturating_add(outline.contours().len()),
        budget.limits.contours,
    )?;
    let points = outline_points(outline, budget)?;
    budget.charge_raw(outline.segments().len(), outline.contours().len(), points)
}
fn outline_points(
    outline: &FontGlyphOutline,
    budget: &mut CaptureBudget<'_>,
) -> CaptureResult<usize> {
    let mut points = 0usize;
    for segment in outline.segments() {
        budget.checkpoint()?;
        points = points.saturating_add(segment_points(segment.kind()));
        budget.check(
            "points",
            budget.work.points.saturating_add(points),
            budget.limits.points,
        )?;
    }
    Ok(points)
}
fn segment_points(kind: FontGlyphOutlineSegmentKind) -> usize {
    match kind {
        FontGlyphOutlineSegmentKind::Line => 2,
        FontGlyphOutlineSegmentKind::QuadraticBezier => 3,
        FontGlyphOutlineSegmentKind::CubicBezier => 4,
    }
}

fn regularize(
    source: &FontTextSource,
    index: u32,
    outline: &FontGlyphOutline,
    tolerance: f64,
    budget: &mut CaptureBudget<'_>,
) -> CaptureResult<BRepFontPlanarRegion> {
    budget.checkpoint()?;
    let mut result =
        BRepFontRegularizer.build_with(outline, &BRepFontRegularizerOptions { tolerance });
    budget.checkpoint()?;
    let failure = FontTextCaptureFailure::Regularizer {
        source: source.id,
        glyph: index,
        status: result.status_value(),
    };
    let region = result.change_planar_region().take().ok_or(failure)?;
    budget.charge_processed(processed_count(&region))?;
    Ok(region)
}

fn processed_count(region: &BRepFontPlanarRegion) -> usize {
    [
        region.vertices().len(),
        region.curves().len(),
        region.uses().len(),
        region.loops().len(),
        region.hole_loops().len(),
        region.regions().len(),
    ]
    .into_iter()
    .fold(0usize, usize::saturating_add)
}

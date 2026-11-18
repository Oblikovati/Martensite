//! Recheck complete original selection provenance against every frozen member receipt.
use super::*;
pub(super) fn certify(
    originals: &[DevelopedRegionRequest],
    selection: &Selection,
    stored: &BTreeMap<SurfaceRegionId, Stored>,
) -> ConeChecked<()> {
    sector(originals, selection)?;
    let mut clearance = Vec::new();
    for prepared in &selection.prepared {
        let original = original(originals, prepared.source.region)?;
        let member = stored
            .get(&original.region)
            .ok_or_else(|| missing(original.region))?;
        bind_member(original, prepared, member, &mut clearance)?;
    }
    if !selection.clearance.matches_sources(&clearance) {
        return Err(family_mismatch("clearance original source binding"));
    }
    Ok(())
}
fn sector(originals: &[DevelopedRegionRequest], selection: &Selection) -> ConeChecked<()> {
    let spans: Vec<_> = originals
        .iter()
        .flat_map(|r| r.loops.iter().flatten())
        .map(|piece| piece.poles.as_slice())
        .collect();
    let angle = selection
        .prepared
        .first()
        .map_or(0., |prepared| prepared.semi_angle);
    let query = ConicalSectorQuery {
        spans: &spans,
        semi_angle: angle,
        options: selection.sector_options,
    };
    if !selection.sector.matches(&query) {
        return Err(family_mismatch("sector original source binding"));
    }
    Ok(())
}
fn original(
    originals: &[DevelopedRegionRequest],
    region: SurfaceRegionId,
) -> ConeChecked<&DevelopedRegionRequest> {
    originals
        .iter()
        .find(|original| original.region == region)
        .ok_or_else(|| missing(region))
}
fn missing(region: SurfaceRegionId) -> ConeFailure {
    failure(region, None, ConeCause::Missing(region))
}
fn family_mismatch(field: &'static str) -> ConeFailure {
    ConeFailure::new(
        None,
        None,
        WrappedFailureClass::Incomplete,
        ConeCause::Binding {
            field,
            actual: vec![0],
            expected: vec![1],
        },
    )
}
fn bind_member<'a>(
    original: &'a DevelopedRegionRequest,
    prepared: &Prepared,
    stored: &Stored,
    clearance: &mut Vec<CylindricalClearanceSource<'a>>,
) -> ConeChecked<()> {
    let pieces: Vec<_> = prepared.source.loops.iter().flatten().collect();
    if pieces.len() != stored.spans.len() {
        return Err(mismatch(
            original.region,
            None,
            "stored span count",
            vec![stored.spans.len() as u64],
            vec![pieces.len() as u64],
        ));
    }
    for (piece, proof) in pieces.into_iter().zip(&stored.spans) {
        bind_span(original, prepared, piece, proof)?;
        clearance.push(clearance_source(original, piece.span)?);
    }
    Ok(())
}
fn bind_span(
    original: &DevelopedRegionRequest,
    prepared: &Prepared,
    piece: &AuthoredPiece,
    proof: &SpanProof,
) -> ConeChecked<()> {
    let query = boundary::query(original, prepared, piece, &proof.curves)?;
    if proof.span != piece.span || !proof.boundary.matches(&query) {
        return Err(mismatch(
            original.region,
            Some(piece.span),
            "boundary original source binding",
            vec![0],
            vec![1],
        ));
    }
    Ok(())
}
fn clearance_source(
    original: &DevelopedRegionRequest,
    span: CurveSpan,
) -> ConeChecked<CylindricalClearanceSource<'_>> {
    let source = source_piece(original, span)?;
    let interval = source_interval(original.region, span)?;
    Ok(CylindricalClearanceSource {
        authored: &source.poles,
        interval,
    })
}

fn source_piece(original: &DevelopedRegionRequest, span: CurveSpan) -> ConeChecked<&AuthoredPiece> {
    original
        .loops
        .iter()
        .flatten()
        .find(|source| source.span.curve == span.curve)
        .ok_or_else(|| {
            mismatch(
                original.region,
                Some(span),
                "original source owner",
                vec![0],
                vec![1],
            )
        })
}
fn source_interval(
    region: SurfaceRegionId,
    span: CurveSpan,
) -> ConeChecked<CylindricalProfileInterval> {
    CylindricalProfileInterval::new(u64::from(span.index), span.level).map_err(|_| {
        mismatch(
            region,
            Some(span),
            "original dyadic interval",
            vec![u64::from(span.index), u64::from(span.level)],
            vec![0, 0],
        )
    })
}

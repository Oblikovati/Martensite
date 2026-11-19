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
    let spans = domain::original_spans(originals);
    let prepared = selection
        .prepared
        .first()
        .ok_or_else(|| family_mismatch("domain member owner"))?;
    let matches = domain_matches(&selection.domain, prepared, &spans);
    let owners_match = selection
        .prepared
        .iter()
        .all(|prepared| authority_matches(&selection.domain, prepared, &spans));
    if !matches || !owners_match {
        return Err(family_mismatch("domain original source binding"));
    }
    Ok(())
}

fn domain_matches(
    domain: &domain::DomainEvidence,
    prepared: &Prepared,
    spans: &[&[[f64; 2]]],
) -> bool {
    match domain {
        domain::DomainEvidence::Legacy { sector, options } => sector.matches(&ConicalSectorQuery {
            spans,
            semi_angle: prepared.semi_angle,
            options: *options,
        }),
        domain::DomainEvidence::Chart(proof) => chart_matches(proof, prepared, spans),
    }
}
fn chart_matches(
    proof: &ConicalChartDomainEvidence,
    prepared: &Prepared,
    spans: &[&[[f64; 2]]],
) -> bool {
    proof.matches(&ConicalChartDomainQuery {
        spans,
        semi_angle: prepared.semi_angle,
        chart_origin: prepared.chart_origin,
        fit_center: prepared.chart_center,
        developed_turn: 0,
        physical_turn: 0,
        options: proof.options(),
    })
}

fn authority_matches(
    selected: &domain::DomainEvidence,
    prepared: &Prepared,
    spans: &[&[[f64; 2]]],
) -> bool {
    match (selected, prepared.chart_domain.as_deref()) {
        (domain::DomainEvidence::Legacy { .. }, None) => true,
        (domain::DomainEvidence::Chart(selected), Some(actual)) => {
            selected.options() == actual.options()
                && chart_matches(selected, prepared, spans)
                && chart_matches(actual, prepared, spans)
        }
        _ => false,
    }
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
    if proof.span != piece.span || !domain::matches_boundary(prepared, &proof.boundary, &query) {
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

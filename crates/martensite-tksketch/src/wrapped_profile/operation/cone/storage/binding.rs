//! Recheck complete original selection provenance against every frozen member receipt.
use super::*;
pub(super) fn certify(
    originals: &[DevelopedRegionRequest],
    selection: &Selection,
    stored: &BTreeMap<SurfaceRegionId, Stored>,
) -> ConeChecked<()> {
    sector(originals, selection)?;
    let originals = operation::source::by_region(originals);
    let mut clearance = Vec::new();
    for prepared in &selection.prepared {
        let original = originals
            .get(&prepared.source.region)
            .copied()
            .ok_or_else(|| missing(prepared.source.region))?;
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
    if let domain::DomainEvidence::Atlas(charts) = &selection.domain {
        return atlas(originals, &selection.prepared, charts);
    }
    let spans = domain::original_spans(originals);
    let prepared = selection
        .prepared
        .first()
        .ok_or_else(|| family_mismatch("domain member owner"))?;
    let matches = domain_matches(&selection.domain, prepared, &spans);
    let owners_match = selection
        .prepared
        .iter()
        .all(|member| authority_matches(&selection.domain, prepared, member, &spans));
    if !matches || !owners_match {
        return Err(family_mismatch("domain original source binding"));
    }
    Ok(())
}

fn atlas(
    originals: &[DevelopedRegionRequest],
    prepared: &[Prepared],
    charts: &BTreeMap<SurfaceRegionId, std::sync::Arc<ConicalChartDomainEvidence>>,
) -> ConeChecked<()> {
    if originals.len() != prepared.len() || charts.len() != prepared.len() {
        return Err(family_mismatch("complete atlas region inventory"));
    }
    for (original, member) in originals.iter().zip(prepared) {
        let region = original.region;
        let spans = domain::original_spans(std::slice::from_ref(original));
        if member.source.region != region || !atlas_matches(charts.get(&region), member, &spans) {
            return Err(mismatch(
                region,
                None,
                "atlas member source binding",
                vec![0],
                vec![1],
            ));
        }
    }
    Ok(())
}
fn atlas_matches(
    selected: Option<&std::sync::Arc<ConicalChartDomainEvidence>>,
    member: &Prepared,
    spans: &[&[[f64; 2]]],
) -> bool {
    let (Some(selected), Some(actual)) = (selected, &member.chart_domain) else {
        return false;
    };
    selected.version() == ConicalChartDomainVersion::AtlasMemberV5
        && selected.version() == actual.version()
        && selected.options() == actual.options()
        && chart_matches(selected, member, spans)
        && (std::sync::Arc::ptr_eq(selected, actual) || chart_matches(actual, member, spans))
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
        domain::DomainEvidence::Chart(proof, _) => chart_matches(proof, prepared, spans),
        domain::DomainEvidence::Atlas(_) => false,
    }
}
fn chart_matches(
    proof: &ConicalChartDomainEvidence,
    prepared: &Prepared,
    spans: &[&[[f64; 2]]],
) -> bool {
    proof.matches_with_version(
        &ConicalChartDomainQuery {
            spans,
            semi_angle: prepared.semi_angle,
            chart_origin: prepared.chart_origin,
            fit_center: prepared.chart_center,
            developed_turn: 0,
            physical_turn: 0,
            options: proof.options(),
        },
        proof.version(),
    )
}

fn authority_matches(
    selected: &domain::DomainEvidence,
    first: &Prepared,
    prepared: &Prepared,
    spans: &[&[[f64; 2]]],
) -> bool {
    match (selected, &prepared.chart_domain) {
        (domain::DomainEvidence::Legacy { .. }, None) => true,
        (domain::DomainEvidence::Chart(selected, _), Some(actual)) => {
            selected.version() == actual.version()
                && selected.span_limit() == actual.span_limit()
                && selected.options() == actual.options()
                && chart_scalars(first) == chart_scalars(prepared)
                && (std::sync::Arc::ptr_eq(selected, actual)
                    || chart_matches(actual, prepared, spans))
        }
        _ => false,
    }
}

fn chart_scalars(prepared: &Prepared) -> [u64; 3] {
    [
        prepared.semi_angle,
        prepared.chart_origin,
        prepared.chart_center,
    ]
    .map(f64::to_bits)
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
    let count: usize = prepared.source.loops.iter().map(Vec::len).sum();
    if count != stored.spans.len() {
        return Err(mismatch(
            original.region,
            None,
            "stored span count",
            vec![stored.spans.len() as u64],
            vec![count as u64],
        ));
    }
    let common_chart = prepared.local_charts.is_empty();
    for (piece, proof) in prepared.source.loops.iter().flatten().zip(&stored.spans) {
        if common_chart {
            bind_span(original, prepared, piece, proof)?;
        }
        clearance.push(clearance_source(original, prepared, piece.span)?);
    }
    if !common_chart {
        // Cap join binding already rechecks each local chart and retained edge.
        bind_cap_joins(original, prepared, stored)?;
    }
    Ok(())
}
fn bind_span(
    original: &DevelopedRegionRequest,
    prepared: &Prepared,
    piece: &AuthoredPiece,
    proof: &SpanProof,
) -> ConeChecked<()> {
    let input = boundary::BoundaryInput::new(original, prepared, piece, "frozen fit membership")?;
    let query = input.query(&proof.curves);
    if proof.span != piece.span || !input.metric()?.matches(&proof.boundary, &query) {
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
fn clearance_source<'a>(
    original: &'a DevelopedRegionRequest,
    prepared: &Prepared,
    span: CurveSpan,
) -> ConeChecked<CylindricalClearanceSource<'a>> {
    let source = source_piece(original, prepared, span)?;
    let interval = source_interval(original.region, span)?;
    Ok(CylindricalClearanceSource {
        authored: &source.poles,
        interval,
    })
}

fn source_piece<'a>(
    original: &'a DevelopedRegionRequest,
    prepared: &Prepared,
    span: CurveSpan,
) -> ConeChecked<&'a AuthoredPiece> {
    prepared
        .authored_piece(original, span.curve)
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

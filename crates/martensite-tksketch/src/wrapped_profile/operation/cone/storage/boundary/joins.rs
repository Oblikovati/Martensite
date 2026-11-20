//! Complete closed contours retain their selected metric and exact source owners.
use super::*;
mod cap;
pub(in super::super::super) use cap::bind as bind_cap_joins;
#[cfg(test)]
pub(in super::super::super) use cap::cap_join_spans;
pub(in super::super::super) fn certify(
    original: &DevelopedRegionRequest,
    prepared: &Prepared,
    spans: &[SpanProof],
    carriers: [ConicalTrimCarrier; 2],
    work: &mut Work,
    cancelled: &mut CancelPoll<'_>,
) -> ConeChecked<ConicalJoinsEvidence> {
    if !prepared.local_charts.is_empty() {
        return cap::certify(original, prepared, spans, carriers, work, cancelled);
    }
    let mut queries = Vec::with_capacity(spans.len());
    for (piece, proof) in prepared.source.loops.iter().flatten().zip(spans) {
        queries.push(ConicalJoinSpan {
            query: query(original, prepared, piece, &proof.curves)?,
            boundary: &proof.boundary,
        });
    }
    let lengths: Vec<_> = prepared.source.loops.iter().map(Vec::len).collect();
    let query = ConicalJoinsQuery {
        spans: &queries,
        contour_lengths: &lengths,
        options: work.remaining(),
    };
    let proof = certify_query(original.region, prepared, spans, query, work, cancelled)?;
    work.charge(original.region, None, proof.operations())?;
    Ok(proof)
}
fn joins_failure(
    region: SurfaceRegionId,
    spans: &[SpanProof],
    cause: ConicalJoinsFailure,
) -> ConeFailure {
    let index = match &cause {
        ConicalJoinsFailure::Boundary { index } | ConicalJoinsFailure::Proof { index, .. } => {
            Some(*index)
        }
        _ => None,
    };
    let span = index
        .and_then(|index| spans.get(index))
        .map(|proof| proof.span);
    failure(region, span, ConeCause::Joins(Box::new(cause)))
}

fn certify_query(
    region: SurfaceRegionId,
    prepared: &Prepared,
    spans: &[SpanProof],
    query: ConicalJoinsQuery<'_>,
    work: &mut Work,
    cancelled: &mut CancelPoll<'_>,
) -> ConeChecked<ConicalJoinsEvidence> {
    let Some(domain) = prepared.chart_domain.as_deref() else {
        return certify_conical_joins(query, cancelled)
            .map_err(|cause| joins_failure(region, spans, cause));
    };
    // #552: the executed chart boundary must never re-enter legacy projection proof.
    let result = selected(query, domain, prepared, cancelled);
    result.map_err(|cause| {
        let span = cause
            .index
            .and_then(|index| spans.get(index))
            .map(|proof| proof.span);
        chart_failure(region, span, cause, work)
    })
}

fn chart_failure(
    region: SurfaceRegionId,
    span: Option<CurveSpan>,
    cause: ConicalChartJoinsError,
    work: &mut Work,
) -> ConeFailure {
    let _ = work.charge(region, span, cause.operations);
    failure(region, span, ConeCause::ChartJoins(Box::new(cause)))
}
#[cfg(test)]
mod tests;

fn selected(
    query: ConicalJoinsQuery<'_>,
    domain: &ConicalChartDomainEvidence,
    prepared: &Prepared,
    cancelled: &mut CancelPoll<'_>,
) -> Result<ConicalJoinsEvidence, ConicalChartJoinsError> {
    if domain::directed(prepared) {
        certify_conical_joins_in_chart_with_refinement(
            query,
            domain,
            ConicalPrimitiveRefinement::HalfSplitV1,
            cancelled,
        )
    } else {
        certify_conical_joins_in_chart(query, domain, cancelled)
    }
}

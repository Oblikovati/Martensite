//! Certify the decoded angle/slant joins using the authority for each fitted span.
use super::*;
pub(super) fn certify(
    original: &DevelopedRegionRequest,
    prepared: &Prepared,
    spans: &[SpanProof],
    work: &mut Work,
    cancelled: &mut CancelPoll<'_>,
) -> ConeChecked<ConicalJoinsEvidence> {
    let cap_spans = cap_join_spans(original, prepared, spans)?;
    let lengths: Vec<_> = prepared.source.loops.iter().map(Vec::len).collect();
    let query = ConicalCapJoinsQuery {
        spans: &cap_spans,
        contour_lengths: &lengths,
        reference_slant: prepared.reference_slant,
        options: work.remaining(),
    };
    let proof = certify_conical_cap_joins(query, cancelled).map_err(|cause| {
        let span = cause
            .index
            .and_then(|index| spans.get(index))
            .map(|proof| proof.span);
        chart_failure(original.region, span, cause, work)
    })?;
    work.charge(original.region, None, proof.operations())?;
    Ok(proof)
}

pub(in crate::surface_sketch::wrapped_profile::operation::cone) fn cap_join_spans<'a>(
    original: &'a DevelopedRegionRequest,
    prepared: &'a Prepared,
    spans: &'a [SpanProof],
) -> ConeChecked<Vec<ConicalCapJoinSpan<'a>>> {
    let count = prepared.source.loops.iter().map(Vec::len).sum::<usize>();
    if count != spans.len() {
        return Err(mismatch(
            original.region,
            None,
            "decoded cap join span count",
            vec![spans.len() as u64],
            vec![count as u64],
        ));
    }
    let pieces = prepared.source.loops.iter().flatten();
    pieces
        .zip(spans)
        .map(|(piece, span)| cap_join_span(original, prepared, piece, span))
        .collect()
}

fn cap_join_span<'a>(
    original: &'a DevelopedRegionRequest,
    prepared: &'a Prepared,
    piece: &'a AuthoredPiece,
    span: &'a SpanProof,
) -> ConeChecked<ConicalCapJoinSpan<'a>> {
    let refuse = || {
        mismatch(
            original.region,
            Some(piece.span),
            "decoded cap join source and chart",
            vec![0],
            vec![1],
        )
    };
    if piece.span != span.span {
        return Err(refuse());
    }
    let evidence = span.developed_boundary.as_ref().ok_or_else(refuse)?;
    let input = BoundaryInput::new(original, prepared, piece, "decoded cap join fit membership")?;
    let chart = input.metric()?.chart.ok_or_else(refuse)?;
    let cap = ConicalDevelopedBoundarySpan {
        source: input.query(&span.curves),
        boundary: &span.boundary,
        evidence,
    };
    Ok(ConicalCapJoinSpan { cap, chart })
}

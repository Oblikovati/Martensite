//! Certify the decoded angle/slant joins using the authority for each fitted span.
use super::*;
pub(super) fn certify(
    original: &DevelopedRegionRequest,
    prepared: &Prepared,
    spans: &[SpanProof],
    carriers: [ConicalTrimCarrier; 2],
    work: &mut Work,
    cancelled: &mut CancelPoll<'_>,
) -> ConeChecked<ConicalJoinsEvidence> {
    let cap_spans = cap_join_spans(original, prepared, spans, carriers)?;
    let lengths = lengths(prepared);
    let query = query(prepared, &cap_spans, &lengths, work.remaining());
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

pub(in crate::surface_sketch::wrapped_profile::operation::cone) fn bind(
    original: &DevelopedRegionRequest,
    prepared: &Prepared,
    stored: &Stored,
) -> ConeChecked<()> {
    let refuse = || {
        mismatch(
            original.region,
            None,
            "complete source-bound decoded cap joins",
            vec![0],
            vec![1],
        )
    };
    let options = stored.joins.cap_join_options().ok_or_else(refuse)?;
    let spans = cap_join_spans(original, prepared, &stored.spans, stored.carriers)?;
    if !stored
        .joins
        .matches_cap_joins(&query(prepared, &spans, &lengths(prepared), options))
    {
        return Err(refuse());
    }
    Ok(())
}

fn lengths(prepared: &Prepared) -> Vec<usize> {
    prepared.source.loops.iter().map(Vec::len).collect()
}

fn query<'a>(
    prepared: &Prepared,
    spans: &'a [ConicalCapJoinSpan<'a>],
    lengths: &'a [usize],
    options: ConeClearanceOptions,
) -> ConicalCapJoinsQuery<'a> {
    ConicalCapJoinsQuery {
        spans,
        contour_lengths: lengths,
        reference_slant: prepared.reference_slant,
        options,
    }
}

pub(in crate::surface_sketch::wrapped_profile::operation::cone) fn cap_join_spans<'a>(
    original: &'a DevelopedRegionRequest,
    prepared: &'a Prepared,
    spans: &'a [SpanProof],
    carriers: [ConicalTrimCarrier; 2],
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
        .map(|(piece, span)| cap_join_span(original, prepared, piece, span, carriers))
        .collect()
}

fn cap_join_span<'a>(
    original: &'a DevelopedRegionRequest,
    prepared: &'a Prepared,
    piece: &'a AuthoredPiece,
    span: &'a SpanProof,
    carriers: [ConicalTrimCarrier; 2],
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
    let evidence = cap_evidence(original.region, span, carriers)?;
    let input = BoundaryInput::new(original, prepared, piece, "decoded cap join fit membership")?;
    let chart = input.metric()?.chart.ok_or_else(refuse)?;
    let cap = ConicalDevelopedBoundarySpan {
        source: input.query(&span.curves),
        boundary: &span.boundary,
        evidence,
    };
    Ok(ConicalCapJoinSpan { cap, chart })
}

fn cap_evidence(
    region: SurfaceRegionId,
    span: &SpanProof,
    carriers: [ConicalTrimCarrier; 2],
) -> ConeChecked<&ConicalDevelopedBoundaryEvidence> {
    let pcurves = span.pcurves.each_ref().map(Vec::as_slice);
    span.developed_boundary
        .as_ref()
        .filter(|proof| proof.matches_decoded_caps(pcurves, carriers))
        .ok_or_else(|| {
            mismatch(
                region,
                Some(span.span),
                "exact decoded cap polynomials and carrier binding",
                vec![0],
                vec![1],
            )
        })
}

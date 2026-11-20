//! Reuse is conditional on actual decoded boundaries, retained UV and complete chart authority.
use super::*;
pub(super) fn certify(
    query: ConicalMaterialQuery<'_>,
    original: &DevelopedRegionRequest,
    prepared: &Prepared,
    proofs: &[storage::SpanProof],
    work: &mut storage::Work,
    cancelled: &mut CancelPoll<'_>,
) -> ConeChecked<ConicalMaterialEvidence> {
    let domain = prepared
        .chart_domain
        .as_deref()
        .ok_or_else(|| mismatch(original.region, None, "developed material chart authority"))?;
    let contexts = contexts(original, prepared, proofs)?;
    let result = selected_certify(query, &contexts, domain, prepared, cancelled);
    let operations = result.as_ref().map_or_else(
        |cause| cause.operations,
        ConicalMaterialEvidence::operations,
    );
    let charged = work.charge(original.region, None, operations);
    match (result, charged) {
        (Err(cause), _) => Err(failure(original.region, proofs, cause)),
        (Ok(_), Err(cause)) => Err(cause),
        (Ok(proof), Ok(())) => Ok(proof),
    }
}
pub(super) fn matches(
    query: &ConicalMaterialQuery<'_>,
    prepared: &Prepared,
    proofs: &[storage::SpanProof],
    material: &MaterialEvidence,
) -> ConeChecked<bool> {
    let MaterialRealization::Common(normal) = &material.normal else {
        return Ok(false);
    };
    let Some(domain) = prepared.chart_domain.as_deref() else {
        return Ok(normal.matches(query));
    };
    let contexts = contexts(&material.original, prepared, proofs)?;
    Ok(if super::super::domain::directed(prepared) {
        normal.matches_developed_with_refinement(
            query,
            &contexts,
            domain,
            ConicalPrimitiveRefinement::HalfSplitV1,
        )
    } else {
        normal.matches_developed(query, &contexts, domain)
    })
}
fn contexts<'a>(
    original: &'a DevelopedRegionRequest,
    prepared: &'a Prepared,
    proofs: &'a [storage::SpanProof],
) -> ConeChecked<Vec<ConicalDevelopedBoundarySpan<'a>>> {
    let pieces = piece_index(prepared)?;
    proofs
        .iter()
        .map(|proof| context(original, prepared, proof, &pieces))
        .collect()
}
fn context<'a>(
    original: &'a DevelopedRegionRequest,
    prepared: &'a Prepared,
    proof: &'a storage::SpanProof,
    pieces: &BTreeMap<CurveSpan, &'a AuthoredPiece>,
) -> ConeChecked<ConicalDevelopedBoundarySpan<'a>> {
    let span = proof.span;
    let piece = pieces.get(&span).copied().ok_or_else(|| {
        mismatch(
            original.region,
            Some(span),
            "developed boundary prepared source owner",
        )
    })?;
    let evidence = retained_witness(original.region, proof)?;
    Ok(ConicalDevelopedBoundarySpan {
        source: storage::boundary_query(original, prepared, piece, &proof.curves)?,
        boundary: &proof.boundary,
        evidence,
    })
}

fn failure(
    region: SurfaceRegionId,
    proofs: &[storage::SpanProof],
    cause: ConicalDevelopedMaterialError,
) -> ConeFailure {
    let span = cause
        .span
        .and_then(|index| proofs.get(index))
        .map(|proof| proof.span);
    storage::failure(region, span, ConeCause::DevelopedMaterial(Box::new(cause)))
}

fn piece_index(prepared: &Prepared) -> ConeChecked<BTreeMap<CurveSpan, &AuthoredPiece>> {
    let mut pieces = BTreeMap::new();
    for piece in prepared.source.loops.iter().flatten() {
        if pieces.insert(piece.span, piece).is_some() {
            return Err(mismatch(
                prepared.source.region,
                Some(piece.span),
                "unique developed boundary source owner",
            ));
        }
    }
    Ok(pieces)
}

fn retained_witness(
    region: SurfaceRegionId,
    proof: &storage::SpanProof,
) -> ConeChecked<&ConicalDevelopedBoundaryEvidence> {
    proof.developed_boundary.as_ref().ok_or_else(|| {
        mismatch(
            region,
            Some(proof.span),
            "developed boundary executed trim witness",
        )
    })
}

fn selected_certify(
    query: ConicalMaterialQuery<'_>,
    contexts: &[ConicalDevelopedBoundarySpan<'_>],
    domain: &ConicalChartDomainEvidence,
    prepared: &Prepared,
    cancelled: &mut CancelPoll<'_>,
) -> Result<ConicalMaterialEvidence, ConicalDevelopedMaterialError> {
    if super::super::domain::directed(prepared) {
        return certify_conical_material_with_developed_boundaries_and_refinement(
            query,
            contexts,
            domain,
            ConicalPrimitiveRefinement::HalfSplitV1,
            cancelled,
        );
    }
    certify_conical_material_with_developed_boundaries(query, contexts, domain, cancelled)
}

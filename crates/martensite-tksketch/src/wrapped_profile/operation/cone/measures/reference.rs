//! Bounded normal volume belongs to the same common-chart q/L reference as the body gate.
use super::*;
pub(super) fn certify(
    request: &WrappedConeNormalRequest,
    prepared: &Prepared,
    work: &mut storage::Work,
    cancelled: &mut CancelPoll<'_>,
) -> ConeChecked<CertifiedConicalReferenceVolume> {
    let spans = spans(prepared)?;
    let query = query(request, prepared, &spans, work);
    let result = certify_conical_reference_volume(query, cancelled);
    let proof = charged(result, request, prepared, work)?;
    if !proof.matches_source(query) {
        return Err(storage::mismatch(
            prepared.source.region,
            None,
            "reference volume source binding",
            vec![0],
            vec![1],
        ));
    }
    Ok(proof)
}
fn spans(prepared: &Prepared) -> ConeChecked<Vec<ConicalReferenceVolumeSpan<'_>>> {
    prepared
        .source
        .loops
        .iter()
        .flatten()
        .map(|piece| reference_span(prepared, piece))
        .collect()
}
fn reference_span<'a>(
    prepared: &'a Prepared,
    piece: &AuthoredPiece,
) -> ConeChecked<ConicalReferenceVolumeSpan<'a>> {
    let fit = prepared.fits.get(&piece.span).ok_or_else(|| {
        storage::mismatch(
            prepared.source.region,
            Some(piece.span),
            "reference volume fit membership",
            vec![0],
            vec![1],
        )
    })?;
    let view = fit.fit.view();
    Ok(ConicalReferenceVolumeSpan {
        angular_fit: view.tangent,
        slant_numerator: view.slant_numerator,
        slant_weights: view.slant_weights,
        reversed: false,
    })
}
fn query<'a>(
    request: &WrappedConeNormalRequest,
    prepared: &Prepared,
    spans: &'a [ConicalReferenceVolumeSpan<'a>],
    work: &storage::Work,
) -> ConicalReferenceVolumeQuery<'a> {
    ConicalReferenceVolumeQuery {
        spans,
        semi_angle: prepared.semi_angle,
        chart_origin: prepared.chart_origin,
        fit_center: prepared.chart_center,
        depths: prepared.source.depths,
        outward: true,
        options: work.integration(request.proof.integration),
        arithmetic: work.profile(),
    }
}
fn charged(
    result: Result<CertifiedConicalReferenceVolume, ConicalReferenceVolumeError>,
    request: &WrappedConeNormalRequest,
    prepared: &Prepared,
    work: &mut storage::Work,
) -> ConeChecked<CertifiedConicalReferenceVolume> {
    let (operations, cells) = result.as_ref().map_or_else(
        |cause| (cause.operations, cause.integration_subintervals),
        |proof| {
            (
                proof.arithmetic_operations(),
                proof.integration_subintervals(),
            )
        },
    );
    let arithmetic = work.charge(prepared.source.region, None, operations);
    let integration =
        work.charge_integration(prepared.source.region, cells, request.proof.integration);
    // Preserve a completed leaf's original failure while still recording each work unit.
    match (result, arithmetic, integration) {
        (Err(cause), _, _) => Err(failed(prepared, cause)),
        (Ok(_), Err(cause), _) | (Ok(_), Ok(()), Err(cause)) => Err(cause),
        (Ok(proof), Ok(()), Ok(())) => Ok(proof),
    }
}
fn failed(prepared: &Prepared, cause: ConicalReferenceVolumeError) -> ConeFailure {
    let span = cause
        .span
        .and_then(|index| prepared.source.loops.iter().flatten().nth(index))
        .map(|piece| piece.span);
    storage::failure(
        prepared.source.region,
        span,
        ConeCause::ReferenceVolume(Box::new(cause)),
    )
}

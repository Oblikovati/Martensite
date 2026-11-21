//! Original authored contours supply independent normal-volume integration evidence.
use super::*;
pub(super) fn certify(
    request: &WrappedConeNormalRequest,
    original: &DevelopedRegionRequest,
    flux: [f64; 2],
    work: &mut storage::Work,
    cancelled: &mut CancelPoll<'_>,
) -> ConeChecked<ConicalAuthoredFlux> {
    let loops: Vec<Vec<&[[f64; 2]]>> = original
        .loops
        .iter()
        .map(|pieces| pieces.iter().map(|piece| piece.poles.as_slice()).collect())
        .collect();
    let contours: Vec<_> = loops
        .iter()
        .map(|spans| ConicalAuthoredContour {
            spans,
            reverse: false,
        })
        .collect();
    let query = query(request, original, &contours, flux, work.remaining());
    let result = certify_conical_authored_flux(query, cancelled);
    let proof = charged(result, original, work)?;
    bind(&proof, query, original.region)?;
    Ok(proof)
}
fn bind(
    proof: &ConicalAuthoredFlux,
    query: ConicalAuthoredFluxQuery<'_>,
    region: SurfaceRegionId,
) -> ConeChecked<()> {
    if !proof.matches(query) {
        return Err(storage::mismatch(
            region,
            None,
            "authored normal volume source binding",
            vec![0],
            vec![1],
        ));
    }
    Ok(())
}
fn charged(
    result: Result<ConicalAuthoredFlux, ConicalAuthoredFluxError>,
    original: &DevelopedRegionRequest,
    work: &mut storage::Work,
) -> ConeChecked<ConicalAuthoredFlux> {
    let operations = result.as_ref().map_or_else(
        ConicalAuthoredFluxError::operations,
        ConicalAuthoredFlux::operations,
    );
    let charged = work.charge(original.region, None, operations);
    match (result, charged) {
        (Err(cause), _) => Err(failed(original, cause)),
        (Ok(_), Err(cause)) => Err(cause),
        (Ok(proof), Ok(())) => Ok(proof),
    }
}

fn query<'a>(
    request: &WrappedConeNormalRequest,
    original: &DevelopedRegionRequest,
    contours: &'a [ConicalAuthoredContour<'a>],
    flux: [f64; 2],
    arithmetic: ConeClearanceOptions,
) -> ConicalAuthoredFluxQuery<'a> {
    let options = request.proof.authored_volume;
    ConicalAuthoredFluxQuery {
        contours,
        semi_angle: request.support.semi_angle,
        depths: original.depths,
        outward: true,
        level: options.subdivision_level,
        clearance_level: options.clearance_level,
        volume_width: options.enclosure_width,
        flux,
        arithmetic,
    }
}
fn failed(original: &DevelopedRegionRequest, cause: ConicalAuthoredFluxError) -> ConeFailure {
    let span = cause
        .depth_cause()
        .and_then(|(contour, span, _)| {
            original
                .loops
                .get(contour)
                .and_then(|pieces| pieces.get(span))
        })
        .map(|piece| piece.span);
    storage::failure(original.region, span, ConeCause::Authored(Box::new(cause)))
}

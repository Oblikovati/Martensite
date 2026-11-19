//! The actual frozen cap and side occurrences supply the conditional centered patch flux.
use super::*;
pub(super) fn certify(
    request: &WrappedConeNormalRequest,
    region: SurfaceRegionId,
    stored: &storage::Stored,
    work: &mut storage::Work,
    cancelled: &mut CancelPoll<'_>,
) -> ConeChecked<CertifiedAnalyticConicalFlux> {
    let spans: Vec<_> = stored
        .spans
        .iter()
        .map(storage::SpanProof::flux_span)
        .collect();
    let query = AnalyticConicalFluxQuery {
        carriers: stored.carriers,
        spans: &spans,
        options: work.integration(request.proof.integration),
        arithmetic: work.profile(),
    };
    let result = certify_analytic_conical_boundary_flux(query, cancelled);
    charged(
        result,
        region,
        &stored.spans,
        request.proof.integration,
        work,
    )
}
fn charged(
    result: Result<CertifiedAnalyticConicalFlux, AnalyticConicalFluxError>,
    region: SurfaceRegionId,
    spans: &[storage::SpanProof],
    integration_options: CylindricalFluxOptions,
    work: &mut storage::Work,
) -> ConeChecked<CertifiedAnalyticConicalFlux> {
    let (operations, cells) = result.as_ref().map_or_else(
        |cause| (cause.operations, cause.integration_subintervals),
        |proof| {
            (
                proof.arithmetic_operations(),
                proof.integration_subintervals(),
            )
        },
    );
    let charged = work.charge(region, None, operations);
    let integration = work.charge_integration(region, cells, integration_options);
    let flux = match (result, charged, integration) {
        (Err(cause), _, _) => return Err(failed(region, spans, cause)),
        (Ok(_), Err(cause), _) | (Ok(_), Ok(()), Err(cause)) => return Err(cause),
        (Ok(proof), Ok(()), Ok(())) => proof,
    };
    Ok(flux)
}
fn failed(
    region: SurfaceRegionId,
    spans: &[storage::SpanProof],
    cause: AnalyticConicalFluxError,
) -> ConeFailure {
    let index = match &cause.cause {
        AnalyticConicalFluxFailure::Arithmetic { span, .. }
        | AnalyticConicalFluxFailure::Input { span, .. }
        | AnalyticConicalFluxFailure::Integration { span, .. } => *span,
        AnalyticConicalFluxFailure::Cap { span, .. } => Some(*span),
        AnalyticConicalFluxFailure::ArithmeticPolicy(_)
        | AnalyticConicalFluxFailure::Carrier { .. }
        | AnalyticConicalFluxFailure::Coefficients { .. } => None,
    };
    let span = index
        .and_then(|index| spans.get(index))
        .map(|proof| proof.span);
    storage::failure(region, span, ConeCause::Flux(Box::new(cause)))
}

#[cfg(test)]
mod tests;

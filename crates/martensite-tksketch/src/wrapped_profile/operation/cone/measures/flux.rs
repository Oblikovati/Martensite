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
    let flux = certify_analytic_conical_boundary_flux(query, cancelled)
        .map_err(|cause| failed(region, &stored.spans, cause))?;
    work.charge(region, None, flux.arithmetic_operations())?;
    work.charge_integration(
        region,
        flux.integration_subintervals(),
        request.proof.integration,
    )?;
    Ok(flux)
}
fn failed(
    region: SurfaceRegionId,
    spans: &[storage::SpanProof],
    cause: AnalyticConicalFluxError,
) -> ConeFailure {
    let index = match &cause.cause {
        AnalyticConicalFluxFailure::Input { span, .. }
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

//! Preserve each retained cap tolerance under the actual stored affine map (#454).
use super::*;

pub(super) fn certify(
    region: SurfaceRegionId,
    member: &pipeline::RetainedMember,
    plan: &Plan,
) -> Checked<WrappedCertifiedBound> {
    member
        .storage
        .iter()
        .try_fold(0.0_f64, |maximum, proof| {
            Ok(maximum.max(certify_span(region, proof, plan)?))
        })
        .map(reports::bound)
}
fn certify_span(
    region: SurfaceRegionId,
    proof: &storage::StoredProof,
    plan: &Plan,
) -> Checked<f64> {
    proof
        .trims
        .cap_residuals
        .iter()
        .enumerate()
        .try_fold(0.0_f64, |maximum, (depth, cap)| {
            let checked = certify_cap(region, proof.span, depth, cap, plan)?;
            Ok(maximum.max(checked))
        })
}
fn certify_cap(
    region: SurfaceRegionId,
    span: CurveSpan,
    depth: usize,
    cap: &martensite_tkmath::predicates::roots::metric_bounds::CertifiedCylindricalTrim,
    plan: &Plan,
) -> Checked<f64> {
    let actual = transport(
        cap.spatial_residual_bound(),
        plan.budget.frame().stretch_bounds()[1],
        plan.budget.evaluation_error_bound(),
    )?;
    enforce(region, span, depth, actual, cap.requested_spatial_limit())
}

fn transport(residual: f64, stretch: f64, evaluation: f64) -> Checked<f64> {
    let quantity = publication_budget::Quantity::Trace;
    let scaled = error::budget(publication_budget::scale_upper(residual, stretch, quantity))?;
    let evaluations = error::budget(publication_budget::scale_upper(evaluation, 2., quantity))?;
    // The common exact translation cancels, but two independently rounded evaluations remain.
    error::budget(publication_budget::add_upper(scaled, evaluations, quantity))
}
fn enforce(
    region: SurfaceRegionId,
    span: CurveSpan,
    depth: usize,
    actual: f64,
    retained_limit: f64,
) -> Checked<f64> {
    use publication_budget::Quantity as Q;
    valid_bound(actual, Q::Trace)?;
    valid_bound(retained_limit, Q::TraceTolerance)?;
    if actual <= retained_limit {
        return Ok(actual);
    }
    Err(trace_refusal(region, span, depth, actual, retained_limit))
}
fn valid_bound(value: f64, quantity: publication_budget::Quantity) -> Checked<()> {
    let bound = publication_budget::scale_upper(value, 1., quantity);
    error::budget(bound).map(|_| ())
}

fn trace_refusal(
    region: SurfaceRegionId,
    span: CurveSpan,
    depth: usize,
    actual: f64,
    retained_limit: f64,
) -> WrappedNormalFailure {
    fail(PlacementFailure::TraceTolerance {
        region,
        span,
        depth,
        actual,
        retained_limit,
    })
}
#[cfg(test)]
#[path = "trace_tests.rs"]
mod tests;

//! Preserve O2 source and policy diagnostics while forwarding operation cancellation.
use super::*;
pub(super) fn prove_profile(
    request: &Request,
    span: CurveSpan,
    curve: &OwnedCurve,
    authored: &[[f64; 2]],
    cancelled: &mut CancelPoll<'_>,
) -> Result<CertifiedCylindricalProfile, StorageFailure> {
    let query = profile_query(request, span, curve, authored)?;
    certify_cylindrical_profile(query, cancelled).map_err(|cause| profile_failure(span, cause))
}
fn profile_query<'a>(
    request: &Request,
    span: CurveSpan,
    curve: &'a OwnedCurve,
    authored: &'a [[f64; 2]],
) -> Result<CylindricalProfileQuery<'a>, StorageFailure> {
    let limit = (request.tolerance / 4.).next_down().max(0.);
    Ok(CylindricalProfileQuery {
        authored,
        stored: curve.borrowed(),
        interval: profile_interval(span, limit)?,
        support_radius: request.radius,
        stored_radius: request.radius + request.depths[0],
        angular_origin: 0.,
        spatial_limit: limit,
        options: Default::default(),
    })
}
fn profile_interval(
    span: CurveSpan,
    limit: f64,
) -> Result<CylindricalProfileInterval, StorageFailure> {
    CylindricalProfileInterval::new(u64::from(span.index), span.level).map_err(|cause| {
        profile_failure(
            span,
            CylindricalProfileError {
                requested_options: Default::default(),
                requested_spatial_limit: limit,
                cause,
            },
        )
    })
}
fn profile_failure(span: CurveSpan, cause: CylindricalProfileError) -> StorageFailure {
    StorageFailure {
        key: key(span, HistoryRole::FirstDepthEdge),
        cause: StorageCause::Profile(Box::new(cause)),
    }
}

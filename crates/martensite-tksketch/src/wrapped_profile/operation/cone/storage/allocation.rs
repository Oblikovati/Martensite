//! Exact endpoint bounds extend through decoded common-weight two-row ruled surfaces.
use super::*;
use martensite_tkmath::predicates::roots::DyadicBound;
#[derive(Clone, Copy, Debug, PartialEq)]
pub(in crate::surface_sketch::wrapped_profile) struct StorageEvidence {
    pub depth_edges: [f64; 2],
    pub ruled_side: f64,
    pub allowance: f64,
}
pub(super) fn certify(
    region: SurfaceRegionId,
    span: CurveSpan,
    boundary: &ConicalBoundaryEvidence,
    epsilon: f64,
) -> ConeChecked<StorageEvidence> {
    let allowance = retained::storage_share(epsilon);
    let depth_edges = [
        admit(region, span, &boundary.storage()[0], allowance)?,
        admit(region, span, &boundary.storage()[1], allowance)?,
    ];
    // Captured common weights and exact control-net rows make depth interpolation affine.
    let ruled_side = depth_edges[0].max(depth_edges[1]);
    Ok(StorageEvidence {
        depth_edges,
        ruled_side,
        allowance,
    })
}
fn admit(
    region: SurfaceRegionId,
    span: CurveSpan,
    actual: &DyadicBound,
    allowance: f64,
) -> ConeChecked<f64> {
    let upper = measures::upper(actual).map_err(|mut cause| {
        cause.region = Some(region);
        cause.span = Some(span);
        cause
    })?;
    let accepted = actual
        .compare_f64(allowance)
        .is_ok_and(|order| !order.is_gt());
    if !accepted {
        return Err(exceeded(region, span, upper, allowance));
    }
    Ok(upper)
}
fn exceeded(region: SurfaceRegionId, span: CurveSpan, actual: f64, limit: f64) -> ConeFailure {
    let cause = publication_budget::BudgetFailure::Limit {
        quantity: publication_budget::Quantity::Storage,
        actual,
        limit,
        region: Some(region),
        span: Some(span),
    };
    failure(region, Some(span), ConeCause::Budget(Box::new(cause)))
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn exact_storage_allocation_refuses_even_when_overall_epsilon_would_pass() {
        let (region, span) = source();
        let allowance = retained::storage_share(1e-3);
        assert!(
            admit(
                region,
                span,
                &DyadicBound::from_f64(allowance).unwrap(),
                allowance
            )
            .is_ok()
        );
        let refusal = admit(
            region,
            span,
            &DyadicBound::from_f64(allowance.next_up()).unwrap(),
            allowance,
        )
        .unwrap_err();
        check_owner(refusal, region, span);
    }
    #[test]
    fn zero_storage_share_admits_only_zero_bound() {
        let (region, span) = source();
        assert!(admit(region, span, &DyadicBound::from_f64(0.).unwrap(), 0.).is_ok());
        assert!(
            admit(
                region,
                span,
                &DyadicBound::from_f64(f64::from_bits(1)).unwrap(),
                0.
            )
            .is_err()
        );
    }
    fn check_owner(refusal: ConeFailure, region: SurfaceRegionId, span: CurveSpan) {
        assert_eq!(refusal.region, Some(region));
        assert_eq!(refusal.span, Some(span));
        assert!(matches!(refusal.cause, ConeCause::Budget(_)));
    }
    fn source() -> (SurfaceRegionId, CurveSpan) {
        (
            SurfaceRegionId(3),
            CurveSpan {
                curve: SurfaceCurveId(9),
                index: 0,
                level: 0,
            },
        )
    }
}

//! Original and decoded-domain planar censuses share one pair-bisection allowance.
use super::*;
impl Work {
    pub(in crate::surface_sketch::wrapped_profile) fn planar_remaining(
        &self,
        requested: PlanarProofWorkLimit,
    ) -> PlanarProofWorkLimit {
        let Some(receipt) = self.planar else {
            return requested;
        };
        PlanarProofWorkLimit::PairBisections(
            receipt.effective_pair_bisections - receipt.consumed_pair_bisections as u16,
        )
    }
    pub(in crate::surface_sketch::wrapped_profile) fn charge_planar(
        &mut self,
        region: SurfaceRegionId,
        actual: PlanarProofWorkReceipt,
    ) -> ConeChecked<()> {
        let receipt = self.planar.unwrap_or(PlanarProofWorkReceipt {
            consumed_pair_bisections: 0,
            ..actual
        });
        check_planar_limit(
            self.planar_remaining(PlanarProofWorkLimit::CurrentDefault),
            actual,
        )
        .map_err(|mut cause| {
            cause.region = Some(region);
            cause
        })?;
        let receipt = accumulated(receipt, actual).map_err(|mut cause| {
            cause.region = Some(region);
            cause
        })?;
        self.planar = Some(receipt);
        Ok(())
    }
    pub(in crate::surface_sketch::wrapped_profile) fn planar_receipt(
        &self,
    ) -> Option<PlanarProofWorkReceipt> {
        self.planar
    }
}
fn check_planar_limit(
    requested: PlanarProofWorkLimit,
    actual: PlanarProofWorkReceipt,
) -> ConeChecked<()> {
    let expected = match requested {
        PlanarProofWorkLimit::CurrentDefault => actual.requested_pair_bisections,
        PlanarProofWorkLimit::PairBisections(limit) => limit,
    };
    if actual.requested_pair_bisections != expected || actual.effective_pair_bisections != expected
    {
        return Err(planar_failure(
            u32::from(actual.effective_pair_bisections),
            u32::from(expected),
        ));
    }
    Ok(())
}
fn planar_failure(actual: u32, limit: u32) -> ConeFailure {
    ConeFailure::new(
        None,
        None,
        WrappedFailureClass::Incomplete,
        ConeCause::Limit {
            field: "aggregate planar pair bisections",
            actual,
            limit,
        },
    )
}

fn accumulated(
    mut receipt: PlanarProofWorkReceipt,
    actual: PlanarProofWorkReceipt,
) -> ConeChecked<PlanarProofWorkReceipt> {
    let limit = u32::from(receipt.effective_pair_bisections);
    let consumed = receipt
        .consumed_pair_bisections
        .checked_add(actual.consumed_pair_bisections)
        .ok_or_else(|| planar_failure(u32::MAX, limit))?;
    if consumed > u64::from(receipt.effective_pair_bisections) {
        return Err(planar_failure(
            u32::try_from(consumed).unwrap_or(u32::MAX),
            limit,
        ));
    }
    receipt.consumed_pair_bisections = consumed;
    Ok(receipt)
}

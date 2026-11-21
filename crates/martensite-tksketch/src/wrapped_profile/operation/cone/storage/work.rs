//! One remaining arithmetic allowance follows each frozen member through all leaves.
use super::*;
use martensite_tkbo::profile_extrusion::{PlanarProofWorkLimit, PlanarProofWorkReceipt};
mod integration;
mod planar;
pub(in crate::surface_sketch::wrapped_profile) struct Work {
    pub requested: ConeClearanceOptions,
    pub consumed: u32,
    pub integration_cells: u32,
    pub planar: Option<PlanarProofWorkReceipt>,
}
impl Work {
    pub(in crate::surface_sketch::wrapped_profile) fn new(requested: ConeClearanceOptions) -> Self {
        Self {
            requested,
            consumed: 0,
            integration_cells: 0,
            planar: None,
        }
    }
    pub(in crate::surface_sketch::wrapped_profile) fn remaining(&self) -> ConeClearanceOptions {
        ConeClearanceOptions {
            max_operations: self.requested.max_operations - self.consumed,
            ..self.requested
        }
    }
    pub(in crate::surface_sketch::wrapped_profile) fn profile(&self) -> CylindricalProfileOptions {
        let options = self.remaining();
        CylindricalProfileOptions {
            fractional_bits: options.fractional_bits,
            max_operations: options.max_operations,
            ..Default::default()
        }
    }
    pub(in crate::surface_sketch::wrapped_profile) fn charge(
        &mut self,
        region: SurfaceRegionId,
        span: Option<CurveSpan>,
        operations: u32,
    ) -> ConeChecked<()> {
        let Some(consumed) = self.consumed.checked_add(operations) else {
            return Err(self.exhausted(region, span, operations));
        };
        if consumed > self.requested.max_operations {
            return Err(self.exhausted(region, span, consumed));
        }
        self.consumed = consumed;
        Ok(())
    }
    fn exhausted(
        &self,
        region: SurfaceRegionId,
        span: Option<CurveSpan>,
        actual: u32,
    ) -> ConeFailure {
        failure(
            region,
            span,
            ConeCause::Limit {
                field: "cone certificate arithmetic operations",
                actual,
                limit: self.requested.max_operations,
            },
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    mod integration;
    mod planar;
    #[test]
    fn admitted_work_is_charged_once_and_remainder_is_shared() {
        let region = SurfaceRegionId(41);
        let mut work = Work::new(Default::default());
        work.charge(region, None, 100).unwrap();
        let requested = work.requested.max_operations;
        assert_eq!(work.remaining().max_operations, requested - 100);
        work.charge(region, None, 200).unwrap();
        assert_eq!(work.profile().max_operations, requested - 300);
        assert_eq!(work.consumed, 300);
    }
    #[test]
    fn exhausted_work_retains_owner_and_does_not_wrap() {
        let region = SurfaceRegionId(41);
        let mut work = Work::new(Default::default());
        let error = work.charge(region, None, u32::MAX).unwrap_err();
        assert_eq!(error.region, Some(region));
        assert_eq!(work.consumed, 0);
        assert_eq!(
            work.remaining().max_operations,
            work.requested.max_operations
        );
    }
}

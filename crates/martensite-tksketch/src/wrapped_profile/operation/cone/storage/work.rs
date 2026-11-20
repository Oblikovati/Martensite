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
    pub actual_operations: u32,
    pub actual_certificate:
        Option<std::sync::Arc<martensite_tkbo::body_check::CertifiedConicalBody>>,
    pub actual_bindings: BTreeMap<SurfaceRegionId, Vec<(BindingKey, Shape)>>,
}
impl Work {
    pub(in crate::surface_sketch::wrapped_profile) fn new(requested: ConeClearanceOptions) -> Self {
        Self {
            requested,
            consumed: 0,
            integration_cells: 0,
            planar: None,
            actual_bindings: BTreeMap::new(),
            actual_operations: 0,
            actual_certificate: None,
        }
    }
    pub(in crate::surface_sketch::wrapped_profile) fn remaining(&self) -> ConeClearanceOptions {
        ConeClearanceOptions {
            max_operations: (self.requested.max_operations - self.consumed).min(1_000_000),
            ..self.requested
        }
    }
    pub(in crate::surface_sketch::wrapped_profile) fn remaining_checked(
        &self,
        region: SurfaceRegionId,
        span: Option<CurveSpan>,
    ) -> ConeChecked<ConeClearanceOptions> {
        let options = self.remaining();
        if options.max_operations == 0 {
            return Err(failure(
                region,
                span,
                ConeCause::Scalar {
                    field: "remaining cone arithmetic operations",
                    actual: 0.,
                    expected: "positive remaining grant from the original finite allowance",
                },
            ));
        }
        Ok(options)
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
    fn aggregate_grants_lend_bounded_leaves_and_keep_one_final_remainder() {
        let mut options = ConeClearanceOptions::default();
        options.max_operations = 2_000_211;
        let mut work = Work::new(options);
        let region = SurfaceRegionId(41);
        assert_eq!(work.remaining().max_operations, 1_000_000);
        work.charge(region, None, 1_000_000).unwrap();
        assert_eq!(work.profile().max_operations, 1_000_000);
        work.charge(region, None, 1_000_000).unwrap();
        assert_eq!(work.remaining().max_operations, 211);
        work.charge(region, None, 211).unwrap();
        assert!(work.remaining_checked(region, None).is_err());
        assert_eq!(work.consumed, 2_000_211);
        assert!(work.charge(region, None, 1).is_err());
        assert_eq!(work.consumed, 2_000_211);
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

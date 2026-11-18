//! Ruled integration cells have their own aggregate units and original finite cap.
use super::*;
impl Work {
    pub(in crate::surface_sketch::wrapped_profile) fn integration(
        &self,
        requested: CylindricalFluxOptions,
    ) -> CylindricalFluxOptions {
        CylindricalFluxOptions {
            max_total_subintervals: requested
                .max_total_subintervals
                .saturating_sub(self.integration_cells),
            ..requested
        }
    }
    pub(in crate::surface_sketch::wrapped_profile) fn charge_integration(
        &mut self,
        region: SurfaceRegionId,
        actual: u64,
        requested: CylindricalFluxOptions,
    ) -> ConeChecked<()> {
        let consumed = u64::from(self.integration_cells).saturating_add(actual);
        if consumed > u64::from(requested.max_total_subintervals) {
            return Err(failure(
                region,
                None,
                ConeCause::Limit {
                    field: "aggregate ruled integration cells",
                    actual: u32::try_from(consumed).unwrap_or(u32::MAX),
                    limit: requested.max_total_subintervals,
                },
            ));
        }
        self.integration_cells = consumed as u32;
        Ok(())
    }
}

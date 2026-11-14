//! Chart rotation preserves canonical periodic material receipts and their source identities.
use super::*;
use martensite_tkmath::predicates::roots::metric_bounds::CertifiedCylindricalProbes;
impl WrappedChartNormalResult {
    /// Immutable canonical probe receipts; `result.material_probes().probes().len()`.
    pub fn material_probes(&self) -> &CertifiedCylindricalProbes {
        self.base.material_probes()
    }
    /// Outside-material parent relations; `result.material_relations().len()`.
    pub fn material_relations(
        &self,
    ) -> impl ExactSizeIterator<Item = WrappedMaterialRelation> + '_ {
        self.base.material_relations()
    }
    /// Exact canonical winding receipts; `result.material_locations().len()`.
    pub fn material_locations(&self) -> impl ExactSizeIterator<Item = WrappedWindingLocation> + '_ {
        self.base.material_locations()
    }
    /// Original settings and charged work; `result.material_work().used_bisections`.
    pub fn material_work(&self) -> WrappedMaterialWork {
        self.base.material_work()
    }
}

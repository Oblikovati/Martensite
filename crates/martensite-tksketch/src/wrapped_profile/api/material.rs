//! Inspect immutable material receipts without copying their proof archive.
use super::*;
use martensite_tkmath::predicates::roots::{
    RootIsolationOptions,
    metric_bounds::{CertifiedCylindricalProbes, CylindricalProbeOptions},
};

/// Certified reason that a witness lies outside another selected component's material.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum WrappedOutsideMaterial {
    /// Outside the target's outer contour.
    OutsideOuter,
    /// Inside the target's unfilled hole.
    InsideHole(SurfaceCurveId),
}
/// Authored owners and witness of one completed periodic material classification.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WrappedMaterialRelation {
    /// Witness's source region.
    pub source: SurfaceRegionId,
    /// Classified target region.
    pub target: SurfaceRegionId,
    /// Authored witness curve.
    pub witness: SurfaceCurveId,
    /// Reference into this result's probe certificate, carrying no persistent identity.
    pub probe_index: usize,
    /// Certified outside-material reason.
    pub reason: WrappedOutsideMaterial,
}
/// One exact winding result with its effective allowance and charged work.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WrappedWindingLocation {
    /// Classified region.
    pub target: SurfaceRegionId,
    /// Authored first curve of the contour.
    pub contour: SurfaceCurveId,
    /// Reference into this result's probe certificate.
    pub probe_index: usize,
    /// Exact winding number.
    pub winding: i64,
    /// Effective bisection allowance after reserving prior work.
    pub effective_bisections: u16,
    /// Bisections actually charged.
    pub used_bisections: u16,
}
/// Actual material-classification policy and aggregate charged work.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WrappedMaterialWork {
    /// Probe construction settings.
    pub probe_options: CylindricalProbeOptions,
    /// Original winding settings.
    pub winding_options: RootIsolationOptions,
    /// Maximum number of winding queries.
    pub maximum_locations: u32,
    /// Aggregate bisections actually charged.
    pub used_bisections: u16,
}
impl WrappedNormalResult {
    /// Source-bound periodic probes; `result.material_probes().probes().len()`.
    pub fn material_probes(&self) -> &CertifiedCylindricalProbes {
        &self.family.family.selection.material.probes
    }
    /// Authored outside-material relations; `result.material_relations().len()`.
    pub fn material_relations(
        &self,
    ) -> impl ExactSizeIterator<Item = WrappedMaterialRelation> + '_ {
        self.family
            .family
            .selection
            .material
            .relations
            .iter()
            .map(relation)
    }
    /// Exact per-contour winding receipts; `result.material_locations().len()`.
    pub fn material_locations(&self) -> impl ExactSizeIterator<Item = WrappedWindingLocation> + '_ {
        self.family
            .family
            .selection
            .material
            .locations
            .iter()
            .map(location)
    }
    /// Requested and charged material work; `result.material_work().used_bisections`.
    pub fn material_work(&self) -> WrappedMaterialWork {
        let material = &self.family.family.selection.material;
        WrappedMaterialWork {
            probe_options: material.options.probes,
            winding_options: material.options.winding,
            maximum_locations: material.options.max_locations,
            used_bisections: material.bisections,
        }
    }
}
fn relation(source: &selection::material::MaterialRelation) -> WrappedMaterialRelation {
    let reason = match source.reason {
        selection::material::OutsideReason::OutsideOuter => WrappedOutsideMaterial::OutsideOuter,
        selection::material::OutsideReason::InsideHole(curve) => {
            WrappedOutsideMaterial::InsideHole(curve)
        }
    };
    WrappedMaterialRelation {
        source: source.source,
        target: source.target,
        witness: source.witness,
        probe_index: source.probe,
        reason,
    }
}
fn location(source: &selection::material::LoopLocation) -> WrappedWindingLocation {
    WrappedWindingLocation {
        target: source.target,
        contour: source.contour,
        probe_index: source.probe,
        winding: source.winding,
        effective_bisections: source.effective_bisections,
        used_bisections: source.bisections,
    }
}

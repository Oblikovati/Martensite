// SPDX-License-Identifier: GPL-2.0-only
use super::*;

/// Actual retained geometry and source-bound ideal-profile inputs for one comparison.
/// Example: `CylindricalMeasureComparisonQuery { flux_source, normal_source, limit: CylindricalMeasureLimit::Report, options: Default::default() }`.
#[derive(Clone, Copy)]
pub struct CylindricalMeasureComparisonQuery<'a> {
    /// Reconstructed actual space curves, cap pcurves, radii and orientations.
    pub flux_source: CylindricalFluxQuery<'a>,
    /// Original sources, exact intervals and retained first-depth O2 proofs.
    pub normal_source: CylindricalMeasureQuery<'a>,
    /// Report or refuse the larger of the discrepancy and representative-error bounds.
    pub limit: CylindricalMeasureLimit,
    /// Precision and work allowance for this comparison, independent of input proof work.
    pub options: CylindricalMeasureOptions,
}

/// Outward comparison of two conditional measures with matching retained sources.
/// Closed boundary, orientation and injectivity premises are still external.
#[derive(Clone, Debug)]
pub struct CertifiedCylindricalMeasureComparison {
    pub(super) flux: CertifiedCylindricalFlux,
    pub(super) normal: CertifiedCylindricalNormalMeasure,
    pub(super) enclosure: [f64; 2],
    pub(super) discrepancy: f64,
    pub(super) representative_error: f64,
    pub(super) options: CylindricalMeasureOptions,
    pub(super) limit: CylindricalMeasureLimit,
    pub(super) operations: u32,
}
impl CertifiedCylindricalMeasureComparison {
    /// Retained flux evidence, with its independent integration error and work receipts.
    /// Example: `comparison.flux_certificate().absolute_error_bound()`.
    pub fn flux_certificate(&self) -> &CertifiedCylindricalFlux {
        &self.flux
    }
    /// Ideal normal measure, with its separate profile displacement and depth inputs.
    /// Example: `comparison.normal_certificate().profile_volume_enclosure()`.
    pub fn normal_certificate(&self) -> &CertifiedCylindricalNormalMeasure {
        &self.normal
    }
    /// Enclose actual signed flux minus ideal normalized-profile volume.
    /// Example: `let [lo, hi] = comparison.difference_enclosure();`.
    pub fn difference_enclosure(&self) -> [f64; 2] {
        self.enclosure
    }
    /// Bound the absolute flux/ideal discrepancy, conservatively including input enclosure widths.
    /// Example: `comparison.discrepancy_bound()`.
    pub fn discrepancy_bound(&self) -> f64 {
        self.discrepancy
    }
    /// Bound flux representative minus ideal volume; no quadrature estimate is used.
    /// Example: `comparison.representative_error_bound()`.
    pub fn representative_error_bound(&self) -> f64 {
        self.representative_error
    }
    /// Classify the difference evidence; `comparison.evidence()`.
    pub fn evidence(&self) -> CylindricalMeasureEvidence {
        CylindricalMeasureEvidence::CertifiedBound
    }
    /// Original comparison-only work policy; `comparison.requested_options()`.
    pub fn requested_options(&self) -> CylindricalMeasureOptions {
        self.options
    }
    /// Original explicit comparison-error policy; `comparison.requested_limit()`.
    pub fn requested_limit(&self) -> CylindricalMeasureLimit {
        self.limit
    }
    /// Charged comparison arithmetic, excluding input proof work; `comparison.operations()`.
    pub fn operations(&self) -> u32 {
        self.operations
    }
    /// Bound geometry and source identities independently of comparison work policy.
    /// Example: `assert!(comparison.matches_sources(query));`.
    pub fn matches_sources(&self, query: CylindricalMeasureComparisonQuery<'_>) -> bool {
        self.flux.matches_source(query.flux_source)
            && self.normal.matches_sources(query.normal_source)
    }
}

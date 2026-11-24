// SPDX-License-Identifier: GPL-2.0-only
use super::super::{CertifiedCylindricalProfile, CylindricalProfileSource};

/// Explicit treatment of the geometric volume discrepancy, in volume units.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CylindricalMeasureLimit {
    /// Report the bound without asserting a caller's volume tolerance.
    Report,
    /// Refuse when the bound exceeds this finite nonnegative volume allowance.
    Refuse(f64),
}

/// Finite, unclamped arithmetic policy; `CylindricalMeasureOptions::default()`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CylindricalMeasureOptions {
    /// Supported revision is one.
    pub behavior_version: u32,
    /// Fixed dyadic fractional precision in `8..=4096`.
    pub fractional_bits: u16,
    /// At most one million charged interval operations; zero refuses work.
    pub max_operations: u32,
    /// Maximum required source pieces in `1..=4096`.
    pub max_sources: u16,
}
impl Default for CylindricalMeasureOptions {
    fn default() -> Self {
        Self {
            behavior_version: 1,
            fractional_bits: 128,
            max_operations: 100_000,
            max_sources: 4096,
        }
    }
}

/// Conditional normal-extrusion measure from source-bound profile discrepancies.
/// Example: `CylindricalMeasureQuery { sources, depths: [-1., 2.], limit: CylindricalMeasureLimit::Report, options: Default::default() }`.
#[derive(Clone, Copy)]
pub struct CylindricalMeasureQuery<'a> {
    /// Directed pieces in one developed support chart, including all hole boundaries.
    pub sources: &'a [CylindricalProfileSource<'a>],
    /// Exact binary64 offsets from the original radius, before radius-addition rounding.
    pub depths: [f64; 2],
    /// Explicit geometric-error reporting or refusal policy.
    pub limit: CylindricalMeasureLimit,
    /// Original precision and aggregate work allowance.
    pub options: CylindricalMeasureOptions,
}

/// Evidence for the conditional scalar enclosures and discrepancy bounds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CylindricalMeasureEvidence {
    /// Outward dyadic arithmetic composed with source-bound O2 error certificates.
    CertifiedBound,
}

/// Arithmetic evidence only; closure, endpoint agreement and injectivity remain external.
#[derive(Clone, Debug)]
pub struct CertifiedCylindricalNormalMeasure {
    pub(super) bounds: MeasureBounds,
    pub(super) profiles: Vec<CertifiedCylindricalProfile>,
    pub(super) depths: [f64; 2],
    pub(super) options: CylindricalMeasureOptions,
    pub(super) limit: CylindricalMeasureLimit,
    pub(super) operations: u32,
}
#[derive(Clone, Debug)]
pub(super) struct MeasureBounds {
    pub(super) authored_area: [f64; 2],
    pub(super) area_error: f64,
    pub(super) authored_volume: [f64; 2],
    pub(super) profile_volume: [f64; 2],
    pub(super) volume_error: f64,
}
impl CertifiedCylindricalNormalMeasure {
    /// Enclose `sum integral s dz`; it is signed area only for complete closed contours.
    /// Example: `let [lo, hi] = proof.authored_area_enclosure();`.
    pub fn authored_area_enclosure(&self) -> [f64; 2] {
        self.bounds.authored_area
    }
    /// Bound the closed-profile area discrepancy; `proof.area_discrepancy_bound()`.
    pub fn area_discrepancy_bound(&self) -> f64 {
        self.bounds.area_error
    }
    /// Enclose the authored profile's signed normal extrusion; `proof.authored_volume_enclosure()`.
    pub fn authored_volume_enclosure(&self) -> [f64; 2] {
        self.bounds.authored_volume
    }
    /// Enclose the normalized stored profile's signed normal extrusion under the closure premises.
    /// Example: `let bounds = proof.profile_volume_enclosure();`.
    pub fn profile_volume_enclosure(&self) -> [f64; 2] {
        self.bounds.profile_volume
    }
    /// Bound the geometric volume discrepancy, excluding any represented-flux integration error.
    /// Example: `proof.volume_discrepancy_bound()`.
    pub fn volume_discrepancy_bound(&self) -> f64 {
        self.bounds.volume_error
    }
    /// Classify the measure evidence; `proof.evidence()`.
    pub fn evidence(&self) -> CylindricalMeasureEvidence {
        CylindricalMeasureEvidence::CertifiedBound
    }
    /// Read the original work policy; `proof.requested_options()`.
    pub fn requested_options(&self) -> CylindricalMeasureOptions {
        self.options
    }
    /// Read the explicit geometric-error policy; `proof.requested_limit()`.
    pub fn requested_limit(&self) -> CylindricalMeasureLimit {
        self.limit
    }
    /// Read the aggregate charged arithmetic work; `proof.operations()`.
    pub fn operations(&self) -> u32 {
        self.operations
    }
    /// Bind every original source, interval and retained O2 proof, independent of work policy.
    /// Example: `assert!(proof.matches_sources(query));`.
    pub fn matches_sources(&self, query: CylindricalMeasureQuery<'_>) -> bool {
        self.depths.map(f64::to_bits) == query.depths.map(f64::to_bits)
            && self.profiles.len() == query.sources.len()
            && self
                .profiles
                .iter()
                .zip(query.sources)
                .all(|(a, b)| b.matches() && a.same_source(b.certificate))
    }
}

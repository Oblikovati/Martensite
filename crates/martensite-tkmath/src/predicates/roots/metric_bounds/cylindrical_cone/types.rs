// SPDX-License-Identifier: GPL-2.0-only
use super::super::{CertifiedCylindricalProfile, CylindricalProfileInterval};
use crate::predicates::roots::DyadicBound;

/// Original source and the immutable O2 certificate supplying its derivative-error bound.
#[derive(Clone, Copy)]
pub struct CylindricalConeSource<'a> {
    /// Original developed polynomial, before floating-point subdivision.
    pub authored: &'a [[f64; 2]],
    /// Exact original-curve interval certified by O2.
    pub interval: CylindricalProfileInterval,
    /// Original support radius used to measure developed angular error.
    pub support_radius: f64,
    /// Original declared angular chart origin.
    pub angular_origin: f64,
    /// Retained first-depth profile's complete error certificate.
    pub certificate: &'a CertifiedCylindricalProfile,
}
impl CylindricalConeSource<'_> {
    pub(super) fn matches(&self) -> bool {
        self.certificate.matches_authored(
            self.authored,
            self.interval,
            self.support_radius,
            self.angular_origin,
        )
    }
}

/// Fixed proof behavior; it never selects geometric subdivisions.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CylindricalConeBehavior {
    /// Cartesian directions followed by normalized control-hull directions.
    V1,
}

/// Explicit unclamped dyadic arithmetic policy; `CylindricalConeOptions::default()`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CylindricalConeOptions {
    /// Requested proof revision.
    pub behavior: CylindricalConeBehavior,
    /// Fractional precision in `8..=4096`.
    pub fractional_bits: u16,
    /// Operation allowance, at most one million; zero refuses arithmetic.
    pub max_operations: u32,
}
impl Default for CylindricalConeOptions {
    fn default() -> Self {
        Self {
            behavior: CylindricalConeBehavior::V1,
            fractional_bits: 128,
            max_operations: 100_000,
        }
    }
}

/// One or two oriented local pieces seeking a common strictly monotone projection.
#[derive(Clone, Copy)]
pub struct CylindricalConeQuery<'a> {
    /// One regularity piece or two consecutive pieces in the same radius/origin chart.
    pub sources: &'a [CylindricalConeSource<'a>],
    /// Original precision and work settings.
    pub options: CylindricalConeOptions,
}

/// Strict derivative margins for every straight homotopy satisfying the retained O2 bounds.
/// Endpoint continuity and nonadjacent separation are separate obligations.
#[derive(Clone, Debug)]
pub struct CertifiedCylindricalCone {
    direction: [f64; 2],
    margins: Vec<DyadicBound>,
    sources: Vec<CertifiedCylindricalProfile>,
    options: CylindricalConeOptions,
    operations: u32,
    attempts: u32,
}
impl CertifiedCylindricalCone {
    /// Chosen finite direction; `proof.direction()`.
    pub fn direction(&self) -> [f64; 2] {
        self.direction
    }
    /// Positive lower bounds on `n dot H_i'` in each piece's local parameter.
    /// Example: `proof.directional_margin_bounds()`.
    pub fn directional_margin_bounds(&self) -> &[DyadicBound] {
        &self.margins
    }
    /// Unchanged arithmetic policy; `proof.requested_options()`.
    pub fn requested_options(&self) -> CylindricalConeOptions {
        self.options
    }
    /// Completed interval operations; `proof.operations()`.
    pub fn operations(&self) -> u32 {
        self.operations
    }
    /// Candidate directions tested; `proof.direction_attempts()`.
    pub fn direction_attempts(&self) -> u32 {
        self.attempts
    }
    /// Compare original inputs, retained curve bits and derivative-error bounds.
    /// Example: `assert!(proof.matches_sources(query.sources));`.
    pub fn matches_sources(&self, sources: &[CylindricalConeSource<'_>]) -> bool {
        self.sources.len() == sources.len()
            && self.sources.iter().zip(sources).all(|(old, new)| {
                new.matches()
                    && old.same_source(new.certificate)
                    && old.developed_derivative_bound().to_bits()
                        == new.certificate.developed_derivative_bound().to_bits()
            })
    }
    pub(super) fn assemble(
        direction: [f64; 2],
        margins: Vec<DyadicBound>,
        query: CylindricalConeQuery<'_>,
        operations: u32,
        attempts: u32,
    ) -> Self {
        Self {
            direction,
            margins,
            sources: query
                .sources
                .iter()
                .map(|s| s.certificate.clone())
                .collect(),
            options: query.options,
            operations,
            attempts,
        }
    }
}

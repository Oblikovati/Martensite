// SPDX-License-Identifier: GPL-2.0-only
use super::*;

/// Direction of an original witness translated into a target contour's unshifted chart.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CylindricalProbeSense {
    /// Query `point - image.offset()`; the target is translated positively.
    Subtract,
    /// Query `point + image.offset()`; the witness is translated positively.
    Add,
}
/// One finite periodic image and an original boundary witness.
/// Example: `CylindricalPeriodicProbe { turn: 1, point: [0., 0.], sense: CylindricalProbeSense::Subtract }`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CylindricalPeriodicProbe {
    /// Positive finite image number present in the clearance report.
    pub turn: u32,
    /// Original finite developed boundary point; membership in its source is external.
    pub point: [f64; 2],
    /// Exact intended addition or subtraction of the true image period.
    pub sense: CylindricalProbeSense,
}
/// Finite aggregate arithmetic policy; `CylindricalProbeOptions::default()`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CylindricalProbeOptions {
    /// Supported revision is one.
    pub behavior_version: u32,
    /// Fractional precision in `8..=4096`.
    pub fractional_bits: u16,
    /// Charged operations across every probe, at most one million.
    pub max_operations: u32,
    /// Required probes at most 65,536; zero admits only an empty batch.
    pub max_probes: u32,
}
impl Default for CylindricalProbeOptions {
    fn default() -> Self {
        Self {
            behavior_version: 1,
            fractional_bits: 128,
            max_operations: 100_000,
            max_probes: 8192,
        }
    }
}
/// Outward guard for one rounded query point, conditional on the certified literal clearance.
#[derive(Clone, Debug)]
pub struct CertifiedCylindricalProbe {
    pub(super) request: CylindricalPeriodicProbe,
    pub(super) point: [f64; 2],
    pub(super) rounding: DyadicBound,
    pub(super) translation: CylindricalPeriodicTranslation,
}
impl CertifiedCylindricalProbe {
    /// Original image, witness and direction; `probe.request()`.
    pub fn request(&self) -> CylindricalPeriodicProbe {
        self.request
    }
    /// Rounded seed for an exact original-curve winding query; `probe.point()`.
    pub fn point(&self) -> [f64; 2] {
        self.point
    }
    /// Outward error from the exact literal-shifted seed; `probe.rounding_error_bound()`.
    pub fn rounding_error_bound(&self) -> &DyadicBound {
        &self.rounding
    }
    /// Actual image translation and its independent true-period error; `probe.translation()`.
    pub fn translation(&self) -> &CylindricalPeriodicTranslation {
        &self.translation
    }
}
/// Every required rounded witness, without a material classification or body-validity claim.
#[derive(Clone, Debug)]
pub struct CertifiedCylindricalProbes {
    pub(super) probes: Vec<CertifiedCylindricalProbe>,
    pub(super) clearance: f64,
    pub(super) options: CylindricalProbeOptions,
    pub(super) operations: u32,
}
impl CertifiedCylindricalProbes {
    /// Complete input-order receipts; an empty slice is a completed empty batch.
    /// Example: `proof.probes()`.
    pub fn probes(&self) -> &[CertifiedCylindricalProbe] {
        &self.probes
    }
    /// Physical boundary margin used for the strict rounding guard; `proof.clearance()`.
    pub fn clearance(&self) -> f64 {
        self.clearance
    }
    /// Original unclamped policy; `proof.requested_options()`.
    pub fn requested_options(&self) -> CylindricalProbeOptions {
        self.options
    }
    /// Charged aggregate arithmetic; `proof.operations()`.
    pub fn operations(&self) -> u32 {
        self.operations
    }
    /// Bind all original witnesses and used image metadata, independent of arithmetic policy.
    /// Example: `assert!(proof.matches_sources(periodic, requests));`.
    pub fn matches_sources(
        &self,
        periodic: &CertifiedCylindricalPeriodicClearance,
        requests: &[CylindricalPeriodicProbe],
    ) -> bool {
        self.clearance.to_bits() == periodic.requested_clearance().to_bits()
            && self.probes.len() == requests.len()
            && self
                .probes
                .iter()
                .zip(requests)
                .all(|(old, new)| binding::same_probe(old, new, periodic))
    }
}

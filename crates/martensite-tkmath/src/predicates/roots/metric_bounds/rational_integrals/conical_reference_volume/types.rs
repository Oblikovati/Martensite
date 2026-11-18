// SPDX-License-Identifier: GPL-2.0-only
use super::*;

/// One directed ideal reference span on its complete local `[0,1]` interval.
#[derive(Clone, Copy)]
pub struct ConicalReferenceVolumeSpan<'a> {
    /// Polynomial Bernstein half-angle tangent q.
    pub angular_fit: &'a [f64],
    /// Bernstein homogeneous slant numerator N; L=N/W.
    pub slant_numerator: &'a [f64],
    /// Positive Bernstein denominator W, with the same count as N.
    pub slant_weights: &'a [f64],
    /// Negate this supplied traversal; holes retain their independently declared direction.
    pub reversed: bool,
}

/// The exact fitted normal reference and its independent scalar work/error policies.
#[derive(Clone, Copy)]
pub struct ConicalReferenceVolumeQuery<'a> {
    /// Complete ordered directed reference spans.
    pub spans: &'a [ConicalReferenceVolumeSpan<'a>],
    /// Semi-angle strictly inside `(0,pi/2)`.
    pub semi_angle: f64,
    /// Common physical chart origin, retained for source binding; rotation does not change volume.
    pub chart_origin: f64,
    /// Common half-angle chart center, retained for source binding.
    pub fit_center: f64,
    /// Increasing signed normal-depth endpoints.
    pub depths: [f64; 2],
    /// Positive depths follow the outward normal when true.
    pub outward: bool,
    /// Original aggregate integration and scalar-error limits.
    pub options: CylindricalFluxOptions,
    /// Shared bounded trigonometric/depth-coefficient policy.
    pub arithmetic: CylindricalProfileOptions,
}

/// Complete signed scalar enclosure bound to the fitted reference operands.
#[derive(Clone, Debug)]
pub struct CertifiedConicalReferenceVolume {
    pub(super) scalar: publish::RoundedScalar,
    pub(super) bits: Vec<u64>,
    pub(super) options: CylindricalFluxOptions,
    pub(super) arithmetic: CylindricalProfileOptions,
    pub(super) work: Vec<CylindricalFluxWork>,
    pub(super) operations: u32,
}
impl CertifiedConicalReferenceVolume {
    /// Outward signed reference-volume enclosure; `proof.enclosure()`.
    pub fn enclosure(&self) -> [f64; 2] {
        self.scalar.enclosure
    }
    /// Lower-rounded enclosure midpoint; `proof.representative()`.
    pub fn representative(&self) -> f64 {
        self.scalar.representative
    }
    /// Certified absolute discrepancy from that representative; `proof.absolute_error_bound()`.
    pub fn absolute_error_bound(&self) -> f64 {
        self.scalar.absolute_error_bound
    }
    /// Original integration policy; `proof.requested_options()`.
    pub fn requested_options(&self) -> CylindricalFluxOptions {
        self.options
    }
    /// Unclamped aggregate policy; `proof.effective_options()`.
    pub fn effective_options(&self) -> CylindricalFluxOptions {
        self.options
    }
    /// Original coefficient arithmetic policy; `proof.requested_arithmetic()`.
    pub fn requested_arithmetic(&self) -> CylindricalProfileOptions {
        self.arithmetic
    }
    /// Unclamped coefficient arithmetic policy; `proof.effective_arithmetic()`.
    pub fn effective_arithmetic(&self) -> CylindricalProfileOptions {
        self.arithmetic
    }
    /// Transcendental coefficients retain bounded evidence even for exact quotient integrals.
    /// Example: `assert_eq!(proof.evidence(), RationalIntegralEvidence::ErrorBoundedIntegration)`.
    pub fn evidence(&self) -> RationalIntegralEvidence {
        self.scalar.evidence
    }
    /// Actual aggregate examined quotient cells; `proof.integration_subintervals()`.
    pub fn integration_subintervals(&self) -> u64 {
        self.scalar.subintervals
    }
    /// Bounded trig/depth operations; exact coefficient algebra is bounded by admitted degrees.
    /// Example: `assert!(proof.arithmetic_operations() <= proof.requested_arithmetic().max_operations)`.
    pub fn arithmetic_operations(&self) -> u32 {
        self.operations
    }
    /// Original per-span effective allowances and actual combined term work; `proof.span_work()`.
    pub fn span_work(&self) -> &[CylindricalFluxWork] {
        &self.work
    }
    /// Bind all fitted, support, orientation, limit and policy bits; `assert!(proof.matches_source(query))`.
    pub fn matches_source(&self, query: ConicalReferenceVolumeQuery<'_>) -> bool {
        admission::admit(query).is_ok() && self.bits == binding::source_bits(query)
    }
}

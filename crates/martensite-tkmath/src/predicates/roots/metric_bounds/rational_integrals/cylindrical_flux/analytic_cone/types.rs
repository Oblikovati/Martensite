// SPDX-License-Identifier: GPL-2.0-only
use super::*;

/// One actual ruled side and its two analytic-cone UV cap occurrences.
#[derive(Clone, Copy)]
pub struct AnalyticConicalFluxSpan<'a> {
    /// Retained first/last space curves with equal counts and common positive weights.
    pub boundaries: [MetricBezier<'a, 3>; 2],
    /// Retained polynomial `(U,V)` controls; `V` is relative to the actual reference circle.
    pub cap_pcurves: [&'a [[f64; 2]]; 2],
    /// Reverse the ruled side's natural `du cross dv` orientation.
    pub side_reversed: bool,
    /// Reverse each cap occurrence relative to the analytic `dU cross dV` orientation.
    pub cap_reversed: [bool; 2],
}

/// Actual carriers and occurrences with independent finite integration/arithmetic policies.
#[derive(Clone, Copy)]
pub struct AnalyticConicalFluxQuery<'a> {
    /// Retained first/last local +Z analytic cone data; no radius or origin is inferred.
    pub carriers: [ConicalTrimCarrier; 2],
    /// Complete ordered side and cap occurrences supplied by the caller.
    pub spans: &'a [AnalyticConicalFluxSpan<'a>],
    /// Original ruled-integration cell and final absolute-error policies.
    pub options: CylindricalFluxOptions,
    /// Shared carrier-coefficient arithmetic policy, charged once across both carriers.
    pub arithmetic: CylindricalProfileOptions,
}

/// Completed conditional flux; immutable evidence does not assert that a body is valid.
#[derive(Clone, Debug)]
pub struct CertifiedAnalyticConicalFlux {
    scalar: publish::RoundedScalar,
    bits: Vec<u64>,
    options: CylindricalFluxOptions,
    arithmetic: CylindricalProfileOptions,
    work: Vec<CylindricalFluxWork>,
    operations: u32,
}
impl CertifiedAnalyticConicalFlux {
    /// Outward signed flux bounds; `proof.enclosure()`.
    pub fn enclosure(&self) -> [f64; 2] {
        self.scalar.enclosure
    }
    /// Deterministic lower-rounded midpoint; `proof.representative()`.
    pub fn representative(&self) -> f64 {
        self.scalar.representative
    }
    /// Certified absolute discrepancy from the representative; `proof.absolute_error_bound()`.
    pub fn absolute_error_bound(&self) -> f64 {
        self.scalar.absolute_error_bound
    }
    /// Original integration policy; `proof.requested_options()`.
    pub fn requested_options(&self) -> CylindricalFluxOptions {
        self.options
    }
    /// Unchanged parent integration policy; scalar term algorithm1 and per-span grants are separate.
    /// Example: `assert_eq!(proof.effective_options(), proof.requested_options());`.
    pub fn effective_options(&self) -> CylindricalFluxOptions {
        self.options
    }
    /// Original shared coefficient policy; `proof.requested_arithmetic()`.
    pub fn requested_arithmetic(&self) -> CylindricalProfileOptions {
        self.arithmetic
    }
    /// Unchanged coefficient policy; `proof.effective_arithmetic()`.
    pub fn effective_arithmetic(&self) -> CylindricalProfileOptions {
        self.arithmetic
    }
    /// Bounded trigonometric and rational integration evidence; `proof.evidence()`.
    pub fn evidence(&self) -> RationalIntegralEvidence {
        self.scalar.evidence
    }
    /// Examined ruled-side cells; `proof.integration_subintervals()`.
    pub fn integration_subintervals(&self) -> u64 {
        self.scalar.subintervals
    }
    /// Effective and measured ruled-side work in supplied order; `proof.span_work()`.
    pub fn span_work(&self) -> &[CylindricalFluxWork] {
        &self.work
    }
    /// Charged coefficient operations, plus outward accumulation operations for parent version2.
    /// Example: `assert!(proof.arithmetic_operations() <= proof.requested_arithmetic().max_operations);`.
    pub fn arithmetic_operations(&self) -> u32 {
        self.operations
    }
    /// Complete supplied occurrence count; `proof.span_count()`.
    pub fn span_count(&self) -> usize {
        self.work.len()
    }
    /// Bind actual geometry; parent2 additionally binds its full precision and resource policy.
    /// Example: `assert!(proof.matches_source(query));`.
    pub fn matches_source(&self, query: AnalyticConicalFluxQuery<'_>) -> bool {
        admission::admit(query).is_ok()
            && self.options.behavior_version == query.options.behavior_version
            && self.bits == binding::source_bits(query)
            && (self.options.behavior_version == 1
                || binding::policy_bits(self.options, self.arithmetic)
                    == binding::policy_bits(query.options, query.arithmetic))
    }
    pub(super) fn assemble(
        query: AnalyticConicalFluxQuery<'_>,
        scalar: publish::RoundedScalar,
        work: Vec<CylindricalFluxWork>,
        operations: u32,
    ) -> Self {
        Self {
            scalar,
            bits: binding::source_bits(query),
            options: query.options,
            arithmetic: query.arithmetic,
            work,
            operations,
        }
    }
}

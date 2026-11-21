// SPDX-License-Identifier: GPL-2.0-only
use super::*;

/// Original comparison settings and typed failure; no partial comparison accompanies it.
#[derive(Clone, Debug, PartialEq)]
pub struct CylindricalMeasureComparisonError {
    /// Requested comparison precision and work policy.
    pub requested_options: CylindricalMeasureOptions,
    /// Requested volume discrepancy/representative allowance.
    pub requested_limit: CylindricalMeasureLimit,
    /// Number of supplied retained flux spans.
    pub flux_spans: usize,
    /// Number of supplied original profile sources.
    pub normal_sources: usize,
    /// Original refusal cause.
    pub cause: CylindricalMeasureComparisonFailure,
}
/// Failed provenance or incomplete outward comparison.
#[derive(Clone, Debug, PartialEq)]
pub enum CylindricalMeasureComparisonFailure {
    /// Settings outside the finite admitted comparison policy.
    Policy,
    /// Nonempty source counts must match and fit the declared source limit.
    Count,
    /// Actual retained flux geometry does not match its completed certificate.
    FluxBinding,
    /// Original profile sources or requested depths do not match the completed normal measure.
    NormalBinding,
    /// The paired first-depth curve does not match the source's O2 evidence.
    SpanBinding {
        /// Offending supplied pair index.
        index: usize,
    },
    /// Original bounded arithmetic failure.
    Arithmetic(Box<super::super::super::BoundedArithmeticFailure>),
    /// At least one bound exceeded the unchanged finite requested volume allowance.
    Uncertified {
        /// Larger of the outward discrepancy and representative-error bounds.
        bound: f64,
        /// Original allowance.
        requested: f64,
    },
}
impl From<super::super::super::BoundedArithmeticFailure> for CylindricalMeasureComparisonFailure {
    fn from(cause: super::super::super::BoundedArithmeticFailure) -> Self {
        Self::Arithmetic(Box::new(cause))
    }
}
impl std::fmt::Display for CylindricalMeasureComparisonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "cylindrical measure comparison flux spans={}, normal sources={}, policy={:?}, limit={:?}: {:?}; expected matching completed source-bound measures, complete paired spans and outward bounds within the unchanged allowance",
            self.flux_spans,
            self.normal_sources,
            self.requested_options,
            self.requested_limit,
            self.cause
        )
    }
}
impl std::error::Error for CylindricalMeasureComparisonError {}

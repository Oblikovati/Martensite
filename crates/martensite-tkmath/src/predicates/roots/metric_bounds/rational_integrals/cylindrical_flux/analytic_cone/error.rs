// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use crate::predicates::roots::metric_bounds::CylindricalProfileFailure;
pub(super) type Checked<T> = Result<T, AnalyticConicalFluxFailure>;

/// No partial scalar accompanies any incomplete supplied occurrence.
#[derive(Debug)]
pub struct AnalyticConicalFluxError {
    /// Original integration/error policy.
    pub requested_options: CylindricalFluxOptions,
    /// Original shared carrier-coefficient policy.
    pub requested_arithmetic: CylindricalProfileOptions,
    /// Complete supplied occurrence count.
    pub span_count: usize,
    /// Owned typed failure, with local span/depth where known.
    pub cause: AnalyticConicalFluxFailure,
}

/// Unsupported data or incomplete work; positions identify local occurrences, not persistent names.
#[derive(Debug)]
pub enum AnalyticConicalFluxFailure {
    /// Existing shared ruled/polynomial admission refused the actual data.
    Input {
        /// Offending local span, absent for policy/count checks.
        span: Option<usize>,
        /// Actual first/last occurrence, absent for a joint pair constraint.
        depth: Option<usize>,
        /// Original shared typed admission cause.
        cause: Box<CylindricalFluxFailure>,
    },
    /// Unsupported coefficient precision/work settings.
    ArithmeticPolicy(CylindricalProfileOptions),
    /// Invalid retained analytic carrier.
    Carrier {
        /// First/last cap position.
        depth: usize,
        /// Original offending carrier values.
        actual: ConicalTrimCarrier,
    },
    /// Shared interval/trigonometric coefficient work did not complete.
    Coefficients {
        /// First/last carrier position; coefficients are computed once before occurrence work.
        depth: usize,
        /// Original bounded-arithmetic/trigonometric cause.
        cause: Box<CylindricalProfileFailure>,
    },
    /// Exact polynomial cap-moment work did not complete.
    Cap {
        /// Actual supplied span position.
        span: usize,
        /// First/last cap position.
        depth: usize,
        /// Original polynomial capture/integration cause.
        cause: Box<PolynomialIntegralFailure>,
    },
    /// Ruled-side integration, operation cancellation or final rounding refused.
    Integration {
        /// Actual side position; absent for an operation-level refusal.
        span: Option<usize>,
        /// Original typed integration cause.
        cause: Box<RationalIntegralFailure>,
    },
}
impl std::fmt::Display for AnalyticConicalFluxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "analytic-cone flux spans={}: {:?}; expected admitted finite +Z cone carriers, common-weight ruled curves, actual polynomial cap UV controls and completion within unchanged error/work policies",
            self.span_count, self.cause
        )
    }
}
impl std::error::Error for AnalyticConicalFluxError {}

pub(super) fn integration(
    span: Option<usize>,
    cause: impl Into<RationalIntegralFailure>,
) -> AnalyticConicalFluxFailure {
    AnalyticConicalFluxFailure::Integration {
        span,
        cause: Box::new(cause.into()),
    }
}
pub(super) fn input(
    span: Option<usize>,
    depth: Option<usize>,
    cause: CylindricalFluxFailure,
) -> AnalyticConicalFluxFailure {
    AnalyticConicalFluxFailure::Input {
        span,
        depth,
        cause: Box::new(cause),
    }
}

// SPDX-License-Identifier: GPL-2.0-only
use super::*;
pub(super) type Checked<T> = Result<T, CylindricalFluxFailure>;

/// No scalar is published when any supplied contribution is incomplete.
#[derive(Debug)]
pub struct CylindricalFluxError {
    /// Original requested policy.
    pub requested_options: CylindricalFluxOptions,
    /// Number of supplied spans.
    pub span_count: usize,
    /// Typed input or incomplete-integration cause.
    pub cause: CylindricalFluxFailure,
}
/// Input and integration failures preserve local occurrence positions, never persistent IDs.
#[derive(Debug)]
pub enum CylindricalFluxFailure {
    /// Unsupported finite resource policy.
    Policy(CylindricalFluxOptions),
    /// Required complete input count is outside the declared range.
    SpanCount {
        /// Observed count.
        actual: usize,
        /// Original maximum.
        maximum: u32,
    },
    /// Actual radii must be finite, positive and strictly increasing.
    Radii([f64; 2]),
    /// A ruled pair must have matching stored degree.
    PairCounts {
        /// Local span.
        span: usize,
        /// Actual first/last counts.
        counts: [usize; 2],
    },
    /// A ruled pair must retain common positive weights.
    PairWeights {
        /// Local span.
        span: usize,
        /// Local pole index.
        pole: usize,
        /// Actual first/last weights.
        weights: [f64; 2],
    },
    /// Metric admission or coefficient capture failed.
    Geometry {
        /// Local span.
        span: usize,
        /// Original offending values and expected shape.
        cause: Box<ResidualError>,
    },
    /// A cap's exact polynomial contribution failed.
    Cap {
        /// Local span.
        span: usize,
        /// First/last cap.
        depth: usize,
        /// Original typed cause.
        cause: Box<PolynomialIntegralFailure>,
    },
    /// A side, cancellation or final enclosure failed.
    Integration {
        /// Local span, or none for operation-level work.
        span: Option<usize>,
        /// Original typed incomplete cause.
        cause: Box<RationalIntegralFailure>,
    },
}
impl std::fmt::Display for CylindricalFluxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "cylindrical boundary flux spans={}: {:?}; expected v1, degree <=16, 1..=4096 spans, 1..=65536 cells/span, total cells <=1000000, finite error >=0, finite common-weight curves and ordered positive radii",
            self.span_count, self.cause
        )
    }
}
impl std::error::Error for CylindricalFluxError {}

pub(super) fn integration(
    span: Option<usize>,
    cause: impl Into<RationalIntegralFailure>,
) -> CylindricalFluxFailure {
    CylindricalFluxFailure::Integration {
        span,
        cause: Box::new(cause.into()),
    }
}
pub(super) fn geometry(span: usize, cause: ResidualError) -> CylindricalFluxFailure {
    CylindricalFluxFailure::Geometry {
        span,
        cause: Box::new(cause),
    }
}

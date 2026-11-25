// SPDX-License-Identifier: GPL-2.0-only
use super::*;
/// A chart bridge refusal retains the actual failed binding or original numerical cause.
#[derive(Clone, Debug, PartialEq)]
pub struct ConicalChartTrimPairError {
    /// Actual failed trace, or no trace when common chart authority was refused.
    pub depth: Option<usize>,
    /// Completed arithmetic operations, without recharging retained proofs.
    pub operations: u32,
    /// Original typed cause; failure payload allocation occurs only on refusal.
    pub cause: Box<ConicalChartTrimPairFailure>,
}
/// Source authority differs from an unsupported or incomplete numerical trace.
#[derive(Clone, Debug, PartialEq)]
pub enum ConicalChartTrimPairFailure {
    /// Actual and required source bits or authority shape differed.
    Binding {
        /// Bound operand that failed the check.
        field: &'static str,
        /// Actual operand bits, counts or presence markers.
        actual: Vec<u64>,
        /// Expected operand bits, counts or presence markers.
        expected: Vec<u64>,
    },
    /// Original decoded trace refusal, without a synthetic legacy failure.
    Trace(ConicalTrimError),
}
impl ConicalChartTrimPairError {
    /// Whether the original bounded arithmetic observed cancellation; `error.is_cancelled()`.
    pub fn is_cancelled(&self) -> bool {
        let ConicalChartTrimPairFailure::Trace(trace) = self.cause.as_ref() else {
            return false;
        };
        matches!(&trace.cause, CylindricalProfileFailure::Arithmetic(cause)
            if matches!(cause.as_ref(), BoundedArithmeticFailure::Cancelled))
    }
}
impl std::fmt::Display for ConicalChartTrimPairError {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            out,
            "chart trim depth={:?}, work={}: {}",
            self.depth, self.operations, self.cause
        )
    }
}
impl std::fmt::Display for ConicalChartTrimPairFailure {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Binding {
                field,
                actual,
                expected,
            } => write!(out, "{field}: actual={actual:?}; expected {expected:?}"),
            Self::Trace(cause) => write!(out, "{cause}"),
        }
    }
}
impl std::error::Error for ConicalChartTrimPairError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self.cause.as_ref() {
            ConicalChartTrimPairFailure::Trace(cause) => Some(cause),
            ConicalChartTrimPairFailure::Binding { .. } => None,
        }
    }
}
pub(super) fn trace(
    query: ConicalTrimQuery<'_>,
    depth: usize,
    operations: u32,
    cause: CylindricalProfileFailure,
) -> ConicalChartTrimPairError {
    ConicalChartTrimPairError {
        depth: Some(depth),
        operations,
        cause: Box::new(ConicalChartTrimPairFailure::Trace(ConicalTrimError {
            carrier: query.carrier,
            requested_spatial_limit: query.spatial_limit,
            requested_options: query.options,
            cause,
        })),
    }
}
pub(super) fn binding(
    depth: Option<usize>,
    operations: u32,
    field: &'static str,
    actual: Vec<u64>,
    expected: Vec<u64>,
) -> ConicalChartTrimPairError {
    ConicalChartTrimPairError {
        depth,
        operations,
        cause: Box::new(ConicalChartTrimPairFailure::Binding {
            field,
            actual,
            expected,
        }),
    }
}

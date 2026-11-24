// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use crate::predicates::roots::certificate_arithmetic::bounded_failure::BoundedArithmeticFailure;
use crate::predicates::roots::metric_bounds::CylindricalProfileFailure;

/// The rational contour term or final scalar publication that refused.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConicalReferenceVolumeTerm {
    /// sin(alpha) * delta(depth) * integral L² q'/(1+q²).
    SquaredSlant,
    /// Signed cos(alpha) * delta(depth²) * integral L q'/(1+q²).
    Slant,
    /// Final aggregate rounding and error acceptance.
    Publication,
}
/// Original typed refusal; no incomplete integral is exposed as successful evidence.
#[derive(Debug)]
pub enum ConicalReferenceVolumeFailure {
    /// Invalid finite input value or an unproved input range.
    Input {
        /// Input quantity and its required finite range.
        role: &'static str,
        /// Offending coefficient or depth position within that input.
        index: usize,
        /// Original supplied value, including its nonfinite or signed-zero bits.
        value: f64,
    },
    /// Invalid coefficient or occurrence collection.
    Shape {
        /// Coefficient or occurrence collection that failed admission.
        role: &'static str,
        /// Original supplied collection length.
        count: usize,
        /// Required nonempty count, matching shape, or requested degree/occurrence cap.
        expected: &'static str,
    },
    /// Unsupported original integration policy.
    Policy(CylindricalFluxOptions),
    /// Original bounded coefficient arithmetic failure.
    Arithmetic(Box<BoundedArithmeticFailure>),
    /// Original trigonometric enclosure or arithmetic-policy failure.
    Trigonometric(Box<CylindricalProfileFailure>),
    /// Exact coefficient, positive-denominator, work, cancellation or scalar-error refusal.
    Integration {
        /// Rational contour term or aggregate publication stage that refused.
        term: ConicalReferenceVolumeTerm,
        /// Original exact-coefficient, denominator, work, cancellation, or rounding failure.
        cause: Box<RationalIntegralFailure>,
    },
}
/// Source position, depth offender and actual completed work on failure.
#[derive(Debug)]
pub struct ConicalReferenceVolumeError {
    /// Local supplied span position, if the failure belongs to one span.
    pub span: Option<usize>,
    /// Actual depth endpoint index, if its admission/capture failed.
    pub depth: Option<usize>,
    /// Original typed cause.
    pub cause: Box<ConicalReferenceVolumeFailure>,
    /// Actual completed bounded coefficient operations.
    pub operations: u32,
    /// Actual aggregate examined quotient cells, including failed or cancelled work.
    pub integration_subintervals: u64,
    /// Original aggregate scalar policy.
    pub requested_options: CylindricalFluxOptions,
    /// Original coefficient arithmetic policy.
    pub requested_arithmetic: CylindricalProfileOptions,
}
impl ConicalReferenceVolumeError {
    pub(super) fn new(
        query: ConicalReferenceVolumeQuery<'_>,
        span: Option<usize>,
        depth: Option<usize>,
        work: (u32, u64),
        cause: ConicalReferenceVolumeFailure,
    ) -> Self {
        Self {
            span,
            depth,
            cause: Box::new(cause),
            operations: work.0,
            integration_subintervals: work.1,
            requested_options: query.options,
            requested_arithmetic: query.arithmetic,
        }
    }
}
impl std::fmt::Display for ConicalReferenceVolumeError {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            out,
            "reference volume span={:?}, depth={:?}, operations={}, cells={}: {}",
            self.span, self.depth, self.operations, self.integration_subintervals, self.cause
        )
    }
}
impl std::fmt::Display for ConicalReferenceVolumeFailure {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            out,
            "{self:?}; expected admitted finite fitted reference operands and complete bounds within the requested policies"
        )
    }
}
impl std::error::Error for ConicalReferenceVolumeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self.cause.as_ref())
    }
}
impl std::error::Error for ConicalReferenceVolumeFailure {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Integration { cause, .. } => Some(cause.as_ref()),
            _ => None,
        }
    }
}
pub(super) fn integration(
    query: ConicalReferenceVolumeQuery<'_>,
    span: Option<usize>,
    work: (u32, u64),
    term: ConicalReferenceVolumeTerm,
    cause: RationalIntegralFailure,
) -> ConicalReferenceVolumeError {
    ConicalReferenceVolumeError::new(
        query,
        span,
        None,
        work,
        ConicalReferenceVolumeFailure::Integration {
            term,
            cause: Box::new(cause),
        },
    )
}

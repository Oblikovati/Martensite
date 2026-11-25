// SPDX-License-Identifier: GPL-2.0-only
use super::*;
/// Exact retained-source refusal or original bounded whole-map failure.
#[derive(Clone, Debug, PartialEq)]
pub struct ConicalDevelopedMaterialError {
    /// Actual offending span index, or no span for whole-domain failures.
    pub span: Option<usize>,
    /// Actual new arithmetic operations; earlier issued proofs are not recharged.
    pub operations: u32,
    /// Actual source operands or original numerical cause.
    pub cause: Box<ConicalDevelopedMaterialFailure>,
}
/// Fresh developed-material failures preserve legacy numerical enums unchanged.
#[derive(Clone, Debug, PartialEq)]
pub enum ConicalDevelopedMaterialFailure {
    /// Issued authority differs from the actual requested operands.
    Binding {
        /// Refused operand's expected role and shape.
        field: &'static str,
        /// Actual raw scalar bits, source order or counts.
        actual: Vec<u64>,
        /// Required issued operand bits, order or counts.
        expected: Vec<u64>,
    },
    /// Original whole-map arithmetic or geometric refusal.
    Numerical(CylindricalProfileFailure),
}
impl ConicalDevelopedMaterialError {
    /// Preserve the original bounded cancellation classification; `error.is_cancelled()`.
    pub fn is_cancelled(&self) -> bool {
        matches!(self.cause.as_ref(), ConicalDevelopedMaterialFailure::Numerical(CylindricalProfileFailure::Arithmetic(cause))
            if matches!(cause.as_ref(), BoundedArithmeticFailure::Cancelled))
    }
}
impl std::fmt::Display for ConicalDevelopedMaterialError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "developed material span={:?}, work={}: {:?}",
            self.span, self.operations, self.cause
        )
    }
}
impl std::error::Error for ConicalDevelopedMaterialError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self.cause.as_ref() {
            ConicalDevelopedMaterialFailure::Numerical(cause) => Some(cause),
            ConicalDevelopedMaterialFailure::Binding { .. } => None,
        }
    }
}
pub(in super::super) fn failed(
    span: Option<usize>,
    cause: ConicalDevelopedMaterialFailure,
    operations: u32,
) -> ConicalDevelopedMaterialError {
    ConicalDevelopedMaterialError {
        span,
        cause: Box::new(cause),
        operations,
    }
}
pub(in super::super) fn numerical(
    span: Option<usize>,
    cause: CylindricalProfileFailure,
    operations: u32,
) -> ConicalDevelopedMaterialError {
    failed(
        span,
        ConicalDevelopedMaterialFailure::Numerical(cause),
        operations,
    )
}
pub(in super::super) fn binding(
    span: Option<usize>,
    field: &'static str,
    actual: Vec<u64>,
    expected: Vec<u64>,
) -> ConicalDevelopedMaterialError {
    failed(
        span,
        ConicalDevelopedMaterialFailure::Binding {
            field,
            actual,
            expected,
        },
        0,
    )
}

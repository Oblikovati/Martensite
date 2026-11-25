// SPDX-License-Identifier: GPL-2.0-only
//! Typed source, coverage and resource refusals preserve actual operands.
use super::*;
/// Stage whose complete numeric certificate was not established.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConicalActualPatchStage {
    /// Source, authority or policy admission.
    Binding,
    /// Family-wide directed displacement.
    Gamma,
    /// Positive rational source capture and C1 transfer.
    Capture,
    /// Nonincident side-pair hierarchy.
    Hierarchy,
    /// Whole adjacent-union injectivity.
    Adjacent,
    /// Uniform family side/carrier barriers.
    SideCap,
    /// Filled-cap envelope and cap-pair separation.
    Cap,
}
/// Original numerical or actual-operand reason a certificate was refused.
#[derive(Clone, Debug, PartialEq)]
pub enum ConicalActualPatchFailure {
    /// The requested arithmetic policy is outside its existing supported range.
    Policy {
        /// Supplied requested policy.
        actual: ConeClearanceOptions,
    },
    /// Source collection does not have the required shape.
    Shape {
        /// Collection or field.
        field: &'static str,
        /// Supplied count.
        actual: usize,
        /// Required shape.
        expected: &'static str,
    },
    /// Complete source or authority bits differ.
    Binding {
        /// Failed source field.
        field: &'static str,
        /// Supplied complete offending bits.
        actual: Vec<u64>,
        /// Expected complete bits.
        expected: Vec<u64>,
    },
    /// A scalar is unsupported or not finite.
    Value {
        /// Scalar field.
        field: &'static str,
        /// Supplied scalar.
        actual: f64,
        /// Required scalar domain.
        expected: &'static str,
    },
    /// Whole-domain bounds did not establish a required strict inequality.
    Unresolved {
        /// Failed quantity.
        quantity: &'static str,
        /// Actual certified interval lower endpoint.
        lower: DyadicBound,
        /// Actual certified interval upper endpoint.
        upper: DyadicBound,
        /// Required inequality.
        expected: &'static str,
    },
    /// A nonincident pair could not be strictly separated by its complete hulls.
    UnresolvedSidePair {
        /// Actual source labels.
        sides: [ConicalActualSideId; 2],
        /// Actual lower control-hull endpoints of both sides.
        lower: Box<[[DyadicBound; 3]; 2]>,
        /// Actual upper control-hull endpoints of both sides.
        upper: Box<[[DyadicBound; 3]; 2]>,
        /// Required separation rule.
        expected: &'static str,
    },
    /// Named actual side pair retaining the original bound or numerical cause.
    SidePair {
        /// Actual source side labels.
        sides: [ConicalActualSideId; 2],
        /// Original typed cause.
        cause: Box<ConicalActualPatchFailure>,
    },
    /// Named actual cap pair retaining the original bound or numerical cause.
    CapPair {
        /// Actual source cap labels.
        caps: [ConicalActualCapId; 2],
        /// Original typed cause.
        cause: Box<ConicalActualPatchFailure>,
    },
    /// Existing charged arithmetic failed or was cancelled.
    Arithmetic(BoundedArithmeticFailure),
}
/// Complete refusal with exact completed work and original requested policy.
#[derive(Clone, Debug, PartialEq)]
pub struct ConicalActualPatchError {
    /// Failed certificate stage.
    pub stage: ConicalActualPatchStage,
    /// Source side when the failure is local.
    pub side: Option<ConicalActualSideId>,
    /// Source cap when the failure is local.
    pub cap: Option<ConicalActualCapId>,
    /// Actual completed work; no partial certificate is returned.
    pub operations: u32,
    /// Actual requested remaining grant.
    pub options: ConeClearanceOptions,
    /// Original typed cause.
    pub cause: Box<ConicalActualPatchFailure>,
}
impl ConicalActualPatchError {
    /// Whether the original cause was cancellation; `error.is_cancelled()`.
    pub fn is_cancelled(&self) -> bool {
        self.cause.cancelled()
    }
}
impl std::fmt::Display for ConicalActualPatchFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "actual conical patch refusal: {self:?}")
    }
}
impl ConicalActualPatchFailure {
    fn cancelled(&self) -> bool {
        match self {
            Self::Arithmetic(BoundedArithmeticFailure::Cancelled) => true,
            Self::SidePair { cause, .. } | Self::CapPair { cause, .. } => cause.cancelled(),
            _ => false,
        }
    }
}
impl std::error::Error for ConicalActualPatchFailure {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::SidePair { cause, .. } | Self::CapPair { cause, .. } => Some(cause.as_ref()),
            _ => None,
        }
    }
}
impl std::fmt::Display for ConicalActualPatchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} side={:?} cap={:?} operations={}: {}",
            self.stage, self.side, self.cap, self.operations, self.cause
        )
    }
}
impl std::error::Error for ConicalActualPatchError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self.cause.as_ref())
    }
}
impl From<BoundedArithmeticFailure> for ConicalActualPatchFailure {
    fn from(cause: BoundedArithmeticFailure) -> Self {
        Self::Arithmetic(cause)
    }
}
pub(super) type Outcome<T> = Result<T, ConicalActualPatchFailure>;
pub(super) fn binding(
    field: &'static str,
    actual: Vec<u64>,
    expected: Vec<u64>,
) -> ConicalActualPatchFailure {
    ConicalActualPatchFailure::Binding {
        field,
        actual,
        expected,
    }
}
pub(super) fn shape(
    field: &'static str,
    actual: usize,
    expected: &'static str,
) -> ConicalActualPatchFailure {
    ConicalActualPatchFailure::Shape {
        field,
        actual,
        expected,
    }
}
pub(super) fn unresolved(
    quantity: &'static str,
    value: &Interval,
    expected: &'static str,
    bits: u16,
) -> ConicalActualPatchFailure {
    let exponent = -i32::from(bits);
    ConicalActualPatchFailure::Unresolved {
        quantity,
        lower: DyadicBound::new(value.lower.clone(), exponent),
        upper: DyadicBound::new(value.upper.clone(), exponent),
        expected,
    }
}
pub(super) fn failed(
    cause: ConicalActualPatchFailure,
    stage: ConicalActualPatchStage,
    options: ConeClearanceOptions,
    operations: u32,
) -> ConicalActualPatchError {
    ConicalActualPatchError {
        stage,
        side: match &cause {
            ConicalActualPatchFailure::SidePair { sides, .. }
            | ConicalActualPatchFailure::UnresolvedSidePair { sides, .. } => Some(sides[0]),
            _ => None,
        },
        cap: match &cause {
            ConicalActualPatchFailure::CapPair { caps, .. } => Some(caps[0]),
            _ => None,
        },
        operations,
        options,
        cause: Box::new(cause),
    }
}

#[derive(Default)]
pub(super) struct FailurePosition {
    pub(super) side: Option<ConicalActualSideId>,
    pub(super) cap: Option<ConicalActualCapId>,
}
impl FailurePosition {
    pub(super) fn clear(&mut self) {
        self.side = None;
        self.cap = None;
    }
    pub(super) fn side(&mut self, id: ConicalActualSideId) {
        self.side = Some(id);
        self.cap = None;
    }
    pub(super) fn cap(&mut self, id: ConicalActualCapId) {
        self.cap = Some(id);
    }
    pub(super) fn error(
        &self,
        cause: ConicalActualPatchFailure,
        stage: ConicalActualPatchStage,
        options: ConeClearanceOptions,
        operations: u32,
    ) -> ConicalActualPatchError {
        let mut error = failed(cause, stage, options, operations);
        error.side = self.side.or(error.side);
        error.cap = self.cap.or(error.cap);
        error
    }
}

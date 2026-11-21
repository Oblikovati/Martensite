// SPDX-License-Identifier: GPL-2.0-only
use super::*;

/// An uncompleted exclusion retains policy errors or the original source/domain cause.
#[derive(Debug)]
pub enum BezierPairExclusionFailure {
    /// Unsupported exclusion behavior; expected one.
    Version(u32),
    /// Unsupported nested pair behavior; expected one or two.
    PairPolicyVersion(u32),
    /// Original unsupported sign-policy version.
    SignPolicy(BezierPairSignFailure),
    /// Original source, denominator, exact-algebra, work or cancellation failure.
    Operation(Box<BezierPairOperationFailure>),
}
impl BezierPairExclusionFailure {
    /// Read the original operation cause through domain context; `failure.original_cause()`.
    pub fn original_cause(&self) -> Option<&BezierPairOperationFailure> {
        match self {
            Self::Operation(cause) => Some(cause.original_cause()),
            _ => None,
        }
    }
    pub(super) fn completion(&self) -> BezierPairExclusionCompletion {
        use BezierPairExclusionCompletion as C;
        let Self::Operation(cause) = self else {
            return C::Invalid;
        };
        match super::super::completion::operation_completion(cause) {
            BezierPairCompletion::Invalid => C::Invalid,
            BezierPairCompletion::Cancelled => C::Cancelled,
            BezierPairCompletion::Incomplete | BezierPairCompletion::Complete => C::Incomplete,
        }
    }
}
impl From<BezierPairOperationFailure> for BezierPairExclusionFailure {
    fn from(cause: BezierPairOperationFailure) -> Self {
        Self::Operation(Box::new(cause))
    }
}
impl From<RootIsolationError> for BezierPairExclusionFailure {
    fn from(cause: RootIsolationError) -> Self {
        BezierPairOperationFailure::Root(cause).into()
    }
}
impl std::fmt::Display for BezierPairExclusionFailure {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            out,
            "Bernstein exclusion: {self:?}; expected valid source domains, behavior 1, nested pair behavior 1/2 and sign behavior 1"
        )
    }
}
impl std::error::Error for BezierPairExclusionFailure {}

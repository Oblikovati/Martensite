// SPDX-License-Identifier: GPL-2.0-only
use super::*;
/// Geometric completion does not imply that any algebraic root solver ran.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BezierPairGeometricCompletion {
    /// The complete queried contact set has a geometric witness.
    Complete,
    /// Unsupported source or absent witness requires a genuine fallback.
    Inconclusive,
    /// The caller stopped before complete coverage existed.
    Cancelled,
    /// Source or requested policy admission failed.
    Invalid,
}
/// Actual geometric method, separate from the algebraic solver's proof kinds.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BezierPairGeometricKind {
    /// Strictly disjoint coordinate hull intervals.
    BoxExclusion,
    /// A strict monotone projection excludes off-diagonal self contacts.
    MonotoneSelf,
    /// Opposite open sectors leave exactly one shared endpoint contact.
    EndpointSector,
    /// An exact supporting line strictly separates the control hulls.
    HullExclusion,
}
/// Original invalid request or cancellation, without a partial certificate.
#[derive(Debug)]
pub enum BezierPairGeometricFailure {
    /// Unsupported pair policy; expected one or two.
    PairPolicyVersion(u32),
    /// Unsupported generated-sign policy, even though signs are not evaluated here.
    SignPolicy(BezierPairSignFailure),
    /// Original root policy or interval admission, or cancellation.
    Root(RootIsolationError),
    /// Actual source slot and original finite-coordinate/weight failure.
    Source {
        /// Captured input curve ordinal.
        source_index: usize,
        /// Original typed source failure.
        cause: BezierPairCaptureFailure,
    },
    /// Original nonfinite or nonpositive construction request.
    WorldRequest(f64),
}
impl From<RootIsolationError> for BezierPairGeometricFailure {
    fn from(cause: RootIsolationError) -> Self {
        Self::Root(cause)
    }
}
impl std::fmt::Display for BezierPairGeometricFailure {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            out,
            "geometric pair: {self:?}; expected finite sources and valid pair/root/sign/world policies"
        )
    }
}
impl std::error::Error for BezierPairGeometricFailure {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Root(cause) => Some(cause),
            Self::Source { cause, .. } => Some(cause),
            _ => None,
        }
    }
}

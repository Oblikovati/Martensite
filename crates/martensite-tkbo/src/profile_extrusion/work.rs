// SPDX-License-Identifier: GPL-2.0-only
//! Explicit computational work is independent of geometric accuracy.
use super::{OperationDecline, PlanarProofWorkReceipt, PlanarRegionSet, ProfileCurveId};

/// Select the exact planar proof behavior while retaining earlier replay semantics.
/// Example: `PlanarProofVersion::BernsteinV3` enables source-bound exact exclusions.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum PlanarProofVersion {
    /// Released common-carrier and pair solver behavior.
    CarrierV2,
    /// Exact Bernstein exclusion followed by that solver under the remaining budget.
    BernsteinV3,
}
impl PlanarProofVersion {
    pub(crate) fn request_version(self) -> u32 {
        match self {
            Self::CarrierV2 => 2,
            Self::BernsteinV3 => 3,
        }
    }
}

/// Original root-isolation refusal and its authored parent curves.
/// Example: `failure.pair_root_failure().map(|pair| pair.curves)`.
#[derive(Clone, Debug, PartialEq)]
pub struct PlanarPairRootFailure {
    /// Authored parents; multiple proof pieces may belong to the same curve.
    pub curves: [ProfileCurveId; 2],
    /// Actual retained numerical cause, never inferred from spent work.
    pub cause: martensite_tkmath::predicates::roots::RootIsolationError,
}

/// Finite shared pair-work choice; zero permits only completion without subdivision.
/// Example: `PlanarProofWorkLimit::PairBisections(8192)`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlanarProofWorkLimit {
    /// Retain the existing graph policy; the observed receipt records its numeric limit.
    CurrentDefault,
    /// Apply exactly this shared bisection limit without changing other proof settings.
    PairBisections(u16),
}
impl PlanarProofWorkLimit {
    pub(crate) fn override_limit(self) -> Option<u16> {
        match self {
            Self::CurrentDefault => None,
            Self::PairBisections(maximum) => Some(maximum),
        }
    }
}
/// Owned typed decline and work evidence, with no partial geometry.
#[derive(Clone, Debug, PartialEq)]
pub struct PlanarRegionWorkFailure {
    decline: OperationDecline,
    requested: PlanarProofWorkLimit,
    observed: Option<PlanarProofWorkReceipt>,
    root_failure: Option<Box<PlanarPairRootFailure>>,
    cancelled: bool,
}
impl PlanarRegionWorkFailure {
    /// Existing operation, stage, entities and cause; `failure.decline().class()`.
    pub fn decline(&self) -> &OperationDecline {
        &self.decline
    }
    /// Original work choice; `failure.requested_work()`.
    pub fn requested_work(&self) -> PlanarProofWorkLimit {
        self.requested
    }
    /// Actual retained work, when observed; `failure.proof_work().is_some()`.
    /// Absence does not assert zero consumed work.
    pub fn proof_work(&self) -> Option<PlanarProofWorkReceipt> {
        self.observed
    }
    /// Original pair root refusal when present; `failure.pair_root_failure()`.
    pub fn pair_root_failure(&self) -> Option<&PlanarPairRootFailure> {
        self.root_failure.as_deref()
    }
    /// Whether the caller cancelled the operation; `failure.is_cancelled()`.
    pub fn is_cancelled(&self) -> bool {
        self.cancelled
    }
    pub(crate) fn with_cancelled(mut self, cancelled: bool) -> Self {
        self.cancelled = cancelled;
        self
    }
    pub(crate) fn with_root_failure(mut self, root_failure: Option<PlanarPairRootFailure>) -> Self {
        self.root_failure = root_failure.map(Box::new);
        self
    }
    pub(crate) fn new(
        requested: PlanarProofWorkLimit,
        decline: OperationDecline,
        observed: Option<PlanarProofWorkReceipt>,
    ) -> Self {
        Self {
            decline,
            requested,
            observed,
            root_failure: None,
            cancelled: false,
        }
    }
    pub(crate) fn into_decline(self) -> OperationDecline {
        self.decline
    }
}
impl std::fmt::Display for PlanarRegionWorkFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}; pair work requested={:?}, observed={:?}",
            self.decline, self.requested, self.observed
        )
    }
}
impl std::error::Error for PlanarRegionWorkFailure {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.decline)
    }
}
/// Complete planar regions or a typed work-bearing failure.
pub type PlanarRegionWorkResult = Result<PlanarRegionSet, PlanarRegionWorkFailure>;

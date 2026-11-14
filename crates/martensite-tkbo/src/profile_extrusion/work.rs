// SPDX-License-Identifier: GPL-2.0-only
//! Explicit computational work is independent of geometric accuracy.
use super::{OperationDecline, PlanarProofWorkReceipt, PlanarRegionSet};

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
    pub(crate) fn new(
        requested: PlanarProofWorkLimit,
        decline: OperationDecline,
        observed: Option<PlanarProofWorkReceipt>,
    ) -> Self {
        Self {
            decline,
            requested,
            observed,
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

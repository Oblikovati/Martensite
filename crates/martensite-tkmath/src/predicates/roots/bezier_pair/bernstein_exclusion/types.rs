// SPDX-License-Identifier: GPL-2.0-only
use super::*;

/// Explicit exclusion behavior and the unchanged policy available to a caller-owned fallback.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BezierPairExclusionPolicy {
    /// Exclusion behavior version; only one is supported.
    pub version: u32,
    /// Original pair behavior 1 or 2, root/sign admission and world-error request.
    pub pair_policy: BezierPairPolicy,
}
/// Exclusion completion is separate from full pair-solver completion.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BezierPairExclusionCompletion {
    /// A strict exact sign certifies an empty queried intersection domain.
    Excluded,
    /// Neither equation supplied the required strict witness; no contact conclusion follows.
    Inconclusive,
    /// Invalid source, interval or requested policy.
    Invalid,
    /// The caller stopped before a complete witness existed.
    Cancelled,
    /// Denominator, root-work or other required proof did not complete.
    Incomplete,
}
/// Owned source-bound attempt; a failed or inconclusive attempt never exposes a partial witness.
#[derive(Debug)]
pub struct BezierPairExclusionReport {
    pub(super) source: CapturedBezierPair,
    pub(super) policy: BezierPairExclusionPolicy,
    pub(super) work: BezierPairWork,
    pub(super) outcome: Result<Option<Box<ExclusionProof>>, BezierPairExclusionFailure>,
}
impl BezierPairExclusionReport {
    /// Read the complete original source capture; `report.source().captured_curves()`.
    pub fn source(&self) -> &CapturedBezierPair {
        &self.source
    }
    /// Read the unchanged nested request; `report.policy().pair_policy.version`.
    pub fn policy(&self) -> BezierPairExclusionPolicy {
        self.policy
    }
    /// Actual completed denominator-isolation splits on every outcome; `report.work().completed`.
    pub fn work(&self) -> BezierPairWork {
        self.work
    }
    /// Distinguish exclusion, lack of a witness and failure; `report.completion()`.
    pub fn completion(&self) -> BezierPairExclusionCompletion {
        match &self.outcome {
            Ok(Some(_)) => BezierPairExclusionCompletion::Excluded,
            Ok(None) => BezierPairExclusionCompletion::Inconclusive,
            Err(cause) => cause.completion(),
        }
    }
    /// Borrow a complete strict witness only; `report.complete().map(|proof| proof.coordinate_axis())`.
    pub fn complete(&self) -> Option<BezierPairBernsteinExclusion<'_>> {
        Some(BezierPairBernsteinExclusion::new(
            self.outcome.as_ref().ok()?.as_deref()?,
        ))
    }
    /// Read the original refusal; `report.failure().and_then(|cause| cause.original_cause())`.
    pub fn failure(&self) -> Option<&BezierPairExclusionFailure> {
        self.outcome.as_ref().err()
    }
    /// Compare every source, domain and policy bit, including rejected inputs; `report.matches_source(&query, policy)`.
    pub fn matches_source(
        &self,
        query: &CapturedBezierPair,
        policy: BezierPairExclusionPolicy,
    ) -> bool {
        binding::same_source(&self.source, query) && binding::same_policy(self.policy, policy)
    }
}

// SPDX-License-Identifier: GPL-2.0-only
//! Conditioned exact leaves retain their actual evidence before the general solver.
use super::domain_certificate::CertifiedPairDomain;
use martensite_tkmath::predicates::roots::bezier_pair::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum ConditionedCompletion {
    Complete,
    Inconclusive,
    Cancelled,
    Invalid,
}
pub(crate) enum ConditionedStage {
    Bernstein(BezierPairExclusionReport),
    Geometric(BezierPairGeometricReport),
}
impl ConditionedStage {
    pub(super) fn source(&self) -> &CapturedBezierPair {
        match self {
            Self::Bernstein(report) => report.source(),
            Self::Geometric(report) => report.source(),
        }
    }
    pub(super) fn policy(&self) -> BezierPairExclusionPolicy {
        match self {
            Self::Bernstein(report) => report.policy(),
            Self::Geometric(report) => BezierPairExclusionPolicy {
                version: 2,
                pair_policy: report.policy(),
            },
        }
    }
    pub(super) fn work(&self) -> BezierPairWork {
        match self {
            Self::Bernstein(report) => report.work(),
            Self::Geometric(report) => report.work(),
        }
    }
    pub(super) fn completion(&self) -> ConditionedCompletion {
        use ConditionedCompletion as C;
        match self {
            Self::Bernstein(report) => match report.completion() {
                BezierPairExclusionCompletion::Excluded => C::Complete,
                BezierPairExclusionCompletion::Inconclusive => C::Inconclusive,
                BezierPairExclusionCompletion::Cancelled => C::Cancelled,
                _ => C::Invalid,
            },
            Self::Geometric(report) => match report.completion() {
                BezierPairGeometricCompletion::Complete => C::Complete,
                BezierPairGeometricCompletion::Inconclusive => C::Inconclusive,
                BezierPairGeometricCompletion::Cancelled => C::Cancelled,
                _ => C::Invalid,
            },
        }
    }
    pub(crate) fn complete(&self) -> Option<CertifiedPairDomain<'_>> {
        match self {
            Self::Bernstein(report) => report
                .complete()
                .map(|proof| CertifiedPairDomain::Excluded { _proof: proof }),
            Self::Geometric(report) => report.complete().map(CertifiedPairDomain::Geometric),
        }
    }
    pub(super) fn matches_source(
        &self,
        query: &CapturedBezierPair,
        policy: BezierPairExclusionPolicy,
    ) -> bool {
        self.policy() == policy
            && match self {
                Self::Bernstein(report) => report.matches_source(query, policy),
                Self::Geometric(report) => report.matches_source(query, policy.pair_policy),
            }
    }
    pub(super) fn failure(&self) -> Option<&BezierPairExclusionFailure> {
        match self {
            Self::Bernstein(report) => report.failure(),
            Self::Geometric(_) => None,
        }
    }
}

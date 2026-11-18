// SPDX-License-Identifier: GPL-2.0-only
//! Issue #539: retain the exact exclusion and genuine fallback as separate evidence.
use super::{outcome::*, paired_types::*};
use martensite_tkmath::predicates::roots::bezier_pair::*;

pub(crate) struct BernsteinStoredReport {
    pub sources: [usize; 2],
    pub mode: PairedQueryMode,
    pub effective: BezierPairPolicy,
    pub outcome: Result<BernsteinExecution, Box<BezierPairCaptureFailure>>,
}
pub(crate) struct BernsteinExecution {
    pub exclusion: BezierPairExclusionReport,
    pub fallback: Option<BezierPairReport>,
}
impl BernsteinStoredReport {
    pub(super) fn completed_work(&self) -> u64 {
        self.outcome.as_ref().map_or(0, |execution| {
            u64::from(execution.exclusion.work().completed)
                + execution
                    .fallback
                    .as_ref()
                    .map_or(0, |report| u64::from(report.work().completed))
        })
    }
    pub(super) fn cancelled(&self) -> bool {
        self.outcome.as_ref().is_ok_and(|execution| {
            execution.exclusion.completion() == BezierPairExclusionCompletion::Cancelled
                || execution
                    .fallback
                    .as_ref()
                    .is_some_and(|report| report.completion() == BezierPairCompletion::Cancelled)
        })
    }
}
pub(super) fn evaluate(
    census: &mut BoundaryCensus,
    pair: [usize; 2],
    sources: [usize; 2],
    mode: PairedQueryMode,
    cancelled: &mut impl FnMut() -> bool,
) -> DomainEvidence {
    let Some(paired) = census.paired.as_mut() else {
        return DomainEvidence::Unsupported(CensusGap::BezierPair);
    };
    if let Some(report) = paired
        .exclusions
        .iter()
        .position(|record| record.sources == sources && record.mode == mode)
    {
        paired.reused_reports += 1;
        return reference(census, pair, report, true);
    }
    let record = execute(census, sources, mode, cancelled);
    census.bisections += record.completed_work();
    let Some(paired) = census.paired.as_mut() else {
        return DomainEvidence::Unsupported(CensusGap::BezierPair);
    };
    let report = paired.exclusions.len();
    paired.exclusions.push(record);
    reference(census, pair, report, false)
}
fn execute(
    census: &BoundaryCensus,
    sources: [usize; 2],
    mode: PairedQueryMode,
    cancelled: &mut impl FnMut() -> bool,
) -> BernsteinStoredReport {
    let effective = super::paired_execution::remaining_policy(census);
    let policy = BezierPairExclusionPolicy {
        version: census
            .paired
            .as_ref()
            .and_then(|paired| paired.exclusion_version)
            .unwrap_or(0),
        pair_policy: effective,
    };
    let outcome = super::paired_execution::captured_query(census, sources, mode)
        .map(|query| certify(&query, policy, cancelled));
    BernsteinStoredReport {
        sources,
        mode,
        effective,
        outcome,
    }
}
fn certify(
    query: &CapturedBezierPair,
    policy: BezierPairExclusionPolicy,
    cancelled: &mut impl FnMut() -> bool,
) -> BernsteinExecution {
    let exclusion = query.certify_bernstein_exclusion(policy, &mut *cancelled);
    let fallback =
        (exclusion.completion() == BezierPairExclusionCompletion::Inconclusive).then(|| {
            query.certify(
                remaining_after_exclusion(&exclusion, policy.pair_policy),
                cancelled,
            )
        });
    BernsteinExecution {
        exclusion,
        fallback,
    }
}
fn reference(
    census: &BoundaryCensus,
    pair: [usize; 2],
    report: usize,
    reused: bool,
) -> DomainEvidence {
    let Some(source_slots) = super::paired_execution::parameter_slots(census, pair) else {
        return DomainEvidence::Unsupported(CensusGap::MissingOccurrence {
            occurrences: pair,
            count: census.inputs.segments.len(),
        });
    };
    DomainEvidence::BernsteinReport {
        report,
        reused,
        source_slots,
    }
}

pub(super) fn remaining_after_exclusion(
    exclusion: &BezierPairExclusionReport,
    requested: BezierPairPolicy,
) -> BezierPairPolicy {
    let mut remaining = requested;
    remaining.roots.max_bisections = remaining
        .roots
        .max_bisections
        .saturating_sub(exclusion.work().completed);
    remaining
}

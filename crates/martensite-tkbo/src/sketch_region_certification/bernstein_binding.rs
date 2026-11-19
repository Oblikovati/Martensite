// SPDX-License-Identifier: GPL-2.0-only
//! Issue #539: validate source, proof choice and both work receipts before consumption.
use super::{domain_certificate::CertifiedPairDomain, outcome::*, paired_types::*};
use martensite_tkmath::predicates::roots::bezier_pair::*;

pub(super) fn certificate<'a>(
    census: &'a BoundaryCensus,
    domain: &BoundaryDomain,
    index: usize,
) -> Option<CertifiedPairDomain<'a>> {
    let paired = census.paired.as_ref()?;
    let record = paired.exclusions.get(index)?;
    if !binding_current(census, domain, record) {
        return None;
    }
    execution_certificate(record.outcome.as_ref().ok()?)
}
fn execution_certificate(
    execution: &super::bernstein_execution::BernsteinExecution,
) -> Option<CertifiedPairDomain<'_>> {
    if let Some(proof) = execution.exclusion.complete() {
        return Some(proof);
    }
    execution
        .fallback
        .as_ref()?
        .complete()
        .map(CertifiedPairDomain::Solver)
}
fn binding_current(
    census: &BoundaryCensus,
    domain: &BoundaryDomain,
    record: &super::bernstein_execution::BernsteinStoredReport,
) -> bool {
    super::paired_binding::parameter_binding_current(census, domain)
        && super::paired_binding::metadata_current(
            census,
            domain.occurrences,
            record.sources,
            record.mode,
        )
        && source_current(census, record)
}

fn source_current(
    census: &BoundaryCensus,
    record: &super::bernstein_execution::BernsteinStoredReport,
) -> bool {
    let Ok(query) = super::paired_execution::captured_query(census, record.sources, record.mode)
    else {
        return false;
    };
    let Ok(execution) = &record.outcome else {
        return false;
    };
    let Some(policy) = exclusion_policy(census, record.effective) else {
        return false;
    };
    execution.exclusion.matches_source(&query, policy)
        && execution
            .fallback
            .as_ref()
            .is_none_or(|report| report.source() == &query)
}
fn exclusion_policy(
    census: &BoundaryCensus,
    pair_policy: BezierPairPolicy,
) -> Option<BezierPairExclusionPolicy> {
    let version = census.paired.as_ref()?.exclusion_version?;
    Some(BezierPairExclusionPolicy {
        version,
        pair_policy,
    })
}

pub(super) fn work_exact(census: &BoundaryCensus, paired: &PairedCensusEvidence) -> bool {
    if !matches!(paired.exclusion_version, Some(1 | 2))
        || !paired.reports.is_empty()
        || !super::paired_binding::paired_policy_current(census, paired)
    {
        return false;
    }
    let mut used = 0;
    for record in &paired.exclusions {
        if !record_work_current(paired, record, used) {
            return false;
        }
        used += record.completed_work();
    }
    used == census.bisections && reuse_current(census, paired)
}
fn record_work_current(
    paired: &PairedCensusEvidence,
    record: &super::bernstein_execution::BernsteinStoredReport,
    used: u64,
) -> bool {
    let Some(remaining) = u64::from(paired.requested.roots.max_bisections).checked_sub(used) else {
        return false;
    };
    let mut expected = paired.requested;
    expected.roots.max_bisections = remaining as u16;
    record.effective == expected
        && record.completed_work() <= remaining
        && record.outcome.as_ref().map_or(true, |execution| {
            execution_work_current(execution, expected, paired.exclusion_version.unwrap_or(0))
        })
}
fn execution_work_current(
    execution: &super::bernstein_execution::BernsteinExecution,
    expected: BezierPairPolicy,
    version: u32,
) -> bool {
    let policy = BezierPairExclusionPolicy {
        version,
        pair_policy: expected,
    };
    if execution.exclusion.policy() != policy {
        return false;
    }
    let inconclusive = execution.exclusion.completion()
        == super::conditioned_stage::ConditionedCompletion::Inconclusive;
    if inconclusive != execution.fallback.is_some() {
        return false;
    }
    let remaining =
        super::bernstein_execution::remaining_after_exclusion(&execution.exclusion, expected);
    execution
        .fallback
        .as_ref()
        .is_none_or(|report| report.policy() == remaining)
}
fn reuse_current(census: &BoundaryCensus, paired: &PairedCensusEvidence) -> bool {
    let mut cold = vec![false; paired.exclusions.len()];
    let mut reused_count = 0;
    for domain in &census.domains {
        if matches!(domain.evidence, DomainEvidence::PairedReport { .. }) {
            return false;
        }
        let DomainEvidence::BernsteinReport { report, reused, .. } = domain.evidence else {
            continue;
        };
        if report >= cold.len() || reused != cold[report] {
            return false;
        }
        cold[report] = true;
        reused_count += u64::from(reused);
    }
    cold.iter().all(|seen| *seen) && reused_count == paired.reused_reports
}

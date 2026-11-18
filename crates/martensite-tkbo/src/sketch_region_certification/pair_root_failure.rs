// SPDX-License-Identifier: GPL-2.0-only
//! Copy an actual retained root refusal for one explicitly identified source pair.
use super::{BoundaryCensus, DomainEvidence};
use martensite_tkmath::predicates::roots::{RootIsolationError, bezier_pair::*};

pub(crate) fn pair_root_failure(
    census: &BoundaryCensus,
    sources: [u64; 2],
) -> Option<RootIsolationError> {
    let domain = retained_domain(census, sources)?;
    if let DomainEvidence::BernsteinReport { report, .. } = domain.evidence {
        let execution = census
            .paired
            .as_ref()?
            .exclusions
            .get(report)?
            .outcome
            .as_ref()
            .ok()?;
        if let Some(fallback) = &execution.fallback {
            return report_root_failure(fallback);
        }
        return operation_root_failure(execution.exclusion.failure()?.original_cause()?);
    }
    report_root_failure(retained_pair_report(census, domain)?)
}
fn report_root_failure(report: &BezierPairReport) -> Option<RootIsolationError> {
    let BezierPairFailure::Operation(cause) = report.failure()? else {
        return None;
    };
    operation_root_failure(cause.original_cause())
}
fn operation_root_failure(cause: &BezierPairOperationFailure) -> Option<RootIsolationError> {
    match cause {
        BezierPairOperationFailure::Root(cause) => Some(cause.clone()),
        _ => None,
    }
}
fn retained_domain(census: &BoundaryCensus, sources: [u64; 2]) -> Option<&super::BoundaryDomain> {
    let positions = [
        pair_source_position(census, sources[0])?,
        pair_source_position(census, sources[1])?,
    ];
    census.domains.iter().find(|domain| {
        domain.occurrences == positions || domain.occurrences == [positions[1], positions[0]]
    })
}
fn retained_pair_report<'a>(
    census: &'a BoundaryCensus,
    domain: &super::BoundaryDomain,
) -> Option<&'a BezierPairReport> {
    let DomainEvidence::PairedReport { report, .. } = domain.evidence else {
        return None;
    };
    let reports = &census.paired.as_ref()?.reports;
    reports.get(report)?.outcome.as_ref().ok()
}
fn pair_source_position(census: &BoundaryCensus, source: u64) -> Option<usize> {
    let mut positions = census
        .inputs
        .segments
        .iter()
        .enumerate()
        .filter(|(_, segment)| segment.parents.len() == 1 && segment.parents[0].source.0 == source);
    let (position, _) = positions.next()?;
    positions.next().is_none().then_some(position)
}

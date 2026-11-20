// SPDX-License-Identifier: GPL-2.0-only
//! Coverage is recomputed from the actual complete occurrence matrix, not a mutable gap vector.
use super::{census, outcome::*, paired_types::*};
use martensite_tkmath::predicates::roots::bezier_pair::*;
mod occurrence;

pub(super) fn refresh_obligations(census: &mut BoundaryCensus) {
    refresh_obligations_with_access(census, super::SourceAccess::Live);
}
pub(super) fn refresh_obligations_with_access(
    census: &mut BoundaryCensus,
    access: super::SourceAccess<'_>,
) {
    let complete = access
        .certificates(census)
        .is_some_and(|view| domains_with(census, &view));
    let work = paired_work_exact(census);
    census.missing.retain(|gap| {
        !(*gap == CertificationObligation::ExhaustivePairAndSelfDomains && complete
            || *gap == CertificationObligation::FailedWorkAccounting && work)
    });
}
pub(super) fn paired_domains_certified(census: &BoundaryCensus) -> bool {
    let Some(certificates) = DomainCertificates::new(census) else {
        return false;
    };
    domains_with(census, &certificates)
}
pub(super) fn domains_with(census: &BoundaryCensus, certificates: &DomainCertificates<'_>) -> bool {
    let expected = census::pair_domains(census.inputs.segments.len()).collect::<Vec<_>>();
    !expected.is_empty()
        && expected.len() == census.domains.len()
        && census.domains.iter().zip(expected).all(|(domain, pair)| {
            domain.occurrences == pair && certificates.unified_certificate(domain).is_some()
        })
}
pub(crate) fn domain_certificate<'a>(
    census: &'a BoundaryCensus,
    domain: &BoundaryDomain,
) -> Option<BezierPairCertificate<'a>> {
    DomainCertificates::new(census)?.certificate(domain)
}

/// Share scalar ledger/source bindings; native geometry remains live through aliased handles.
/// Issue #449: repeating the complete ledger validation for every domain is cubic in pair count.
pub(crate) struct DomainCertificates<'a> {
    census: &'a BoundaryCensus,
    source: super::source_status::CurrentSourceRepresentation<'a>,
    owned_valid_prefix: Option<usize>,
    #[cfg(test)]
    pub(crate) occurrence_visits: std::cell::Cell<[usize; 2]>,
    owned_current: Option<(bool, &'a crate::sketch_region_prism::OwnedGraphAccess<'a>)>,
}
impl<'a> DomainCertificates<'a> {
    /// Bind immutable evidence once; `DomainCertificates::new(&census)` never freezes native aliases.
    pub(crate) fn new(census: &'a BoundaryCensus) -> Option<Self> {
        if !paired_work_exact(census) {
            return None;
        }
        let source = super::source_status::CurrentSourceRepresentation::new(census)?;
        Some(Self {
            census,
            source,
            owned_current: None,
            owned_valid_prefix: None,
            #[cfg(test)]
            occurrence_visits: Default::default(),
        })
    }
    pub(super) fn validate_owned(
        &mut self,
        owner: &'a crate::sketch_region_prism::OwnedGraphAccess<'a>,
    ) {
        if !self.first_report_available() {
            return;
        }
        if owner.binds(self.census) {
            #[cfg(test)]
            let current = self
                .source
                .current_with_observer(|| owner.observed_capture());
            #[cfg(not(test))]
            let current = self.source.current();
            self.owned_valid_prefix = current.then(|| {
                occurrence::capture(self.census, &mut |kind| self.observe_occurrence(kind))
            });
            self.owned_current = Some((current, owner));
        }
    }
    fn first_report_available(&self) -> bool {
        self.census
            .domains
            .first()
            .and_then(|first| self.bound_certificate(first))
            .is_some()
    }
    fn current_source(&self) -> bool {
        match self.owned_current {
            Some((current, _owner)) => current,
            None => self.source.current(),
        }
    }
    pub(crate) fn current_obligations(&self) -> Vec<CertificationObligation> {
        if self.owned_current.is_none() {
            return self.census.current_obligations();
        }
        super::outcome::obligations_from(
            self.census,
            || self.current_source(),
            || domains_with(self.census, self),
        )
    }
    /// Borrow only genuine solver evidence; exact exclusions have no solver report.
    pub(crate) fn certificate(&self, domain: &BoundaryDomain) -> Option<BezierPairCertificate<'a>> {
        self.unified_certificate(domain)?.solver()
    }
    /// Consume a source-bound empty domain or a genuine solver certificate.
    pub(crate) fn unified_certificate(
        &self,
        domain: &BoundaryDomain,
    ) -> Option<super::domain_certificate::CertifiedPairDomain<'a>> {
        if !self.current_source() {
            return None;
        }
        self.bound_certificate(domain)
    }
    fn bound_certificate(
        &self,
        domain: &BoundaryDomain,
    ) -> Option<super::domain_certificate::CertifiedPairDomain<'a>> {
        if let DomainEvidence::BernsteinReport { report, .. } = domain.evidence {
            return super::bernstein_binding::certificate(self, domain, report);
        }
        self.solver_certificate(domain)
            .map(super::domain_certificate::CertifiedPairDomain::Solver)
    }
    fn solver_certificate(&self, domain: &BoundaryDomain) -> Option<BezierPairCertificate<'a>> {
        let DomainEvidence::PairedReport { report, .. } = domain.evidence else {
            return None;
        };
        let retained = self.census.paired.as_ref()?.reports.get(report)?;
        if !self.domain_binding_current(domain, retained) {
            return None;
        }
        let report = retained.outcome.as_ref().ok()?;
        (report.policy() == retained.effective)
            .then(|| report.complete())
            .flatten()
    }
    pub(super) fn census(&self) -> &'a BoundaryCensus {
        self.census
    }
    fn domain_binding_current(&self, domain: &BoundaryDomain, record: &PairedStoredReport) -> bool {
        if !parameter_binding_current(self.census, domain) {
            return false;
        }
        self.metadata_current(domain.occurrences, record.sources, record.mode)
            && report_source_current(self.census, record)
    }
    pub(super) fn metadata_current(
        &self,
        pair: [usize; 2],
        sources: [usize; 2],
        mode: PairedQueryMode,
    ) -> bool {
        current_sources(self.census, pair) == Some(sources)
            && super::paired_execution::query_mode(pair) == mode
            && pair.into_iter().all(|index| {
                occurrence::current_or_retained(
                    self.census,
                    index,
                    self.owned_valid_prefix,
                    &mut |kind| self.observe_occurrence(kind),
                )
            })
    }
    fn observe_occurrence(&self, _kind: occurrence::Predicate) {
        #[cfg(test)]
        {
            let slot = match _kind {
                occurrence::Predicate::Relation => 0,
                occurrence::Predicate::PhysicalPrefix => 1,
            };
            let mut visits = self.occurrence_visits.get();
            visits[slot] += 1;
            self.occurrence_visits.set(visits);
        }
    }
}
pub(super) fn paired_policy_current(
    census: &BoundaryCensus,
    paired: &PairedCensusEvidence,
) -> bool {
    selected_signs_current(paired)
        && paired.requested.signs == paired.selected_signs
        && paired.requested.roots == census.requested.roots
        && paired.requested.requested_world_error.to_bits()
            == census.requested.max_world_error.to_bits()
}
fn selected_signs_current(paired: &PairedCensusEvidence) -> bool {
    let mut expected = BezierPairSignOptions::derived();
    expected.version = paired.selected_signs.version;
    let mode = (
        paired.requested.version,
        paired.exclusion_version,
        expected.version,
    );
    matches!(
        mode,
        (1 | 2, None, 1) | (2, Some(1), 1 | 2) | (2, Some(2), 2)
    ) && paired.selected_signs == expected
}

fn current_sources(census: &BoundaryCensus, pair: [usize; 2]) -> Option<[usize; 2]> {
    let records = &census.paired.as_ref()?.occurrences;
    let mut sources = [
        records.get(pair[0])?.physical_source,
        records.get(pair[1])?.physical_source,
    ];
    sources.sort();
    Some(sources)
}
fn report_source_current(census: &BoundaryCensus, record: &PairedStoredReport) -> bool {
    let Ok(report) = &record.outcome else {
        return false;
    };
    let Ok(expected) = super::paired_execution::captured_query(census, record.sources, record.mode)
    else {
        return false;
    };
    report.source() == &expected
}
pub(super) fn paired_work_exact(census: &BoundaryCensus) -> bool {
    let Some(paired) = &census.paired else {
        return false;
    };
    if paired.exclusion_version.is_some() {
        return super::bernstein_binding::work_exact(census, paired);
    }
    if !paired.exclusions.is_empty() {
        return false;
    }
    let mut used = 0;
    for record in &paired.reports {
        if !effective_work_current(paired, record, used) {
            return false;
        }
        used += record.completed_work();
    }
    used == census.bisections
        && paired_policy_current(census, paired)
        && reuse_work_current(census, paired)
}
fn effective_work_current(
    paired: &PairedCensusEvidence,
    record: &PairedStoredReport,
    used: u64,
) -> bool {
    let Some(remaining) = u64::from(paired.requested.roots.max_bisections).checked_sub(used) else {
        return false;
    };
    let mut expected = paired.requested;
    expected.roots.max_bisections = remaining as u16;
    record.effective == expected
        && record.completed_work() <= remaining
        && record
            .outcome
            .as_ref()
            .map_or(true, |report| report.policy() == expected)
}
fn reuse_work_current(census: &BoundaryCensus, paired: &PairedCensusEvidence) -> bool {
    let mut cold = vec![false; paired.reports.len()];
    let (mut cold_count, mut reused_count) = (0, 0);
    for domain in &census.domains {
        let DomainEvidence::PairedReport { report, reused, .. } = domain.evidence else {
            continue;
        };
        if report >= cold.len() || reused != cold[report] {
            return false;
        }
        cold[report] = true;
        cold_count += usize::from(!reused);
        reused_count += u64::from(reused);
    }
    cold_count == paired.reports.len() && reused_count == paired.reused_reports
}

pub(super) fn restore_operation_obligations(current: &mut Vec<CertificationObligation>) {
    for gap in [
        CertificationObligation::MappedSplitCorrespondence,
        CertificationObligation::CandidateSearchAndCoverage,
        CertificationObligation::SelfIntersection,
        CertificationObligation::Tolerance,
    ] {
        if !current.contains(&gap) {
            current.push(gap);
        }
    }
}

pub(super) fn parameter_binding_current(census: &BoundaryCensus, domain: &BoundaryDomain) -> bool {
    let source_slots = match domain.evidence {
        DomainEvidence::PairedReport { source_slots, .. }
        | DomainEvidence::BernsteinReport { source_slots, .. } => source_slots,
        _ => return false,
    };
    Some(source_slots) == super::paired_execution::parameter_slots(census, domain.occurrences)
}

// SPDX-License-Identifier: GPL-2.0-only
//! Report cardinalities consume the existing source-bound domain and topology views.
use super::super::topology::incidence_view::{self, DiscoveryStep, ObservedIncidences};
use super::*;
use std::collections::BTreeSet;
#[cfg(test)]
mod tests;
pub(super) fn report_cardinality_gaps(
    retained: &PairedContactTransaction,
    proofs: Option<&crate::sketch_region_certification::DomainCertificates<'_>>,
    sources: Option<&super::super::topology::SourceIndex>,
) -> Vec<ContactTransportGap> {
    report_cardinality_gaps_observed(retained, proofs, sources, &mut |_| {})
}
fn report_cardinality_gaps_observed(
    retained: &PairedContactTransaction,
    proofs: Option<&crate::sketch_region_certification::DomainCertificates<'_>>,
    sources: Option<&super::super::topology::SourceIndex>,
    observe: &mut impl FnMut(DiscoveryStep),
) -> Vec<ContactTransportGap> {
    // #555: immutable source/domain views do not borrow the mutable diagnostic output.
    let live = if proofs.is_none() {
        crate::sketch_region_certification::DomainCertificates::new(&retained.census)
    } else {
        None
    };
    let certificates = proofs.or(live.as_ref());
    let observations = incidence_view::compile_observed(
        &retained.census.inputs,
        &retained.mapped.arrangement.partition,
        retained
            .current_partition
            .as_ref()
            .and_then(|row| row.as_ref().ok()),
        sources,
        observe,
    );
    collect_gaps(retained, certificates, observations.as_ref(), observe)
}
fn collect_gaps(
    retained: &PairedContactTransaction,
    certificates: Option<&crate::sketch_region_certification::DomainCertificates<'_>>,
    observations: Option<&ObservedIncidences<'_>>,
    observe: &mut impl FnMut(DiscoveryStep),
) -> Vec<ContactTransportGap> {
    let mut gaps = Vec::new();
    let mut seen = BTreeSet::new();
    for (index, domain) in retained.census.domains.iter().enumerate() {
        let Some(report) = unvisited_report(domain, &mut seen, observe) else {
            continue;
        };
        if let Some(gap) =
            current_cardinality_gap(retained, certificates, index, report, observations, observe)
        {
            gaps.push(gap);
        }
    }
    gaps
}
fn unvisited_report(
    domain: &crate::sketch_region_certification::BoundaryDomain,
    seen: &mut BTreeSet<usize>,
    observe: &mut impl FnMut(DiscoveryStep),
) -> Option<usize> {
    let report = paired_report_index(domain)?;
    observe(DiscoveryStep::ReportMembership);
    if !seen.insert(report) {
        return None;
    }
    Some(report)
}
fn paired_report_index(
    domain: &crate::sketch_region_certification::BoundaryDomain,
) -> Option<usize> {
    match domain.evidence {
        crate::sketch_region_certification::DomainEvidence::PairedReport { report, .. } => {
            Some(report)
        }
        _ => None,
    }
}
fn current_cardinality_gap(
    retained: &PairedContactTransaction,
    certificates: Option<&crate::sketch_region_certification::DomainCertificates<'_>>,
    domain_index: usize,
    report: usize,
    observations: Option<&ObservedIncidences<'_>>,
    observe: &mut impl FnMut(DiscoveryStep),
) -> Option<ContactTransportGap> {
    let domain = &retained.census.domains[domain_index];
    let proof = certificates?.certificate(domain)?;
    let observed = observations.map_or(0, |view| {
        super::super::topology::domain_axes(retained, domain_index)
            .map_or(0, |axes| view.cardinality(axes, observe))
    });
    (observed != proof.contact_count()).then_some(ContactTransportGap::ContactSetCardinality {
        report,
        expected: proof.contact_count(),
        observed,
    })
}

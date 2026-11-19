// SPDX-License-Identifier: GPL-2.0-only
//! Report cardinalities consume the existing source-bound domain and topology views.
use super::*;
pub(super) fn report_cardinality_gaps(
    retained: &PairedContactTransaction,
    proofs: Option<&crate::sketch_region_certification::DomainCertificates<'_>>,
    sources: Option<&super::super::topology::SourceIndex>,
) -> Vec<ContactTransportGap> {
    // #555: immutable source/domain views do not borrow the mutable diagnostic output.
    let mut gaps = Vec::new();
    let live = if proofs.is_none() {
        crate::sketch_region_certification::DomainCertificates::new(&retained.census)
    } else {
        None
    };
    let certificates = proofs.or(live.as_ref());
    let mut seen = Vec::new();
    for (index, domain) in retained.census.domains.iter().enumerate() {
        let Some(report) = unvisited_report(domain, &mut seen) else {
            continue;
        };
        if let Some(gap) = current_cardinality_gap(retained, certificates, index, report, sources) {
            gaps.push(gap);
        }
    }
    gaps
}
fn unvisited_report(
    domain: &crate::sketch_region_certification::BoundaryDomain,
    seen: &mut Vec<usize>,
) -> Option<usize> {
    let report = paired_report_index(domain)?;
    if seen.contains(&report) {
        return None;
    }
    seen.push(report);
    Some(report)
}
fn paired_report_index(
    domain: &crate::sketch_region_certification::BoundaryDomain,
) -> Option<usize> {
    match domain.evidence {
        crate::sketch_region_certification::DomainEvidence::PairedReport { report, .. }
        | crate::sketch_region_certification::DomainEvidence::BernsteinReport { report, .. } => {
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
    sources: Option<&super::super::topology::SourceIndex>,
) -> Option<ContactTransportGap> {
    let domain = &retained.census.domains[domain_index];
    let proof = certificates?.unified_certificate(domain)?;
    let rows = super::super::topology::observed_domain_vertices_with_index(
        retained,
        domain_index,
        sources,
    );
    let observed = unique_vertex_indices(&rows).len();
    (observed != proof.contact_count()).then_some(ContactTransportGap::ContactSetCardinality {
        report,
        expected: proof.contact_count(),
        observed,
    })
}
fn unique_vertex_indices(rows: &[ObservedContactVertex]) -> Vec<i32> {
    let mut indices = Vec::new();
    for row in rows {
        if !indices.contains(&row.vertex_index) {
            indices.push(row.vertex_index);
        }
    }
    indices
}

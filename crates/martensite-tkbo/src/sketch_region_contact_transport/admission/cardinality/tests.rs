// SPDX-License-Identifier: GPL-2.0-only
//! Old report traversal and full Cartesian native rows are an independent oracle.
use super::super::super::general_tests::{bridge_for, degenerate_fixture};
use super::super::super::test_fixture::actual_bridge;
use super::*;
use crate::sketch_region_certification::{BoundaryDomain, DomainCertificates, DomainEvidence};
type Cardinality = (usize, usize, usize);
fn result(gaps: Vec<ContactTransportGap>) -> Vec<Cardinality> {
    gaps.into_iter()
        .map(|gap| match gap {
            ContactTransportGap::ContactSetCardinality {
                report,
                expected,
                observed,
            } => (report, expected, observed),
            _ => panic!("cardinality gaps only"),
        })
        .collect()
}
fn literal(retained: &PairedContactTransaction) -> Vec<Cardinality> {
    let certificates = DomainCertificates::new(&retained.census).unwrap();
    let mut seen = Vec::new();
    let mut gaps = Vec::new();
    for (index, domain) in retained.census.domains.iter().enumerate() {
        let Some(report) = paired_report_index(domain) else {
            continue;
        };
        if seen.contains(&report) {
            continue;
        }
        seen.push(report);
        if let Some(gap) = literal_gap(retained, &certificates, index, report) {
            gaps.push(gap);
        }
    }
    gaps
}
fn literal_gap(
    retained: &PairedContactTransaction,
    certificates: &DomainCertificates<'_>,
    index: usize,
    report: usize,
) -> Option<Cardinality> {
    let proof = certificates.certificate(&retained.census.domains[index])?;
    let rows = super::super::super::topology::observed_domain_vertices(retained, index);
    let mut vertices = Vec::new();
    for row in rows {
        if !vertices.contains(&row.vertex_index) {
            vertices.push(row.vertex_index);
        }
    }
    (vertices.len() != proof.contact_count()).then_some((
        report,
        proof.contact_count(),
        vertices.len(),
    ))
}
fn compare(retained: &PairedContactTransaction) -> Vec<Cardinality> {
    let index = super::super::super::topology::compile_source_index(retained);
    let new = result(report_cardinality_gaps(retained, None, index.as_ref()));
    assert_eq!(new, literal(retained));
    new
}
#[test]
fn actual_complete_report_cardinalities_equal_legacy_replay() {
    let retained = actual_bridge();
    assert!(compare(&retained).is_empty());
}
#[test]
fn exact_empty_report_still_rejects_extra_observed_native_vertex() {
    let retained = bridge_for(degenerate_fixture(15));
    assert!(
        compare(&retained)
            .iter()
            .any(|(_, expected, observed)| *expected == 0 && *observed > 0)
    );
}
#[test]
fn missing_current_retains_ordered_missing_contact_gaps() {
    let mut retained = actual_bridge();
    retained.current_partition = None;
    assert!(!compare(&retained).is_empty());
}
#[test]
fn report_membership_visits_once_and_preserves_first_encounter_order() {
    let mut seen = BTreeSet::new();
    let mut visits = 0;
    let mut actual = Vec::new();
    for report in [3, 1, 3, 2, 1, 0] {
        let domain = BoundaryDomain {
            occurrences: [0, 1],
            evidence: DomainEvidence::PairedReport {
                report,
                reused: false,
                source_slots: [0, 1],
            },
        };
        actual.extend(unvisited_report(&domain, &mut seen, &mut |_| visits += 1));
    }
    assert_eq!(actual, vec![3, 1, 2, 0]);
    assert_eq!(visits, 6);
}
#[test]
fn repeated_report_queries_do_not_repeat_native_discovery() {
    let retained = actual_bridge();
    let index = super::super::super::topology::compile_source_index(&retained);
    let mut endpoints = 0;
    let mut products = 0;
    let mut reports = 0;
    let gaps =
        report_cardinality_gaps_observed(&retained, None, index.as_ref(), &mut |step| match step {
            DiscoveryStep::Endpoint => endpoints += 1,
            DiscoveryStep::ParameterProduct => products += 1,
            DiscoveryStep::ReportMembership => reports += 1,
            _ => {}
        });
    assert_eq!(result(gaps), literal(&retained));
    assert!(endpoints > 0);
    assert!(reports > 0);
    assert_eq!(products, 0);
    assert_eq!(endpoints, endpoint_count(&retained));
}

fn endpoint_count(retained: &PairedContactTransaction) -> usize {
    retained
        .current_partition
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap()
        .endpoints
        .as_ref()
        .unwrap()
        .endpoints
        .len()
}
#[test]
fn rejected_actual_source_domain_preserves_literal_cardinality_refusal() {
    let mut retained = actual_bridge();
    let current = retained
        .current_partition
        .as_mut()
        .unwrap()
        .as_mut()
        .unwrap();
    for row in &mut current.domains {
        row.decision=Err(crate::sketch_region_partition_bindings::PartitionSourceDomainCause::CopiedSourceChanged);
    }
    assert!(!compare(&retained).is_empty());
}

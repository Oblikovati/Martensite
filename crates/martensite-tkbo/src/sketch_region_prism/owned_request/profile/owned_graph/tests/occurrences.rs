// SPDX-License-Identifier: GPL-2.0-only
//! #572: owned occurrence reuse preserves the real graph reader's first refusal.
use super::*;
use crate::sketch_region_certification::DomainCertificates;
use std::cell::Cell;

fn prepared() -> super::super::super::PreparedProfile {
    super::super::super::pipeline::prepare_owned_profile(request(), &mut Continue)
        .unwrap_or_else(|_| panic!("fresh owned occurrence fixture preparation"))
}
fn compare_graph(prepared: &super::super::super::PreparedProfile) {
    let expected = prepared.inspect_plane_graph(|| false);
    let captures = Cell::new(0);
    let owner = OwnedGraphAccess::fresh(&prepared.request, &prepared.edges, &captures);
    let actual = super::super::super::regions::inspect_graph(
        &prepared.request,
        &prepared.edges,
        &prepared.bridge,
        || false,
        SourceAccess::Owned(&owner),
    );
    assert_eq!(actual, expected);
    assert_eq!(format!("{actual:?}"), format!("{expected:?}"));
}
#[test]
fn owned_occurrence_refusals_match_live_for_first_and_later_history_and_source_edits() {
    for index in [0, 2] {
        for mutation in 0..6 {
            let mut prepared = prepared();
            mutate_occurrence(&mut prepared.bridge.census, index, mutation);
            assert!(prepared.inspect_plane_graph(|| false).is_err());
            compare_graph(&prepared);
        }
    }
}
fn mutate_occurrence(census: &mut BoundaryCensus, index: usize, mutation: usize) {
    if mutation == 5 {
        census.paired.as_mut().unwrap().occurrences.truncate(index);
        return;
    }
    let record = &mut census.paired.as_mut().unwrap().occurrences[index];
    match mutation {
        0 => record.source.reversed = !record.source.reversed,
        1 => record.source.parents.clear(),
        2 => record.relations.clear(),
        3 => record.physical_source = usize::MAX,
        4 => census.inputs.segments[index].controls[0][0] += 0.25,
        _ => panic!("mutation={mutation}; expected 0..5"),
    }
}
#[test]
fn owned_occurrence_views_keep_per_domain_parameter_and_mode_checks() {
    let prepared = prepared();
    let census = &prepared.bridge.census;
    let captures = Cell::new(0);
    let owner = OwnedGraphAccess::fresh(&prepared.request, &prepared.edges, &captures);
    let owned = SourceAccess::Owned(&owner).certificates(census).unwrap();
    let live = DomainCertificates::new(census).unwrap();
    for domain in &census.domains {
        compare_domain(&owned, &live, domain);
    }
    assert_eq!(captures.get(), census.inputs.segments.len());
}

fn compare_domain(
    owned: &DomainCertificates<'_>,
    live: &DomainCertificates<'_>,
    domain: &crate::sketch_region_certification::BoundaryDomain,
) {
    let mut changed = crate::sketch_region_certification::BoundaryDomain {
        occurrences: domain.occurrences,
        evidence: domain.evidence,
    };
    changed.occurrences.swap(0, 1);
    let actual = owned
        .certificate(&changed)
        .map(|proof| proof.contact_count());
    let expected = live
        .certificate(&changed)
        .map(|proof| proof.contact_count());
    assert_eq!(actual, expected);
}

#[test]
fn owned_occurrence_real_view_counts_one_capture_and_no_repeated_history_or_prefix_work() {
    let prepared = prepared();
    let census = &prepared.bridge.census;
    let captures = Cell::new(0);
    let owner = OwnedGraphAccess::fresh(&prepared.request, &prepared.edges, &captures);
    let owned = SourceAccess::Owned(&owner).certificates(census).unwrap();
    let live = DomainCertificates::new(census).unwrap();
    assert_eq!(owned.occurrence_visits.get(), [72, 12]);
    repeated_reads(census, &owned, &live);
    assert_eq!(owned.occurrence_visits.get(), [72, 12]);
    assert_eq!(captures.get(), 4);
    assert!(live.occurrence_visits.get()[0] > 48);
    assert!(live.occurrence_visits.get()[1] > 10);
    eprintln!(
        r#"{{"event":"owned_occurrence_work","owned":{:?},"live":{:?}}}"#,
        owned.occurrence_visits.get(),
        live.occurrence_visits.get()
    );
}
fn repeated_reads(
    census: &BoundaryCensus,
    owned: &DomainCertificates<'_>,
    live: &DomainCertificates<'_>,
) {
    for _ in 0..3 {
        for domain in &census.domains {
            let actual = owned.certificate(domain).unwrap();
            let expected = live.certificate(domain).unwrap();
            assert_eq!(actual.contact_count(), expected.contact_count());
            assert_eq!(actual.sign_stage(), expected.sign_stage());
        }
    }
}
#[test]
fn owned_occurrence_flags_never_replace_first_or_later_report_source_mode_or_policy_checks() {
    for last in [false, true] {
        for mutation in 0..3 {
            let mut prepared = prepared();
            corrupt_report(&mut prepared.bridge.census, last, mutation);
            assert!(prepared.inspect_plane_graph(|| false).is_err());
            compare_graph(&prepared);
        }
    }
}
fn corrupt_report(census: &mut BoundaryCensus, last: bool, mutation: usize) {
    let records = &mut census.paired.as_mut().unwrap().reports;
    let index = if last { records.len() - 1 } else { 0 };
    let alternate = records
        .iter()
        .find(|row| row.mode != records[index].mode)
        .unwrap()
        .mode;
    let record = &mut records[index];
    match mutation {
        0 => record.sources[0] = usize::MAX,
        1 => record.mode = alternate,
        2 => record.effective.requested_world_error *= 2.,
        _ => panic!("mutation={mutation}; expected 0..3"),
    }
}

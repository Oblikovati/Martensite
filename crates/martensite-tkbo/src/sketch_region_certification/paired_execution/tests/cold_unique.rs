// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use std::collections::BTreeSet;

fn unique_fixture(case: u8) -> BoundaryCensus {
    let mut census = reset(paired(&curved_pair()));
    match case {
        0 => (),
        1 => census.inputs.segments[0].controls.clear(),
        2 => census.paired.as_mut().unwrap().requested.version = 99,
        _ => unreachable!(),
    }
    census
}
fn execute(
    census: BoundaryCensus,
    lookup: &mut impl ReportLookup,
    stop: usize,
) -> (Result<BoundaryCensus, CensusCause>, usize) {
    let mut polls = 0;
    let mut cancelled = || {
        polls += 1;
        polls == stop
    };
    let result = enumerate_with_lookup(
        census,
        &mut cancelled,
        super::super::super::SourceAccess::Live,
        lookup,
    );
    (result, polls)
}
#[test]
fn cold_unique_driver_matches_literal_proofs_failures_work_and_callbacks() {
    for case in 0..3 {
        let mut literal = LiteralLookup::default();
        let (old, old_polls) = execute(unique_fixture(case), &mut literal, usize::MAX);
        let census = unique_fixture(case);
        let mut index = ReportReuseIndex::capture(&census);
        assert!(matches!(index, ReportReuseIndex::ColdUnique));
        let (new, new_polls) = execute(census, &mut index, usize::MAX);
        assert_eq!(old_polls, new_polls);
        assert_unique_results(old, new, case);
        assert!(matches!(index, ReportReuseIndex::ColdUnique));
        assert_eq!(literal.visits, 3);
    }
}
fn assert_unique_results(
    old: Result<BoundaryCensus, CensusCause>,
    new: Result<BoundaryCensus, CensusCause>,
    case: u8,
) {
    let old = old.unwrap_or_else(|_| panic!("literal census cancelled"));
    let new = new.unwrap_or_else(|_| panic!("cold unique census cancelled"));
    compare_censuses(&old, &new);
    assert_eq!(new.paired.as_ref().unwrap().reused_reports, 0);
    assert_eq!(new.paired.as_ref().unwrap().reports.len(), 3);
    assert_disposition(&new, case);
}
#[test]
fn cold_unique_cancellation_preserves_partial_reports_and_first_stopped_domain() {
    let (_, total) = execute(unique_fixture(0), &mut LiteralLookup::default(), usize::MAX);
    for stop in [1, 2, 3, total / 2, total] {
        let (old, old_polls) = execute(unique_fixture(0), &mut LiteralLookup::default(), stop);
        let census = unique_fixture(0);
        let mut index = ReportReuseIndex::capture(&census);
        let (new, new_polls) = execute(census, &mut index, stop);
        assert_eq!(old_polls, new_polls);
        assert_eq!(result_snapshot(old), result_snapshot(new));
    }
}
#[test]
fn aliases_seeded_reports_and_incomplete_source_inventories_keep_the_map() {
    assert!(matches!(
        ReportReuseIndex::capture(&fresh()),
        ReportReuseIndex::IndexedMap(_)
    ));
    let mut census = unique_fixture(0);
    census.paired.as_mut().unwrap().occurrences.pop();
    assert!(matches!(
        ReportReuseIndex::capture(&census),
        ReportReuseIndex::IndexedMap(_)
    ));
    let census = paired(&curved_pair());
    assert!(matches!(
        ReportReuseIndex::capture(&census),
        ReportReuseIndex::IndexedMap(_)
    ));
    assert!(matches!(
        ReportReuseIndex::from_keys([]),
        ReportReuseIndex::IndexedMap(_)
    ));
}
#[test]
fn complete_enumeration_has_one_query_key_per_identity_mapped_source_pair() {
    for count in 0..33 {
        let mut keys = BTreeSet::new();
        for pair in census::pair_domains(count) {
            let key = (
                pair,
                matches!(query_mode(pair), PairedQueryMode::SelfOffDiagonal),
            );
            assert!(keys.insert(key), "duplicate complete-domain query {pair:?}");
        }
        assert_eq!(keys.len(), count * (count + 1) / 2);
    }
}

fn assert_disposition(census: &BoundaryCensus, case: u8) {
    let reports = &census.paired.as_ref().unwrap().reports;
    match case {
        0 => assert_eq!(
            super::super::super::paired_tests::complete_distinct(census)
                .contact(0)
                .unwrap()
                .point,
            [0.5, 0.25]
        ),
        1 => {
            assert!(reports[0].outcome.is_err());
            assert_eq!(reports[2].completion(), BezierPairCompletion::Complete);
        }
        2 => assert!(
            reports
                .iter()
                .all(|record| record.completion() == BezierPairCompletion::Invalid)
        ),
        _ => unreachable!(),
    }
}
#[test]
fn missing_metadata_and_nonidentity_source_permutations_keep_the_map() {
    let mut census = unique_fixture(0);
    let paired = census.paired.as_mut().unwrap();
    paired.occurrences[0].physical_source = 1;
    paired.occurrences[1].physical_source = 0;
    assert_indexed(&census);
    census.paired = None;
    assert_indexed(&census);
}
fn assert_indexed(census: &BoundaryCensus) {
    assert!(matches!(
        ReportReuseIndex::capture(census),
        ReportReuseIndex::IndexedMap(_)
    ));
}

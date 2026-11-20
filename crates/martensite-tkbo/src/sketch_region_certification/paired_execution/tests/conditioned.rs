// SPDX-License-Identifier: GPL-2.0-only
//! The selected conditioned store uses the same literal/indexed enumeration oracle.
use super::*;
use crate::sketch_region_certification::{self as certification, SourceAccess};
mod failures;
mod snapshot;

fn fresh(case: u8, aliases: bool) -> BoundaryCensus {
    let inputs = inputs(aliases);
    let policy = certification::tests::policy();
    let source = certification::source_binding_tests::source_policy();
    let census = match case {
        0 => certification::enumerate_exclusion_census(inputs, policy, source, || false),
        1 => certification::enumerate_filtered_exclusion_census(inputs, policy, source, || false),
        2 => certification::enumerate_geometric_census_with_access(
            inputs,
            policy,
            source,
            || false,
            SourceAccess::Live,
        ),
        3 => return clear(certification::geometric_tests::endpoint_census()),
        _ => panic!("case={case}; expected 0..4"),
    }
    .unwrap_or_else(|_| panic!("actual conditioned fixture"));
    clear(census)
}
fn inputs(aliases: bool) -> certification::BoundaryInputs {
    let mut shapes = curved_pair();
    if aliases {
        shapes.push(shapes[0].reversed());
    }
    certification::tests::capture(&shapes)
}
fn clear(mut census: BoundaryCensus) -> BoundaryCensus {
    census = reset(census);
    census.paired.as_mut().unwrap().exclusions.clear();
    census
}
fn execute(
    census: BoundaryCensus,
    lookup: &mut impl ReportLookup,
    stop: usize,
) -> (Result<BoundaryCensus, CensusCause>, Vec<usize>) {
    let mut trace = Vec::new();
    let result = enumerate_with_lookup(
        census,
        &mut || {
            trace.push(trace.len() + 1);
            trace.len() == stop
        },
        SourceAccess::Live,
        lookup,
    );
    (result, trace)
}
#[test]
fn conditioned_cold_unique_matches_full_literal_outcomes_slots_and_callbacks() {
    for case in 0..4 {
        compare_complete(case, false);
    }
}
#[test]
fn conditioned_aliases_keep_map_reuse_and_full_source_slot_order() {
    for case in 0..3 {
        compare_complete(case, true);
    }
}
fn compare_complete(case: u8, aliases: bool) {
    let (old, old_trace) = execute(
        fresh(case, aliases),
        &mut LiteralLookup::default(),
        usize::MAX,
    );
    let census = fresh(case, aliases);
    let mut lookup = ReportReuseIndex::capture(&census);
    assert_eq!(matches!(lookup, ReportReuseIndex::ColdUnique), !aliases);
    let (new, new_trace) = execute(census, &mut lookup, usize::MAX);
    assert_eq!(old_trace, new_trace);
    let old = old.unwrap_or_else(|_| panic!("literal complete census cancelled"));
    let new = new.unwrap_or_else(|_| panic!("indexed complete census cancelled"));
    assert_complete(&new, case, aliases);
    assert_eq!(snapshot::census_rows(&old), snapshot::census_rows(&new));
    assert_eq!(matches!(lookup, ReportReuseIndex::ColdUnique), !aliases);
}
#[test]
fn conditioned_cancelled_partial_ledgers_match_literal_first_domain_and_poll_sequence() {
    for case in 0..4 {
        let (_, trace) = execute(
            fresh(case, false),
            &mut LiteralLookup::default(),
            usize::MAX,
        );
        compare_cancelled(case, trace.len());
    }
}
fn compare_cancelled(case: u8, total: usize) {
    for stop in [1, 2, 3, total / 2, total] {
        let (old, old_trace) = execute(fresh(case, false), &mut LiteralLookup::default(), stop);
        let census = fresh(case, false);
        let mut lookup = ReportReuseIndex::capture(&census);
        let (new, new_trace) = execute(census, &mut lookup, stop);
        assert_eq!(old_trace, new_trace);
        assert_eq!(snapshot::result(old), snapshot::result(new));
    }
}

fn assert_complete(census: &BoundaryCensus, case: u8, aliases: bool) {
    let paired = census.paired.as_ref().unwrap();
    assert!(paired.reports.is_empty());
    assert_eq!(paired.exclusions.len(), 3);
    assert_eq!(paired.reused_reports, if aliases { 2 } else { 0 });
    certification::bernstein_tests::assert_complete_crossing_domain(census);
    assert_eq!(conditioned_proof(&paired.exclusions[0]).contact_count(), 0);
    let record = &paired.exclusions[1];
    if case == 3 {
        let proof = conditioned_proof(record);
        assert_eq!(proof.contact_count(), 1);
        assert_eq!(proof.contact(0).unwrap().exact_endpoints(), Some([1, 0]));
        return;
    }
    certification::bernstein_tests::assert_crossing_fallback(record);
}

fn conditioned_proof(
    record: &certification::bernstein_execution::BernsteinStoredReport,
) -> certification::CertifiedPairDomain<'_> {
    record
        .outcome
        .as_ref()
        .unwrap()
        .exclusion
        .complete()
        .unwrap()
}

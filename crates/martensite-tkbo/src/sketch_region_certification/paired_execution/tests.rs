// SPDX-License-Identifier: GPL-2.0-only
//! The old literal lookup drives the same numerical calls as the indexed implementation.
use super::*;
use crate::sketch_region_certification::paired_tests::{curved_pair, paired};
mod cold_unique;
mod replay;
use replay::{compare_censuses, result_snapshot};

#[derive(Default)]
struct LiteralLookup {
    records: Vec<([usize; 2], PairedQueryMode)>,
    visits: usize,
}
impl ReportLookup for LiteralLookup {
    fn find(&mut self, sources: [usize; 2], mode: PairedQueryMode) -> Option<usize> {
        self.records.iter().position(|record| {
            self.visits += 1;
            *record == (sources, mode)
        })
    }
    fn remember(&mut self, sources: [usize; 2], mode: PairedQueryMode, report: usize) {
        assert_eq!(report, self.records.len());
        self.records.push((sources, mode));
    }
}
fn fresh() -> BoundaryCensus {
    let mut shapes = curved_pair();
    shapes.push(shapes[0].reversed());
    reset(paired(&shapes))
}
fn reset(mut census: BoundaryCensus) -> BoundaryCensus {
    census.domains.clear();
    census.bisections = 0;
    let retained = census.paired.as_mut().unwrap();
    retained.reports.clear();
    retained.reused_reports = 0;
    census
}
fn run_lookup(
    lookup: &mut impl ReportLookup,
    stop: usize,
) -> (Result<BoundaryCensus, CensusCause>, usize) {
    let mut polls = 0;
    let census = fresh();
    let mut cancelled = || {
        polls += 1;
        polls == stop
    };
    let result = enumerate_with_lookup(
        census,
        &mut cancelled,
        super::super::SourceAccess::Live,
        lookup,
    );
    (result, polls)
}
#[test]
fn indexed_enumeration_preserves_actual_alias_reports_and_callback_count() {
    let (literal, old_polls) = run_lookup(&mut LiteralLookup::default(), usize::MAX);
    let (indexed, new_polls) = run_lookup(&mut ReportReuseIndex::from_keys([]), usize::MAX);
    assert_eq!(old_polls, new_polls);
    let literal = literal.unwrap_or_else(|_| panic!("literal census refused"));
    let indexed = indexed.unwrap_or_else(|_| panic!("indexed census refused"));
    compare_censuses(&literal, &indexed);
    let proof = super::super::paired_tests::complete_distinct(&indexed);
    assert_eq!(proof.contact(0).unwrap().point, [0.5, 0.25]);
    assert!(indexed.paired.as_ref().unwrap().reused_reports > 0);
}
#[test]
fn cancellation_keeps_first_domain_typed_outcome_work_and_callbacks() {
    let (_, total) = run_lookup(&mut LiteralLookup::default(), usize::MAX);
    for stop in [1, 2, 3, total / 2, total] {
        let (old, old_polls) = run_lookup(&mut LiteralLookup::default(), stop);
        let (new, new_polls) = run_lookup(&mut ReportReuseIndex::from_keys([]), stop);
        assert_eq!(old_polls, new_polls, "cancel checkpoint {stop}");
        assert_eq!(
            result_snapshot(old),
            result_snapshot(new),
            "cancel checkpoint {stop}"
        );
    }
}
#[test]
fn cold_miss_oracle_exposes_the_original_quadratic_record_scan() {
    let mut literal = LiteralLookup::default();
    let mut indexed = ReportReuseIndex::from_keys([]);
    for slot in 0..210 {
        let sources = [slot, slot + 1];
        assert_eq!(
            literal.find(sources, PairedQueryMode::Distinct),
            indexed.find(sources, PairedQueryMode::Distinct)
        );
        literal.remember(sources, PairedQueryMode::Distinct, slot);
        indexed.remember(sources, PairedQueryMode::Distinct, slot);
    }
    assert_eq!(literal.visits, 210 * 209 / 2);
}
#[test]
fn cached_capture_policy_and_incomplete_outcomes_are_not_retried() {
    for case in 0..3 {
        let (census, pair) = failing_census(case);
        let old = repeated_outcome(census, pair, &mut LiteralLookup::default());
        let (census, pair) = failing_census(case);
        let new = repeated_outcome(census, pair, &mut ReportReuseIndex::from_keys([]));
        assert_eq!(old, new);
    }
}
fn failing_census(case: u8) -> (BoundaryCensus, [usize; 2]) {
    let mut census = fresh();
    let pair = match case {
        0 => {
            census.inputs.segments[0].controls.clear();
            [0, 0]
        }
        1 => {
            census.paired.as_mut().unwrap().requested.version = 99;
            [0, 0]
        }
        2 => {
            census.inputs.segments[1].controls = census.inputs.segments[0].controls.clone();
            [0, 1]
        }
        _ => unreachable!(),
    };
    (census, pair)
}
fn repeated_outcome(
    mut census: BoundaryCensus,
    pair: [usize; 2],
    lookup: &mut impl ReportLookup,
) -> Vec<String> {
    let polls = std::cell::Cell::new(0);
    let mut cancelled = || {
        polls.set(polls.get() + 1);
        false
    };
    let first = evaluate_pair(&mut census, pair, &mut cancelled, lookup);
    let after_cold = polls.get();
    let second = evaluate_pair(&mut census, pair, &mut cancelled, lookup);
    assert_eq!(polls.get(), after_cold);
    assert_reuse(first, second);
    assert_eq!(census.paired.as_ref().unwrap().reports.len(), 1);
    replay::census_snapshot(&census)
}
fn assert_reuse(first: DomainEvidence, second: DomainEvidence) {
    assert!(matches!(
        first,
        DomainEvidence::PairedReport { reused: false, .. }
    ));
    assert!(matches!(
        second,
        DomainEvidence::PairedReport { reused: true, .. }
    ));
}
#[test]
fn seeded_duplicate_keys_reuse_the_first_failure_without_a_solver_retry() {
    let (mut census, pair) = failing_census(1);
    append_duplicate_failure(&mut census, pair);
    let mut index = ReportReuseIndex::capture(&census);
    let evidence = evaluate_pair(
        &mut census,
        pair,
        &mut || panic!("cached query retried"),
        &mut index,
    );
    assert!(matches!(
        evidence,
        DomainEvidence::PairedReport {
            report: 0,
            reused: true,
            ..
        }
    ));
    assert_duplicate_outcomes(&census);
}
fn append_duplicate_failure(census: &mut BoundaryCensus, pair: [usize; 2]) {
    let mode = PairedQueryMode::SelfOffDiagonal;
    let first = cold_execution(census, pair, mode, &mut || false);
    census.paired.as_mut().unwrap().requested.version = 1;
    let second = cold_execution(census, pair, mode, &mut || false);
    census
        .paired
        .as_mut()
        .unwrap()
        .reports
        .extend([first, second]);
}
fn assert_duplicate_outcomes(census: &BoundaryCensus) {
    let retained = census.paired.as_ref().unwrap();
    assert_eq!(
        retained.reports[0].completion(),
        BezierPairCompletion::Invalid
    );
    assert_eq!(
        retained.reports[1].completion(),
        BezierPairCompletion::Complete
    );
}

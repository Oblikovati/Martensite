// SPDX-License-Identifier: GPL-2.0-only
//! Compare CarrierV2 against literal per-pair capture with the same report lookup.
use super::*;
use crate::sketch_region_certification::{
    SourceAccess, enumerate_carrier_census, source_binding_tests::source_policy, tests,
};

fn fixture() -> BoundaryCensus {
    let mut shapes = curved_pair();
    shapes.push(shapes[0].reversed());
    let census = enumerate_carrier_census(
        tests::capture(&shapes),
        tests::policy(),
        source_policy(),
        || false,
    )
    .unwrap_or_else(|_| panic!("expected admitted CarrierV2 fixture"));
    reset(census)
}

fn execute(
    census: BoundaryCensus,
    source_pool: &PairSourcePool,
    stop: usize,
) -> (Result<BoundaryCensus, CensusCause>, usize) {
    let mut reports = ReportReuseIndex::capture(&census);
    let mut polls = 0;
    let result = enumerate_with_lookup(
        census,
        &mut || {
            polls += 1;
            polls == stop
        },
        SourceAccess::Live,
        &mut reports,
        source_pool,
    );
    (result, polls)
}

fn literal(census: BoundaryCensus, stop: usize) -> (Result<BoundaryCensus, CensusCause>, usize) {
    execute(census, &PairSourcePool::default(), stop)
}

fn carrier_pool(
    census: BoundaryCensus,
    stop: usize,
) -> (Result<BoundaryCensus, CensusCause>, usize) {
    let source_pool = PairSourcePool::capture(&census);
    execute(census, &source_pool, stop)
}

#[test]
fn carrier_pool_preserves_alias_and_self_reports_with_shared_source_owners() {
    let (before, literal_polls) = literal(fixture(), usize::MAX);
    let (after, pooled_polls) = carrier_pool(fixture(), usize::MAX);
    assert_eq!(literal_polls, pooled_polls);
    let before = before.unwrap_or_else(|_| panic!("literal CarrierV2 census cancelled"));
    let after = after.unwrap_or_else(|_| panic!("pooled CarrierV2 census cancelled"));
    compare_censuses(&before, &after);
    assert_reversed_alias(&after);
    let proof = super::super::super::paired_tests::complete_distinct(&after);
    assert_eq!(proof.contact(0).unwrap().point, [0.5, 0.25]);
    for slot in 0..2 {
        assert!(!shared_controls(&before, slot));
        assert!(shared_controls(&after, slot));
    }
}

fn assert_reversed_alias(census: &BoundaryCensus) {
    assert!(census.paired.as_ref().unwrap().reused_reports > 0);
    let evidence = census
        .domains
        .iter()
        .find(|domain| domain.occurrences == [1, 2])
        .unwrap()
        .evidence;
    assert!(matches!(
        evidence,
        DomainEvidence::PairedReport {
            reused: true,
            source_slots: [1, 0],
            ..
        }
    ));
}

fn shared_controls(census: &BoundaryCensus, slot: usize) -> bool {
    let self_domain = report(census, [slot; 2]).outcome.as_ref().unwrap();
    let distinct = report(census, [0, 1]).outcome.as_ref().unwrap();
    std::ptr::eq(
        self_domain.source().captured_curves()[0].controls,
        distinct.source().captured_curves()[slot].controls,
    )
}

fn report(census: &BoundaryCensus, sources: [usize; 2]) -> &PairedStoredReport {
    census
        .paired
        .as_ref()
        .unwrap()
        .reports
        .iter()
        .find(|report| report.sources == sources)
        .unwrap()
}

#[test]
fn carrier_pool_keeps_literal_capture_refusals_for_both_pair_slots() {
    for slot in 0..2 {
        let (before, literal_polls) = literal(malformed(slot), usize::MAX);
        let (after, pooled_polls) = carrier_pool(malformed(slot), usize::MAX);
        assert_eq!(literal_polls, pooled_polls);
        let before = before.unwrap_or_else(|_| panic!("literal capture refusal cancelled census"));
        let after = after.unwrap_or_else(|_| panic!("pooled capture refusal cancelled census"));
        compare_censuses(&before, &after);
        assert_invalid_distinct_slot(&after, slot);
    }
}

fn malformed(slot: usize) -> BoundaryCensus {
    let mut census = fixture();
    census.inputs.segments[slot].controls.clear();
    census
}

fn assert_invalid_distinct_slot(census: &BoundaryCensus, slot: usize) {
    let cause = report(census, [0, 1]).outcome.as_ref().err().unwrap();
    assert_eq!(
        **cause,
        BezierPairCaptureFailure::Curve {
            curve_index: slot,
            cause: Box::new(BezierPairCaptureFailure::EmptyControls),
        }
    );
}

#[test]
fn carrier_pool_preserves_cancellation_before_and_during_pair_execution() {
    let (_, polls) = literal(fixture(), usize::MAX);
    for (stop, expected_during_domain) in [(1, Some(false)), (2, Some(true)), (polls, None)] {
        let (before, literal_polls) = literal(fixture(), stop);
        let (after, pooled_polls) = carrier_pool(fixture(), stop);
        assert_eq!(literal_polls, pooled_polls, "checkpoint {stop}");
        assert_cancelled_at(&after, expected_during_domain);
        assert_eq!(result_snapshot(before), result_snapshot(after));
    }
}

fn assert_cancelled_at(
    result: &Result<BoundaryCensus, CensusCause>,
    expected_during_domain: Option<bool>,
) {
    let Err(CensusCause::Cancelled { during_domain, .. }) = result else {
        panic!("expected cancellation at configured checkpoint");
    };
    if let Some(expected) = expected_during_domain {
        assert_eq!(*during_domain, expected);
    }
}

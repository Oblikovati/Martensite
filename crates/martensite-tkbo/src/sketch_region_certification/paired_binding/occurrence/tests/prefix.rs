// SPDX-License-Identifier: GPL-2.0-only
//! A failed occurrence is a boundary for reuse, never a refusal for every later row.
use super::*;

fn refuse(census: &mut BoundaryCensus, index: usize) {
    let record = &mut census.paired.as_mut().unwrap().occurrences[index];
    record.source.reversed = !record.source.reversed;
}
fn compare_indices(census: &BoundaryCensus, marker: usize) {
    for index in (0..=census.inputs.segments.len()).chain([usize::MAX]) {
        let expected = observed(census, index, &mut |_| {});
        assert_eq!(
            current_or_retained(census, index, Some(marker), &mut |_| {}),
            expected
        );
        assert_eq!(
            current_or_retained(census, index, None, &mut |_| {}),
            expected
        );
    }
}
#[test]
fn first_failure_marker_matches_literal_checks_at_every_first_and_later_index() {
    for index in 0..3 {
        let mut census = alias_census();
        refuse(&mut census, index);
        let marker = capture(&census, &mut |_| {});
        assert_eq!(marker, index);
        compare_indices(&census, marker);
    }
}
#[test]
fn valid_prefix_and_failed_row_do_not_rescan_but_later_valid_rows_still_validate() {
    let mut census = alias_census();
    refuse(&mut census, 1);
    let mut initial = Visits::default();
    let marker = capture(&census, &mut |kind| initial.record(kind));
    assert_eq!(marker, 1);
    assert_eq!([initial.relations, initial.prefixes], [9, 1]);
    let mut reused = Visits::default();
    assert!(observed_read(&census, 0, marker, &mut reused));
    assert!(!observed_read(&census, 1, marker, &mut reused));
    assert_eq!(reused, Visits::default());
    assert!(observed_read(&census, 2, marker, &mut reused));
    assert_eq!([reused.relations, reused.prefixes], [9, 1]);
    compare_indices(&census, marker);
}
#[test]
fn later_failures_after_the_first_refusal_use_the_same_literal_predicate() {
    let mut census = alias_census();
    refuse(&mut census, 0);
    refuse(&mut census, 2);
    let marker = capture(&census, &mut |_| {});
    assert_eq!(marker, 0);
    assert!(current_or_retained(&census, 1, Some(marker), &mut |_| {}));
    assert!(!current_or_retained(&census, 2, Some(marker), &mut |_| {}));
    compare_indices(&census, marker);
}
#[test]
fn empty_inventory_and_maximum_index_never_claim_a_valid_occurrence() {
    let mut census = alias_census();
    census.inputs.segments.clear();
    let mut visits = Visits::default();
    let marker = capture(&census, &mut |kind| visits.record(kind));
    assert_eq!(marker, 0);
    assert_eq!(visits, Visits::default());
    assert!(!current_or_retained(&census, 0, Some(marker), &mut |_| {}));
    assert!(!current_or_retained(
        &census,
        usize::MAX,
        Some(marker),
        &mut |_| {}
    ));
    compare_indices(&census, marker);
}

fn observed_read(
    census: &BoundaryCensus,
    index: usize,
    marker: usize,
    visits: &mut Visits,
) -> bool {
    current_or_retained(census, index, Some(marker), &mut |kind| visits.record(kind))
}
#[test]
fn missing_first_and_later_occurrence_rows_preserve_the_valid_prefix() {
    for kept in 0..3 {
        let mut census = alias_census();
        census.paired.as_mut().unwrap().occurrences.truncate(kept);
        let marker = capture(&census, &mut |_| {});
        assert_eq!(marker, kept);
        compare_indices(&census, marker);
    }
}

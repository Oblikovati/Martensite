// SPDX-License-Identifier: GPL-2.0-only
//! Literal predicate outputs and actual observer counts constrain #572's retained prefix.
use super::super::super::paired_tests::{curved_pair, paired};
use super::*;
mod prefix;

#[derive(Default, Debug, PartialEq)]
struct Visits {
    relations: usize,
    prefixes: usize,
}
impl Visits {
    fn record(&mut self, predicate: Predicate) {
        match predicate {
            Predicate::Relation => self.relations += 1,
            Predicate::PhysicalPrefix => self.prefixes += 1,
        }
    }
}
fn alias_census() -> BoundaryCensus {
    let mut shapes = curved_pair();
    shapes.push(shapes[0].reversed());
    paired(&shapes)
}
#[test]
fn retained_occurrences_match_literal_self_and_reversed_alias_reads_without_repeated_scans() {
    let census = alias_census();
    let mut captured = Visits::default();
    let marker = capture(&census, &mut |kind| captured.record(kind));
    assert_eq!(marker, 3);
    let mut literal = Visits::default();
    let mut reused = Visits::default();
    compare_domains(&census, marker, &mut literal, &mut reused);
    assert!(captured.relations > 0 && captured.prefixes > 0);
    assert!(literal.relations > captured.relations);
    assert!(literal.prefixes > captured.prefixes);
    assert_eq!(reused, Visits::default());
    assert_eq!([captured.relations, captured.prefixes], [27, 4]);
    assert_eq!([literal.relations, literal.prefixes], [108, 16]);
}
fn compare_domains(census: &BoundaryCensus, marker: usize, a: &mut Visits, b: &mut Visits) {
    for domain in &census.domains {
        for index in domain.occurrences {
            let literal = current_or_retained(census, index, None, &mut |kind| a.record(kind));
            let cached =
                current_or_retained(census, index, Some(marker), &mut |kind| b.record(kind));
            assert_eq!(literal, cached);
        }
    }
}
#[test]
fn retained_occurrence_constructor_is_total_for_missing_or_corrupted_rows() {
    for mutation in 0..4 {
        let mut census = alias_census();
        corrupt(&mut census, mutation);
        let marker = capture(&census, &mut |_| {});
        assert_eq!(marker, 0);
        assert!(!current_or_retained(
            &census,
            usize::MAX,
            Some(marker),
            &mut |_| {}
        ));
        for index in 0..=census.inputs.segments.len() {
            assert_eq!(
                current_or_retained(&census, index, Some(marker), &mut |_| {}),
                observed(&census, index, &mut |_| {})
            );
        }
    }
}
fn corrupt(census: &mut BoundaryCensus, mutation: usize) {
    let paired = census.paired.as_mut().unwrap();
    match mutation {
        0 => paired.occurrences.clear(),
        1 => paired.occurrences[0].physical_source = usize::MAX,
        2 => paired.occurrences[0].relations.clear(),
        3 => paired.occurrences[0].source.parents.clear(),
        _ => panic!("mutation={mutation}; expected 0..4"),
    }
}
#[test]
fn occurrence_bindings_keep_raw_signed_zero_and_full_history_matching() {
    let mut census = alias_census();
    assert_eq!(
        census.inputs.segments[0].controls[0][0].to_bits(),
        0.0f64.to_bits()
    );
    census.inputs.segments[0].controls[0][0] = -0.0;
    assert_eq!(capture(&census, &mut |_| {}), 0);
    assert!(!observed(&census, 0, &mut |_| {}));
    let mut census = alias_census();
    census.inputs.relations[0].modified.clear();
    assert_eq!(capture(&census, &mut |_| {}), 0);
    assert!(!observed(&census, 0, &mut |_| {}));
}

// SPDX-License-Identifier: GPL-2.0-only
//! Metadata-only controls compare the literal old predicate and independent row identities.
use super::*;
use crate::sketch_region::{PartitionOccurrenceIdentity, RegionSourceId};
use martensite_tkg3d::top_abs::Orientation;
use martensite_tkmath::toploc::Location;
mod equality;
mod exhaustive;
type Domain = (i32, Option<OriginalMemberIdentity>, bool);
fn parent(id: u64) -> OriginalMemberIdentity {
    OriginalMemberIdentity {
        source: RegionSourceId(id),
        topology_address: id as usize,
        location: Location::default(),
        orientation: Orientation::Forward,
    }
}
fn source(index: i32, parents: Vec<OriginalMemberIdentity>) -> SourcePartitionObservation {
    SourcePartitionObservation {
        source_index: index,
        parents,
        occurrence: PartitionOccurrenceIdentity {
            topology_address: 0,
            location: Location::default(),
            orientation: Orientation::Forward,
        },
        range: [0., 1.],
        space: None,
        has_reference: false,
        source_vertices: Vec::new(),
        blocks: Vec::new(),
        images: None,
    }
}
fn index(
    inputs: &[Vec<OriginalMemberIdentity>],
    sources: &[SourcePartitionObservation],
    domains: &[Domain],
) -> SourceIndex {
    SourceIndex::new(
        inputs.iter().map(Vec::as_slice),
        sources,
        domains
            .iter()
            .map(|(index, parent, good)| (*index, parent.as_ref(), *good)),
    )
}
fn old_matches(
    input: &[OriginalMemberIdentity],
    source: &SourcePartitionObservation,
    domains: &[Domain],
) -> bool {
    !input.is_empty()
        && input.len() == source.parents.len()
        && input.iter().all(|parent| source.parents.contains(parent))
        && source.parents.iter().all(|parent| {
            domains.iter().any(|(index, candidate, good)| {
                *index == source.source_index && candidate.as_ref() == Some(parent) && *good
            })
        })
}
fn expected_rows(
    input: &[OriginalMemberIdentity],
    sources: &[SourcePartitionObservation],
    domains: &[Domain],
) -> Vec<usize> {
    sources
        .iter()
        .enumerate()
        .filter_map(|(row, source)| old_matches(input, source, domains).then_some(row))
        .collect()
}
#[test]
fn original_source_row_order_and_alias_duplicates_survive_indexing() {
    let a = parent(1);
    let inputs = vec![vec![a.clone()]];
    let sources = vec![
        source(9, vec![a.clone()]),
        source(2, vec![a.clone()]),
        source(9, vec![a.clone()]),
    ];
    let domains = vec![(2, Some(a.clone()), true), (9, Some(a), true)];
    let indexed = index(&inputs, &sources, &domains);
    assert_eq!(indexed.matching_source_rows(0), &[0, 1, 2]);
    assert_eq!(indexed.matching_source_rows(1), &[]);
    assert_eq!(indexed.matching_source_rows(usize::MAX), &[]);
}
#[test]
fn duplicate_input_parents_keep_literal_one_way_subset_semantics() {
    let (a, b) = (parent(1), parent(2));
    let inputs = vec![vec![a.clone(), a.clone()], vec![a.clone(), b.clone()]];
    let sources = vec![
        source(7, vec![a.clone(), b.clone()]),
        source(7, vec![a.clone(), a.clone()]),
    ];
    let domains = vec![(7, Some(a), true), (7, Some(b), true)];
    let indexed = index(&inputs, &sources, &domains);
    assert_eq!(indexed.matching_source_rows(0), &[0, 1]);
    assert_eq!(indexed.matching_source_rows(1), &[0]);
}
#[test]
fn every_source_parent_needs_an_accepted_matching_domain_row() {
    let (a, b) = (parent(1), parent(2));
    let inputs = vec![vec![a.clone(), a.clone()]];
    let sources = vec![source(7, vec![a.clone(), b.clone()])];
    let mut domains = vec![
        (7, Some(a), true),
        (7, Some(b.clone()), false),
        (8, Some(b.clone()), true),
        (7, None, true),
    ];
    assert_no_first_match(&inputs, &sources, &domains);
    domains.push((7, Some(b), true));
    assert_eq!(
        index(&inputs, &sources, &domains).matching_source_rows(0),
        &[0]
    );
}
#[test]
fn empty_inputs_and_wrong_vector_lengths_never_match() {
    let a = parent(1);
    let inputs = vec![Vec::new(), vec![a.clone()], vec![a.clone(), a.clone()]];
    let sources = vec![source(1, Vec::new()), source(1, vec![a.clone()])];
    let domains = vec![(1, Some(a), true)];
    let indexed = index(&inputs, &sources, &domains);
    assert!(indexed.matching_source_rows(0).is_empty());
    assert_eq!(indexed.matching_source_rows(1), &[1]);
    assert!(indexed.matching_source_rows(2).is_empty());
}

fn assert_no_first_match(
    inputs: &[Vec<OriginalMemberIdentity>],
    sources: &[SourcePartitionObservation],
    domains: &[Domain],
) {
    let indexed = index(inputs, sources, domains);
    assert!(indexed.matching_source_rows(0).is_empty());
}

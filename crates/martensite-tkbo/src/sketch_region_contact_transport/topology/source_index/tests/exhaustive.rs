// SPDX-License-Identifier: GPL-2.0-only
use super::*;
#[test]
fn indexed_rows_equal_the_literal_predicate_for_generated_duplicate_permutations() {
    let variants = parent_vectors();
    let sources = sources(&variants);
    for mask in 0..16 {
        let domains = domains(mask);
        let indexed = index(&variants, &sources, &domains);
        for (occurrence, input) in variants.iter().enumerate() {
            assert_eq!(
                indexed.matching_source_rows(occurrence),
                expected_rows(input, &sources, &domains)
            );
        }
    }
}
fn parent_vectors() -> Vec<Vec<OriginalMemberIdentity>> {
    let (a, b) = (parent(1), parent(2));
    vec![
        vec![],
        vec![a.clone()],
        vec![b.clone()],
        vec![a.clone(), a.clone()],
        vec![a.clone(), b.clone()],
        vec![b.clone(), a],
        vec![b.clone(), b],
    ]
}
fn sources(variants: &[Vec<OriginalMemberIdentity>]) -> Vec<SourcePartitionObservation> {
    variants
        .iter()
        .flat_map(|parents| [source(7, parents.clone()), source(2, parents.clone())])
        .collect()
}
fn domains(mask: u32) -> Vec<Domain> {
    vec![
        (7, Some(parent(1)), mask & 1 != 0),
        (7, Some(parent(2)), mask & 2 != 0),
        (2, Some(parent(1)), mask & 4 != 0),
        (2, Some(parent(2)), mask & 8 != 0),
        (7, None, true),
        (7, Some(parent(1)), false),
    ]
}
#[test]
fn permuted_domain_rows_preserve_output_source_row_order() {
    let variants = parent_vectors();
    let sources = sources(&variants);
    let mut domains = domains(15);
    let before = index(&variants, &sources, &domains);
    domains.reverse();
    domains.extend(domains.clone());
    let after = index(&variants, &sources, &domains);
    for occurrence in 0..variants.len() {
        assert_eq!(
            before.matching_source_rows(occurrence),
            after.matching_source_rows(occurrence)
        );
    }
}

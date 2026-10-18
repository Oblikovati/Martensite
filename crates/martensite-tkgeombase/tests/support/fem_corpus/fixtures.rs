//! retained answers were frozen with the original full-corpus index modulo 64.
pub const ORIGINAL_INDICES: &[usize] =
    include!("../../fixtures/finite_element_corpus/original_indices.rs");
pub const CHUNKS: &[(&str, &str)] = &[
    (
        include_str!("../../fixtures/finite_element_corpus/inputs-00.txt"),
        include_str!("../../fixtures/finite_element_corpus/expected-00.txt"),
    ),
    (
        include_str!("../../fixtures/finite_element_corpus/inputs-01.txt"),
        include_str!("../../fixtures/finite_element_corpus/expected-01.txt"),
    ),
    (
        include_str!("../../fixtures/finite_element_corpus/inputs-02.txt"),
        include_str!("../../fixtures/finite_element_corpus/expected-02.txt"),
    ),
    (
        include_str!("../../fixtures/finite_element_corpus/inputs-03.txt"),
        include_str!("../../fixtures/finite_element_corpus/expected-03.txt"),
    ),
    (
        include_str!("../../fixtures/finite_element_corpus/inputs-04.txt"),
        include_str!("../../fixtures/finite_element_corpus/expected-04.txt"),
    ),
];
pub fn shard_count(shard: usize) -> usize {
    ORIGINAL_INDICES
        .iter()
        .filter(|index| **index % 64 == shard)
        .count()
}

#[test]
fn frozen_inputs_retained_answers_and_all_five_family_counts_stay_exact() {
    use crate::construction_corpus::replay::fixture_hash;
    assert_eq!(
        fixture_hash(CHUNKS.iter().map(|(q, _)| *q)),
        0x68a95bc79344dc57
    );
    assert_eq!(
        fixture_hash(CHUNKS.iter().map(|(_, a)| *a)),
        0xc66d26e085939719
    );
    let mut counts = std::collections::BTreeMap::new();
    for (q, _) in CHUNKS {
        for row in q.lines() {
            *counts
                .entry(row.split_whitespace().next().unwrap())
                .or_insert(0usize) += 1;
        }
    }
    assert_eq!(
        counts,
        [
            ("fem_elements", 432),
            ("fem_criterion", 750),
            ("fem_profile", 24),
            ("fem_curve", 10),
            ("fem_assembly", 14)
        ]
        .into_iter()
        .collect()
    );
    assert!(ORIGINAL_INDICES.windows(2).all(|pair| pair[0] < pair[1]));
    assert_eq!((0..64).map(shard_count).sum::<usize>(), 1230);
}

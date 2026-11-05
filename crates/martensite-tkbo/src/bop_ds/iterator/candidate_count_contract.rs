//! Counting candidates must share selection policy without advancing iteration.
use super::{BOPDSIterator, Pair, ShapeEnum, interference_index};

#[test]
fn counts_preserve_active_pair_order_and_cursor() {
    let mut iterator = grouped_iterator();
    iterator.initialize(ShapeEnum::Vertex, ShapeEnum::Edge);
    iterator.position = 1;
    let before = iterator.active_pairs.clone();
    assert_eq!(
        iterator.candidate_count(ShapeEnum::Vertex, ShapeEnum::Vertex),
        1
    );
    assert_eq!(
        iterator.candidate_count(ShapeEnum::Edge, ShapeEnum::Vertex),
        2
    );
    assert_eq!(
        iterator.position, 1,
        "counting must not reset current cursor"
    );
    assert_eq!(iterator.active_pairs, before);
    assert_eq!(iterator.expected_length(), 2);
}

#[test]
fn extended_groups_and_unsupported_shape_pairs_follow_selection_policy() {
    let mut iterator = grouped_iterator();
    iterator.use_extended = true;
    assert_eq!(
        iterator.candidate_count(ShapeEnum::Vertex, ShapeEnum::Vertex),
        3
    );
    assert_eq!(
        iterator.candidate_count(ShapeEnum::Face, ShapeEnum::Face),
        4
    );
    assert_eq!(
        iterator.candidate_count(ShapeEnum::Solid, ShapeEnum::Solid),
        0
    );
    iterator.initialize(ShapeEnum::Vertex, ShapeEnum::Vertex);
    assert_eq!(iterator.expected_length(), 3);
    assert_eq!(iterator.position, 0);
}

fn grouped_iterator() -> BOPDSIterator {
    let mut iterator = BOPDSIterator::default();
    let vv = interference_index(ShapeEnum::Vertex, ShapeEnum::Vertex).unwrap();
    let ve = interference_index(ShapeEnum::Vertex, ShapeEnum::Edge).unwrap();
    let ff = interference_index(ShapeEnum::Face, ShapeEnum::Face).unwrap();
    iterator.lists[vv].append(Pair::new(2, 1));
    iterator.lists[ve].append(Pair::new(5, 3));
    iterator.lists[ve].append(Pair::new(6, 4));
    for ordinal in 0..3 {
        iterator.extended_lists[vv].append(Pair::new(ordinal, ordinal + 1));
    }
    for ordinal in 0..4 {
        iterator.lists[ff].append(Pair::new(ordinal, ordinal + 1));
    }
    iterator
}

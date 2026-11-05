//! Identity and multiplicity expectations are independent of allocation and bucket order.
use super::{add_edge_set, occurrence_keys::EdgeOccurrencePolicy};
use crate::bop_tools::ShapeSet;
use martensite_tkbrep::{
    brep::BRepBuilder,
    topods::{Edge, Face, Shape, Wire},
};
use martensite_tkernel::ncollection::{HashPolicy, IndexedDataMap};

struct CollidingOccurrencePolicy(EdgeOccurrencePolicy);
impl HashPolicy<ShapeSet> for CollidingOccurrencePolicy {
    fn hash(&self, _: &ShapeSet) -> usize {
        0
    }
    fn equal(&self, first: &ShapeSet, second: &ShapeSet) -> bool {
        self.0.equal(first, second)
    }
}

fn independent_edge() -> Shape {
    let mut edge = Edge::default();
    BRepBuilder::default().make_edge(&mut edge).unwrap();
    edge.into_shape()
}

fn occurrence_face(edges: &[Shape]) -> Shape {
    let builder = BRepBuilder::default();
    let mut wire = Wire::default();
    builder.make_wire(&mut wire);
    let mut wire = wire.into_shape();
    for edge in edges {
        builder.add(&mut wire, edge).unwrap();
    }
    let mut face = Face::default();
    builder.make_face(&mut face);
    let mut face = face.into_shape();
    builder.add(&mut face, &wire).unwrap();
    face
}

fn collected_face_groups(first: &[Shape], second: &[Shape]) -> usize {
    let policy = CollidingOccurrencePolicy(EdgeOccurrencePolicy::default());
    let mut groups = IndexedDataMap::with_hasher(policy, 1);
    add_edge_set(&occurrence_face(first), &mut groups).unwrap();
    add_edge_set(&occurrence_face(second), &mut groups).unwrap();
    groups.extent()
}

#[test]
fn duplicate_edge_subset_cannot_merge_distinct_faces_on_hash_collision() {
    let a = independent_edge();
    let b = independent_edge();
    assert_eq!(collected_face_groups(&[a.clone(), b], &[a.clone(), a]), 2);
}

#[test]
fn equal_support_with_different_edge_multiplicities_remains_distinct() {
    let a = independent_edge();
    let b = independent_edge();
    assert_eq!(
        collected_face_groups(&[a.clone(), a.clone(), b.clone()], &[a, b.clone(), b]),
        2
    );
}

#[test]
fn reordered_matching_edge_multiplicities_share_one_group() {
    let a = independent_edge();
    let b = independent_edge();
    assert_eq!(collected_face_groups(&[a.clone(), b.clone()], &[b, a]), 1);
}

#[test]
fn legacy_occurrence_policy_remains_selectable_for_collision_replay() {
    use crate::bop_algo::FaceGroupingVersion;
    let a = independent_edge();
    let b = independent_edge();
    let policy = CollidingOccurrencePolicy(EdgeOccurrencePolicy(
        FaceGroupingVersion::LegacyOccurrencesV1,
    ));
    let mut groups = IndexedDataMap::with_hasher(policy, 1);
    add_edge_set(&occurrence_face(&[a.clone(), b]), &mut groups).unwrap();
    add_edge_set(&occurrence_face(&[a.clone(), a]), &mut groups).unwrap();
    assert_eq!(groups.extent(), 1);
}

fn occurrence_word(mut code: usize, edges: &[Shape; 3]) -> (Vec<Shape>, [usize; 3]) {
    let mut counts = [0; 3];
    let mut word = Vec::new();
    for _ in 0..3 {
        counts[code % 3] += 1;
        word.push(edges[code % 3].clone());
        code /= 3;
    }
    (word, counts)
}

#[test]
fn all_three_edge_words_group_exactly_by_identity_multiplicity() {
    let edges = std::array::from_fn(|_| independent_edge());
    for first in 0..27 {
        for second in 0..27 {
            let (left, left_counts) = occurrence_word(first, &edges);
            let (right, right_counts) = occurrence_word(second, &edges);
            let expected = if left_counts == right_counts { 1 } else { 2 };
            assert_eq!(
                collected_face_groups(&left, &right),
                expected,
                "words={first},{second}"
            );
        }
    }
}

#[test]
fn empty_and_nonempty_occurrences_remain_distinct_under_collisions() {
    assert_eq!(collected_face_groups(&[], &[independent_edge()]), 2);
}

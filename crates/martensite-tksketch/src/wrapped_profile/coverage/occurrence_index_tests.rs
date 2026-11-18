//! Sorted indexes retain typed source keys and actual orientations without repeated scans.
use super::*;
fn key(region: u64, role: HistoryRole) -> BindingKey {
    BindingKey {
        source: Source::Region(SurfaceRegionId(region)),
        role,
    }
}
fn face(region: u64, orientation: Orientation, edges: Vec<BoundUse>) -> FaceEvidence {
    FaceEvidence {
        owner: key(region, HistoryRole::FirstDepthFace),
        orientation,
        wire_count: 1,
        edges,
    }
}
#[test]
fn sorted_receipt_retains_actual_values_for_shuffled_source_keys() {
    let edge = key(3, HistoryRole::FirstDepthEdge);
    let other = key(1, HistoryRole::FirstDepthEdge);
    let edges = vec![BoundUse::forward(edge), BoundUse::forward(other)];
    let cap = face(2, Orientation::Reversed, edges);
    let receipt = Occurrences::new(vec![cap, face(1, Orientation::Forward, vec![])]).unwrap();
    let owner = key(2, HistoryRole::FirstDepthFace);
    assert_reversed(receipt.face_orientation(owner));
    assert_reversed(receipt.edge_orientation(edge, owner));
    assert_reversed(receipt.edge_orientation(other, owner));
}
fn assert_reversed(orientation: Checked<Orientation>) {
    assert_eq!(orientation.unwrap(), Orientation::Reversed);
}
#[test]
fn duplicate_face_index_preserves_exact_offender_and_count() {
    let faces = (0..3)
        .map(|_| face(2, Orientation::Forward, vec![]))
        .collect();
    let refusal = Occurrences::new(faces).err().unwrap();
    let owner = key(2, HistoryRole::FirstDepthFace);
    assert_count(refusal, owner, "actual face occurrences", 3);
}
#[test]
fn duplicate_edge_index_preserves_exact_offender_and_count() {
    let edge = key(3, HistoryRole::FirstDepthEdge);
    let edges = vec![BoundUse::forward(edge); 3];
    let refusal = Occurrences::new(vec![face(2, Orientation::Forward, edges)])
        .err()
        .unwrap();
    assert_count(refusal, edge, "actual oriented occurrences", 3);
}
fn assert_count(refusal: CoverageFailure, owner: BindingKey, role: &'static str, actual: usize) {
    assert_eq!(refusal.owner, CoverageOwner::Binding(owner));
    assert!(matches!(refusal.cause, CoverageCause::Count {
        role: found_role, actual: found_actual, expected: 1
    } if found_role == role && found_actual == actual));
}
#[test]
fn missing_face_and_local_edge_have_distinct_typed_owners() {
    let receipt = Occurrences::new(vec![face(2, Orientation::Forward, vec![])]).unwrap();
    let missing_face = key(3, HistoryRole::FirstDepthFace);
    assert_missing(
        receipt.face_orientation(missing_face).unwrap_err(),
        missing_face,
    );
    let edge = key(3, HistoryRole::FirstDepthEdge);
    let owner = key(2, HistoryRole::FirstDepthFace);
    assert_missing(receipt.edge_orientation(edge, owner).unwrap_err(), edge);
    assert_missing(
        receipt.edge_orientation(edge, missing_face).unwrap_err(),
        missing_face,
    );
}
fn assert_missing(refusal: CoverageFailure, owner: BindingKey) {
    assert_eq!(refusal.owner, CoverageOwner::Binding(owner));
    assert!(matches!(refusal.cause, CoverageCause::MissingBinding(actual) if actual == owner));
}
#[test]
fn shell_use_index_is_keyed_and_refuses_all_duplicate_occurrences() {
    let first = key(1, HistoryRole::FirstDepthFace);
    let last = key(2, HistoryRole::LastDepthFace);
    let mut uses = vec![
        BoundUse::forward(last),
        BoundUse::new(first, Orientation::Reversed),
    ];
    occurrences::order_uses(&mut uses).unwrap();
    assert_reversed(occurrences::face_orientation(first, &uses));
    assert_eq!(
        occurrences::face_orientation(last, &uses).unwrap(),
        Orientation::Forward
    );
    uses.extend([BoundUse::forward(last); 2]);
    let refusal = occurrences::order_uses(&mut uses).unwrap_err();
    assert_count(refusal, last, "actual oriented occurrences", 3);
}

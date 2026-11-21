//! #540: actual shell and cap-wire uses, rather than canonical binding flags, determine orientation.
use super::tests::{direct, replace};
use super::*;
use operation::cone::tests::frozen_native;

fn certify_cone(
    source: &DevelopedRegionRequest,
    root: &Shape,
    bindings: &[(BindingKey, Shape)],
) -> Checked<(CoverageEvidence, Occurrences)> {
    certify_developed_oriented(source, root, bindings, DevelopedMapHandedness::Reversing)
}
fn binding(bindings: &[(BindingKey, Shape)], key: BindingKey) -> Shape {
    bindings
        .iter()
        .find(|(actual, _)| *actual == key)
        .unwrap()
        .1
        .clone()
}
#[test]
fn cone_occurrences_compose_actual_cap_and_side_uses_once() {
    let (source, root, bindings) = frozen_native();
    let (_, occurrences) = certify_cone(&source, &root, &bindings).unwrap();
    for piece in source.loops.iter().flatten() {
        assert_piece(&source, piece, &occurrences);
    }
    assert!(
        bindings
            .iter()
            .all(|(_, shape)| shape.orientation() == Orientation::Forward)
    );
    fn send_sync<T: Send + Sync>() {}
    send_sync::<Occurrences>();
}
fn assert_piece(source: &DevelopedRegionRequest, piece: &AuthoredPiece, actual: &Occurrences) {
    let first = expected::region(source, HistoryRole::FirstDepthFace);
    let last = expected::region(source, HistoryRole::LastDepthFace);
    let side = expected::span(piece.span, HistoryRole::SideFace);
    for (face, orientation) in [
        (first, Orientation::Reversed),
        (last, Orientation::Forward),
        (side, Orientation::Reversed),
    ] {
        assert_eq!(actual.face_orientation(face).unwrap(), orientation);
    }
    assert_edges(piece, [first, last, side], actual);
}
fn assert_edges(piece: &AuthoredPiece, [first, last, side]: [BindingKey; 3], actual: &Occurrences) {
    let first_edge = expected::span(piece.span, HistoryRole::FirstDepthEdge);
    let last_edge = expected::span(piece.span, HistoryRole::LastDepthEdge);
    let expected = [
        (first_edge, first, Orientation::Forward),
        (last_edge, last, Orientation::Reversed),
        (first_edge, side, Orientation::Reversed),
        (last_edge, side, Orientation::Forward),
    ];
    for (edge, face, orientation) in expected {
        assert_eq!(actual.edge_orientation(edge, face).unwrap(), orientation);
    }
}
#[test]
fn cone_policy_rejects_one_changed_actual_side_face_use() {
    let (source, root, bindings) = frozen_native();
    let key = expected::span(source.loops[0][0].span, HistoryRole::SideFace);
    let face = binding(&bindings, key).oriented(Orientation::Reversed);
    let mut shell = direct(&root).remove(0);
    replace(&mut shell, &face, &face.oriented(Orientation::Forward));
    let refusal = certify_cone(&source, &root, &bindings).err().unwrap();
    assert!(matches!(refusal.cause, CoverageCause::Uses { .. }));
}
#[test]
fn cone_policy_rejects_one_changed_actual_cap_edge_use() {
    let (source, root, bindings) = frozen_native();
    let key = expected::region(&source, HistoryRole::FirstDepthFace);
    let mut wire = direct(&binding(&bindings, key)).remove(0);
    let edge = direct(&wire).remove(0);
    assert_eq!(edge.orientation(), Orientation::Reversed);
    replace(&mut wire, &edge, &edge.oriented(Orientation::Forward));
    let refusal = certify_cone(&source, &root, &bindings).err().unwrap();
    assert_eq!(refusal.owner, CoverageOwner::Binding(key));
    assert!(matches!(refusal.cause, CoverageCause::Contour { .. }));
}
#[test]
fn cone_occurrences_are_keyed_independently_of_binding_order() {
    let (source, root, mut bindings) = frozen_native();
    let (report, _) = certify_cone(&source, &root, &bindings).unwrap();
    bindings.reverse();
    let (reordered, occurrences) = certify_cone(&source, &root, &bindings).unwrap();
    assert_eq!(report, reordered);
    for piece in source.loops.iter().flatten() {
        assert_piece(&source, piece, &occurrences);
    }
}
#[test]
fn cone_occurrences_refuse_an_edge_bound_to_an_unrelated_face() {
    let (source, root, bindings) = frozen_native();
    let (_, occurrences) = certify_cone(&source, &root, &bindings).unwrap();
    let edge = expected::span(source.loops[0][0].span, HistoryRole::FirstDepthEdge);
    let face = expected::span(source.loops[0][1].span, HistoryRole::SideFace);
    let refusal = occurrences.edge_orientation(edge, face).unwrap_err();
    assert_eq!(refusal.owner, CoverageOwner::Binding(edge));
    assert!(matches!(refusal.cause, CoverageCause::MissingBinding(actual) if actual == edge));
}
#[test]
fn cone_semantic_bindings_cannot_replace_actual_occurrence_flags() {
    let (source, root, mut bindings) = frozen_native();
    let key = expected::span(source.loops[0][0].span, HistoryRole::SideFace);
    reverse_binding(&mut bindings, key);
    let refusal = certify_cone(&source, &root, &bindings).err().unwrap();
    assert_eq!(refusal.owner, CoverageOwner::Binding(key));
    assert!(matches!(
        refusal.cause,
        CoverageCause::Placement {
            orientation: Orientation::Reversed,
            ..
        }
    ));
}
#[test]
fn cone_occurrences_refuse_missing_source_binding() {
    let (source, root, mut bindings) = frozen_native();
    bindings.pop();
    let missing = certify_cone(&source, &root, &bindings).err().unwrap();
    assert!(matches!(
        missing.cause,
        CoverageCause::Count {
            role: "semantic bindings",
            ..
        }
    ));
}
#[test]
fn cone_occurrences_refuse_extra_actual_face() {
    let (source, root, bindings) = frozen_native();
    let mut shell = direct(&root).remove(0);
    let face = direct(&shell).remove(0);
    shell.set_free(true).unwrap();
    martensite_tkbrep::topods::Builder
        .add(&mut shell, &face)
        .unwrap();
    let extra = certify_cone(&source, &root, &bindings).err().unwrap();
    assert!(matches!(
        extra.cause,
        CoverageCause::Count {
            role: "direct children",
            ..
        }
    ));
}

fn reverse_binding(bindings: &mut [(BindingKey, Shape)], key: BindingKey) {
    let shape = &mut bindings
        .iter_mut()
        .find(|(actual, _)| *actual == key)
        .unwrap()
        .1;
    *shape = shape.oriented(Orientation::Reversed);
}

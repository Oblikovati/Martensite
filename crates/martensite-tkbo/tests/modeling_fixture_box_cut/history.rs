use super::geometry::{bounds, entities};
use martensite_tkbo::bop_algo::BOPAlgoBOP;
use martensite_tkbrep::topods::Shape;
use martensite_tkg3d::top_abs::ShapeEnum;

pub(super) fn require_relations(operation: &BOPAlgoBOP, inputs: &[Shape; 2]) {
    let result_faces = entities(operation.shape(), ShapeEnum::Face);
    let mut covered = Vec::<Shape>::new();
    for parent in inputs
        .iter()
        .flat_map(|shape| entities(shape, ShapeEnum::Face))
    {
        require_parent(operation, &parent, &result_faces, &mut covered);
    }
    assert_eq!(
        covered.len(),
        6,
        "every box result face needs a genuine history parent"
    );
}

fn require_parent(
    operation: &BOPAlgoBOP,
    parent: &Shape,
    result: &[Shape],
    covered: &mut Vec<Shape>,
) {
    let modified = operation.modified(parent).unwrap();
    require_children_present(&operation.generated(parent).unwrap(), operation.shape());
    let Some(expected) = expected_face(bounds(parent)) else {
        require_removed(operation, parent, &modified);
        return;
    };
    require_retained(operation, parent, &modified, expected, result, covered);
}

fn require_retained(
    operation: &BOPAlgoBOP,
    parent: &Shape,
    modified: &[Shape],
    expected: [f64; 6],
    result: &[Shape],
    covered: &mut Vec<Shape>,
) {
    assert!(
        !operation.is_deleted(parent).unwrap(),
        "retained source face marked deleted"
    );
    let face = result.iter().find(|face| bounds(face) == expected).unwrap();
    assert!(
        face.is_same(parent) || modified.iter().any(|child| child.is_same(face)),
        "expected retained face has no actual parent/history relation"
    );
    require_modified(modified, face);
    if !covered.iter().any(|known| known.is_same(face)) {
        covered.push(face.clone());
    }
}

fn require_removed(operation: &BOPAlgoBOP, parent: &Shape, modified: &[Shape]) {
    assert!(
        operation.is_deleted(parent).unwrap(),
        "discarded source face must be deleted"
    );
    assert!(
        modified.is_empty(),
        "discarded face has modified descendants"
    );
}

fn expected_face(parent: [f64; 6]) -> Option<[f64; 6]> {
    let result = [0., 1., 0., 2., 0., 2.];
    let mut clipped = parent;
    let mut positive_axes = 0;
    for axis in 0..3 {
        clipped[2 * axis] = parent[2 * axis].max(result[2 * axis]);
        clipped[2 * axis + 1] = parent[2 * axis + 1].min(result[2 * axis + 1]);
        if clipped[2 * axis] > clipped[2 * axis + 1] {
            return None;
        }
        positive_axes += usize::from(clipped[2 * axis] < clipped[2 * axis + 1]);
    }
    (positive_axes == 2).then_some(clipped)
}

fn require_modified(modified: &[Shape], expected: &Shape) {
    for child in modified {
        assert!(
            child.is_same(expected),
            "modified face does not match analytic retained rectangle"
        );
    }
}

fn require_children_present(children: &[Shape], result: &Shape) {
    for child in children {
        let kind = child.shape_type().unwrap();
        assert!(
            entities(result, kind)
                .iter()
                .any(|entity| entity.is_same(child)),
            "generated {kind:?} absent from result"
        );
    }
}

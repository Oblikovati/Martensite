//! History identity comes from actual source handles and reported relations, not geometry rank.
use super::geometry::entities;
use martensite_tkbo::bop_algo::BOPAlgoBOP;
use martensite_tkbrep::topods::Shape;
use martensite_tkg3d::top_abs::ShapeEnum;

pub(super) fn require_supported_entities(operation: &BOPAlgoBOP, inputs: &[Shape; 2]) {
    let mut parents = Vec::new();
    for kind in [ShapeEnum::Face, ShapeEnum::Edge, ShapeEnum::Vertex] {
        parents.extend(inputs.iter().flat_map(|shape| entities(shape, kind)));
    }
    for parent in &parents {
        require_reported_relations(operation, parent);
    }
    for kind in [ShapeEnum::Edge, ShapeEnum::Vertex] {
        for child in entities(operation.shape(), kind) {
            assert!(
                parents
                    .iter()
                    .any(|parent| related(operation, parent, &child)),
                "result {kind:?} lacks an actual unchanged/modified/generated parent"
            );
        }
    }
}

fn related(operation: &BOPAlgoBOP, parent: &Shape, child: &Shape) -> bool {
    if parent.is_same(child) {
        return true;
    }
    let mut images = operation.modified(parent).unwrap();
    images.extend(operation.generated(parent).unwrap());
    images.iter().any(|image| image.is_same(child))
}

fn require_reported_relations(operation: &BOPAlgoBOP, parent: &Shape) {
    let modified = operation.modified(parent).unwrap();
    for child in &modified {
        assert_eq!(child.shape_type().unwrap(), parent.shape_type().unwrap());
        require_member(operation.shape(), child);
    }
    for child in operation.generated(parent).unwrap() {
        require_member(operation.shape(), &child);
    }
    if operation.is_deleted(parent).unwrap() {
        require_deleted(operation.shape(), parent, &modified);
    }
}

fn require_deleted(result: &Shape, parent: &Shape, modified: &[Shape]) {
    let kind = parent.shape_type().unwrap();
    assert!(
        !entities(result, kind)
            .iter()
            .any(|child| child.is_same(parent)),
        "deleted source {kind:?} still belongs to result"
    );
    assert!(
        modified.is_empty(),
        "deleted source entity has modified images"
    );
}

fn require_member(result: &Shape, child: &Shape) {
    let kind = child.shape_type().unwrap();
    assert!(
        entities(result, kind)
            .iter()
            .any(|member| member.is_same(child)),
        "reported history {kind:?} image absent from result"
    );
}

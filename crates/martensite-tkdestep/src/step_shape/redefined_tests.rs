//! Tests of the redefined fields, through base handles.
use super::*;

fn named_vertex() -> Handle<StepShapeVertex> {
    Handle::new(StepShapeVertex::new())
}

#[test]
fn an_oriented_edge_answers_through_its_base_handle() {
    let (start, end) = (named_vertex(), named_vertex());
    let edge = Handle::new(StepShapeEdge::new());
    edge.init(&Handle::default(), &start, &end);
    let oriented = Handle::new(StepShapeOrientedEdge::new());
    oriented.init(&Handle::default(), &edge, false);
    let base = oriented.cast::<StepShapeEdge>();
    assert_eq!(base.edge_start(), end);
    assert_eq!(base.edge_end(), start);
    oriented.set_orientation(true);
    assert_eq!(base.edge_start(), start);
}

#[test]
fn an_oriented_edge_without_element_answers_null_and_ignores_setters() {
    let oriented = StepShapeOrientedEdge::new();
    assert!(oriented.edge_start().is_null());
    oriented.set_edge_start(&named_vertex());
    assert!(oriented.edge_start().is_null());
}

#[test]
fn an_oriented_face_without_element_fails_as_the_dereference() {
    let oriented = StepShapeOrientedFace::new();
    assert!(
        oriented
            .nb_bounds()
            .unwrap_err()
            .is_kind("Standard_NullObject")
    );
    let plain = StepShapeFace::new();
    assert_eq!(plain.nb_bounds().unwrap(), 0);
}

// SPDX-License-Identifier: GPL-2.0-only
//! Contract tests for #6 (OperationError), #18 (HistoryGraph), and #27 (OperationResult).
use martensite_tkbrep::brep::BRepBuilder;
use martensite_tkbrep::brep_tools::History;
use martensite_tkbrep::operation::{
    FailureClass, HistoryGraph, HistoryRelationKind, OperationError, OperationKind, OperationResult,
};
use martensite_tkbrep::topods::{Shape, Vertex};
use martensite_tkernel::standard::Failure;
use std::error::Error;

fn make_vertex(_x: f64) -> Shape {
    let mut vertex = Vertex::default();
    BRepBuilder::default().make_vertex(&mut vertex);
    vertex.into_shape()
}

#[test]
fn operation_error_covers_every_failure_class_entity_and_cause() {
    let entity = make_vertex(1.0);
    let root_cause = Failure::with_message("BOPAlgo_Alert", "invalid input face");
    let cases = [
        (
            OperationError::bad_input(OperationKind::Prism, "zero direction"),
            FailureClass::BadInput,
        ),
        (
            OperationError::named_decline(OperationKind::Fillet, "unsupported corner"),
            FailureClass::NamedDecline,
        ),
        (
            OperationError::kernel_failure(OperationKind::Boolean, "stitch failed"),
            FailureClass::KernelFailure,
        ),
        (
            OperationError::incomplete_check(OperationKind::Shell, "budget exhausted"),
            FailureClass::IncompleteCheck,
        ),
        (
            OperationError::cancelled(OperationKind::Loft, "cancelled by user"),
            FailureClass::Cancelled,
        ),
    ];
    for (err, expected_class) in cases {
        let err = err.with_entity(entity.clone()).with_cause(root_cause.clone());
        assert_eq!(err.failure_class(), expected_class);
        assert_eq!(err.affected_entities().len(), 1);
        assert!(err.affected_entities()[0].is_same(&entity));
        assert!(err.source().is_some());
        assert!(err.source().unwrap().to_string().contains("invalid input face"));
    }
}

#[test]
fn history_graph_preserves_all_five_relations_and_non_unique_markers() {
    let p1 = make_vertex(1.0);
    let p2 = make_vertex(2.0);
    let p_split = make_vertex(3.0);
    let p_del = make_vertex(4.0);
    let p_gen = make_vertex(5.0);
    let q_merged = make_vertex(10.0);
    let q_s1 = make_vertex(11.0);
    let q_s2 = make_vertex(12.0);
    let q_gen = make_vertex(13.0);
    let q_created = make_vertex(14.0);

    let brep_history = History::new();
    brep_history.add_modified(&p1, &q_merged).unwrap();
    brep_history.add_modified(&p2, &q_merged).unwrap();
    brep_history.add_modified(&p_split, &q_s1).unwrap();
    brep_history.add_modified(&p_split, &q_s2).unwrap();
    brep_history.remove(&p_del).unwrap();
    brep_history.add_generated(&p_gen, &q_gen).unwrap();
    brep_history.add_generated(&Shape::default(), &q_created).unwrap();

    let graph = HistoryGraph::from_brep_history(
        &[p1.clone(), p2.clone(), p_split.clone(), p_del.clone(), p_gen.clone()],
        &brep_history,
    )
    .unwrap();

    let merged = graph.by_kind(HistoryRelationKind::Merged);
    assert_eq!(merged.len(), 1);
    assert_eq!(merged[0].parents().len(), 2);
    assert!(merged[0].is_non_unique());

    let splits = graph.by_kind(HistoryRelationKind::Split);
    assert_eq!(splits.len(), 1);
    assert_eq!(splits[0].products().len(), 2);
    assert!(splits[0].is_non_unique());

    assert_eq!(graph.by_kind(HistoryRelationKind::Deleted).len(), 1);
    assert_eq!(graph.by_kind(HistoryRelationKind::Derived).len(), 1);
    assert_eq!(graph.by_kind(HistoryRelationKind::Created).len(), 1);
}

#[test]
fn operation_result_is_send_sync_sealed_and_monotonic() {
    fn assert_send_sync<T: Send + Sync + 'static>() {}
    assert_send_sync::<OperationResult>();
    assert_send_sync::<OperationError>();
    assert_send_sync::<HistoryGraph>();

    assert!(OperationResult::seal(OperationKind::Prism, Shape::default(), HistoryGraph::empty()).is_err());

    let r1 = OperationResult::seal(OperationKind::Prism, make_vertex(1.0), HistoryGraph::empty()).unwrap();
    let r2 = OperationResult::seal(OperationKind::Boolean, make_vertex(2.0), HistoryGraph::empty()).unwrap();
    assert!(r2.revision() > r1.revision());
    assert_eq!(r1.owner_thread(), std::thread::current().id());
}

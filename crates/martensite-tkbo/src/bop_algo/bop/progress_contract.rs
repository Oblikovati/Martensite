//! Independent retained identities and empty-set laws pin actual Boolean receiver dispatch.
use super::*;
use crate::bop_algo::progress_test_support::{indicator, pulse_indicator};
use martensite_tkbrep::{
    brep::BRepBuilder,
    topods::{Compound, ShapeIterator},
};
use martensite_tkmath::gp::Point3;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeVertex;
use std::sync::atomic::Ordering;
fn vertex(x: f64) -> Shape {
    BRepBuilderAPIMakeVertex::new(Point3::new(x, 0., 0.))
        .unwrap()
        .vertex()
        .unwrap()
        .into_shape()
}
fn empty() -> Shape {
    let mut empty = Compound::default();
    BRepBuilder::default().make_compound(&mut empty);
    empty.into_shape()
}
fn receiver(operation: Operation, argument: &Shape, tool: &Shape) -> BOPAlgoBOP {
    let mut bop = BOPAlgoBOP::new();
    bop.set_operation(operation);
    bop.add_argument(argument);
    bop.add_tool(tool);
    bop
}
#[test]
fn all_four_disjoint_vertex_operations_keep_their_exact_expected_inputs() {
    let sources = [vertex(0.), vertex(10.)];
    for (operation, expected) in [
        (Operation::Common, vec![]),
        (Operation::Fuse, vec![0, 1]),
        (Operation::Cut, vec![0]),
        (Operation::Cut21, vec![1]),
    ] {
        let mut bop = receiver(operation, &sources[0], &sources[1]);
        assert_eq!(
            bop.perform_with_progress(&ProgressRange::default())
                .unwrap(),
            ProgressCompletion::Completed
        );
        let parts: Vec<_> = ShapeIterator::new(bop.shape(), true, true).collect();
        assert_eq!(parts.len(), expected.len());
        for i in expected {
            assert!(parts.iter().any(|part| part.is_same(&sources[i])));
        }
        assert!(bop.has_filled_history());
        assert!(!bop.options().has_errors());
    }
}
#[test]
fn empty_argument_shortcuts_follow_set_laws_without_creating_the_weighted_build_scope() {
    let tool = vertex(10.);
    for (operation, count) in [
        (Operation::Common, 0),
        (Operation::Fuse, 1),
        (Operation::Cut, 0),
        (Operation::Cut21, 1),
    ] {
        let mut bop = receiver(operation, &empty(), &tool);
        let (progress, trace) = indicator(0);
        assert_eq!(
            bop.perform_with_progress(&progress.start()).unwrap(),
            ProgressCompletion::Completed
        );
        assert_eq!(ShapeIterator::new(bop.shape(), true, true).count(), count);
        assert!(bop.options().has_warnings());
        assert!(bop.has_filled_history());
        let events = trace.events.lock().unwrap();
        assert!(!events.iter().any(|(_, names)| {
            names
                .iter()
                .any(|name| name == "Building the result of Boolean operation")
        }));
    }
}
#[test]
fn actual_boolean_receiver_preserves_intersection_cancellation() {
    let sources = [vertex(0.), vertex(0.)];
    let mut bop = receiver(Operation::Fuse, &sources[0], &sources[1]);
    let (progress, trace) = pulse_indicator(1);
    let completion = bop.perform_with_progress(&progress.start()).unwrap();
    assert!(!matches!(completion, ProgressCompletion::Completed));
    assert_eq!(trace.queries.load(Ordering::SeqCst), 1);
    assert!(bop.pave_filler().is_some());
    assert!(bop.options().has_errors());
}
#[test]
fn unknown_boolean_operation_cannot_run_a_general_fuse_substitute() {
    let mut bop = receiver(Operation::Unknown, &vertex(0.), &vertex(10.));
    assert!(matches!(
        bop.perform_with_progress(&ProgressRange::default())
            .unwrap(),
        ProgressCompletion::Failed { .. }
    ));
    assert!(bop.options().has_errors());
    assert!(bop.shape().is_null());
}
#[test]
fn prepared_filler_selects_the_actual_boolean_build_program() {
    let sources = [vertex(0.), vertex(10.)];
    let mut bop = receiver(Operation::Cut, &sources[0], &sources[1]);
    let mut filler = BOPAlgoPaveFiller::new();
    for source in &sources {
        filler.add_argument(source);
    }
    filler.perform();
    let (progress, trace) = indicator(0);
    assert_eq!(
        crate::bop_algo::perform_with_filler_progress(&mut bop, &mut filler, &progress.start())
            .unwrap(),
        ProgressCompletion::Completed
    );
    let parts: Vec<_> = ShapeIterator::new(bop.shape(), true, true).collect();
    assert_eq!(parts.len(), 1);
    assert!(parts[0].is_same(&sources[0]));
    let events = trace.events.lock().unwrap();
    assert!(events.iter().any(|(_, names)| {
        names
            .iter()
            .any(|name| name == "Building the result of Boolean operation")
    }));
}

#[test]
fn collection_and_duplicate_removal_cancellation_precede_result_and_history_publication() {
    for target in [4, 5] {
        let mut bop = receiver(Operation::Common, &vertex(0.), &vertex(10.));
        let (progress, trace) = pulse_indicator(target);
        let completion = bop.perform_with_progress(&progress.start()).unwrap();
        assert!(!matches!(completion, ProgressCompletion::Completed));
        assert_eq!(trace.queries.load(Ordering::SeqCst), target);
        assert_eq!(ShapeIterator::new(bop.shape(), true, true).count(), 2);
        assert!(!bop.has_filled_history());
        assert!(bop.options().has_errors());
    }
}

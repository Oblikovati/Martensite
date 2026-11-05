//! Actual-entry controls preserve source identity, tool exclusion and lower non-completion.
use super::*;
use crate::bop_algo::{
    BOPAlgoSplitter, ProgressFailureClass, ProgressOperation, ProgressStage,
    progress_test_support::{indicator, pulse_indicator},
};
use martensite_tkbrep::topods::ShapeIterator;
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
fn vertex_builder() -> (BOPAlgoBuilder, [Shape; 2]) {
    let sources = [vertex(0.), vertex(10.)];
    let mut builder = BOPAlgoBuilder::new();
    for source in &sources {
        builder.add_argument(source);
    }
    (builder, sources)
}
#[test]
fn whole_builder_retains_its_filler_and_both_untouched_vertex_identities() {
    let (mut builder, sources) = vertex_builder();
    let (progress, _) = indicator(0);
    let completion = builder.perform_with_progress(&progress.start()).unwrap();
    assert_eq!(completion, ProgressCompletion::Completed);
    assert!(builder.pave_filler().unwrap().data_structure().is_some());
    let parts: Vec<_> = ShapeIterator::new(builder.shape(), true, true).collect();
    assert_eq!(parts.len(), 2);
    assert!(
        sources
            .iter()
            .all(|source| parts.iter().any(|part| part.is_same(source)))
    );
}
#[test]
fn whole_splitter_intersects_the_tool_and_keeps_it_out_of_the_result() {
    let mut splitter = BOPAlgoSplitter::new();
    let argument = vertex(0.);
    let tool = vertex(10.);
    splitter.add_argument(&argument);
    splitter.add_tool(&tool);
    assert_eq!(
        splitter
            .perform_with_progress(&ProgressRange::default())
            .unwrap(),
        ProgressCompletion::Completed
    );
    let filler = splitter.pave_filler().unwrap();
    assert_eq!(filler.arguments().len(), 2);
    assert!(splitter.shape().is_same(&argument));
    assert!(!splitter.shape().is_same(&tool));
}
struct LegacyWholeReceiver {
    builder: BOPAlgoBuilder,
}
impl BuilderHooks for LegacyWholeReceiver {
    fn general_fuse(&mut self) -> &mut BOPAlgoBuilder {
        &mut self.builder
    }
}
#[test]
fn unknown_whole_receiver_refuses_a_base_builder_substitution() {
    let mut maker = LegacyWholeReceiver {
        builder: BOPAlgoBuilder::new(),
    };
    maker.builder.add_argument(&vertex(0.));
    maker.builder.add_argument(&vertex(10.));
    let completion =
        BuilderHooks::perform_with_progress(&mut maker, &ProgressRange::default()).unwrap();
    assert!(matches!(
        completion,
        ProgressCompletion::Incomplete {
            class: ProgressFailureClass::UnqualifiedStage,
            ..
        }
    ));
    assert!(maker.builder.pave_filler().is_none());
    assert!(maker.builder.shape().is_null());
}
#[test]
fn actual_filler_cancellation_is_not_replaced_by_the_merged_builder_report() {
    let mut builder = BOPAlgoBuilder::new();
    for _ in 0..2 {
        builder.add_argument(&vertex(0.));
    }
    let (progress, trace) = pulse_indicator(1);
    let completion = builder.perform_with_progress(&progress.start()).unwrap();
    assert!(matches!(
        completion,
        ProgressCompletion::Cancelled {
            operation: ProgressOperation::PaveFiller,
            ..
        }
    ));
    assert_eq!(trace.queries.load(Ordering::SeqCst), 1);
    assert!(builder.pave_filler().is_some());
    assert!(builder.options().has_errors());
}
#[test]
fn original_typed_intersection_failure_precedes_a_later_algorithm_report() {
    let cause = Failure::with_message("Standard_DomainError", "intersection sentinel");
    let original = ProgressCompletion::Failed {
        operation: ProgressOperation::PaveFiller,
        stage: ProgressStage::Intersections,
        class: ProgressFailureClass::KernelFailure,
        cause: Some(cause.clone()),
        context: None,
    };
    let reported = ProgressCompletion::Failed {
        operation: ProgressOperation::Builder,
        stage: ProgressStage::Completion,
        class: ProgressFailureClass::AlgorithmReport,
        cause: None,
        context: None,
    };
    assert_eq!(
        retain_intersection_completion(original.clone(), reported),
        original
    );
}
#[test]
fn a_real_builder_failure_can_follow_an_earlier_cancelled_child() {
    let cancelled = ProgressCompletion::Cancelled {
        operation: ProgressOperation::PaveFiller,
        stage: ProgressStage::Worker,
        context: None,
    };
    let failure = ProgressCompletion::Failed {
        operation: ProgressOperation::Builder,
        stage: ProgressStage::Worker,
        class: ProgressFailureClass::KernelFailure,
        cause: Some(Failure::new("builder sentinel")),
        context: None,
    };
    assert_eq!(
        retain_intersection_completion(cancelled, failure.clone()),
        failure
    );
}

#[test]
fn whole_box_partition_has_three_cells_and_keeps_both_shared_cell_parents() {
    for parallel in [false, true] {
        let mut case = super::fixture::whole_case(6, parallel);
        assert_eq!(
            case.receiver.perform(&ProgressRange::default()).unwrap(),
            ProgressCompletion::Completed
        );
        assert_three_cell_parents(case.receiver.builder(), &case.sources);
    }
}
fn assert_three_cell_parents(builder: &BOPAlgoBuilder, sources: &[Shape]) {
    let parts: Vec<_> = ShapeIterator::new(builder.shape(), true, true).collect();
    assert_eq!(parts.len(), 3);
    let first = builder.images().seek(&sources[0]).unwrap();
    let second = builder.images().seek(&sources[1]).unwrap();
    assert_eq!((first.len(), second.len()), (2, 2));
    let shared: Vec<_> = first
        .iter()
        .filter(|image| second.iter().any(|other| image.is_same(other)))
        .collect();
    assert_eq!(shared.len(), 1);
    let parents = builder.origins().seek(shared[0]).unwrap();
    assert_eq!(parents.len(), 2);
    assert!(
        sources
            .iter()
            .all(|source| parents.iter().any(|parent| parent.is_same(source)))
    );
    assert!(parts.iter().any(|part| part.is_same(shared[0])));
}

//! Prepared-filler operation controls exercise real geometry and actual virtual receivers.
use super::*;
use crate::bop_algo::{
    BOPAlgoBuilder, BOPAlgoSplitter, perform_with_filler_progress,
    progress_test_support::{indicator, pulse_indicator},
};
use martensite_tkbrep::topods::{Shape, ShapeIterator};
use martensite_tkmath::gp::Point3;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeShape, BRepBuilderAPIMakeVertex};
use std::sync::atomic::Ordering;

pub(super) fn vertex(x: f64) -> Shape {
    BRepBuilderAPIMakeVertex::new(Point3::new(x, 0., 0.))
        .unwrap()
        .vertex()
        .unwrap()
        .into_shape()
}
fn separated_boxes() -> [Shape; 2] {
    [0., 10.].map(|x| {
        BRepPrimAPIMakeBox::from_points(Point3::new(x, 0., 0.), Point3::new(x + 2., 3., 4.))
            .unwrap()
            .shape()
            .unwrap()
            .clone()
    })
}
pub(super) fn prepared_case(shapes: &[Shape]) -> (BOPAlgoBuilder, BOPAlgoPaveFiller) {
    let mut builder = BOPAlgoBuilder::new();
    let mut filler = BOPAlgoPaveFiller::new();
    for shape in shapes {
        builder.add_argument(shape);
        filler.add_argument(shape);
    }
    filler.perform();
    assert!(!filler.options().has_errors());
    (builder, filler)
}
fn result_children(builder: &BOPAlgoBuilder) -> Vec<Shape> {
    ShapeIterator::new(builder.shape(), true, true).collect()
}
#[test]
fn prepared_vertices_preserve_identity_and_history_policy() {
    let sources = [vertex(0.), vertex(10.)];
    for history in [false, true] {
        let (mut builder, mut filler) = prepared_case(&sources);
        builder.set_to_fill_history(history);
        let completion =
            perform_with_filler_progress(&mut builder, &mut filler, &ProgressRange::default())
                .unwrap();
        assert_eq!(completion, ProgressCompletion::Completed);
        let parts = result_children(&builder);
        assert_eq!(parts.len(), 2);
        assert!(
            sources
                .iter()
                .all(|source| parts.iter().any(|part| part.is_same(source)))
        );
        assert_eq!(builder.has_filled_history(), history);
        assert!(!builder.has_deleted());
        assert!(!builder.options().has_errors());
    }
}
#[test]
fn real_solids_complete_with_history_and_post_treatment_scopes() {
    let sources = separated_boxes();
    for parallel in [false, true] {
        let (mut builder, mut filler) = prepared_case(&sources);
        builder.options_mut().set_run_parallel(parallel);
        let (progress, trace) = indicator(0);
        let completion =
            perform_with_filler_progress(&mut builder, &mut filler, &progress.start()).unwrap();
        assert_eq!(completion, ProgressCompletion::Completed);
        assert_eq!(result_children(&builder).len(), 2);
        assert!(builder.has_filled_history());
        assert!(!builder.options().has_errors());
        assert_closed_box_sources(&builder, &sources);
        let events = trace.events.lock().unwrap();
        // Source-weight normalization accumulates fractions; execution success is checked above.
        assert!(
            events
                .windows(2)
                .all(|pair| f64::from_bits(pair[0].0) <= f64::from_bits(pair[1].0))
        );
        for name in [
            "Building the result of General Fuse operation",
            "Preparing history information",
            "Post treatment of result shape",
        ] {
            assert!(
                events
                    .iter()
                    .any(|(_, names)| names.iter().any(|found| found == name)),
                "missing {name}"
            );
        }
    }
}
fn assert_closed_box_sources(builder: &BOPAlgoBuilder, sources: &[Shape]) {
    for part in result_children(builder) {
        assert_eq!(part.shape_type().unwrap(), ShapeEnum::Solid);
        assert!(sources.iter().any(|source| source.is_same(&part)));
        let mut faces =
            martensite_tkbrep::topexp::Explorer::new(&part, ShapeEnum::Face, ShapeEnum::Shape)
                .unwrap();
        assert_eq!(faces.by_ref().count(), 6);
    }
}
#[test]
fn first_observed_cancellation_stops_history_and_stays_cancelled_at_one() {
    let (mut builder, mut filler) = prepared_case(&separated_boxes());
    let (progress, trace) = pulse_indicator(1);
    let completion =
        perform_with_filler_progress(&mut builder, &mut filler, &progress.start()).unwrap();
    assert!(matches!(completion, ProgressCompletion::Cancelled { .. }));
    assert!(builder.options().has_errors());
    assert!(!builder.has_filled_history());
    assert_eq!(trace.queries.load(Ordering::SeqCst), 1);
    let events = trace.events.lock().unwrap();
    assert_eq!(events.last().unwrap().0, 1f64.to_bits());
    assert!(!events.iter().any(|(_, names)| {
        names
            .iter()
            .any(|name| name == "Post treatment of result shape")
    }));
}
#[test]
fn splitter_keeps_single_argument_receiver_and_omits_tool() {
    let argument = vertex(0.);
    let tool = vertex(10.);
    let (_, mut filler) = prepared_case(&[argument.clone(), tool.clone()]);
    let mut splitter = BOPAlgoSplitter::new();
    splitter.add_argument(&argument);
    splitter.add_tool(&tool);
    let completion =
        perform_with_filler_progress(&mut splitter, &mut filler, &ProgressRange::default())
            .unwrap();
    assert_eq!(completion, ProgressCompletion::Completed);
    assert!(!splitter.options().has_errors());
    assert!(splitter.shape().is_same(&argument));
}
#[test]
fn unperformed_filler_retains_failure_type_and_existing_alert() {
    let (mut builder, _) = prepared_case(&[vertex(0.), vertex(10.)]);
    let mut filler = BOPAlgoPaveFiller::new();
    let failure =
        perform_with_filler_progress(&mut builder, &mut filler, &ProgressRange::default())
            .unwrap_err();
    assert_eq!(failure.exception_type(), "Standard_NullObject");
    assert!(builder.options().has_errors());
    assert!(!builder.has_filled_history());
}
struct LegacyOnlyReceiver {
    builder: BOPAlgoBuilder,
    legacy_calls: usize,
}
impl BuilderHooks for LegacyOnlyReceiver {
    fn general_fuse(&mut self) -> &mut BOPAlgoBuilder {
        &mut self.builder
    }
    fn perform_internal1(&mut self, _: &mut BOPAlgoPaveFiller) -> Result<(), Failure> {
        self.legacy_calls += 1;
        Ok(())
    }
}
#[test]
fn unknown_program_is_incomplete_while_legacy_entry_keeps_virtual_hook() {
    let (builder, mut filler) = prepared_case(&[vertex(0.), vertex(10.)]);
    let mut receiver = LegacyOnlyReceiver {
        builder,
        legacy_calls: 0,
    };
    let completion =
        perform_with_filler_progress(&mut receiver, &mut filler, &ProgressRange::default())
            .unwrap();
    assert!(matches!(completion, ProgressCompletion::Incomplete { .. }));
    assert_eq!(receiver.legacy_calls, 0);
    perform::perform_with(&mut receiver, &mut filler);
    assert_eq!(receiver.legacy_calls, 1);
}

#[test]
fn overlapping_boxes_build_three_cells_and_keep_both_parent_histories() {
    let sources = [0., 1.].map(|x| {
        BRepPrimAPIMakeBox::at_point(Point3::new(x, 0., 0.), 2., 3., 4.)
            .unwrap()
            .shape()
            .unwrap()
            .clone()
    });
    for parallel in [false, true] {
        let (mut builder, mut filler) = prepared_case(&sources);
        builder.options_mut().set_run_parallel(parallel);
        let completion =
            perform_with_filler_progress(&mut builder, &mut filler, &ProgressRange::default())
                .unwrap();
        assert_eq!(completion, ProgressCompletion::Completed);
        let cells = result_children(&builder);
        assert_eq!(cells.len(), 3);
        for cell in &cells {
            assert_eq!(cell.shape_type().unwrap(), ShapeEnum::Solid);
            assert_eq!(
                martensite_tkbrep::topexp::Explorer::new(cell, ShapeEnum::Face, ShapeEnum::Shape)
                    .unwrap()
                    .count(),
                6
            );
        }
        assert_shared_cell_history(&mut builder, &sources);
    }
}
fn assert_shared_cell_history(builder: &mut BOPAlgoBuilder, sources: &[Shape; 2]) {
    let first = builder.modified(&sources[0]).unwrap();
    let second = builder.modified(&sources[1]).unwrap();
    assert_eq!((first.len(), second.len()), (2, 2));
    assert_eq!(
        first
            .iter()
            .filter(|cell| second.iter().any(|other| cell.is_same(other)))
            .count(),
        1
    );
    assert!(!builder.is_deleted(&sources[0]).unwrap());
    assert!(!builder.is_deleted(&sources[1]).unwrap());
}

#[test]
fn history_cancellation_keeps_partial_diagnostics_and_blocks_post_treatment() {
    let (mut builder, mut filler) = prepared_case(&[vertex(0.), vertex(10.)]);
    let (progress, trace) =
        crate::bop_algo::progress_test_support::named_indicator("Preparing history information");
    let completion =
        perform_with_filler_progress(&mut builder, &mut filler, &progress.start()).unwrap();
    assert!(matches!(completion, ProgressCompletion::Cancelled { .. }));
    assert!(builder.has_filled_history());
    assert!(builder.options().has_errors());
    assert!(!trace.events.lock().unwrap().iter().any(|(_, names)| {
        names
            .iter()
            .any(|name| name == "Post treatment of result shape")
    }));
}
#[test]
fn request_during_query_free_post_treatment_does_not_fabricate_cancellation() {
    let (mut builder, mut filler) = prepared_case(&[vertex(0.), vertex(10.)]);
    let (progress, trace) =
        crate::bop_algo::progress_test_support::named_indicator("Post treatment of result shape");
    let completion =
        perform_with_filler_progress(&mut builder, &mut filler, &progress.start()).unwrap();
    assert_eq!(completion, ProgressCompletion::Completed);
    assert!(trace.cancelled.load(Ordering::SeqCst));
    assert!(!trace.break_observed.load(Ordering::SeqCst));
    assert!(!builder.options().has_errors());
}

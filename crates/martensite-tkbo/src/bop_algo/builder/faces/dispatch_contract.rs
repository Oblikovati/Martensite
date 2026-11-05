//! Actual face inputs test the composite caller, including parent cancellation.
use super::*;
use crate::bop_algo::progress_test_support::{indicator, pulse_indicator};
use crate::int_tools::Context;
use martensite_tkernel::standard::Handle;
use martensite_tkmath::gp::Plane3;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeFace;
use std::sync::atomic::Ordering;

fn face_case() -> (BOPAlgoBuilder, DataStructure) {
    let face = BRepBuilderAPIMakeFace::from_plane_bounds(Plane3::default(), (-2., 2., -2., 2.))
        .unwrap()
        .face()
        .unwrap();
    let mut structure = DataStructure::default();
    structure.set_arguments(&[face.into_shape()]);
    structure.init(1e-7).unwrap();
    let mut builder = BOPAlgoBuilder::new();
    builder.context = Handle::new(Context::default());
    (builder, structure)
}
#[test]
fn composite_face_entry_retains_complete_geometry_and_weighted_scopes() {
    let (mut builder, structure) = face_case();
    let (progress, trace) = indicator(0);
    let observed = ProgressCancellation::default();
    let outcome = builder
        .fill_images_faces_progress_raw(&structure, &progress.start(), &observed)
        .unwrap();
    assert_eq!(outcome, ProgressCompletion::Completed);
    assert!(!builder.options().has_errors());
    assert_eq!(builder.images().len(), 0);
    let events = trace.events.lock().unwrap();
    assert!(
        events
            .iter()
            .any(|(_, names)| names.iter().any(|n| n == "Filling splits of faces"))
    );
    assert_eq!(events.last().unwrap().0, 1.0f64.to_bits());
    assert_eq!(trace.queries.load(Ordering::SeqCst), 4);
}
#[test]
fn composite_face_cancellation_stops_before_later_stage_queries() {
    let (mut builder, structure) = face_case();
    let (progress, trace) = pulse_indicator(1);
    let observed = ProgressCancellation::default();
    let outcome = builder
        .fill_images_faces_progress_raw(&structure, &progress.start(), &observed)
        .unwrap();
    assert!(matches!(outcome, ProgressCompletion::Cancelled { .. }));
    assert_eq!(trace.queries.load(Ordering::SeqCst), 1);
    assert!(builder.images().is_empty());
}
#[test]
fn legacy_empty_face_entry_preserves_missing_context_refusal() {
    let mut builder = BOPAlgoBuilder::new();
    assert!(
        builder
            .fill_images_faces(&mut DataStructure::default())
            .is_err()
    );
}

#[test]
fn composite_face_reports_cannot_become_completed_results() {
    use crate::bop_algo::{AlertUserBreak, ProgressFailureClass, alerts::alert};
    let (mut builder, structure) = face_case();
    builder.options().add_error(&alert(AlertUserBreak));
    let outcome = builder
        .fill_images_faces_progress_raw(
            &structure,
            &ProgressRange::default(),
            &ProgressCancellation::default(),
        )
        .unwrap();
    assert!(matches!(
        outcome,
        ProgressCompletion::Failed {
            class: ProgressFailureClass::AlgorithmReport,
            ..
        }
    ));
}

#[test]
fn child_failure_cause_and_incomplete_status_survive_composition() {
    use crate::bop_algo::{ProgressFailureClass, ProgressStage};
    let cancelled = ProgressCompletion::Cancelled {
        operation: ProgressOperation::Builder,
        stage: ProgressStage::Images,
        context: None,
    };
    let failed = ProgressCompletion::Failed {
        operation: ProgressOperation::Builder,
        stage: ProgressStage::Images,
        class: ProgressFailureClass::KernelFailure,
        cause: Some(Failure::with_message(
            "Standard_ProgramError",
            "owned child cause",
        )),
        context: None,
    };
    assert_eq!(
        merge_face_outcomes(cancelled.clone(), failed.clone()),
        failed
    );
    let incomplete = ProgressCompletion::Incomplete {
        operation: ProgressOperation::Builder,
        stage: ProgressStage::Images,
        class: ProgressFailureClass::UnqualifiedStage,
        cause: None,
        context: None,
    };
    assert!(terminal_face_stop(&incomplete));
    assert_eq!(
        merge_face_outcomes(cancelled.clone(), incomplete.clone()),
        incomplete
    );
    assert_eq!(
        merge_face_outcomes(cancelled.clone(), ProgressCompletion::Completed),
        cancelled
    );
}

//! Closed-form cylindrical UVs and actual cancellation boundaries qualify the raw execution contract.
use super::fixture::{PCurveCase, pcurve_case};
use super::*;
use crate::bop_algo::progress_test_support::{indicator, pulse_indicator};
use std::sync::atomic::Ordering;
#[test]
fn cylindrical_pcurve_matches_the_analytic_vertical_generator() {
    for parallel in [false, true] {
        let mut case = pcurve_case(4, parallel);
        let (progress, _) = indicator(0);
        let observed = ProgressCancellation::default();
        let outcomes = case
            .filler
            .make_pcurves_with_progress(&progress.start(), &observed)
            .unwrap();
        assert_eq!(outcomes, vec![ProgressCompletion::Completed]);
        assert!(!observed.is_cancelled());
        assert_vertical_generator(&case);
    }
}
fn assert_vertical_generator(case: &PCurveCase) {
    let curve = BRepTool::curve_on_face(&case.edges[0], &case.faces[0]).unwrap();
    assert!(!curve.pcurve.is_null());
    assert_eq!((curve.first, curve.last), (0., 2.));
    for parameter in [0., 0.5, 1., 2.] {
        let uv = curve.pcurve.value(parameter).unwrap();
        assert_eq!([uv.x(), uv.y()], [0., parameter - 1.]);
    }
    assert_eq!(BRepTool::edge_tolerance(&case.edges[0]).unwrap(), 1e-7);
}
#[test]
fn cancelled_worker_becomes_a_warning_and_retains_the_cancelled_outcome() {
    let mut case = pcurve_case(4, false);
    let (progress, trace) = pulse_indicator(2);
    let observed = ProgressCancellation::default();
    let outcomes = case
        .filler
        .make_pcurves_with_progress(&progress.start(), &observed)
        .unwrap();
    assert!(matches!(
        outcomes.as_slice(),
        [ProgressCompletion::Cancelled { .. }]
    ));
    assert!(observed.is_cancelled());
    assert!(!case.filler.options().has_errors());
    assert!(case.filler.options().has_warnings());
    assert_eq!(trace.queries.load(Ordering::SeqCst), 3);
    assert!(
        BRepTool::curve_on_face(&case.edges[0], &case.faces[0])
            .unwrap()
            .pcurve
            .is_null()
    );
}
#[test]
fn storage_cancellation_does_not_publish_the_already_computed_pcurve() {
    let mut case = pcurve_case(4, false);
    let (progress, trace) = pulse_indicator(3);
    let observed = ProgressCancellation::default();
    let outcomes = case
        .filler
        .make_pcurves_with_progress(&progress.start(), &observed)
        .unwrap();
    assert_eq!(outcomes, vec![ProgressCompletion::Completed]);
    assert!(observed.is_cancelled());
    assert!(case.filler.options().has_errors());
    assert!(!case.filler.options().has_warnings());
    assert_eq!(trace.queries.load(Ordering::SeqCst), 3);
    assert!(
        BRepTool::curve_on_face(&case.edges[0], &case.faces[0])
            .unwrap()
            .pcurve
            .is_null()
    );
}
#[test]
fn repeated_section_edge_face_pairs_are_scheduled_once() {
    for parallel in [false, true] {
        let mut case = pcurve_case(8, parallel);
        let (progress, trace) = indicator(0);
        let observed = ProgressCancellation::default();
        let outcomes = case
            .filler
            .make_pcurves_with_progress(&progress.start(), &observed)
            .unwrap();
        assert_eq!(outcomes, vec![ProgressCompletion::Completed; 2]);
        assert_eq!(trace.queries.load(Ordering::SeqCst), 5);
        assert!(!case.filler.options().has_warnings());
        assert_vertical_generator(&case);
    }
}
#[test]
fn caught_curveless_projection_has_failed_completion_and_a_native_warning() {
    for parallel in [false, true] {
        let mut case = pcurve_case(10, parallel);
        let (progress, _) = indicator(0);
        let observed = ProgressCancellation::default();
        let outcomes = case
            .filler
            .make_pcurves_with_progress(&progress.start(), &observed)
            .unwrap();
        assert!(matches!(
            outcomes.as_slice(),
            [ProgressCompletion::Failed { .. }]
        ));
        assert!(!observed.is_cancelled());
        assert!(!case.filler.options().has_errors());
        assert!(case.filler.options().has_warnings());
    }
}
#[test]
fn avoiding_pcurves_does_not_require_a_structure_or_invent_a_query() {
    let mut filler = BOPAlgoPaveFiller::new();
    filler.set_avoid_build_pcurve(true);
    let (progress, trace) = pulse_indicator(1);
    let observed = ProgressCancellation::default();
    assert!(
        filler
            .make_pcurves_with_progress(&progress.start(), &observed)
            .unwrap()
            .is_empty()
    );
    assert!(!observed.is_cancelled());
    assert_eq!(trace.queries.load(Ordering::SeqCst), 0);
}
#[test]
fn required_missing_structure_returns_its_typed_failure_without_parent_report() {
    let mut filler = BOPAlgoPaveFiller::new();
    let observed = ProgressCancellation::default();
    let failure = filler
        .make_pcurves_with_progress(&ProgressRange::default(), &observed)
        .unwrap_err();
    assert_eq!(failure.exception_type(), "Standard_NullObject");
    assert!(!filler.options().has_errors());
    assert!(!observed.is_cancelled());
}

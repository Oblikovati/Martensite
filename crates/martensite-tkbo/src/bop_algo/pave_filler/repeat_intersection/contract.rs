//! Increased-source selection and real child intersections preserve geometry and report boundaries.
use super::*;
use crate::bop_algo::progress_test_support::{indicator, pulse_indicator};
use martensite_tkbrep::{brep::BRepTool, topods::Vertex};
use martensite_tkernel::standard::RuntimeTyped;
use martensite_tkg3d::top_abs::ShapeEnum;
use std::sync::atomic::Ordering;

pub(super) fn nearby_vertices() -> BOPAlgoPaveFiller {
    let mut filler = BOPAlgoPaveFiller::new();
    for bytes in [
        include_bytes!("../face_face_store_fixture/vertex_first.brep").as_slice(),
        include_bytes!("../face_face_store_fixture/vertex_second.brep").as_slice(),
    ] {
        filler.add_argument(
            &crate::bop_algo::pave_filler::owned_progress_fixture::read_owned_progress_shape(
                bytes,
                ShapeEnum::Compound,
            ),
        );
    }
    filler.options_mut().set_fuzzy_value(1e-6);
    filler.initialize().unwrap();
    filler
}
fn assert_merged_vertex(filler: &BOPAlgoPaveFiller) {
    let structure = filler.data_structure().unwrap();
    assert_eq!(structure.nb_shapes(), 5);
    assert_eq!(structure.shape_sd_items().count(), 2);
    assert_eq!(structure.interf_vv_records().len(), 1);
    let vertex = Vertex::unchecked(structure.shape(4).unwrap().clone());
    assert_eq!(
        *BRepTool::pnt(&vertex).unwrap().coordinates(),
        [2.5e-7, 0., 0.]
    );
    assert_eq!(BRepTool::vertex_tolerance(&vertex).unwrap(), 3.5e-7);
}
#[test]
fn no_increased_vertex_allocates_parent_scope_without_a_cancellation_query() {
    let mut filler = nearby_vertices();
    filler.increased_ss.add(0); // A compound is deliberately not an eligible increased source.
    let (progress, trace) = pulse_indicator(1);
    let observed = ProgressCancellation::default();
    let result = filler
        .repeat_intersection_with_progress(&progress.start(), &observed)
        .unwrap();
    assert_eq!(result, ProgressCompletion::Completed);
    assert_eq!(trace.queries.load(Ordering::SeqCst), 0);
    assert!(!observed.is_cancelled());
    assert_eq!(filler.data_structure().unwrap().nb_shapes(), 4);
    assert!(
        trace
            .events
            .lock()
            .unwrap()
            .iter()
            .any(|(_, names)| names.iter().any(|name| name == "Repeat intersection"))
    );
}
#[test]
fn increased_source_runs_actual_vertex_intersection_in_both_execution_settings() {
    for parallel in [false, true] {
        let mut filler = nearby_vertices();
        filler.options_mut().set_run_parallel(parallel);
        filler.increased_ss.add(1);
        let (progress, trace) = indicator(0);
        let result = filler
            .repeat_intersection_with_progress(&progress.start(), &ProgressCancellation::default())
            .unwrap();
        assert_eq!(result, ProgressCompletion::Completed);
        assert_eq!(trace.queries.load(Ordering::SeqCst), 4);
        assert_merged_vertex(&filler);
        assert!(!filler.options.has_errors());
    }
}
#[test]
fn every_actual_vertex_poll_cancels_without_later_repeat_stages() {
    for target in 1..=4 {
        let mut filler = nearby_vertices();
        filler.increased_ss.add(1);
        let (progress, trace) = pulse_indicator(target);
        let result = filler
            .repeat_intersection_with_progress(&progress.start(), &ProgressCancellation::default())
            .unwrap();
        assert!(matches!(result, ProgressCompletion::Cancelled { .. }));
        assert_eq!(trace.queries.load(Ordering::SeqCst), target);
        assert!(
            filler
                .options
                .has_error(crate::bop_algo::AlertUserBreak::static_type())
        );
        assert_eq!(
            filler.data_structure().unwrap().nb_shapes(),
            if target >= 3 { 5 } else { 4 }
        );
    }
}
#[test]
fn source_selected_through_increased_same_domain_representative() {
    let mut filler = nearby_vertices();
    filler.perform_vv().unwrap();
    filler.increased_ss.add(4);
    let selected = filler.increased_source_vertices().unwrap();
    assert_eq!(selected.extent(), 2);
    assert!(selected.contains(&1));
    assert!(selected.contains(&3));
    filler.repeat_intersection().unwrap();
    assert_merged_vertex(&filler);
}
#[test]
fn missing_structure_refuses_repeat_without_inventing_parent_alert() {
    let mut filler = BOPAlgoPaveFiller::new();
    let failure = filler
        .repeat_intersection_with_progress(
            &ProgressRange::default(),
            &ProgressCancellation::default(),
        )
        .unwrap_err();
    assert_eq!(failure.exception_type(), "Standard_NullObject");
    assert!(!filler.options.has_errors());
}

fn owned_child_failure() -> ProgressCompletion {
    ProgressCompletion::Failed {
        operation: ProgressOperation::PaveFiller,
        stage: crate::bop_algo::ProgressStage::Worker,
        class: crate::bop_algo::ProgressFailureClass::KernelFailure,
        cause: Some(Failure::with_message(
            "Standard_DomainError",
            "repeat worker sentinel",
        )),
        context: None,
    }
}
#[test]
fn child_failure_cause_survives_later_cancelled_and_incomplete_metadata() {
    let cancelled = ProgressCompletion::Cancelled {
        operation: ProgressOperation::PaveFiller,
        stage: crate::bop_algo::ProgressStage::Worker,
        context: None,
    };
    let incomplete = ProgressCompletion::Incomplete {
        operation: ProgressOperation::PaveFiller,
        stage: crate::bop_algo::ProgressStage::Worker,
        class: crate::bop_algo::ProgressFailureClass::UnqualifiedStage,
        cause: None,
        context: None,
    };
    let expected = owned_child_failure();
    let observed = ProgressCancellation::default();
    let actual = retain_repeat_outcomes(expected.clone(), vec![cancelled, incomplete], &observed);
    assert_eq!(actual, expected);
    assert!(observed.is_cancelled());
    let filler = nearby_vertices();
    assert_eq!(filler.finish_repeat(actual, &observed), expected);
    assert!(!filler.options.has_errors());
}

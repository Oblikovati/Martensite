//! Ordinary edges still own native post-work cancellation checks.
use super::*;
use crate::bop_algo::progress_test_support::{indicator, pulse_indicator};
use std::sync::atomic::Ordering;
fn ordinary_case(edge: bool) -> BOPAlgoPaveFiller {
    super::fixture::degenerate_case(usize::from(edge), false).filler
}
#[test]
fn ordinary_source_edge_queries_after_work_without_changing_geometry() {
    let mut filler = ordinary_case(true);
    let (progress, trace) = pulse_indicator(1);
    let observed = ProgressCancellation::default();
    filler
        .process_de_with_progress(&progress.start(), &observed)
        .unwrap();
    assert!(observed.is_cancelled());
    assert!(filler.options().has_errors());
    assert_eq!(trace.queries.load(Ordering::SeqCst), 1);
    assert_eq!(filler.data_structure().unwrap().nb_shapes(), 4);
}
#[test]
fn only_vertices_have_no_query_or_fabricated_cancellation() {
    let mut filler = ordinary_case(false);
    let (progress, trace) = pulse_indicator(1);
    let observed = ProgressCancellation::default();
    filler
        .process_de_with_progress(&progress.start(), &observed)
        .unwrap();
    assert!(!observed.is_cancelled());
    assert!(!filler.options().has_errors());
    assert_eq!(trace.queries.load(Ordering::SeqCst), 0);
}
#[test]
fn unobserved_next_query_keeps_ordinary_edge_completed() {
    let mut filler = ordinary_case(true);
    let (progress, trace) = indicator(2);
    let observed = ProgressCancellation::default();
    filler
        .process_de_with_progress(&progress.start(), &observed)
        .unwrap();
    assert!(!observed.is_cancelled());
    assert_eq!(trace.queries.load(Ordering::SeqCst), 1);
    assert!(!filler.options().has_errors());
}

#[test]
fn cancelled_degenerated_edge_already_has_its_copy_and_preserves_the_source_pole() {
    for parallel in [false, true] {
        let mut case = super::fixture::degenerate_case(2, parallel);
        let (progress, trace) = pulse_indicator(1);
        let observed = ProgressCancellation::default();
        case.filler
            .process_de_with_progress(&progress.start(), &observed)
            .unwrap();
        assert!(observed.is_cancelled());
        assert_eq!(trace.queries.load(Ordering::SeqCst), 1);
        assert_degenerated_copy(&case, false);
    }
}
#[test]
fn same_domain_pole_is_used_by_both_degenerated_owner_kinds() {
    for kind in [5, 6] {
        for parallel in [false, true] {
            let mut case = super::fixture::degenerate_case(kind, parallel);
            case.filler.process_de().unwrap();
            assert_degenerated_copy(&case, true);
        }
    }
}
fn assert_degenerated_copy(case: &super::fixture::DegenerateCase, mapped: bool) {
    let structure = case.filler.data_structure().unwrap();
    let block = structure.pave_blocks(case.original).unwrap()[0]
        .lock()
        .unwrap();
    assert_eq!(block.indices(), (case.mapped, case.mapped));
    let copy = Edge::unchecked(structure.shape(block.edge()).unwrap().clone());
    assert!(!copy.is_same(&case.source));
    assert!(BRepTool::degenerated(&copy).unwrap());
    assert_eq!(
        BRepTool::edge_tolerance(&copy).unwrap(),
        precision::confusion()
    );
    assert_copy_pole_identities(case, &copy, mapped);
    assert_conical_pole_and_interval(structure, case.pole, block.range());
}
fn assert_copy_pole_identities(case: &super::fixture::DegenerateCase, copy: &Edge, mapped: bool) {
    let structure = case.filler.data_structure().unwrap();
    let original_pole = structure.shape(case.pole).unwrap();
    let mapped_pole = structure.shape(case.mapped).unwrap();
    assert!(super::census::edge_contains(copy, mapped_pole));
    assert_eq!(super::census::edge_contains(copy, original_pole), !mapped);
    assert!(super::census::edge_contains(&case.source, original_pole));
    if mapped {
        assert!(!super::census::edge_contains(&case.source, mapped_pole));
    }
}
fn assert_conical_pole_and_interval(
    structure: &crate::bop_ds::DataStructure,
    pole: i32,
    range: (f64, f64),
) {
    let vertex = Vertex::unchecked(structure.shape(pole).unwrap().clone());
    let point = BRepTool::pnt(&vertex).unwrap();
    // Text serialization of the conical apex/range bounds contributes less than 5e-14 here.
    assert!(point.x().abs() < 5e-14 && point.y().abs() < 5e-14);
    assert_eq!(point.z(), 20.);
    assert_eq!(range.0, 0.);
    assert!((range.1 - std::f64::consts::TAU).abs() < 5e-14);
}
#[test]
fn untouched_degenerated_face_drops_its_pool_without_creating_geometry() {
    let mut case = super::fixture::degenerate_case(3, false);
    let original_count = case.filler.data_structure().unwrap().nb_shapes();
    case.filler.process_de().unwrap();
    let structure = case.filler.data_structure().unwrap();
    assert_eq!(structure.nb_shapes(), original_count);
    assert_eq!(structure.shape_info(case.original).unwrap().reference(), -1);
    assert!(structure.pave_blocks(case.original).unwrap().is_empty());
}
#[test]
fn empty_degenerated_pool_skips_its_query_but_ordinary_edges_still_poll() {
    let mut case = super::fixture::degenerate_case(4, false);
    let (progress, trace) = pulse_indicator(3);
    let observed = ProgressCancellation::default();
    case.filler
        .process_de_with_progress(&progress.start(), &observed)
        .unwrap();
    assert_eq!(trace.queries.load(Ordering::SeqCst), 2);
    assert!(!observed.is_cancelled());
    assert!(!case.filler.options().has_errors());
    let structure = case.filler.data_structure().unwrap();
    assert_eq!(structure.shape_info(case.original).unwrap().reference(), 0);
}
#[test]
fn missing_degenerated_structure_keeps_the_typed_failure_and_parent_report() {
    let mut filler = BOPAlgoPaveFiller::new();
    let observed = ProgressCancellation::default();
    let failure = filler
        .process_de_with_progress(&ProgressRange::default(), &observed)
        .unwrap_err();
    assert_eq!(failure.exception_type(), "Standard_NullObject");
    assert!(!filler.options().has_errors());
    assert!(!observed.is_cancelled());
}

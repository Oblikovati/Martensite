//! Independent set laws distinguish Cells from an inherited General Fuse result.
use super::fixture::cells_case;
use super::*;
use crate::bop_algo::{
    ProgressFailureClass,
    progress_test_support::{indicator, pulse_indicator},
};
use martensite_tkbrep::{topexp, topods::ShapeIterator, toptools::IndexedMapOfShape};
use martensite_tkg3d::top_abs::ShapeEnum;
use std::sync::atomic::Ordering;
fn part_count(shape: &Shape, kind: ShapeEnum) -> usize {
    let mut parts = IndexedMapOfShape::with_hasher(ShapeMapHasher, 1);
    topexp::map_shapes_of_type(shape, kind, &mut parts).unwrap();
    parts.len()
}
#[test]
fn distinct_vertices_are_indexed_but_not_selected_until_requested() {
    let mut case = cells_case(1, false);
    assert_eq!(
        case.perform(&ProgressRange::default()).unwrap(),
        ProgressCompletion::Completed
    );
    assert_eq!(part_count(case.cells.all_parts(), ShapeEnum::Vertex), 2);
    assert_eq!(
        ShapeIterator::new(case.cells.shape(), true, true).count(),
        0
    );
    assert!(case.cells.has_history());
    assert!(!case.cells.has_filled_history());
    let take = [case.sources[0].clone()].into_iter().collect();
    let avoid = [case.sources[1].clone()].into_iter().collect();
    case.cells.add_to_result(&take, &avoid, 17, false).unwrap();
    let selected: Vec<_> = ShapeIterator::new(case.cells.shape(), true, true).collect();
    assert_eq!(selected.len(), 1);
    assert!(selected[0].is_same(&case.sources[0]));
    assert!(case.cells.has_filled_history());
}
#[test]
fn crossing_segments_leave_four_edges_with_two_parts_per_original() {
    let mut case = cells_case(3, false);
    assert_eq!(
        case.perform(&ProgressRange::default()).unwrap(),
        ProgressCompletion::Completed
    );
    assert_eq!(part_count(case.cells.all_parts(), ShapeEnum::Edge), 4);
    assert_eq!(case.cells.index.extent(), 4);
    for source in &case.sources {
        let take = [source.clone()].into_iter().collect();
        case.cells
            .add_to_result(&take, &List::default(), 0, false)
            .unwrap();
        assert_eq!(part_count(case.cells.shape(), ShapeEnum::Edge), 2);
        case.cells.remove_all_from_result().unwrap();
    }
}
#[test]
fn overlapping_boxes_retain_three_cells_and_both_overlap_parents() {
    let mut case = cells_case(6, false);
    assert_eq!(
        case.perform(&ProgressRange::default()).unwrap(),
        ProgressCompletion::Completed
    );
    assert_eq!(part_count(case.cells.all_parts(), ShapeEnum::Solid), 3);
    assert_eq!(case.cells.index.extent(), 3);
    let origins: Vec<_> = (1..=3)
        .map(|i| case.cells.index.find_from_index(i).unwrap().extent())
        .collect();
    assert_eq!(origins.iter().filter(|count| **count == 2).count(), 1);
    assert_eq!(origins.iter().filter(|count| **count == 1).count(), 2);
}
#[test]
fn successful_history_off_and_failed_history_on_keep_their_actual_policies() {
    let mut disabled = cells_case(7, false);
    assert_eq!(
        disabled.perform(&ProgressRange::default()).unwrap(),
        ProgressCompletion::Completed
    );
    assert!(!disabled.cells.has_history());
    assert!(!disabled.cells.has_filled_history());
    let mut empty = cells_case(0, false);
    assert!(matches!(
        empty.perform(&ProgressRange::default()).unwrap(),
        ProgressCompletion::Failed {
            class: ProgressFailureClass::AlgorithmReport,
            ..
        }
    ));
    assert!(!empty.cells.has_history());
    assert!(empty.cells.all_parts().is_null());
}
#[test]
fn prepared_cancellation_keeps_history_disabled_and_has_no_indexed_parts() {
    let mut case = cells_case(20, false);
    let (indicator, trace) = pulse_indicator(1);
    assert!(matches!(
        case.perform(&indicator.start()).unwrap(),
        ProgressCompletion::Cancelled { .. }
    ));
    assert_eq!(trace.queries.load(Ordering::SeqCst), 1);
    assert!(case.cells.options().has_errors());
    assert!(!case.cells.has_history());
    assert!(case.cells.all_parts().is_null());
    assert!(case.cells.index.is_empty());
}
#[test]
fn whole_entry_keeps_the_cells_parent_scope_inside_general_fuse() {
    let mut case = cells_case(4, false);
    let (indicator, trace) = indicator(0);
    assert_eq!(
        case.perform(&indicator.start()).unwrap(),
        ProgressCompletion::Completed
    );
    let events = trace.events.lock().unwrap();
    for expected in [
        "Performing General Fuse operation",
        "Performing MakeCells operation",
        "Building the result of General Fuse operation",
    ] {
        assert!(
            events
                .iter()
                .any(|(_, names)| names.iter().any(|name| name == expected)),
            "missing {expected}"
        );
    }
}
#[test]
fn a_parallel_interior_pulse_never_becomes_a_successful_completion() {
    for kind in 20..28 {
        let mut case = cells_case(kind, true);
        let (indicator, trace) = pulse_indicator(32);
        let result = case.perform(&indicator.start());
        assert!(trace.break_observed.load(Ordering::SeqCst));
        assert!(!matches!(result, Ok(ProgressCompletion::Completed)));
        assert!(case.cells.pave_filler().is_none());
    }
}

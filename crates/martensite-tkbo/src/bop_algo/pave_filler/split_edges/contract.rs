//! Crossing straight segments give four independently known split ranges and endpoint positions.
use super::*;
use crate::bop_algo::progress_test_support::{indicator, pulse_indicator};
use martensite_tkbrep::brep::BRepTool;
use martensite_tkmath::gp::Point3;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeEdge;
use std::sync::atomic::Ordering;
pub(super) fn crossing_case(parallel: bool) -> BOPAlgoPaveFiller {
    let mut filler = BOPAlgoPaveFiller::new();
    for points in [
        [Point3::new(-1., 0., 0.), Point3::new(1., 0., 0.)],
        [Point3::new(0., -1., 0.), Point3::new(0., 1., 0.)],
    ] {
        filler.add_argument(
            &BRepBuilderAPIMakeEdge::from_points(points[0], points[1])
                .unwrap()
                .edge()
                .unwrap()
                .into_shape(),
        );
    }
    filler.options_mut().set_run_parallel(parallel);
    filler.initialize().unwrap();
    filler.perform_ee().unwrap();
    filler.update_pave_blocks_with_sd_vertices().unwrap();
    assert!(!filler.options().has_errors());
    filler
}
#[test]
fn crossing_edges_create_four_exact_ranges_in_both_execution_settings() {
    for parallel in [false, true] {
        let mut filler = crossing_case(parallel);
        let (progress, _) = indicator(0);
        let observed = ProgressCancellation::default();
        let outcomes = filler
            .make_split_edges_with_progress(&progress.start(), &observed)
            .unwrap();
        assert_eq!(outcomes, vec![ProgressCompletion::Completed; 4]);
        assert!(!observed.is_cancelled());
        let structure = filler.data_structure().unwrap();
        assert_eq!(structure.nb_shapes(), 11);
        assert_crossing_outputs(structure);
    }
}
fn assert_crossing_outputs(structure: &crate::bop_ds::DataStructure) {
    for original in [0, 3] {
        let blocks = structure.pave_blocks(original).unwrap();
        assert_eq!(blocks.len(), 2);
        for (block, range) in blocks.iter().zip([(0., 1.), (1., 2.)]) {
            let locked = block.lock().unwrap();
            let edge = Edge::unchecked(structure.shape(locked.edge()).unwrap().clone());
            let (curve, first, last) = BRepTool::curve(&edge).unwrap();
            assert_eq!((first, last), range);
            let axis = if original == 0 {
                [1., 0., 0.]
            } else {
                [0., 1., 0.]
            };
            let actual = [first, last].map(|parameter| {
                *curve
                    .get()
                    .unwrap()
                    .eval_d0(parameter)
                    .unwrap()
                    .coordinates()
            });
            let expected = [first, last]
                .map(|parameter| [axis[0] * (parameter - 1.), axis[1] * (parameter - 1.), 0.]);
            assert_eq!(actual, expected);
        }
    }
}
#[test]
fn first_pool_poll_stops_before_scheduling_or_storage() {
    let mut filler = crossing_case(false);
    let (progress, trace) = pulse_indicator(1);
    let observed = ProgressCancellation::default();
    assert!(
        filler
            .make_split_edges_with_progress(&progress.start(), &observed)
            .unwrap()
            .is_empty()
    );
    assert!(observed.is_cancelled());
    assert_eq!(trace.queries.load(Ordering::SeqCst), 1);
    assert_eq!(filler.data_structure().unwrap().nb_shapes(), 7);
}

#[test]
fn missing_structure_keeps_typed_failure_without_a_parent_report() {
    let mut filler = BOPAlgoPaveFiller::new();
    let failure = filler
        .make_split_edges_with_progress(&ProgressRange::default(), &ProgressCancellation::default())
        .unwrap_err();
    assert_eq!(failure.exception_type(), "Standard_NullObject");
    assert!(!filler.options().has_errors());
}
#[test]
fn non_destructive_common_block_splits_while_ordinary_mode_reuses_original() {
    for (kind, expected_tasks) in [(4, 0), (5, 1)] {
        let mut case = super::fixture::split_case(kind, false);
        let outcomes = case
            .filler
            .make_split_edges_with_progress(
                &ProgressRange::default(),
                &ProgressCancellation::default(),
            )
            .unwrap();
        assert_eq!(outcomes.len(), expected_tasks);
        assert!(
            outcomes
                .iter()
                .all(|outcome| *outcome == ProgressCompletion::Completed)
        );
        let structure = case.filler.data_structure().unwrap();
        let edge = structure.pave_blocks(case.originals[0].0).unwrap()[0]
            .lock()
            .unwrap()
            .edge();
        assert_eq!(
            structure.shape(edge).unwrap().is_same(&case.originals[0].1),
            kind == 4
        );
        for (_, source) in &case.originals {
            let (_, first, last) = BRepTool::curve(source).unwrap();
            assert_eq!((first, last), (0., 2.));
        }
    }
}

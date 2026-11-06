//! Coincident finite edges exercise the real forced intersection and common-block identities.
use super::*;
use crate::bop_algo::progress_test_support::{indicator, pulse_indicator};
use martensite_tkernel::standard::RuntimeTyped;
use martensite_tkmath::gp::Point3;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeEdge;
use std::sync::atomic::Ordering;

pub(super) fn coincident_case(count: usize, parallel: bool) -> BOPAlgoPaveFiller {
    let mut filler = BOPAlgoPaveFiller::new();
    for _ in 0..count {
        let edge = BRepBuilderAPIMakeEdge::from_points(Point3::default(), Point3::new(2., 0., 0.))
            .unwrap()
            .edge()
            .unwrap();
        filler.add_argument(&edge.into_shape());
    }
    filler.options_mut().set_fuzzy_value(1e-6);
    filler.options_mut().set_run_parallel(parallel);
    filler.initialize().unwrap();
    filler.perform_vv().unwrap();
    filler.update_pave_blocks_with_sd_vertices().unwrap();
    assert!(!filler.options().has_errors());
    filler
}
pub(super) fn assert_bound_coincidence(filler: &BOPAlgoPaveFiller, count: usize) {
    let structure = filler.data_structure().unwrap();
    let mut blocks = Vec::new();
    for index in 0..structure.nb_source_shapes() {
        if structure.shape_info(index).unwrap().shape_type() == ShapeEnum::Edge {
            blocks.push(Arc::clone(&structure.pave_blocks(index).unwrap()[0]));
        }
    }
    assert_eq!(blocks.len(), count);
    let common = structure.common_block(&blocks[0]).unwrap();
    assert_eq!(common.lock().unwrap().pave_blocks().len(), count);
    for block in &blocks {
        assert!(Arc::ptr_eq(
            &common,
            &structure.common_block(block).unwrap()
        ));
    }
    assert_eq!(structure.interf_ee_records().len(), count * (count - 1) / 2);
    for record in structure.interf_ee_records().iter() {
        assert_eq!(record.common_part().shape_type(), ShapeEnum::Edge);
    }
}
#[test]
fn real_coincident_tasks_bind_one_common_block_in_both_execution_settings() {
    for parallel in [false, true] {
        for count in [2, 3] {
            let mut filler = coincident_case(count, parallel);
            let (progress, trace) = indicator(0);
            let observed = ProgressCancellation::default();
            let outcomes = filler
                .force_interf_ee_with_progress(&progress.start(), &observed)
                .unwrap();
            assert_eq!(outcomes.len(), count * (count - 1) / 2);
            assert!(
                outcomes
                    .iter()
                    .all(|outcome| *outcome == ProgressCompletion::Completed)
            );
            assert!(!observed.is_cancelled());
            assert_bound_coincidence(&filler, count);
            assert!(trace.events.lock().unwrap().iter().any(|(_, names)| {
                names
                    .iter()
                    .any(|name| name == "Checking for coinciding edges")
            }));
        }
    }
}
#[test]
fn first_source_poll_prevents_task_scheduling_without_erasing_prepared_blocks() {
    let mut filler = coincident_case(2, false);
    let (progress, trace) = pulse_indicator(1);
    let observed = ProgressCancellation::default();
    let outcomes = filler
        .force_interf_ee_with_progress(&progress.start(), &observed)
        .unwrap();
    assert!(outcomes.is_empty());
    assert!(observed.is_cancelled());
    assert!(
        filler
            .options()
            .has_error(crate::bop_algo::AlertUserBreak::static_type())
    );
    assert_eq!(trace.queries.load(Ordering::SeqCst), 1);
    assert!(
        filler
            .data_structure()
            .unwrap()
            .interf_ee_records()
            .is_empty()
    );
}
#[test]
fn original_unbound_entry_keeps_the_same_common_block_geometry() {
    let mut filler = coincident_case(2, false);
    filler.force_interf_ee().unwrap();
    assert_bound_coincidence(&filler, 2);
    assert!(!filler.options().has_errors());
}

#[test]
fn exact_common_ranges_and_original_edge_identities_are_retained() {
    let mut filler = coincident_case(3, true);
    filler.force_interf_ee().unwrap();
    let structure = filler.data_structure().unwrap();
    for record in structure.interf_ee_records().iter() {
        let part = record.common_part();
        assert_eq!(part.range1_bounds(), (0., 2.));
        assert_eq!(part.ranges2().len(), 1);
        assert_eq!(part.ranges2()[0].range(), (0., 2.));
        assert!(
            part.edge1()
                .is_same(structure.shape(record.index1()).unwrap())
        );
        assert!(
            part.edge2()
                .is_same(structure.shape(record.index2()).unwrap())
        );
    }
}
#[test]
fn missing_structure_is_a_typed_failure_without_an_invented_report() {
    let mut filler = BOPAlgoPaveFiller::new();
    let failure = filler
        .force_interf_ee_with_progress(&ProgressRange::default(), &ProgressCancellation::default())
        .unwrap_err();
    assert_eq!(failure.exception_type(), "Standard_NullObject");
    assert!(!filler.options().has_errors());
}

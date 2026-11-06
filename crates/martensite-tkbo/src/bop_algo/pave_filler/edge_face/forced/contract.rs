//! Finite-plane fixtures exercise both overloads without substituting the section policy.
use super::*;
use crate::bop_algo::progress_test_support::{indicator, pulse_indicator};
use martensite_tkmath::gp::{Direction3, Plane3, Point3};
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeFace};
use std::sync::atomic::Ordering;
pub(super) fn on_plane_case(count: usize, parallel: bool) -> (BOPAlgoPaveFiller, i32) {
    let mut filler = BOPAlgoPaveFiller::new();
    for i in 0..count {
        let edge = BRepBuilderAPIMakeEdge::from_points(
            Point3::new(-1., i as f64 * 0.5, 0.),
            Point3::new(1., i as f64 * 0.5, 0.),
        )
        .unwrap()
        .edge()
        .unwrap();
        filler.add_argument(&edge.into_shape());
    }
    let plane = Plane3::new(Point3::default(), Direction3::Z).unwrap();
    let face = BRepBuilderAPIMakeFace::from_plane_bounds(plane, (-2., 2., -2., 2.))
        .unwrap()
        .face()
        .unwrap()
        .into_shape();
    filler.add_argument(&face);
    filler.options_mut().set_run_parallel(parallel);
    filler.initialize().unwrap();
    filler.perform_vf().unwrap();
    let structure = filler.data_structure.as_mut().unwrap();
    let face_index = structure.index(&face);
    for index in 0..face_index {
        if structure.shape_info(index).unwrap().shape_type() == ShapeEnum::Edge {
            structure.change_pave_blocks(index).unwrap();
        }
    }
    (filler, face_index)
}
pub(super) fn input_blocks(filler: &BOPAlgoPaveFiller, face: i32) -> PaveBlockIndexMap {
    let structure = filler.data_structure().unwrap();
    let mut blocks = PaveBlockIndexMap::default();
    for index in 0..face {
        if structure.shape_info(index).unwrap().shape_type() != ShapeEnum::Edge {
            continue;
        }
        for block in structure.pave_blocks(index).unwrap() {
            blocks.add(Arc::clone(block));
        }
    }
    blocks
}
#[test]
fn both_record_policies_keep_the_actual_in_block_and_source_identity() {
    for add_interf in [false, true] {
        let (mut filler, face) = on_plane_case(1, false);
        if !add_interf {
            filler.is_primary = false;
        }
        let blocks = input_blocks(&filler, face);
        let (progress, _) = indicator(0);
        let outcomes = filler
            .force_interf_ef_blocks_with_progress(
                &blocks,
                add_interf,
                &progress.start(),
                &ProgressCancellation::default(),
            )
            .unwrap();
        assert_eq!(outcomes, [ProgressCompletion::Completed]);
        let structure = filler.data_structure().unwrap();
        let info = structure.face_info(face).unwrap();
        assert_eq!(info.pave_blocks_in().extent(), 1);
        assert!(info.pave_blocks_in().contains(blocks.find_key(1).unwrap()));
        assert_eq!(structure.interf_ef_records().len(), usize::from(add_interf));
        assert_eq!(
            structure
                .common_block(blocks.find_key(1).unwrap())
                .is_some(),
            add_interf
        );
    }
}
#[test]
fn primary_top_entry_uses_actual_context_workers_in_both_execution_settings() {
    for parallel in [false, true] {
        let (mut filler, face) = on_plane_case(3, parallel);
        let outcomes = filler
            .force_interf_ef_with_progress(
                &ProgressRange::default(),
                &ProgressCancellation::default(),
            )
            .unwrap();
        assert_eq!(outcomes, vec![ProgressCompletion::Completed; 3]);
        assert_eq!(
            filler
                .data_structure()
                .unwrap()
                .face_info(face)
                .unwrap()
                .pave_blocks_in()
                .extent(),
            3
        );
        assert_eq!(
            filler.data_structure().unwrap().interf_ef_records().len(),
            3
        );
    }
}
#[test]
fn non_primary_top_entry_has_no_query_even_with_missing_structure() {
    let mut filler = BOPAlgoPaveFiller::new();
    filler.is_primary = false;
    let (progress, trace) = pulse_indicator(1);
    let outcomes = filler
        .force_interf_ef_with_progress(&progress.start(), &ProgressCancellation::default())
        .unwrap();
    assert!(outcomes.is_empty());
    assert_eq!(trace.queries.load(Ordering::SeqCst), 0);
    assert!(!filler.options().has_errors());
}

#[test]
fn empty_block_entry_has_no_query_even_without_a_structure() {
    let mut filler = BOPAlgoPaveFiller::new();
    let (progress, trace) = pulse_indicator(1);
    let outcomes = filler
        .force_interf_ef_blocks_with_progress(
            &PaveBlockIndexMap::default(),
            false,
            &progress.start(),
            &ProgressCancellation::default(),
        )
        .unwrap();
    assert!(outcomes.is_empty());
    assert_eq!(trace.queries.load(Ordering::SeqCst), 0);
    assert!(!filler.options().has_errors());
}
#[test]
fn primary_missing_structure_retains_typed_cause_without_parent_alert() {
    let mut filler = BOPAlgoPaveFiller::new();
    let failure = filler
        .force_interf_ef_with_progress(&ProgressRange::default(), &ProgressCancellation::default())
        .unwrap_err();
    assert_eq!(failure.exception_type(), "Standard_NullObject");
    assert!(!filler.options().has_errors());
}
#[test]
fn original_unbound_entry_keeps_exact_range_source_edge_and_face_relation() {
    let (mut filler, face) = on_plane_case(1, false);
    let source = filler.arguments().iter().next().unwrap().clone();
    let blocks = input_blocks(&filler, face);
    filler.force_interf_ef().unwrap();
    let structure = filler.data_structure().unwrap();
    assert_eq!(structure.interf_ef_records().len(), 1);
    for record in structure.interf_ef_records().iter() {
        assert_eq!(
            (record.index1(), record.index2()),
            (structure.index(&source), face)
        );
        assert_eq!(record.common_part().range1_bounds(), (0., 2.));
        assert!(record.common_part().edge1().is_same(&source));
    }
    let common = structure.common_block(blocks.find_key(1).unwrap()).unwrap();
    assert_eq!(common.lock().unwrap().faces(), &[face]);
}

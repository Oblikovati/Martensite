//! Closed forms and explicit nested-stop states supplement native progress equivalence.
use super::*;
use crate::bop_algo::{
    AlertPostTreatFF, AlertUserBreak,
    progress_test_support::{indicator, pulse_indicator},
};
use martensite_tkbrep::{
    brep::BRepTool,
    topods::{Edge, Face, Shape},
};
use martensite_tkernel::{message::ProgressRange, standard::RuntimeTyped};
use std::sync::atomic::Ordering;
#[test]
fn crossed_segments_keep_four_closed_form_intervals_after_the_whole_program() {
    for parallel in [false, true] {
        let mut filler = super::fixture::intersection_case(3, parallel);
        let sources: Vec<_> = filler.arguments().iter().cloned().collect();
        assert_eq!(
            filler.perform_on_progress(&ProgressRange::default()),
            ProgressCompletion::Completed
        );
        let structure = filler.data_structure().unwrap();
        assert_eq!(structure.nb_shapes(), 11);
        for source in &sources {
            assert_crossing_intervals(structure, source);
        }
    }
}
fn assert_crossing_intervals(structure: &crate::bop_ds::DataStructure, source: &Shape) {
    let index = structure.index(source);
    let blocks = structure.pave_blocks(index).unwrap();
    assert_eq!(blocks.len(), 2);
    for (block, expected) in blocks.iter().zip([(0., 1.), (1., 2.)]) {
        let block = block.lock().unwrap();
        assert_eq!(block.range(), expected);
        let edge = Edge::unchecked(structure.shape(block.edge()).unwrap().clone());
        let (_, first, last) = BRepTool::curve(&edge).unwrap();
        assert_eq!((first, last), expected);
    }
    let (_, first, last) = BRepTool::curve(&Edge::unchecked(source.clone())).unwrap();
    assert_eq!((first, last), (0., 2.));
}
#[test]
fn perpendicular_faces_have_the_closed_form_section_and_consistent_pcurves() {
    for parallel in [false, true] {
        let mut filler = super::fixture::intersection_case(4, parallel);
        assert_eq!(
            filler.perform_on_progress(&ProgressRange::default()),
            ProgressCompletion::Completed
        );
        assert_plane_section(&filler);
    }
}
fn assert_plane_section(filler: &BOPAlgoPaveFiller) {
    let structure = filler.data_structure().unwrap();
    let record = &structure.interf_ff_records()[0];
    let blocks: Vec<_> = record
        .curves()
        .iter()
        .flat_map(|curve| curve.pave_blocks())
        .collect();
    assert_eq!(blocks.len(), 1);
    let block = blocks[0].lock().unwrap();
    let edge = Edge::unchecked(structure.shape(block.edge()).unwrap().clone());
    let (curve, first, last) = BRepTool::curve(&edge).unwrap();
    assert_eq!((first, last), (-1., 1.));
    for parameter in [first, 0., last] {
        assert_section_point(curve.eval_d0(parameter).unwrap(), parameter);
    }
    let (first_face, second_face) = record.indices();
    for index in [first_face, second_face] {
        assert_sampled_pcurve(
            &edge,
            &Face::unchecked(structure.shape(index).unwrap().clone()),
        );
    }
}
fn assert_section_point(point: martensite_tkmath::gp::Point3, parameter: f64) {
    assert_eq!(point.x(), 0.);
    assert_eq!(point.z(), 0.);
    assert_eq!(point.y().abs(), parameter.abs());
}
fn assert_sampled_pcurve(edge: &Edge, face: &Face) {
    let (curve, first, last) = BRepTool::curve(edge).unwrap();
    let pcurve = BRepTool::curve_on_face(edge, face).unwrap();
    let surface = BRepTool::surface(face).unwrap();
    assert!(!pcurve.pcurve.is_null());
    for parameter in [first, 0., last] {
        let uv = pcurve.pcurve.value(parameter).unwrap();
        let actual = surface.value(uv.x(), uv.y()).unwrap();
        assert_eq!(actual, curve.eval_d0(parameter).unwrap());
    }
}
#[test]
fn nested_root_cancellation_keeps_post_treatment_report_and_actual_observation() {
    let mut filler = super::fixture::intersection_case(24, false);
    let (progress, trace) = pulse_indicator(4);
    let observed = ProgressCancellation::default();
    let outcomes = filler
        .make_blocks_with_progress(&progress.start(), &observed)
        .unwrap();
    assert!(observed.is_cancelled());
    assert_eq!(trace.queries.load(Ordering::SeqCst), 4);
    assert!(filler.options().has_error(AlertPostTreatFF::static_type()));
    assert!(!filler.options().has_error(AlertUserBreak::static_type()));
    assert!(matches!(
        outcomes.as_slice(),
        [ProgressCompletion::Cancelled { .. }]
    ));
}
#[test]
fn nested_worker_pulse_survives_report_clear_completion_of_parent_geometry() {
    let mut filler = super::fixture::intersection_case(24, false);
    let (progress, trace) = pulse_indicator(12);
    let observed = ProgressCancellation::default();
    let outcomes = filler
        .make_blocks_with_progress(&progress.start(), &observed)
        .unwrap();
    assert!(observed.is_cancelled());
    assert_eq!(trace.queries.load(Ordering::SeqCst), 76);
    assert!(!filler.options().has_errors());
    assert!(
        outcomes
            .iter()
            .any(|outcome| matches!(outcome, ProgressCompletion::Cancelled { .. }))
    );
    assert_eq!(filler.data_structure().unwrap().nb_shapes(), 67);
}
#[test]
fn empty_pair_program_does_not_infer_cancellation_from_a_completed_bar() {
    let mut filler = super::fixture::intersection_case(1, false);
    let (progress, trace) = indicator(1);
    assert_eq!(
        filler.perform_on_progress(&progress.start()),
        ProgressCompletion::Completed
    );
    assert_eq!(trace.queries.load(Ordering::SeqCst), 0);
}

#[test]
fn three_plane_sections_share_one_actual_incident_center_vertex() {
    for parallel in [false, true] {
        let mut filler = super::fixture::intersection_case(5, parallel);
        assert_eq!(
            filler.perform_on_progress(&ProgressRange::default()),
            ProgressCompletion::Completed
        );
        assert_shared_section_center(&filler);
    }
}
fn assert_shared_section_center(filler: &BOPAlgoPaveFiller) {
    let structure = filler.data_structure().unwrap();
    let mut centers = std::collections::HashSet::new();
    assert_eq!(structure.interf_ff_records().len(), 3);
    for record in structure.interf_ff_records().iter() {
        let blocks: Vec<_> = record
            .curves()
            .iter()
            .flat_map(|curve| curve.pave_blocks())
            .collect();
        assert_eq!(blocks.len(), 2);
        let first = blocks[0].lock().unwrap().indices();
        let second = blocks[1].lock().unwrap().indices();
        assert_eq!(first.1, second.0);
        centers.insert(first.1);
    }
    assert_eq!(centers.len(), 1);
    let center = martensite_tkbrep::topods::Vertex::unchecked(
        structure
            .shape(*centers.iter().next().unwrap())
            .unwrap()
            .clone(),
    );
    assert_eq!(
        BRepTool::pnt(&center).unwrap(),
        martensite_tkmath::gp::Point3::default()
    );
}

#[test]
fn prior_parent_cancellation_is_not_assigned_to_a_completed_nested_call() {
    let mut filler = super::fixture::intersection_case(24, false);
    let observed = ProgressCancellation::default();
    observed.record_cancelled();
    let outcomes = filler
        .make_blocks_with_progress(&ProgressRange::default(), &observed)
        .unwrap();
    assert!(
        outcomes
            .iter()
            .all(|outcome| *outcome == ProgressCompletion::Completed)
    );
    assert!(observed.is_cancelled());
    assert!(!filler.options().has_errors());
}

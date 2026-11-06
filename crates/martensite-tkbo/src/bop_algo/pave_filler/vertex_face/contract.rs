//! Owned planar VF expectations and actual one-shot cancellation boundaries.
use super::*;
use crate::bop_algo::AlertUserBreak;
use martensite_tkbrep::{brep::BRepBuilder, topods::Vertex};
use martensite_tkernel::{
    message::{ProgressIndicator, ProgressIndicatorHooks, ProgressScope},
    standard::RuntimeTyped,
};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::{Plane3, Point3};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeFace;
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

struct VertexFaceBreakProbe {
    target: usize,
    polls: Arc<AtomicUsize>,
    shows: Arc<AtomicUsize>,
    journal: super::census::VfCensusJournal,
}

impl ProgressIndicatorHooks for VertexFaceBreakProbe {
    fn user_break(&self) -> bool {
        let ordinal = self.polls.fetch_add(1, Ordering::SeqCst) + 1;
        ordinal == self.target
    }

    fn show(&self, scope: &ProgressScope<'_>, position: f64, _: bool) {
        // Progress callbacks must never be counted as algorithm cancellation queries.
        self.shows.fetch_add(1, Ordering::SeqCst);
        self.journal.record(scope, position);
    }
}

#[test]
fn vertex_face_completed_projection_matches_owned_plane_and_native_poll_formula() {
    for parallel in [false, true] {
        let mut filler = planar_vertex_face_fixture(3, parallel);
        let candidates = vf_candidate_count(&mut filler);
        let (outcomes, polls, cancelled) = run_vf_census(&mut filler, 0);
        assert_eq!(outcomes, vec![ProgressCompletion::Completed; 3]);
        assert_eq!(polls, candidates + 3 + 1 + 3);
        assert!(!cancelled);
        assert_planar_vertex_face_records(&filler, 3);
    }
}

#[test]
fn vertex_face_one_shot_break_keeps_native_task_warning_and_parent_error_distinct() {
    for boundary in 1..=4 {
        let mut filler = planar_vertex_face_fixture(1, false);
        assert_eq!(vf_candidate_count(&mut filler), 1);
        let (outcomes, polls, cancelled) = run_vf_census(&mut filler, boundary);
        assert!(cancelled);
        assert!(polls >= boundary);
        assert_vf_cancel_boundary(&filler, &outcomes, boundary);
    }
}

fn assert_vf_cancel_boundary(
    filler: &BOPAlgoPaveFiller,
    outcomes: &[ProgressCompletion],
    boundary: usize,
) {
    let options = filler.options();
    let task_break = boundary == 2;
    assert_eq!(
        options.has_error(AlertUserBreak::static_type()),
        !task_break
    );
    assert_eq!(options.has_warnings(), task_break);
    if task_break {
        assert!(matches!(outcomes, [ProgressCompletion::Cancelled { .. }]));
    }
    assert_eq!(
        filler.data_structure().unwrap().interf_vf_records().len(),
        0
    );
}

#[test]
fn vertex_face_glue_and_no_candidates_do_not_invent_cancellation_polls() {
    for count in [0, 1] {
        let mut filler = planar_vertex_face_fixture(count, false);
        if count == 1 {
            filler.set_glue(Glue::Full);
        }
        let (outcomes, polls, cancelled) = run_vf_census(&mut filler, 1);
        assert!(outcomes.is_empty());
        assert_eq!(polls, 0);
        assert!(!cancelled);
        assert!(!filler.options().has_errors());
    }
}

fn run_vf_census(
    filler: &mut BOPAlgoPaveFiller,
    target: usize,
) -> (Vec<ProgressCompletion>, usize, bool) {
    run_vf_profile_census(filler, target, "owned-control")
}

fn run_vf_profile_census(
    filler: &mut BOPAlgoPaveFiller,
    target: usize,
    profile: &str,
) -> (Vec<ProgressCompletion>, usize, bool) {
    let polls = Arc::new(AtomicUsize::new(0));
    let shows = Arc::new(AtomicUsize::new(0));
    let (journal, indicator) = vf_census_observer(target, &polls, &shows);
    let range = indicator.start();
    let observed = ProgressCancellation::default();
    let outcomes = filler.perform_vf_with_progress(&range, &observed).unwrap();
    let result = (
        outcomes,
        polls.load(Ordering::SeqCst),
        observed.is_cancelled(),
    );
    assert!(shows.load(Ordering::SeqCst) > 0);
    journal.emit(filler, profile, target, result.1);
    result
}

fn vf_candidate_count(filler: &mut BOPAlgoPaveFiller) -> usize {
    let iterator = filler.iterator_mut().unwrap();
    iterator.initialize(ShapeEnum::Vertex, ShapeEnum::Face);
    iterator.expected_length() as usize
}

pub(super) fn planar_vertex_face_fixture(count: usize, parallel: bool) -> BOPAlgoPaveFiller {
    let glue = Glue::Off;
    configured_planar_vf_fixture(count, parallel, glue)
}

fn configured_planar_vf_fixture(count: usize, parallel: bool, glue: Glue) -> BOPAlgoPaveFiller {
    let mut filler = BOPAlgoPaveFiller::new();
    for ordinal in 0..count {
        let vertex = vf_owned_vertex(ordinal);
        filler.add_argument(&vertex.into_shape());
    }
    let face = vf_owned_bounded_face();
    filler.add_argument(&face.into_shape());
    filler.options_mut().set_run_parallel(parallel);
    filler.set_glue(glue);
    filler.initialize().unwrap();
    filler
}

fn vf_owned_vertex(ordinal: usize) -> Vertex {
    let mut vertex = Vertex::default();
    let point = Point3::new(0.125 * (ordinal + 1) as f64, 0.25, 0.);
    BRepBuilder::default()
        .make_vertex_at(&mut vertex, point, 1e-7)
        .unwrap();
    vertex
}

fn assert_planar_vertex_face_records(filler: &BOPAlgoPaveFiller, expected: usize) {
    let structure = filler.data_structure().unwrap();
    let records = structure.interf_vf_records();
    assert_eq!(records.len(), expected);
    for (ordinal, record) in records.iter().enumerate() {
        assert_eq!(record.uv(), (0.125 * (ordinal + 1) as f64, 0.25));
        assert_eq!(record.index1(), ordinal as i32);
    }
    assert!(!filler.options().has_errors());
    assert!(!filler.options().has_warnings());
}

#[test]
fn vertex_face_task_failure_keeps_typed_cause_and_native_warning_only_report() {
    let mut filler = planar_vertex_face_fixture(1, false);
    remove_vf_source_surface(&mut filler);
    let (outcomes, _, cancelled) = run_vf_census(&mut filler, 0);
    assert!(!cancelled);
    assert!(matches!(
        outcomes.as_slice(),
        [ProgressCompletion::Failed { cause: Some(_), .. }]
    ));
    assert!(filler.options().has_warnings());
    assert!(!filler.options().has_errors());
    assert_eq!(
        filler.data_structure().unwrap().interf_vf_records().len(),
        0
    );
}

#[test]
fn vertex_face_unranged_tail_propagates_actual_failure_without_parent_warning() {
    let mut filler = planar_vertex_face_fixture(1, false);
    remove_vf_source_surface(&mut filler);
    let structure = filler.data_structure_mut().unwrap();
    // Explicit tail-state injection tests its catch boundary, not genuine EE construction.
    let record = structure
        .interf_ee()
        .append(crate::bop_ds::InterfEE::default());
    record.set_index_new(0);
    let cause = filler.treat_vertices_ee().unwrap_err();
    assert!(!cause.exception_type().is_empty());
    assert!(!filler.options().has_warnings());
    assert!(!filler.options().has_errors());
}

fn remove_vf_source_surface(filler: &mut BOPAlgoPaveFiller) {
    let index = vf_source_face_index(filler);
    filler
        .data_structure_mut()
        .unwrap()
        .change_face_info(index)
        .unwrap();
    vf_clear_source_surface(filler, index);
}

fn vf_source_face_index(filler: &BOPAlgoPaveFiller) -> i32 {
    let structure = filler.data_structure().unwrap();
    (0..structure.nb_source_shapes())
        .find(|index| structure.shape_info(*index).unwrap().shape_type() == ShapeEnum::Face)
        .unwrap()
}

fn vf_clear_source_surface(filler: &BOPAlgoPaveFiller, index: i32) {
    let structure = filler.data_structure().unwrap();
    let face = martensite_tkbrep::topods::Face::unchecked(structure.shape(index).unwrap().clone());
    BRepBuilder::default()
        .update_face(&face, &Default::default(), &Default::default(), 1e-7)
        .unwrap();
}

fn vertex_face_indicator(
    target: usize,
    polls: &Arc<AtomicUsize>,
    shows: &Arc<AtomicUsize>,
    journal: super::census::VfCensusJournal,
) -> ProgressIndicator {
    let hooks = VertexFaceBreakProbe {
        target,
        polls: polls.clone(),
        shows: shows.clone(),
        journal,
    };
    ProgressIndicator::new(Box::new(hooks))
}

fn vf_owned_bounded_face() -> martensite_tkbrep::topods::Face {
    BRepBuilderAPIMakeFace::from_plane_bounds(Plane3::default(), (-2., 2., -2., 2.))
        .unwrap()
        .face()
        .unwrap()
}

#[test]
fn vertex_face_uncancelled_census_covers_exact_native_profiles() {
    for (profile, mut filler) in native_fixture::uncancelled_vf_fixtures() {
        let _actual = run_vf_profile_census(&mut filler, 0, profile);
        // This test emits observations; the external bound comparison decides equivalence.
    }
}

#[path = "native_fixture.rs"]
mod native_fixture;

fn vf_census_observer(
    target: usize,
    polls: &Arc<AtomicUsize>,
    shows: &Arc<AtomicUsize>,
) -> (super::census::VfCensusJournal, ProgressIndicator) {
    let journal = super::census::VfCensusJournal::default();
    let indicator = vertex_face_indicator(target, polls, shows, journal.clone());
    (journal, indicator)
}

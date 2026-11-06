//! Native four-poll VV observations pin actual cursor and partial same-domain state.
use super::super::owned_progress_fixture::read_owned_progress_shape;
use super::*;
use martensite_tkernel::standard::RuntimeTyped;
use std::sync::{
    Mutex,
    atomic::{AtomicUsize, Ordering},
};
type VertexPolls = Arc<AtomicUsize>;
type VertexPositions = Arc<Mutex<Vec<f64>>>;

struct VertexBoundaryProbe {
    target: usize,
    polls: VertexPolls,
    positions: VertexPositions,
}

impl martensite_tkernel::message::ProgressIndicatorHooks for VertexBoundaryProbe {
    fn user_break(&self) -> bool {
        let ordinal = self.polls.fetch_add(1, Ordering::SeqCst) + 1;
        ordinal == self.target
    }

    fn show(&self, _: &ProgressScope<'_>, position: f64, _: bool) {
        let mut positions = self.positions.lock().unwrap_or_else(|cause| {
            panic!("VV positions {cause:?}, expected an unpoisoned observation mutex")
        });
        positions.push(position);
    }
}

#[test]
fn actual_vv_four_boundaries_keep_native_partial_state_and_live_cursor() {
    for target in 1..=4 {
        let (filler, polls, _) = run_vertex_stage(target);
        assert_eq!(polls, target);
        let state = assert_native_vertex_state(&filler, target >= 3);
        println!(
            "{{\"stage\":\"vv\",\"target\":{target},\"state\":{state:?},\"iterator_more\":{},\"user_break\":true,\"poll_count\":{polls}}}",
            filler.iterator().unwrap().more()
        );
        assert_eq!(filler.iterator().unwrap().more(), target == 1);
        assert!(
            filler
                .options()
                .has_error(crate::bop_algo::AlertUserBreak::static_type())
        );
    }
}

#[test]
fn completed_vv_preserves_native_state_and_actual_halfway_progress() {
    static_assertions::assert_impl_all!(BOPAlgoPaveFiller: Send);
    let (filler, polls, positions) = run_vertex_stage(0);
    assert_eq!(polls, 4);
    let state = assert_native_vertex_state(&filler, true);
    println!(
        "{{\"stage\":\"vv\",\"target\":0,\"state\":{state:?},\"iterator_more\":false,\"user_break\":false,\"poll_count\":{polls}}}"
    );
    assert!(!filler.options().has_errors());
    assert!(
        positions
            .iter()
            .any(|position| *position > 0.0 && *position < 1.0),
        "VV positions {positions:?}, expected actual candidate work before scope closure"
    );
    assert_vertex_closed_form(&filler);
}

fn assert_vertex_closed_form(filler: &BOPAlgoPaveFiller) {
    let structure = filler.data_structure().unwrap();
    let merged = Vertex::unchecked(structure.shape(4).unwrap().clone());
    let point = BRepTool::pnt(&merged).unwrap();
    assert_eq!(*point.coordinates(), [2.5e-7, 0.0, 0.0]);
    assert_eq!(BRepTool::vertex_tolerance(&merged).unwrap(), 3.5e-7);
}

fn assert_native_vertex_state(filler: &BOPAlgoPaveFiller, merged: bool) -> [usize; 3] {
    let structure = filler.data_structure().unwrap();
    let state = [
        structure.nb_shapes() as usize,
        structure.interf_vv_records().len(),
        structure.shape_sd_items().count(),
    ];
    let expected = if merged { [5, 1, 2] } else { [4, 0, 0] };
    assert_eq!(state, expected);
    state
}

fn run_vertex_stage(target: usize) -> (BOPAlgoPaveFiller, usize, Vec<f64>) {
    let (indicator, polls, positions) = vertex_indicator(target);
    let range = indicator.start();
    let observed = ProgressCancellation::default();
    let mut filler = vertex_fixture();
    filler.perform_vv_with_progress(&range, &observed).unwrap();
    assert_eq!(observed.is_cancelled(), target != 0);
    let polls = polls.load(Ordering::SeqCst);
    let positions = positions
        .lock()
        .unwrap_or_else(|cause| {
            panic!("VV positions {cause:?}, expected an unpoisoned observation mutex")
        })
        .clone();
    (filler, polls, positions)
}

fn vertex_indicator(
    target: usize,
) -> (
    martensite_tkernel::message::ProgressIndicator,
    VertexPolls,
    VertexPositions,
) {
    let polls = Arc::new(AtomicUsize::new(0));
    let positions = Arc::new(Mutex::new(Vec::new()));
    let hooks = VertexBoundaryProbe {
        target,
        polls: polls.clone(),
        positions: positions.clone(),
    };
    let indicator = martensite_tkernel::message::ProgressIndicator::new(Box::new(hooks));
    (indicator, polls, positions)
}

fn vertex_fixture() -> BOPAlgoPaveFiller {
    let mut filler = BOPAlgoPaveFiller::new();
    let first = include_bytes!("face_face_store_fixture/vertex_first.brep");
    let second = include_bytes!("face_face_store_fixture/vertex_second.brep");
    filler.add_argument(&read_owned_progress_shape(first, ShapeEnum::Compound));
    filler.add_argument(&read_owned_progress_shape(second, ShapeEnum::Compound));
    filler.options_mut().set_fuzzy_value(1.0e-6);
    filler.initialize().unwrap();
    filler
}

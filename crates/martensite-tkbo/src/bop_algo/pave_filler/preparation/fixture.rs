//! Owned planar inputs and non-sticky boundary observations for both front stages.
use super::super::owned_progress_fixture::{progress_scope_ancestry, read_owned_progress_shape};
use super::*;
use martensite_tkernel::message::{ProgressIndicator, ProgressIndicatorHooks};
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicUsize, Ordering},
};

pub(super) type FrontEvents = Arc<Mutex<Vec<Vec<(String, f64, f64)>>>>;
struct FrontBoundaryProbe {
    target: usize,
    polls: Arc<AtomicUsize>,
    events: FrontEvents,
}

impl ProgressIndicatorHooks for FrontBoundaryProbe {
    fn user_break(&self) -> bool {
        // A one-shot request distinguishes worker cancellation from later parent polls.
        self.polls.fetch_add(1, Ordering::SeqCst) + 1 == self.target
    }

    fn show(&self, scope: &ProgressScope<'_>, _: f64, _: bool) {
        self.events
            .lock()
            .unwrap()
            .push(progress_scope_ancestry(scope));
    }
}

/// Counts real polls without sticky cancellation; `front_indicator(1)` stops only the first poll.
pub(super) fn front_indicator(target: usize) -> (ProgressIndicator, Arc<AtomicUsize>, FrontEvents) {
    let polls = Arc::new(AtomicUsize::new(0));
    let events = Arc::new(Mutex::new(Vec::new()));
    let hooks = FrontBoundaryProbe {
        target,
        polls: polls.clone(),
        events: events.clone(),
    };
    (ProgressIndicator::new(Box::new(hooks)), polls, events)
}

/// Uses independent finite plane bytes; `plane_input()` returns one real boundary edge and face.
pub(super) fn plane_input() -> PlanarPcurveInput {
    let bytes = include_bytes!("../face_face_store_fixture/plane.brep");
    let face = Face::unchecked(read_owned_progress_shape(bytes, ShapeEnum::Face));
    let mut edges = topexp::Explorer::new(&face, ShapeEnum::Edge, ShapeEnum::Shape).unwrap();
    let edge = Edge::unchecked(
        Iterator::next(&mut edges).expect("finite plane, expected a boundary edge"),
    );
    PlanarPcurveInput { edge, face }
}

/// Exposes the native missing-pcurve preparation input; `remove_stored_pcurve(&input)`.
pub(super) fn remove_stored_pcurve(input: &PlanarPcurveInput) {
    let tolerance = BRepTool::edge_tolerance(&input.edge).unwrap();
    martensite_tkbrep::brep::BRepBuilder::default()
        .update_edge_pcurve_on_face(
            &input.edge,
            &martensite_tkernel::standard::Handle::default(),
            &input.face,
            tolerance,
        )
        .unwrap();
    assert!(
        !BRepTool::curve_on_face(&input.edge, &input.face)
            .unwrap()
            .is_stored
    );
}

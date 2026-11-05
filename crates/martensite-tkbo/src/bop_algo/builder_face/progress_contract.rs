//! Named real stages and non-sticky cancellation qualify the Face receiver, not a dummy update.
use super::BuilderFace;
use crate::bop_algo::{ProgressCompletion, ProgressOperation, ProgressStage};
use martensite_tkbrep::{topexp::Explorer, topods::Shape};
use martensite_tkernel::{
    message::{ProgressIndicator, ProgressIndicatorHooks, ProgressScope},
    ncollection::List,
};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::Plane3;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeFace;
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicUsize, Ordering},
};

type CapturedNames = Arc<Mutex<Vec<Vec<u8>>>>;

struct CapturedFaceProgress {
    names: CapturedNames,
    polls: AtomicUsize,
    cancel_at: Option<usize>,
}

impl ProgressIndicatorHooks for CapturedFaceProgress {
    fn show(&self, scope: &ProgressScope<'_>, _: f64, _: bool) {
        let mut cursor = Some(scope);
        while let Some(current) = cursor {
            if let Some(name) = current.name() {
                self.names.lock().unwrap().push(name.to_vec());
            }
            cursor = current.parent();
        }
    }

    fn user_break(&self) -> bool {
        let poll = self.polls.fetch_add(1, Ordering::AcqRel) + 1;
        self.cancel_at == Some(poll)
    }
}

fn square_builder() -> BuilderFace {
    let face = BRepBuilderAPIMakeFace::from_plane_bounds(Plane3::default(), (0., 10., 0., 10.))
        .unwrap()
        .face()
        .unwrap()
        .clone();
    let edges: List<Shape> = Explorer::new(&face, ShapeEnum::Edge, ShapeEnum::Shape)
        .unwrap()
        .collect();
    let mut builder = BuilderFace::new();
    builder.set_face(&face);
    builder.set_shapes(&edges);
    builder
}

fn captured_indicator(cancel_at: Option<usize>) -> (ProgressIndicator, CapturedNames) {
    let names = Arc::new(Mutex::new(Vec::new()));
    let hooks = CapturedFaceProgress {
        names: Arc::clone(&names),
        polls: AtomicUsize::new(0),
        cancel_at,
    };
    (ProgressIndicator::new(Box::new(hooks)), names)
}

#[test]
fn real_face_stages_reach_nested_wire_and_face_scopes() {
    let (indicator, names) = captured_indicator(None);
    let mut builder = square_builder();
    assert_eq!(
        builder.perform_with_progress(&indicator.start()),
        ProgressCompletion::Completed
    );
    assert_eq!(builder.areas().len(), 1);
    let captured = names.lock().unwrap();
    for required in [
        b"Making wires".as_slice(),
        b"Building wires",
        b"Making faces",
    ] {
        assert!(
            captured.iter().any(|name| name == required),
            "missing actual scope {required:?}"
        );
    }
}

#[test]
fn actual_nonsticky_wire_cancellation_never_becomes_face_success() {
    let (indicator, _) = captured_indicator(Some(2));
    let mut builder = square_builder();
    let outcome = builder.perform_with_progress(&indicator.start());
    assert!(matches!(
        outcome,
        ProgressCompletion::Cancelled {
            operation: ProgressOperation::BuilderFace,
            stage: ProgressStage::Completion,
            context: None,
        }
    ));
    assert!(
        !builder.options().has_errors(),
        "native Face does not merge the Wire report"
    );
}

#[test]
fn ranged_face_missing_input_is_failed_not_completed_empty() {
    let (indicator, _) = captured_indicator(None);
    let mut builder = BuilderFace::new();
    assert!(matches!(
        builder.perform_with_progress(&indicator.start()),
        ProgressCompletion::Failed { .. }
    ));
    assert!(builder.options().has_errors());
    assert_eq!(
        indicator.get_position(),
        1.,
        "native root scope closes even on failed input"
    );
}

#[test]
fn completed_empty_face_is_distinct_from_missing_input() {
    let (indicator, _) = captured_indicator(None);
    let mut builder = square_builder();
    builder.set_shapes(&List::default());
    assert_eq!(
        builder.perform_with_progress(&indicator.start()),
        ProgressCompletion::Completed
    );
    assert!(builder.areas().is_empty());
    assert!(!builder.options().has_errors());
}

#[test]
fn cancelled_reuse_never_publishes_retained_raw_areas_as_success() {
    let mut builder = square_builder();
    builder.perform().unwrap();
    assert_eq!(builder.areas().len(), 1);
    let (indicator, _) = captured_indicator(Some(1));
    assert!(matches!(
        builder.perform_with_progress(&indicator.start()),
        ProgressCompletion::Cancelled { .. }
    ));
    assert_eq!(
        builder.areas().len(),
        1,
        "native early stop retains raw prior areas"
    );
    assert!(
        builder.options().has_errors(),
        "actual Face UserBreak boundary adds its own alert"
    );
}

//! External callers exercise complete removal entries and truthful raw completion.
use martensite_tkbo::bop_algo;
use martensite_tkbo::bop_algo::{
    AlertUserBreak, BOPAlgoRemoveFeatures, ProgressCompletion, ProgressFailureClass,
};
use martensite_tkbo::brep_algo_api::BRepAlgoAPIDefeaturing;
use martensite_tkbrep::{topexp::Explorer, topods::Shape};
use martensite_tkernel::{
    message::{AlertHandle, Gravity, ProgressRange, ProgressScope},
    standard::RuntimeTyped,
};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;
use std::sync::{Arc, atomic::Ordering};
// This entry suite uses only the shared recorder’s basic and one-pulse policies.
#[allow(dead_code)]
#[path = "../src/bop_algo/progress_test_support/recording.rs"]
mod recording;
fn box_and_face() -> (Shape, Shape) {
    let shape = BRepPrimAPIMakeBox::from_sizes(2., 3., 4.)
        .unwrap()
        .shape()
        .unwrap()
        .clone();
    let face = Explorer::new(&shape, ShapeEnum::Face, ShapeEnum::Shape)
        .unwrap()
        .collect::<Vec<_>>()
        .remove(0);
    (shape, face)
}
fn configured_receiver() -> BOPAlgoRemoveFeatures {
    let (shape, face) = box_and_face();
    let mut remover = BOPAlgoRemoveFeatures::new();
    remover.set_shape(&shape);
    remover.add_face_to_remove(&face);
    remover
}
fn configured_facade() -> BRepAlgoAPIDefeaturing {
    let (shape, face) = box_and_face();
    let mut api = BRepAlgoAPIDefeaturing::new();
    api.set_shape(&shape);
    api.add_face_to_remove(&face);
    api
}
#[test]
fn both_public_entries_have_concrete_receiver_function_types_and_send_sync_state() {
    let _: for<'a, 'b, 'p> fn(
        &'a mut BOPAlgoRemoveFeatures,
        &'b ProgressRange<'p>,
    ) -> ProgressCompletion = BOPAlgoRemoveFeatures::perform_with_progress;
    let _: for<'a, 'b, 'p> fn(
        &'a mut BRepAlgoAPIDefeaturing,
        &'b ProgressRange<'p>,
    ) -> ProgressCompletion = BRepAlgoAPIDefeaturing::build_with_progress;
    static_assertions::assert_impl_all!(BOPAlgoRemoveFeatures: Send, Sync);
    static_assertions::assert_impl_all!(BRepAlgoAPIDefeaturing: Send, Sync);
}
#[test]
fn a_null_input_returns_an_owned_failure_without_polling_or_signal_handling() {
    let mut remover = BOPAlgoRemoveFeatures::new();
    let (indicator, trace) = recording::pulse_indicator(1);
    let cause = match remover.perform_with_progress(&indicator.start()) {
        ProgressCompletion::Failed {
            cause: Some(cause), ..
        } => cause,
        other => panic!("status={other:?}; expected owned null-input failure"),
    };
    assert_eq!(cause.exception_type(), "Standard_NullObject");
    assert_eq!(trace.queries.load(Ordering::SeqCst), 0);
    assert!(remover.options().has_errors());
}
#[test]
fn both_entries_keep_the_external_caller_and_a_completed_inability_warning() {
    let mut remover = configured_receiver();
    let (indicator, trace) = recording::indicator(0);
    let root = indicator.start();
    let caller = ProgressScope::with_literal(&root, "External removal caller", 10., false);
    assert_eq!(
        remover.perform_with_progress(&caller.next(3.)),
        ProgressCompletion::Completed
    );
    assert!(remover.options().has_warnings());
    assert!(
        trace
            .events
            .lock()
            .unwrap()
            .iter()
            .any(|(_, names)| names.iter().any(|name| name == "External removal caller"))
    );
    let mut api = configured_facade();
    let (indicator, trace) = recording::indicator(0);
    let root = indicator.start();
    let caller = ProgressScope::with_literal(&root, "External facade caller", 10., false);
    assert_eq!(
        api.build_with_progress(&caller.next(3.)),
        ProgressCompletion::Completed
    );
    assert!(api.is_done() && api.has_warnings());
    assert!(
        trace
            .events
            .lock()
            .unwrap()
            .iter()
            .any(|(_, names)| names.iter().any(|name| name == "External facade caller"))
    );
}
#[test]
fn actual_collection_stops_are_cancelled_and_do_not_publish_a_facade_result() {
    let mut remover = configured_receiver();
    let (indicator, trace) = recording::pulse_indicator(1);
    assert!(matches!(
        remover.perform_with_progress(&indicator.start()),
        ProgressCompletion::Cancelled { .. }
    ));
    assert!(trace.break_observed.load(Ordering::SeqCst));
    assert!(remover.options().has_error(AlertUserBreak::static_type()));
    let mut api = configured_facade();
    let (indicator, trace) = recording::pulse_indicator(1);
    assert!(matches!(
        api.build_with_progress(&indicator.start()),
        ProgressCompletion::Cancelled { .. }
    ));
    assert!(trace.break_observed.load(Ordering::SeqCst));
    assert!(!api.is_done());
    assert!(api.shape().is_null());
}
#[test]
fn a_stale_user_break_report_is_a_report_failure_without_a_new_stop_observation() {
    let mut remover = configured_receiver();
    let alert: AlertHandle = Arc::new(AlertUserBreak);
    remover.options().add_error(&alert);
    let (indicator, trace) = recording::pulse_indicator(1);
    assert!(matches!(
        remover.perform_with_progress(&indicator.start()),
        ProgressCompletion::Failed {
            class: ProgressFailureClass::AlgorithmReport,
            cause: None,
            ..
        }
    ));
    assert_eq!(trace.queries.load(Ordering::SeqCst), 0);
}
#[test]
fn facade_clear_discards_prior_reports_before_the_complete_receiver_runs() {
    let mut api = configured_facade();
    let alert: AlertHandle = Arc::new(AlertUserBreak);
    api.get_report().add_alert(Gravity::Fail, &alert);
    assert_eq!(
        api.build_with_progress(&ProgressRange::default()),
        ProgressCompletion::Completed
    );
    assert!(api.is_done());
    assert!(!api.has_error(AlertUserBreak::static_type()));
}

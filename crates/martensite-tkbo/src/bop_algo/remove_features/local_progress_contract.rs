//! Owned stage controls use exact boxes and actual progress hooks, without source-case credit.
use super::super::{
    BOPAlgoRemoveFeatures,
    helpers::{compound_of, explore},
};
use crate::bop_algo::{AlertNoFacesToRemove, AlertUserBreak};
use martensite_tkbrep::brep_tools::History;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::message::{ProgressIndicator, ProgressIndicatorHooks, ProgressScope};
use martensite_tkernel::standard::{Handle, RuntimeTyped};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicUsize, Ordering},
};

#[derive(Clone)]
struct LocalStageAdvance {
    name: Option<Vec<u8>>,
    position: f64,
    portion: f64,
}

#[derive(Default)]
struct LocalStageObservations {
    queries: AtomicUsize,
    advances: Mutex<Vec<LocalStageAdvance>>,
}

#[derive(Clone)]
struct RecordedLocalStageProgress {
    observations: Arc<LocalStageObservations>,
    cancel_at: usize,
}

impl ProgressIndicatorHooks for RecordedLocalStageProgress {
    fn show(&self, scope: &ProgressScope<'_>, position: f64, _: bool) {
        let advance = LocalStageAdvance {
            name: scope.name().map(<[u8]>::to_vec),
            position,
            portion: scope.get_portion(),
        };
        self.observations.advances.lock().unwrap().push(advance);
    }

    fn user_break(&self) -> bool {
        let query = self.observations.queries.fetch_add(1, Ordering::Relaxed) + 1;
        self.cancel_at > 0 && query == self.cancel_at
    }
}

fn stage_indicator(cancel_at: usize) -> (ProgressIndicator, Arc<LocalStageObservations>) {
    let observations = Arc::new(LocalStageObservations::default());
    let hooks = RecordedLocalStageProgress {
        observations: Arc::clone(&observations),
        cancel_at,
    };
    (ProgressIndicator::new(Box::new(hooks)), observations)
}

fn exact_box_shape() -> Shape {
    let mut maker = BRepPrimAPIMakeBox::from_sizes(2.0, 3.0, 4.0).unwrap();
    maker.shape().unwrap().clone()
}

fn local_remover_for(shape: &Shape) -> BOPAlgoRemoveFeatures {
    let mut remover = BOPAlgoRemoveFeatures::new();
    remover.set_shape(shape);
    remover.base.shape = shape.clone();
    remover.base.history = Handle::new(History::new());
    remover
}

fn add_source_face(remover: &mut BOPAlgoRemoveFeatures) {
    let faces = explore(&remover.input_shape, ShapeEnum::Face).unwrap();
    let face = faces.first().expect("box expected a source face");
    remover.add_face_to_remove(face);
}

fn prepared_local_remover() -> BOPAlgoRemoveFeatures {
    let mut remover = local_remover_for(&exact_box_shape());
    add_source_face(&mut remover);
    remover.prepare_features().unwrap();
    assert!(!remover.options().has_errors());
    remover
}

fn assert_native_break(remover: &BOPAlgoRemoveFeatures, observed: &LocalStageObservations) {
    assert!(remover.options().has_error(AlertUserBreak::static_type()));
    assert_eq!(observed.queries.load(Ordering::Relaxed), 1);
    assert!(
        !remover
            .options()
            .has_error(AlertNoFacesToRemove::static_type())
    );
}

fn assert_named_work(observed: &LocalStageObservations, name: &[u8]) {
    let advances = observed.advances.lock().unwrap();
    assert!(
        advances
            .iter()
            .any(|actual| actual.name.as_deref() == Some(name))
    );
    assert!(observed.queries.load(Ordering::Relaxed) > 0);
}

#[test]
fn actual_face_collection_preserves_original_input_identity() {
    let shape = exact_box_shape();
    let mut legacy = local_remover_for(&shape);
    let mut ranged = local_remover_for(&shape);
    add_source_face(&mut legacy);
    add_source_face(&mut ranged);
    legacy.prepare_features().unwrap();
    let (indicator, observed) = stage_indicator(0);
    ranged
        .prepare_features_progress_raw(&indicator.start())
        .unwrap();
    assert!(!ranged.options().has_errors());
    assert_shared_features(&legacy, &ranged);
    assert_named_work(&observed, b"Preparing the faces to remove");
    trace_stage("prepare", 0, &ranged, &observed);
}

#[test]
fn collection_cancellation_keeps_native_alert_and_no_features() {
    let mut remover = local_remover_for(&exact_box_shape());
    add_source_face(&mut remover);
    let (indicator, observed) = stage_indicator(1);
    remover
        .prepare_features_progress_raw(&indicator.start())
        .unwrap();
    assert_native_break(&remover, &observed);
    assert!(remover.features.is_empty());
    trace_stage("prepare", 1, &remover, &observed);
}

#[test]
fn history_cancellation_happens_after_actual_supported_shape_filter() {
    let mut remover = prepared_local_remover();
    let source = remover.input_shape.clone();
    let (indicator, observed) = stage_indicator(1);
    remover
        .update_history_progress_raw(&indicator.start())
        .unwrap();
    assert_native_break(&remover, &observed);
    assert!(remover.base.shape.is_same(&source));
    assert!(
        !remover
            .filled_history()
            .get()
            .unwrap()
            .is_removed(&source)
            .unwrap()
    );
    trace_stage("history", 1, &remover, &observed);
}

#[test]
fn history_progress_reaches_actual_inputs_without_changing_relations() {
    let mut remover = prepared_local_remover();
    let (indicator, observed) = stage_indicator(0);
    remover
        .update_history_progress_raw(&indicator.start())
        .unwrap();
    assert!(!remover.options().has_errors());
    assert_named_work(&observed, b"Updating history");
    let handle = remover.filled_history();
    let history = handle.get().unwrap();
    for shape in remover.inputs_map.iter() {
        assert!(!history.is_removed(shape).unwrap());
    }
    trace_stage("history", 0, &remover, &observed);
}

#[test]
fn simplify_cancellation_precedes_actual_unifier_build_and_publication() {
    let mut remover = prepared_local_remover();
    remover.base.shape = compound_of(&[remover.input_shape.clone()]).unwrap();
    let original_result = remover.base.shape.clone();
    let (indicator, observed) = stage_indicator(1);
    remover
        .simplify_result_progress_raw(&indicator.start())
        .unwrap();
    assert_native_break(&remover, &observed);
    assert!(remover.base.shape.is_same(&original_result));
    assert!(!remover.base.map_shape.is_empty());
    trace_stage("simplify", 1, &remover, &observed);
}

#[test]
fn missing_faces_failure_does_not_poll_or_become_late_cancellation() {
    let mut remover = local_remover_for(&exact_box_shape());
    let (indicator, observed) = stage_indicator(1);
    remover
        .prepare_features_progress_raw(&indicator.start())
        .unwrap();
    assert!(
        remover
            .options()
            .has_error(AlertNoFacesToRemove::static_type())
    );
    assert!(!remover.options().has_error(AlertUserBreak::static_type()));
    assert_eq!(observed.queries.load(Ordering::Relaxed), 0);
    trace_stage("prepare", 1, &remover, &observed);
}

fn assert_shared_features(legacy: &BOPAlgoRemoveFeatures, ranged: &BOPAlgoRemoveFeatures) {
    assert_eq!(legacy.features.len(), ranged.features.len());
    let expected = explore(&legacy.features[0], ShapeEnum::Face).unwrap();
    let actual = explore(&ranged.features[0], ShapeEnum::Face).unwrap();
    assert!(actual[0].is_same(&expected[0]));
}

fn hex_scope_name(name: Option<&[u8]>) -> String {
    let bytes = name.unwrap_or_default();
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn advance_json(advance: &LocalStageAdvance) -> String {
    format!(
        r#"{{"name_hex":"{}","position_bits":"{:016x}","portion_bits":"{:016x}"}}"#,
        hex_scope_name(advance.name.as_deref()),
        advance.position.to_bits(),
        advance.portion.to_bits(),
    )
}

fn event_array_json(observed: &LocalStageObservations) -> String {
    let advances = observed.advances.lock().unwrap();
    advances
        .iter()
        .map(advance_json)
        .collect::<Vec<_>>()
        .join(",")
}

fn stage_flags_json(remover: &BOPAlgoRemoveFeatures, observed: &LocalStageObservations) -> String {
    format!(
        r#""queries":{},"has_errors":{},"user_break":{},"no_faces":{},"features":{}"#,
        observed.queries.load(Ordering::Relaxed),
        remover.options().has_errors(),
        remover.options().has_error(AlertUserBreak::static_type()),
        remover
            .options()
            .has_error(AlertNoFacesToRemove::static_type()),
        remover.features.len(),
    )
}

fn trace_stage(
    stage: &str,
    cancel_at: usize,
    remover: &BOPAlgoRemoveFeatures,
    observed: &LocalStageObservations,
) {
    let events = event_array_json(observed);
    let flags = stage_flags_json(remover, observed);
    println!(r#"{{"stage":"{stage}","cancel_at":{cancel_at},{flags},"events":[{events}]}}"#);
}

#[test]
fn simplification_reports_actual_native_named_work() {
    let mut remover = prepared_local_remover();
    remover.base.shape = compound_of(&[remover.input_shape.clone()]).unwrap();
    let (indicator, observed) = stage_indicator(0);
    remover
        .simplify_result_progress_raw(&indicator.start())
        .unwrap();
    assert!(!remover.options().has_errors());
    assert_named_work(&observed, b"Simplifyingthe result");
    trace_stage("simplify", 0, &remover, &observed);
}

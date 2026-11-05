//! History progress keeps partial diagnostic state distinct from operation completion.
use super::{
    BOPAlgoBuilder,
    image_progress_contract::{compound_structure, edge_structure},
};
use crate::bop_algo::{
    ProgressCancellation,
    progress_test_support::{indicator, pulse_indicator},
};
use std::sync::atomic::Ordering;

#[test]
fn cancellation_initializes_history_before_first_source_relation() {
    let structure = edge_structure();
    let mut builder = BOPAlgoBuilder::new();
    builder.prepare();
    let observed = ProgressCancellation::default();
    let (progress, trace) = pulse_indicator(1);
    builder
        .prepare_history_progress_raw(&structure, &progress.start(), &observed)
        .unwrap();
    assert!(observed.is_cancelled());
    assert!(!builder.has_deleted());
    assert_eq!(trace.queries.load(Ordering::SeqCst), 1);
    assert!(builder.options().has_errors());
    record_history("history_edges", "query_1", &builder, &trace);
}

#[test]
fn successful_history_preserves_all_unbound_removed_relations() {
    let structure = edge_structure();
    let mut expected = BOPAlgoBuilder::new();
    expected.prepare();
    expected.prepare_history(&structure).unwrap();
    let mut actual = BOPAlgoBuilder::new();
    actual.prepare();
    let observed = ProgressCancellation::default();
    let (progress, trace) = indicator(0);
    actual
        .prepare_history_progress_raw(&structure, &progress.start(), &observed)
        .unwrap();
    assert_removed_relations(&mut expected, &mut actual, &structure);
    assert!(!observed.is_cancelled());
    record_history("history_edges", "", &actual, &trace);
}

#[test]
fn unsupported_compound_advances_before_supported_source_poll() {
    let structure = compound_structure();
    let mut builder = BOPAlgoBuilder::new();
    builder.prepare();
    let observed = ProgressCancellation::default();
    let (progress, trace) = pulse_indicator(1);
    builder
        .prepare_history_progress_raw(&structure, &progress.start(), &observed)
        .unwrap();
    assert!(observed.is_cancelled());
    assert_source_slot_advanced(&trace, structure.nb_source_shapes());
    record_history("history_compound", "query_1", &builder, &trace);
}

#[test]
fn disabled_history_never_opens_scope_or_queries_cancellation() {
    let structure = edge_structure();
    let mut builder = BOPAlgoBuilder::new();
    builder.set_to_fill_history(false);
    let observed = ProgressCancellation::default();
    let (progress, trace) = pulse_indicator(1);
    builder
        .prepare_history_progress_raw(&structure, &progress.start(), &observed)
        .unwrap();
    assert!(!observed.is_cancelled());
    assert_eq!(trace.queries.load(Ordering::SeqCst), 0);
    assert!(builder.history().is_none());
    record_history("history_disabled", "query_1", &builder, &trace);
}

fn assert_removed_relations(
    expected: &mut BOPAlgoBuilder,
    actual: &mut BOPAlgoBuilder,
    structure: &crate::bop_ds::DataStructure,
) {
    let left_owner = expected.history().unwrap();
    let right_owner = actual.history().unwrap();
    let left = left_owner.get().unwrap();
    let right = right_owner.get().unwrap();
    for index in 0..structure.nb_source_shapes() {
        let shape = structure.shape(index).unwrap();
        assert_eq!(
            left.is_removed(shape).unwrap(),
            right.is_removed(shape).unwrap()
        );
    }
}

fn record_history(
    kind: &str,
    cancel: &str,
    builder: &BOPAlgoBuilder,
    trace: &crate::bop_algo::progress_test_support::Trace,
) {
    let record = crate::bop_algo::progress_test_support::observation(
        kind,
        cancel,
        builder.options(),
        usize::from(builder.has_deleted()),
        trace,
    );
    println!("{record}");
}

fn assert_source_slot_advanced(trace: &crate::bop_algo::progress_test_support::Trace, count: i32) {
    let events = trace.events.lock().unwrap();
    let expected = (1.0 / count as f64).to_bits();
    assert!(events.iter().any(|(position, _)| *position == expected));
    assert_eq!(trace.queries.load(Ordering::SeqCst), 1);
}

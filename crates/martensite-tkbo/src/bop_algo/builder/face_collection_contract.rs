//! Thread-confined collection probes preserve source-face polling before face-info selection.
use super::BOPAlgoBuilder;
use crate::bop_algo::builder::image_progress_contract::{compound_structure, edge_structure};
use std::cell::Cell;

#[test]
fn untouched_source_faces_poll_before_missing_face_info_is_skipped() {
    let structure = compound_structure();
    let mut builder = BOPAlgoBuilder::new();
    let queries = Cell::new(0_usize);
    let (splits, tasks) = builder
        .collect_face_splits(&structure, |_| {
            queries.set(queries.get() + 1);
            false
        })
        .unwrap()
        .unwrap();
    assert_eq!(queries.get(), 6);
    assert_eq!(splits.extent(), 0);
    assert!(tasks.is_empty());
}

#[test]
fn confirmed_collection_stop_discards_unpublished_local_splits() {
    let structure = compound_structure();
    let mut builder = BOPAlgoBuilder::new();
    let queries = Cell::new(0_usize);
    let stopped = builder
        .collect_face_splits(&structure, |_| {
            queries.set(queries.get() + 1);
            true
        })
        .unwrap();
    assert!(stopped.is_none());
    assert_eq!(queries.get(), 1);
    assert!(builder.images().is_empty());
}

#[test]
fn nonface_sources_do_not_query_face_collection_stop() {
    let structure = edge_structure();
    let mut builder = BOPAlgoBuilder::new();
    let (splits, tasks) = builder
        .collect_face_splits(&structure, |_| {
            panic!("unexpected nonface poll; expected only source Face records")
        })
        .unwrap()
        .unwrap();
    assert_eq!(splits.extent(), 0);
    assert!(tasks.is_empty());
}

#[test]
fn raw_split_face_preparation_cancellation_never_becomes_completed_empty() {
    let structure = compound_structure();
    let mut builder = BOPAlgoBuilder::new();
    let observed = crate::bop_algo::ProgressCancellation::default();
    let (progress, trace) = crate::bop_algo::progress_test_support::pulse_indicator(1);
    let completion = builder
        .build_split_faces_progress_raw(&structure, &progress.start(), &observed)
        .unwrap();
    assert!(matches!(
        completion,
        crate::bop_algo::ProgressCompletion::Cancelled { .. }
    ));
    assert!(observed.is_cancelled());
    assert!(builder.images().is_empty());
    record_split_stage("query_1", &builder, &trace);
}

#[test]
fn actual_empty_task_batch_preserves_native_face_and_post_dispatch_polls() {
    let structure = compound_structure();
    let mut builder = BOPAlgoBuilder::new();
    let observed = crate::bop_algo::ProgressCancellation::default();
    let (progress, trace) = crate::bop_algo::progress_test_support::indicator(0);
    let completion = builder
        .build_split_faces_progress_raw(&structure, &progress.start(), &observed)
        .unwrap();
    assert_eq!(completion, crate::bop_algo::ProgressCompletion::Completed);
    assert_eq!(trace.queries.load(std::sync::atomic::Ordering::SeqCst), 7);
    assert!(builder.images().is_empty());
    record_split_stage("", &builder, &trace);
}

fn record_split_stage(
    cancel: &str,
    builder: &BOPAlgoBuilder,
    trace: &crate::bop_algo::progress_test_support::Trace,
) {
    let record = crate::bop_algo::progress_test_support::observation(
        "face_splits",
        cancel,
        builder.options(),
        builder.images().len(),
        trace,
    );
    println!("{record}");
}

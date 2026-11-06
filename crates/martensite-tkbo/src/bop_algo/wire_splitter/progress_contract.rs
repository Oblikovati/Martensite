//! Exercise actual irregular blocks and preserve native silent cancellation as non-success.
use super::{
    fixture::{empty_splitter, input_identity_records, split_square_splitter},
    hooks::{WorkerRendezvous, wire_indicator},
};
use crate::bop_algo::{ProgressCompletion, WireSplitter};
use std::sync::Arc;

#[test]
fn real_parallel_wire_blocks_overlap_and_preserve_input_order() {
    let (mut serial, serial_inputs) = split_square_splitter(false);
    assert_eq!(
        serial.perform_with_progress(&wire_indicator(None, None).start()),
        ProgressCompletion::Completed
    );
    let serial_rows = input_identity_records(&serial, &serial_inputs);
    let rendezvous = Arc::new(WorkerRendezvous::default());
    let (mut parallel, parallel_inputs) = split_square_splitter(true);
    let indicator = wire_indicator(None, Some(Arc::clone(&rendezvous)));
    assert_eq!(
        parallel.perform_with_progress(&indicator.start()),
        ProgressCompletion::Completed
    );
    assert_two_actual_irregular_blocks(&parallel);
    let parallel_rows = input_identity_records(&parallel, &parallel_inputs);
    assert_and_emit_parallel_rows(&parallel_rows, &serial_rows, rendezvous.count());
}

#[test]
fn actual_silent_worker_cancel_never_becomes_success() {
    let (mut splitter, _) = split_square_splitter(false);
    let indicator = wire_indicator(Some(4), None);
    assert!(matches!(
        splitter.perform_with_progress(&indicator.start()),
        ProgressCompletion::Cancelled { .. }
    ));
    assert!(
        !splitter.options().has_errors(),
        "native task More stop adds no local alert"
    );
    assert!(
        !splitter.wes().unwrap().shapes().is_empty(),
        "later real task retains raw partial loops"
    );
}

#[test]
fn wire_empty_completion_and_cancelled_empty_are_distinct() {
    let mut completed = empty_splitter();
    let mut cancelled = empty_splitter();
    assert_eq!(
        completed.perform_with_progress(&wire_indicator(None, None).start()),
        ProgressCompletion::Completed
    );
    assert!(matches!(
        cancelled.perform_with_progress(&wire_indicator(Some(1), None).start()),
        ProgressCompletion::Cancelled { .. }
    ));
    assert!(
        cancelled.options().has_errors(),
        "actual Perform UserBreak boundary adds its typed alert"
    );
    assert!(completed.wes().unwrap().shapes().is_empty());
    assert!(cancelled.wes().unwrap().shapes().is_empty());
}

fn assert_two_actual_irregular_blocks(splitter: &WireSplitter) {
    assert_eq!(splitter.blocks.len(), 2);
    assert!(splitter.blocks.iter().all(|block| !block.is_regular()));
}

fn assert_and_emit_parallel_rows(
    parallel: &[Vec<(usize, martensite_tkg3d::top_abs::Orientation)>],
    serial: &[Vec<(usize, martensite_tkg3d::top_abs::Orientation)>],
    workers: usize,
) {
    assert_eq!(parallel, serial);
    assert_eq!(workers, 2);
    assert!(!parallel.is_empty());
    println!(
        "{{\"wire_progress_records\":{:?},\"parallel_workers\":{workers}}}",
        numeric_rows(parallel)
    );
}

pub(super) fn numeric_rows(
    rows: &[Vec<(usize, martensite_tkg3d::top_abs::Orientation)>],
) -> Vec<Vec<[usize; 2]>> {
    rows.iter()
        .map(|edges| {
            edges
                .iter()
                .map(|(slot, orientation)| [*slot, *orientation as usize])
                .collect()
        })
        .collect()
}

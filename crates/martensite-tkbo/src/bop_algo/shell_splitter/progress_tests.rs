//! Owned lower progress controls do not replace any original geometry predicate.
use super::{ShellSplitter, perform_shell_with_progress};
use crate::bop_algo::progress_test_support as recording;
#[path = "progress_test_fixtures.rs"]
mod fixtures;
use crate::bop_algo::{AlertUserBreak, ProgressCompletion, ProgressStage};
use fixtures::{irregular_splitter, native_shell_fixture, two_irregular_blocks};
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::standard::RuntimeTyped;

#[test]
fn actual_shell_scopes_reach_injected_indicator() {
    let (indicator, trace) = recording::indicator(0);
    let mut splitter = irregular_splitter(false);
    let _: &(dyn Send + Sync) = &*trace;
    let _: &(dyn Send + Sync) = &splitter;
    perform_shell_with_progress(&mut splitter, &indicator.start());
    assert!(recording::has_nested_shell_progress(&trace));
    assert_eq!(splitter.shells().len(), 2);
    assert!(!splitter.options().has_errors());
}

#[test]
fn discarded_child_range_fails_nested_propagation_control() {
    let (indicator, trace) = recording::indicator(0);
    let _root = indicator.start();
    let mut splitter = irregular_splitter(false);
    perform_shell_with_progress(&mut splitter, &ProgressRange::default());
    assert!(!recording::has_nested_shell_progress(&trace));
    assert_eq!(trace.events.lock().unwrap().len(), 1);
}

#[test]
fn native_block_scan_cancellation_preserves_typed_alert() {
    let (indicator, _) = recording::indicator(2);
    let mut splitter = irregular_splitter(false);
    perform_shell_with_progress(&mut splitter, &indicator.start());
    assert!(splitter.options().has_error(AlertUserBreak::static_type()));
    assert_eq!(splitter.shells().len(), 0);
}

#[test]
fn native_worker_cancellation_keeps_raw_local_report() {
    let (indicator, trace) = recording::indicator(3);
    let mut splitter = irregular_splitter(false);
    let completion = perform_shell_with_progress(&mut splitter, &indicator.start());
    assert!(matches!(
        completion,
        ProgressCompletion::Cancelled {
            stage: ProgressStage::Worker,
            ..
        }
    ));
    assert!(trace.cancelled.load(std::sync::atomic::Ordering::SeqCst));
    assert!(!splitter.options().has_errors());
    assert_eq!(splitter.shells().len(), 0);
}

/// How many parallel runs may pass before a second worker has to have taken part.
const PARALLEL_RUNS: usize = 200;

#[test]
fn real_parallel_shell_tasks_match_serial_topology_and_progress() {
    // A busy global pool permits caller-only execution; participation requires an owned pool.
    if recording::run_isolated_progress_census(
        "bop_algo::shell_splitter::progress_tests::real_parallel_shell_tasks_match_serial_topology_and_progress",
    ) {
        return;
    }
    let mut serial = two_irregular_blocks(false);
    perform_shell_with_progress(&mut serial, &ProgressRange::default());
    // In an optimized build one worker can finish both short tasks before another takes one, so
    // real participation may take more than one run. Every run must match the serial one.
    let most_threads = (0..PARALLEL_RUNS).fold(0, |most, _| {
        let (indicator, trace) = recording::indicator(0);
        let mut splitter = two_irregular_blocks(true);
        perform_shell_with_progress(&mut splitter, &indicator.start());
        assert_eq!(splitter.shells().len(), 4);
        assert_eq!(shell_membership(&splitter), shell_membership(&serial));
        assert!(!splitter.options().has_errors());
        assert!(recording::has_nested_shell_progress(&trace));
        let threads = trace.threads.lock().unwrap().len();
        most.max(threads)
    });
    assert!(most_threads >= 2);
}

fn shell_membership(splitter: &ShellSplitter) -> Vec<Vec<(usize, i32)>> {
    use martensite_tkbrep::topods::ShapeIterator;
    splitter
        .shells()
        .iter()
        .map(|shell| {
            assert!(shell.closed().unwrap());
            ShapeIterator::new(shell, true, true)
                .map(|face| {
                    let index = splitter
                        .start_elements()
                        .iter()
                        .position(|source| source.is_same(&face))
                        .unwrap();
                    (index, face.orientation() as i32)
                })
                .collect()
        })
        .collect()
}

#[test]
fn nonsticky_worker_cancellation_remains_cancelled_after_later_false_poll() {
    let (indicator, _) = recording::pulse_indicator(3);
    let mut splitter = irregular_splitter(false);
    let completion = perform_shell_with_progress(&mut splitter, &indicator.start());
    assert!(!indicator.root_scope().user_break());
    assert!(matches!(
        completion,
        ProgressCompletion::Cancelled {
            stage: ProgressStage::Worker,
            ..
        }
    ));
    assert!(!splitter.options().has_errors());
    assert_eq!(splitter.shells().len(), 0);
}

#[test]
fn actual_shell_callbacks_and_reports_match_all_native_profiles() {
    let mut records = Vec::new();
    for kind in ["regular_shell", "irregular_shell"] {
        for cancel in ["", "before"] {
            records.push(observe_shell(kind, cancel));
        }
    }
    for cancel in ["query_2", "query_3", "query_4"] {
        records.push(observe_shell("irregular_shell", cancel));
    }
    let actual = records.join("\n") + "\n";
    println!("{actual}");
    assert_eq!(actual, include_str!("progress_tests/native-shell.jsonl"));
}

fn observe_shell(kind: &str, cancel: &str) -> String {
    let (indicator, trace) = recording::profile_indicator(cancel);
    let mut splitter = native_shell_fixture(kind);
    let completion = perform_shell_with_progress(&mut splitter, &indicator.start());
    assert_eq!(
        matches!(completion, ProgressCompletion::Cancelled { .. }),
        trace.cancelled.load(std::sync::atomic::Ordering::SeqCst)
    );
    recording::observation(
        kind,
        cancel,
        splitter.options(),
        splitter.shells().len(),
        &trace,
    )
}

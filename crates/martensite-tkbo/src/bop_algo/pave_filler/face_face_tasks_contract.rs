//! Prepared inputs and terminal results remain ordered under the actual configured executor.
use super::super::face_face_task_tests::prepared_plane_input;
use super::super::owned_progress_fixture::{
    native_scope_json, progress_scope_ancestry, progress_scope_names,
};
use super::*;
use crate::bop_algo::{ParallelAlgo, ProgressCompletion};
use martensite_tkernel::message::ProgressRange;

#[test]
fn configured_parallel_and_serial_execution_preserve_ordered_owned_results() {
    let serial = execute_rows(false);
    let parallel = execute_rows(true);
    assert_eq!(serial, parallel);
    assert_eq!(serial.len(), 4);
    for (ordinal, row) in serial.iter().enumerate() {
        assert_eq!(row.0, [100 + ordinal as i32, 200 + ordinal as i32]);
        assert_eq!(row.1.map(f64::from_bits), [0.5, 0.0, 0.0]);
    }
}

fn execute_rows(parallel: bool) -> Vec<([i32; 2], [u64; 3])> {
    let range = ProgressRange::default();
    let scope = ProgressScope::unnamed(&range, 1.0, false);
    execute_face_inputs(prepared_inputs(parallel), &scope, parallel)
        .unwrap()
        .into_iter()
        .map(|output| terminal_row(output, parallel))
        .collect()
}

fn prepared_inputs(parallel: bool) -> Vec<FaceFaceTaskInput> {
    let shared_faces = prepared_plane_input().faces;
    (0..4)
        .map(|ordinal| {
            let mut input = prepared_plane_input();
            input.faces = shared_faces.clone();
            input.indices = [100 + ordinal, 200 + ordinal];
            input.parallel = parallel;
            input
        })
        .collect()
}

fn terminal_row(output: FaceFaceTaskOutput, parallel: bool) -> ([i32; 2], [u64; 3]) {
    assert_eq!(output.completion, ProgressCompletion::Completed);
    let mut task = output.into_task();
    assert_eq!(task.parallel_options().run_parallel(), parallel);
    task.intersector_mut().prepare_lines_3d(false).unwrap();
    task.apply_trsf().unwrap();
    let curve = task.intersector().lines().unwrap()[0].curve();
    let point = curve.get().unwrap().eval_d0(0.0).unwrap();
    (task.indices(), point.coordinates().map(f64::to_bits))
}

#[test]
fn empty_inputs_return_an_empty_terminal_sequence() {
    let range = ProgressRange::default();
    let scope = ProgressScope::unnamed(&range, 1.0, false);
    assert!(
        execute_face_inputs(Vec::new(), &scope, true)
            .unwrap()
            .is_empty()
    );
}

#[test]
fn missing_owned_callback_input_and_output_report_the_original_ordinal() {
    let mut work = FaceFaceWork {
        ordinal: 7,
        input: None,
        output: None,
        progress: None,
    };
    let failure = execute_face_work(&mut work).unwrap_err();
    assert_failure_ordinal(failure, 7);
    let failure = work
        .into_output()
        .err()
        .expect("missing output must return a typed failure");
    assert_failure_ordinal(failure, 7);
}

fn assert_failure_ordinal(failure: Failure, ordinal: usize) {
    assert!(
        failure
            .to_string()
            .contains(&format!("work item {ordinal}")),
        "failure {failure:?}, expected original work ordinal {ordinal}"
    );
}

#[derive(Debug)]
struct FaceProgressEvent {
    position: f64,
    names: Vec<String>,
    chain: Vec<(String, f64, f64)>,
}

struct FaceTaskProgressHooks {
    cancelled: bool,
    events: std::sync::Arc<std::sync::Mutex<Vec<FaceProgressEvent>>>,
}

type FaceProgressEvents = std::sync::Arc<std::sync::Mutex<Vec<FaceProgressEvent>>>;

fn face_progress_indicator(
    cancelled: bool,
) -> (
    martensite_tkernel::message::ProgressIndicator,
    FaceProgressEvents,
) {
    let events = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
    let hooks = FaceTaskProgressHooks {
        cancelled,
        events: events.clone(),
    };
    let indicator = martensite_tkernel::message::ProgressIndicator::new(Box::new(hooks));
    (indicator, events)
}

impl martensite_tkernel::message::ProgressIndicatorHooks for FaceTaskProgressHooks {
    fn show(&self, scope: &ProgressScope<'_>, position: f64, _: bool) {
        let event = FaceProgressEvent {
            position,
            names: progress_scope_names(scope),
            chain: progress_scope_ancestry(scope),
        };
        self.events
            .lock()
            .unwrap_or_else(|cause| {
                panic!("FF progress events {cause:?}, expected an unpoisoned event mutex")
            })
            .push(event);
    }

    fn user_break(&self) -> bool {
        // Request every task stop so an unbound forwarding mutant cannot pass by another task.
        self.cancelled
    }
}

#[test]
fn actual_task_scope_cancels_each_worker_and_is_not_just_start_zero() {
    for parallel in [false, true] {
        let outputs = execute_with_hooks(parallel, true).0;
        assert_eq!(outputs.len(), 4);
        for output in outputs {
            assert!(matches!(
                output.completion,
                ProgressCompletion::Cancelled { .. }
            ));
            let task = output.into_task();
            assert!(!task.intersector().is_done());
            assert!(task.parallel_options().has_errors());
        }
    }
}

#[test]
fn named_task_progress_advances_while_actual_geometry_completes() {
    let (outputs, events) = execute_with_hooks(true, false);
    assert!(
        outputs
            .iter()
            .all(|output| output.completion == ProgressCompletion::Completed)
    );
    assert!(
        events.iter().any(|event| event.position > 0.0
            && event
                .names
                .iter()
                .any(|name| name == "Performing Face-Face intersection")),
        "actual events {events:?}, expected named FF progress beyond Start 0%"
    );
}

fn execute_with_hooks(
    parallel: bool,
    cancelled: bool,
) -> (Vec<FaceFaceTaskOutput>, Vec<FaceProgressEvent>) {
    let inputs = prepared_inputs(parallel);
    execute_inputs_with_hooks(inputs, parallel, cancelled)
}

fn execute_inputs_with_hooks(
    inputs: Vec<FaceFaceTaskInput>,
    parallel: bool,
    cancelled: bool,
) -> (Vec<FaceFaceTaskOutput>, Vec<FaceProgressEvent>) {
    let (indicator, events) = face_progress_indicator(cancelled);
    let range = indicator.start();
    let scope = ProgressScope::unnamed(&range, 1.0, false);
    let outputs = execute_face_inputs(inputs, &scope, parallel).unwrap();
    let events = std::mem::take(&mut *events.lock().unwrap_or_else(|cause| {
        panic!("FF progress events {cause:?}, expected an unpoisoned event mutex")
    }));
    (outputs, events)
}

#[test]
fn repeated_configured_parallel_tasks_with_shared_faces_keep_serial_results() {
    let expected = execute_rows(false);
    for repetition in 0..64 {
        let actual = execute_rows(true);
        assert_eq!(
            actual, expected,
            "FF repetition {repetition}, expected serial ordered geometry"
        );
    }
}

#[test]
fn native_singleton_dispatch_keeps_failure_unwrapped_in_both_configurations() {
    for parallel in [false, true] {
        let mut work = [FaceFaceWork {
            ordinal: 0,
            input: None,
            output: None,
            progress: None,
        }];
        let failure = run_face_work(&mut work, parallel).unwrap_err();
        assert_eq!(failure.exception_type(), "Standard_Failure");
        assert_failure_ordinal(failure, 0);
    }
}

#[test]
fn native_multi_task_parallel_dispatch_keeps_the_launcher_failure_wrapper() {
    let mut work = [
        FaceFaceWork {
            ordinal: 0,
            input: None,
            output: None,
            progress: None,
        },
        FaceFaceWork::new((1, prepared_plane_input()), ProgressRange::default()),
    ];
    let failure = run_face_work(&mut work, true).unwrap_err();
    assert_eq!(failure.exception_type(), "Standard_ProgramError");
    assert_failure_ordinal(failure, 0);
    assert!(work[0].output.is_none());
}

#[test]
fn singleton_direct_execution_preserves_the_configured_parallel_kernel_flag() {
    let mut input = prepared_plane_input();
    input.parallel = true;
    let scope = ProgressScope::default();
    let mut outputs = execute_face_inputs(vec![input], &scope, true).unwrap();
    assert_eq!(outputs.len(), 1);
    let row = terminal_row(outputs.pop().unwrap(), true);
    assert_eq!(row.1.map(f64::from_bits), [0.5, 0.0, 0.0]);
}

#[test]
fn actual_ff_worker_ancestry_matches_native_single_scope_max_and_value() {
    let (outputs, events) = execute_inputs_with_hooks(vec![prepared_plane_input()], false, false);
    assert_eq!(outputs.len(), 1);
    assert_eq!(outputs[0].completion, ProgressCompletion::Completed);
    let event = native_ff_worker_close(&events);
    let expected = native_ff_worker_chain();
    println!(
        "{{\"ff_worker_scope\":{}}}",
        native_scope_json(&event.chain)
    );
    assert_eq!(event.chain, expected);
}

fn native_ff_worker_chain() -> Vec<(String, f64, f64)> {
    vec![
        ("<null>".to_owned(), 1.0, 1.0),
        ("Performing Face-Face intersection".to_owned(), 1.0, 1.0),
        ("<null>".to_owned(), 1.0, 1.0),
        ("<null>".to_owned(), 1.0, 1.0),
    ]
}

fn native_ff_worker_close(events: &[FaceProgressEvent]) -> &FaceProgressEvent {
    events
        .iter()
        .find(|event| {
            event.position == 1.0
                && event
                    .names
                    .iter()
                    .any(|name| name == "Performing Face-Face intersection")
        })
        .unwrap_or_else(|| panic!("FF events {events:?}, expected actual native worker close"))
}

static_assertions::assert_impl_all!(FaceFaceWork<'static>: Send);
static_assertions::assert_not_impl_any!(FaceFaceWork<'static>: Sync);

#[path = "face_face_range_contract.rs"]
mod range_contract;

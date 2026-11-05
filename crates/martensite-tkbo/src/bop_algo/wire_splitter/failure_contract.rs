//! The actual receiver must use the native context-overload exception boundary, even serially.
use super::fixture::split_square_splitter;
use crate::bop_algo::{ProgressCompletion, ProgressFailureClass, WireSplitter};
use martensite_tkernel::{
    message::{ProgressIndicator, ProgressIndicatorHooks, ProgressScope},
    standard::{Failure, raise_native_failure},
};
use std::sync::atomic::{AtomicUsize, Ordering};

struct ThrowingTaskProgress {
    polls: AtomicUsize,
}

impl ProgressIndicatorHooks for ThrowingTaskProgress {
    fn show(&self, _: &ProgressScope<'_>, position: f64, _: bool) {
        // Position is observed only; it is never an operation-success condition.
        std::hint::black_box(position);
    }

    fn user_break(&self) -> bool {
        let poll = self.polls.fetch_add(1, Ordering::AcqRel) + 1;
        if poll == 4 {
            raise_native_failure::<()>(Err(Failure::with_type(
                "Standard_DomainError",
                "wire actual task callback",
                "owned-native-stack",
            )));
        }
        false
    }
}

fn throwing_indicator() -> ProgressIndicator {
    let hooks = ThrowingTaskProgress {
        polls: AtomicUsize::new(0),
    };
    ProgressIndicator::new(Box::new(hooks))
}

fn wrapped_failure(parallel: bool) -> Failure {
    let (mut splitter, _) = split_square_splitter(parallel);
    let outcome = splitter.perform_with_progress(&throwing_indicator().start());
    let ProgressCompletion::Failed {
        class: ProgressFailureClass::KernelFailure,
        cause: Some(cause),
        ..
    } = outcome
    else {
        panic!("unexpected outcome {outcome:?}; expected typed kernel failure");
    };
    assert_raw_failure_state(&splitter);
    cause
}

fn assert_raw_failure_state(splitter: &WireSplitter) {
    assert!(
        !splitter.options().has_errors(),
        "native task exception does not invent an alert"
    );
    assert!(
        splitter.wes().unwrap().shapes().is_empty(),
        "failed task forbids irregular publication"
    );
}

fn assert_native_callback_failure(cause: Failure) {
    assert_eq!(cause.exception_type(), "Standard_ProgramError");
    assert_eq!(
        cause.message(),
        "Standard_DomainError: wire actual task callback"
    );
    assert_eq!(cause.stack_string(), "owned-native-stack");
}

#[test]
fn actual_serial_wire_callback_failure_uses_context_launcher_policy() {
    let cause = wrapped_failure(false);
    assert_native_callback_failure(cause);
}

#[test]
fn actual_parallel_wire_callback_failure_uses_context_launcher_policy() {
    let cause = wrapped_failure(true);
    assert_native_callback_failure(cause);
}

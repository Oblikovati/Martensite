//! Caller-specific serial transport must not alter the frozen pure-task policy.
use super::super::context_task_launcher;
use super::context_launcher_on;
use crate::bop_algo::run_launcher_context_tasks;
use crate::int_tools::Context;
use martensite_tkernel::{
    osd::ThreadPool,
    standard::{Failure, Handle},
};
use std::sync::atomic::{AtomicUsize, Ordering};

#[test]
fn serial_context_policy_wraps_first_failure_and_retains_stack() {
    let main = Context::default();
    let mut tasks = [0usize; 3];
    let error = wrapped_serial_failure(&mut tasks, &main);
    assert_eq!(error.exception_type(), "Standard_ProgramError");
    assert_eq!(
        error.message(),
        "Standard_ConstructionError: context serial failure"
    );
    assert_eq!(error.stack_string(), "retained-native-stack");
    assert_eq!(tasks, [1, 0, 0]);
}

fn wrapped_serial_failure(tasks: &mut [usize], main: &Context) -> Failure {
    run_launcher_context_tasks(tasks, main, false, Context::default, |task, actual| {
        assert!(std::ptr::eq(actual, main));
        *task += 1;
        Err(Failure::with_type(
            "Standard_ConstructionError",
            "context serial failure",
            "retained-native-stack",
        ))
    })
    .unwrap_err()
}

fn serial_family<C: Send + Sync>(main: &C, make: impl Fn() -> C + Sync) {
    let mut tasks = [0usize; 3];
    run_launcher_context_tasks(&mut tasks, main, false, make, |task, actual| {
        assert!(std::ptr::eq(actual, main));
        *task += 1;
        Ok(())
    })
    .unwrap();
    assert_eq!(tasks, [1; 3]);
}

#[test]
fn serial_context_policy_keeps_actual_main_context_and_never_constructs_workers() {
    let main = Context::default();
    let created = AtomicUsize::new(0);
    serial_family(&main, || {
        created.fetch_add(1, Ordering::SeqCst);
        Context::default()
    });
    assert_eq!(created.load(Ordering::SeqCst), 0);
}

#[test]
fn serial_context_policy_keeps_the_exact_provided_handle() {
    let main = Handle::new(Context::default());
    let created = AtomicUsize::new(0);
    serial_family(&main, || {
        created.fetch_add(1, Ordering::SeqCst);
        Handle::new(Context::default())
    });
    assert_eq!(created.load(Ordering::SeqCst), 0);
}

#[test]
fn context_launcher_policy_reserves_caller_only_or_owned_pool_workers() {
    let pool = ThreadPool::new(2);
    let serial = context_launcher_on(&pool, false, usize::MAX);
    assert_eq!(serial.nb_threads(), 1);
    assert_eq!(serial.upper_thread_index(), 0);
    drop(serial);
    let parallel = context_launcher_on(&pool, true, usize::MAX);
    assert_eq!(parallel.nb_threads(), 2);
    assert_eq!(parallel.upper_thread_index(), 1);
}

#[test]
fn empty_context_policy_does_not_construct_contexts_or_run_callbacks() {
    let main = Context::default();
    let mut tasks: [usize; 0] = [];
    let created = AtomicUsize::new(0);
    run_launcher_context_tasks(
        &mut tasks,
        &main,
        true,
        || {
            created.fetch_add(1, Ordering::SeqCst);
            Context::default()
        },
        |_, _| panic!("empty task callback must not execute"),
    )
    .unwrap();
    assert_eq!(created.load(Ordering::SeqCst), 0);
    assert_eq!(context_task_launcher(true, 0).nb_threads(), 1);
}

#[test]
fn parallel_context_policy_transports_one_actual_typed_failure() {
    let main = Context::default();
    let mut tasks = [true, false];
    let error =
        run_launcher_context_tasks(&mut tasks, &main, true, Context::default, parallel_failure)
            .unwrap_err();
    assert_eq!(error.exception_type(), "Standard_ProgramError");
    assert_eq!(
        error.message(),
        "Standard_ConstructionError: context parallel failure"
    );
    assert_eq!(error.stack_string(), "retained-native-stack");
}

struct RequestedCancellation;
impl martensite_tkernel::message::ProgressIndicatorHooks for RequestedCancellation {
    fn show(
        &self,
        scope: &martensite_tkernel::message::ProgressScope<'_>,
        position: f64,
        _force: bool,
    ) {
        assert!(scope.max_value().is_finite());
        assert!(position.is_finite());
    }
    fn user_break(&self) -> bool {
        // A sticky request must suppress the deliberately failing callback.
        true
    }
}

struct CancellationTask<'a, 'p> {
    scope: &'a martensite_tkernel::message::ProgressScope<'p>,
    cancelled: bool,
}

#[test]
fn context_policy_keeps_native_initial_cancellation_before_failing_work() {
    use martensite_tkernel::message::{ProgressIndicator, ProgressScope};
    let indicator = ProgressIndicator::new(Box::new(RequestedCancellation));
    let outer = ProgressScope::unnamed(&indicator.start(), 2., false);
    let scopes: Vec<_> = (0..2)
        .map(|_| ProgressScope::unnamed(&outer.next(1.), 1., false))
        .collect();
    let mut tasks: Vec<_> = scopes
        .iter()
        .map(|scope| CancellationTask {
            scope,
            cancelled: false,
        })
        .collect();
    let main = Context::default();
    run_launcher_context_tasks(&mut tasks, &main, false, Context::default, cancelled_task).unwrap();
    assert!(tasks.iter().all(|task| task.cancelled));
}

fn cancelled_task(task: &mut CancellationTask<'_, '_>, _context: &Context) -> Result<(), Failure> {
    if !task.scope.more() {
        task.cancelled = true;
        return Ok(());
    }
    Err(Failure::with_type(
        "Standard_ConstructionError",
        "uncancelled work must not be performed",
        "",
    ))
}

fn parallel_failure(fails: &mut bool, _context: &Context) -> Result<(), Failure> {
    if *fails {
        return Err(Failure::with_type(
            "Standard_ConstructionError",
            "context parallel failure",
            "retained-native-stack",
        ));
    }
    Ok(())
}

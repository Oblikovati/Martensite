//! The native executor owns parallel exception capture; serial calls retain Result semantics.
#[path = "lifetime.rs"]
mod lifetime;
#[path = "observations.rs"]
mod observations;
use super::execute_with_launcher;
use crate::bop_algo::run_context_tasks;
use crate::int_tools::Context;
use lifetime::DropTrackedContext;
use martensite_tkernel::{
    osd::{Launcher, ThreadPool},
    standard::{Failure, Handle},
};
use observations::{ActualContext, ContextTrace};
use std::sync::atomic::Ordering;
use std::sync::{Arc, atomic::AtomicUsize};

fn context_family<C: Sync>(main: C) {
    let pool = ThreadPool::new(2);
    let launcher = Launcher::new(&pool, 2);
    assert_eq!(launcher.lower_thread_index(), 0);
    assert_eq!(launcher.upper_thread_index(), launcher.nb_threads() - 1);
    let trace = ContextTrace::new(&main, launcher.nb_threads() as usize);
    let _: &(dyn Send + Sync) = &trace;
    let mut tasks = [0usize; 32];
    execute_with_launcher(&mut tasks, &main, &launcher, |task, context| {
        trace.record(context);
        *task += 1;
        Ok(())
    })
    .unwrap();
    assert_eq!(tasks, [1; 32]);
    trace.assert_shared(launcher.nb_threads() as usize);
}

#[test]
fn workers_and_caller_share_the_exact_caller_context() {
    context_family(Context::default());
}

#[test]
fn workers_and_caller_share_the_exact_caller_handle() {
    context_family(Handle::new(Context::default()));
}

#[test]
fn serial_tasks_use_the_caller_context_and_stop_at_first_error() {
    let main = Context::default();
    let mut tasks = [0usize; 3];
    let failure = serial_failure(&mut tasks, &main);
    assert_eq!(failure.exception_type(), "Standard_ConstructionError");
    assert_eq!(failure.stack_string(), "kept-stack");
    assert_eq!(tasks, [1, 0, 0]);
}

fn serial_failure(tasks: &mut [usize], main: &Context) -> Failure {
    run_context_tasks(
        tasks,
        main,
        crate::bop_algo::Parallelism::SERIAL,
        |task, context| {
            assert!(std::ptr::eq(context, main));
            *task = 1;
            Err(Failure::with_type(
                "Standard_ConstructionError",
                "first",
                "kept-stack",
            ))
        },
    )
    .unwrap_err()
}

fn parallel_failure(multiple: bool) -> Failure {
    let main = Context::default();
    let pool = ThreadPool::new(2);
    let launcher = Launcher::new(&pool, 2);
    let trace = ContextTrace::new(&main, launcher.nb_threads() as usize);
    let caller = std::thread::current().id();
    let mut tasks = [0usize; 2];
    execute_with_launcher(&mut tasks, &main, &launcher, |_, context| {
        trace.record(context);
        failing_thread(std::thread::current().id() == caller, multiple)
    })
    .unwrap_err()
}

// With one shared context only the thread distinguishes the caller from the worker.
fn failing_thread(caller: bool, multiple: bool) -> Result<(), Failure> {
    if caller && !multiple {
        return Ok(());
    }
    let message = if caller { "main" } else { "worker" };
    Err(Failure::with_type(
        "Standard_ConstructionError",
        message,
        "retained-stack",
    ))
}

#[test]
fn parallel_single_failure_uses_existing_sdk_type_message_and_stack() {
    let error = parallel_failure(false);
    assert_eq!(error.exception_type(), "Standard_ProgramError");
    assert_eq!(error.message(), "Standard_ConstructionError: worker");
    assert_eq!(error.stack_string(), "retained-stack");
}

#[test]
fn parallel_multiple_failures_follow_sdk_worker_index_aggregation() {
    let error = parallel_failure(true);
    assert_eq!(error.exception_type(), "Standard_ProgramError");
    assert_eq!(
        error.message(),
        "Multiple exceptions:\nStandard_ConstructionError: worker\nStandard_ConstructionError: main"
    );
    assert_eq!(error.stack_string(), "");
}

#[test]
fn serial_unwind_keeps_its_original_payload() {
    let main = Context::default();
    let mut tasks = [0usize; 3];
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        run_context_tasks(
            &mut tasks,
            &main,
            crate::bop_algo::Parallelism::SERIAL,
            |task, _| {
                *task = 1;
                std::panic::panic_any(73u32)
            },
        )
    }));
    let payload = result.unwrap_err();
    assert_eq!(payload.downcast_ref::<u32>(), Some(&73));
    assert_eq!(tasks, [1, 0, 0]);
}

#[test]
fn worker_unwind_is_typed_and_the_caller_context_outlives_it() {
    let drops = Arc::new(AtomicUsize::new(0));
    let main = DropTrackedContext::new(drops.clone());
    let _: &(dyn Send + Sync) = &main;
    let pool = ThreadPool::new(2);
    let launcher = Launcher::new(&pool, 2);
    let trace = ContextTrace::new(&main, launcher.nb_threads() as usize);
    let error = worker_unwind(&main, &launcher, &trace);
    assert_eq!(error.exception_type(), "Standard_ProgramError");
    assert_eq!(error.message(), "panic: worker-owned unwind");
    assert_eq!(drops.load(Ordering::SeqCst), 0);
    drop(main);
    assert_eq!(drops.load(Ordering::SeqCst), 1);
}

fn worker_unwind(
    main: &DropTrackedContext,
    launcher: &Launcher<'_>,
    trace: &ContextTrace,
) -> Failure {
    let caller = std::thread::current().id();
    let mut tasks = [0usize; 2];
    execute_with_launcher(&mut tasks, main, launcher, |_, context| {
        let _: &(dyn Send + Sync) = &context.context;
        trace.record(context);
        if std::thread::current().id() != caller {
            panic!("worker-owned unwind");
        }
        Ok(())
    })
    .unwrap_err()
}

#[test]
fn empty_task_slice_runs_no_callback() {
    let main = Handle::new(Context::default());
    let mut tasks: [usize; 0] = [];
    run_context_tasks(
        &mut tasks,
        &main,
        crate::bop_algo::Parallelism::UNLIMITED,
        |_, _| {
            panic!("empty input must not invoke the task callback");
        },
    )
    .unwrap();
}

#[test]
fn context_cached_services_are_shared_by_every_worker() {
    cached_context_family(Context::default());
}

#[test]
fn handle_cached_services_are_shared_by_every_worker() {
    cached_context_family(Handle::new(Context::default()));
}

fn cached_context_family<C: ActualContext>(main: C) {
    let edge = context_probe_edge();
    let cached = main.actual_context().proj_pc(&edge).unwrap();
    let pool = ThreadPool::new(2);
    let launcher = Launcher::new(&pool, 2);
    let trace = ContextTrace::for_cache(&cached, launcher.nb_threads() as usize);
    let mut tasks = [0usize; 32];
    execute_with_launcher(&mut tasks, &main, &launcher, |_, context| {
        trace.record_cache(&context.actual_context().proj_pc(&edge)?);
        Ok(())
    })
    .unwrap();
    trace.assert_shared(launcher.nb_threads() as usize);
}

fn context_probe_edge() -> martensite_tkbrep::topods::Edge {
    martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeEdge::from_points(
        martensite_tkmath::gp::Point3::default(),
        martensite_tkmath::gp::Point3::new(0., 0., 2.),
    )
    .unwrap()
    .edge()
    .unwrap()
}

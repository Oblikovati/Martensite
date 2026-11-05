//! The native executor owns parallel exception capture; serial calls retain Result semantics.
#[path = "lifetime.rs"]
mod lifetime;
#[path = "observations.rs"]
mod observations;
use super::{ContextTaskSlots, context_worker_cap, execute_with_launcher};
use crate::bop_algo::run_context_tasks;
use crate::int_tools::Context;
use lifetime::DropTrackedContext;
use martensite_tkernel::{
    osd::{Launcher, ThreadPool},
    standard::{Failure, Handle},
};
use observations::{ActualContext, ContextTrace, NativeContextFactory};
use std::sync::atomic::Ordering;
use std::sync::{Arc, atomic::AtomicUsize};

fn context_family<C: Send + Sync>(main: C, factory: impl Fn() -> C + Sync) {
    let pool = ThreadPool::new(2);
    let launcher = Launcher::new(&pool, 2);
    assert_eq!(launcher.lower_thread_index(), 0);
    assert_eq!(launcher.upper_thread_index(), launcher.nb_threads() - 1);
    let trace = ContextTrace::new(&main, launcher.nb_threads() as usize);
    let _: &(dyn Send + Sync) = &trace;
    let mut tasks = [0usize; 32];
    execute_with_launcher(&mut tasks, &main, &launcher, factory, |task, context| {
        trace.record(context);
        *task += 1;
        Ok(())
    })
    .unwrap();
    assert_eq!(tasks, [1; 32]);
    trace.assert_isolated(launcher.nb_threads() as usize);
}

#[test]
fn actual_contexts_reuse_main_and_isolate_lazy_worker_state() {
    let main = Context::default();
    let factory = NativeContextFactory::default();
    let _: &(dyn Send + Sync) = &factory;
    context_family(main, || factory.context());
    assert_eq!(factory.created.load(Ordering::SeqCst), 1);
}

#[test]
fn actual_context_handles_keep_the_exact_supplied_main_handle() {
    let main = Handle::new(Context::default());
    let factory = NativeContextFactory::default();
    context_family(main, || factory.handle());
    assert_eq!(factory.created.load(Ordering::SeqCst), 1);
}

#[test]
fn serial_tasks_never_construct_worker_contexts_and_stop_at_first_error() {
    let main = Context::default();
    let factory = NativeContextFactory::default();
    let mut tasks = [0usize; 3];
    let failure = serial_failure(&mut tasks, &main, &factory);
    assert_eq!(failure.exception_type(), "Standard_ConstructionError");
    assert_eq!(failure.stack_string(), "kept-stack");
    assert_eq!(tasks, [1, 0, 0]);
    assert_eq!(factory.created.load(Ordering::SeqCst), 0);
}

fn serial_failure(tasks: &mut [usize], main: &Context, factory: &NativeContextFactory) -> Failure {
    run_context_tasks(
        tasks,
        main,
        false,
        || factory.context(),
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

#[test]
fn invalid_worker_indices_are_typed_and_do_not_invoke_factory() {
    let main = Context::default();
    let factory = NativeContextFactory::default();
    let pool = ThreadPool::new(2);
    let launcher = Launcher::new(&pool, 2);
    let slots = ContextTaskSlots::new(&main, &launcher);
    let _: &(dyn Send + Sync) = &slots;
    for index in [-1, launcher.upper_thread_index() + 1] {
        let error = slots.at(index, &|| factory.context()).err().unwrap();
        assert_eq!(error.exception_type(), "Standard_OutOfRange");
        assert!(error.message().contains(&format!("worker_index={index}")));
    }
    assert_eq!(factory.created.load(Ordering::SeqCst), 0);
}

#[test]
fn each_context_slot_is_reused_and_main_factory_is_never_called() {
    let main = Handle::new(Context::default());
    let factory = NativeContextFactory::default();
    let pool = ThreadPool::new(2);
    let launcher = Launcher::new(&pool, 2);
    let slots = ContextTaskSlots::new(&main, &launcher);
    for index in launcher.lower_thread_index()..=launcher.upper_thread_index() {
        let first = slots.at(index, &|| factory.handle()).unwrap();
        let second = slots.at(index, &|| factory.handle()).unwrap();
        assert!(std::ptr::eq(first, second));
        assert_eq!(
            std::ptr::eq(first, &main),
            index == launcher.upper_thread_index()
        );
    }
    assert_eq!(
        factory.created.load(Ordering::SeqCst),
        (launcher.nb_threads() - 1) as usize
    );
}

fn parallel_failure(multiple: bool) -> Failure {
    let main = Context::default();
    let pool = ThreadPool::new(2);
    let launcher = Launcher::new(&pool, 2);
    let trace = ContextTrace::new(&main, launcher.nb_threads() as usize);
    let mut tasks = [0usize; 2];
    execute_with_launcher(
        &mut tasks,
        &main,
        &launcher,
        Context::default,
        |_, context| {
            trace.record(context);
            failing_context(context, &main, multiple)
        },
    )
    .unwrap_err()
}

fn failing_context(context: &Context, main: &Context, multiple: bool) -> Result<(), Failure> {
    let caller = std::ptr::eq(context, main);
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
        run_context_tasks(&mut tasks, &main, false, Context::default, |task, _| {
            *task = 1;
            std::panic::panic_any(73u32)
        })
    }));
    let payload = result.unwrap_err();
    assert_eq!(payload.downcast_ref::<u32>(), Some(&73));
    assert_eq!(tasks, [1, 0, 0]);
}

#[test]
fn worker_unwind_is_typed_and_owned_context_is_dropped_after_join() {
    let drops = Arc::new(AtomicUsize::new(0));
    let main = DropTrackedContext::main(drops.clone());
    let _: &(dyn Send + Sync) = &main;
    let pool = ThreadPool::new(2);
    let launcher = Launcher::new(&pool, 2);
    let trace = ContextTrace::new(&main, launcher.nb_threads() as usize);
    let error = worker_unwind(&main, &launcher, &trace, drops.clone());
    assert_eq!(error.exception_type(), "Standard_ProgramError");
    assert_eq!(error.message(), "panic: worker-owned unwind");
    assert_eq!(drops.load(Ordering::SeqCst), 1);
}

fn worker_unwind(
    main: &DropTrackedContext,
    launcher: &Launcher<'_>,
    trace: &ContextTrace,
    drops: Arc<AtomicUsize>,
) -> Failure {
    let mut tasks = [0usize; 2];
    execute_with_launcher(
        &mut tasks,
        main,
        launcher,
        || DropTrackedContext::worker(drops.clone()),
        |_, context| injected_worker_unwind(context, trace),
    )
    .unwrap_err()
}

fn injected_worker_unwind(
    context: &DropTrackedContext,
    trace: &ContextTrace,
) -> Result<(), Failure> {
    let _: &(dyn Send + Sync) = &context.context;
    trace.record(context);
    if context.worker {
        panic!("worker-owned unwind");
    }
    Ok(())
}

#[test]
fn empty_task_slice_has_no_context_factory_or_callback_activity() {
    let main = Handle::new(Context::default());
    let factory = NativeContextFactory::default();
    let mut tasks: [usize; 0] = [];
    run_context_tasks(
        &mut tasks,
        &main,
        false,
        || factory.handle(),
        |_, _| {
            panic!("empty input must not invoke the task callback");
        },
    )
    .unwrap();
    assert_eq!(factory.created.load(Ordering::SeqCst), 0);
}

#[test]
fn actual_context_cached_services_are_reused_and_isolated() {
    let main = Context::default();
    let factory = Context::default;
    cached_context_family(main, factory);
}

#[test]
fn actual_handle_cached_services_are_reused_and_isolated() {
    let main = Handle::new(Context::default());
    let factory = || Handle::new(Context::default());
    cached_context_family(main, factory);
}

fn cached_context_family<C: ActualContext>(main: C, factory: impl Fn() -> C + Sync) {
    let edge = context_probe_edge();
    let cached = main.actual_context().proj_pc(&edge).unwrap();
    let pool = ThreadPool::new(2);
    let launcher = Launcher::new(&pool, 2);
    let trace = ContextTrace::for_cache(&cached, launcher.nb_threads() as usize);
    let mut tasks = [0usize; 32];
    execute_with_launcher(&mut tasks, &main, &launcher, factory, |_, context| {
        trace.record_cache(&context.actual_context().proj_pc(&edge)?);
        Ok(())
    })
    .unwrap();
    trace.assert_isolated(launcher.nb_threads() as usize);
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

#[test]
fn worker_hint_saturates_without_wrapping_large_task_counts_to_serial() {
    let maximum = i32::MAX as usize;
    let cases = [
        (0usize, 0),
        (1, 1),
        (maximum, i32::MAX),
        (maximum + 1, i32::MAX),
        (usize::MAX, i32::MAX),
    ];
    for (count, expected) in cases {
        assert_eq!(context_worker_cap(count), expected);
    }
    if let Some(wrapped_zero) = 1usize.checked_shl(32) {
        assert_eq!(context_worker_cap(wrapped_zero), i32::MAX);
    }
}

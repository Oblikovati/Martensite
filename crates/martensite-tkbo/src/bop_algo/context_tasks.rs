//! Raw native task choreography; completion and geometry publication remain caller-owned.
//! `run_context_tasks` returns the first serial callback error directly. Its parallel path
//! and both `run_launcher_context_tasks` paths use the existing native-failure bridge;
//! the SDK launcher owns unwind, stop/wait and worker-index diagnostic aggregation.
//! Raw error text is not a cross-thread determinism promise; geometry merges by input order.
use martensite_tkernel::{
    osd::{Launcher, ThreadPool},
    standard::{Failure, raise_native_failure},
};
#[path = "context_tasks/slots.rs"]
mod slots;
use slots::ContextTaskSlots;

/// Reuse the exact caller context and lazy worker contexts on the existing executor.
/// Example: `run_context_tasks(tasks, &context, parallel, Context::default, perform)?`.
/// The caller owns any task scopes; each worker creates its borrowed range locally.
pub(crate) fn run_context_tasks<T: Send, C: Send + Sync>(
    tasks: &mut [T],
    main: &C,
    parallel: bool,
    make_context: impl Fn() -> C + Sync,
    perform: impl Fn(&mut T, &C) -> Result<(), Failure> + Sync,
) -> Result<(), Failure> {
    if !parallel {
        for task in tasks {
            perform(task, main)?;
        }
        return Ok(());
    }
    let launcher = Launcher::new(
        ThreadPool::default_pool(-1),
        context_worker_cap(tasks.len()),
    );
    execute_with_launcher(tasks, main, &launcher, make_context, perform)
}

/// Use caller-owned worker reservations without replacing task choreography.
/// Example: `execute_with_launcher(tasks, main, launcher, make_context, perform)?`.
pub(super) fn execute_with_launcher<T: Send, C: Send + Sync>(
    tasks: &mut [T],
    main: &C,
    launcher: &Launcher<'_>,
    make_context: impl Fn() -> C + Sync,
    perform: impl Fn(&mut T, &C) -> Result<(), Failure> + Sync,
) -> Result<(), Failure> {
    let contexts = ContextTaskSlots::new(main, launcher);
    launcher.perform_iter(tasks.iter_mut(), |thread_index, task| {
        let result = contexts
            .at(thread_index, &make_context)
            .and_then(|context| perform(task, context));
        raise_native_failure(result);
    })
}

#[path = "context_tasks/launcher_policy.rs"]
mod launcher_policy;
pub(super) use launcher_policy::context_task_launcher;
pub(crate) use launcher_policy::run_launcher_context_tasks;

#[cfg(test)]
#[path = "context_tasks/tests.rs"]
mod tests;

fn context_worker_cap(task_count: usize) -> i32 {
    // Only the launcher's worker hint is bounded; the full usize iterator remains intact.
    let maximum = i32::MAX as usize;
    task_count.min(maximum) as i32
}

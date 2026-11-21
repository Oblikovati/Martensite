//! Raw native task choreography; completion and geometry publication remain caller-owned.
//! `run_context_tasks` returns the first serial callback error directly. Its parallel path
//! and both `run_launcher_context_tasks` paths use the existing native-failure bridge;
//! the SDK launcher owns unwind, stop/wait and worker-index diagnostic aggregation.
//! Raw error text is not a cross-thread determinism promise; geometry merges by input order.
//! Every worker uses the caller's context. Its caches are internally locked, so one context
//! serves all workers. A context per worker, as in the native design, rebuilt each cached
//! service once per worker: on the 239-curve conical text check of #419 eight workers then
//! used 813 MB against 243 MB serial, and the shared context used 267 MB.
use martensite_tkernel::{
    osd::{Launcher, ThreadPool},
    standard::{Failure, raise_native_failure},
};
#[path = "context_tasks/parallelism.rs"]
mod parallelism;
pub(crate) use parallelism::Parallelism;

/// Run tasks with the caller's context, serially or on the existing executor.
/// Example: `run_context_tasks(tasks, &context, options.parallelism(), perform)?`.
/// The caller owns any task scopes; each worker creates its borrowed range locally.
pub(crate) fn run_context_tasks<T: Send, C: Sync>(
    tasks: &mut [T],
    main: &C,
    parallel: Parallelism,
    perform: impl Fn(&mut T, &C) -> Result<(), Failure> + Sync,
) -> Result<(), Failure> {
    if !parallel.enabled() {
        for task in tasks {
            perform(task, main)?;
        }
        return Ok(());
    }
    let launcher = Launcher::new(
        ThreadPool::default_pool(-1),
        parallel.worker_cap(tasks.len()),
    );
    execute_with_launcher(tasks, main, &launcher, perform)
}

/// Use caller-owned worker reservations without replacing task choreography.
/// Example: `execute_with_launcher(tasks, main, launcher, perform)?`.
pub(super) fn execute_with_launcher<T: Send, C: Sync>(
    tasks: &mut [T],
    main: &C,
    launcher: &Launcher<'_>,
    perform: impl Fn(&mut T, &C) -> Result<(), Failure> + Sync,
) -> Result<(), Failure> {
    launcher.perform_iter(tasks.iter_mut(), |_, task| {
        raise_native_failure(perform(task, main));
    })
}

#[path = "context_tasks/launcher_policy.rs"]
mod launcher_policy;
pub(super) use launcher_policy::context_task_launcher;
pub(crate) use launcher_policy::run_launcher_context_tasks;

#[cfg(test)]
#[path = "context_tasks/tests.rs"]
mod tests;

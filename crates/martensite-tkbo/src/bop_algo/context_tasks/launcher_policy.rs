//! Native internal-pool context tasks catch failures in a launcher even in serial mode.
use super::{Parallelism, execute_with_launcher};
use martensite_tkernel::{
    osd::{Launcher, ThreadPool},
    standard::Failure,
};

/// Preserve the native context-overload transport without certifying operation completion.
/// Example: `run_launcher_context_tasks(tasks, &main, options.parallelism(), perform)?`.
/// Matches the native internal-pool context overload, except that every worker shares the
/// caller's context. The caller supplies a usable main context (non-null for a Handle); opaque
/// `C` is not inspected or repaired.
/// The caller still owns null initialization, cancellation and successful geometry publication.
pub(crate) fn run_launcher_context_tasks<T: Send, C: Sync>(
    tasks: &mut [T],
    main: &C,
    parallel: Parallelism,
    perform: impl Fn(&mut T, &C) -> Result<(), Failure> + Sync,
) -> Result<(), Failure> {
    let launcher = context_task_launcher(parallel, tasks.len());
    execute_with_launcher(tasks, main, &launcher, perform)
}

/// Reserve the native context-overload worker policy; `context_task_launcher(Parallelism::SERIAL, count)`.
/// Serial mode deliberately selects maximum zero, retaining the caller and its failure catcher.
pub(in crate::bop_algo) fn context_task_launcher(
    parallel: Parallelism,
    count: usize,
) -> Launcher<'static> {
    let pool = ThreadPool::default_pool(-1);
    context_launcher_on(pool, parallel, count)
}

fn context_launcher_on(pool: &ThreadPool, parallel: Parallelism, count: usize) -> Launcher<'_> {
    let maximum = if parallel.enabled() {
        parallel.worker_cap(count)
    } else {
        0
    };
    Launcher::new(pool, maximum)
}

#[cfg(test)]
#[path = "launcher_policy_tests.rs"]
mod tests;

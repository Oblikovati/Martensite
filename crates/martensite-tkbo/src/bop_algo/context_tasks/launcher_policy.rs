//! Native internal-pool context tasks catch failures in a launcher even in serial mode.
use super::{context_worker_cap, execute_with_launcher};
use martensite_tkernel::{
    osd::{Launcher, ThreadPool},
    standard::Failure,
};

/// Preserve the native context-overload transport without certifying operation completion.
/// Example: `run_launcher_context_tasks(tasks, &main, parallel, Context::default, perform)?`.
/// Matches the native internal-pool context overload, not its alternative context-map backend.
/// The caller supplies a usable main context (non-null for a Handle); opaque `C` is not inspected
/// or repaired. The factory must yield fresh, independent, non-null native-compatible worker
/// contexts using the common base allocator, as `Context::default()` does.
/// The caller still owns null initialization, cancellation and successful geometry publication.
pub(crate) fn run_launcher_context_tasks<T: Send, C: Send + Sync>(
    tasks: &mut [T],
    main: &C,
    parallel: bool,
    make_context: impl Fn() -> C + Sync,
    perform: impl Fn(&mut T, &C) -> Result<(), Failure> + Sync,
) -> Result<(), Failure> {
    let launcher = context_task_launcher(parallel, tasks.len());
    execute_with_launcher(tasks, main, &launcher, make_context, perform)
}

/// Reserve the native context-overload worker policy; `context_task_launcher(false, count)`.
/// Serial mode deliberately selects maximum zero, retaining the caller and its failure catcher.
pub(in crate::bop_algo) fn context_task_launcher(
    parallel: bool,
    count: usize,
) -> Launcher<'static> {
    let pool = ThreadPool::default_pool(-1);
    context_launcher_on(pool, parallel, count)
}

fn context_launcher_on(pool: &ThreadPool, parallel: bool, count: usize) -> Launcher<'_> {
    let maximum = if parallel {
        context_worker_cap(count)
    } else {
        0
    };
    Launcher::new(pool, maximum)
}

#[cfg(test)]
#[path = "launcher_policy_tests.rs"]
mod tests;

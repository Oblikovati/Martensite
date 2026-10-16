//! Parallel loops on
//! the default thread pool.
use super::{Launcher, ThreadPool, thread::native};
use crate::standard::Failure;
use std::sync::OnceLock;

/// Parallel loops; `Parallel::for_range(0, 10, |index| {}, false)?`.
pub struct Parallel;

impl Parallel {
    /// Always true: there is no TBB backend, so the kernel thread pool is the only backend;
    /// `Parallel::use_kernel_threads()`.
    pub const fn use_kernel_threads() -> bool {
        true
    }
    /// Ignored: there is no other backend; `Parallel::set_use_kernel_threads(false)`.
    pub const fn set_use_kernel_threads(_use_kernel_threads: bool) {}
    /// The online processor count, read once: `sysconf(_SC_NPROCESSORS_ONLN)` on Linux and
    /// macOS (not the affinity mask; -1 on error), `dwNumberOfProcessors` on Windows;
    /// `Parallel::nb_logical_processors()`.
    pub fn nb_logical_processors() -> i32 {
        static THE_COUNT: OnceLock<i32> = OnceLock::new();
        *THE_COUNT.get_or_init(native::online_processors)
    }
    /// Calls `functor(index)` for `begin..end` on the default pool; the caller runs
    /// every index itself when forced or when the range holds one index;
    /// `Parallel::for_range(0, n, |index| {}, false)?`.
    ///
    /// Sequential runs let a panic unwind unchanged;
    /// pooled runs return it as `Standard_ProgramError` (see [`Launcher::perform`]).
    pub fn for_range(
        begin: i32,
        end: i32,
        functor: impl Fn(i32) + Sync,
        is_force_single_thread_execution: bool,
    ) -> Result<(), Failure> {
        let range: i32 = end.wrapping_sub(begin);
        if is_force_single_thread_execution || range == 1 {
            // `for (it = begin; it != end; ++it)`.
            let mut index: i32 = begin;
            while index != end {
                functor(index);
                index = index.wrapping_add(1);
            }
            return Ok(());
        }
        let launcher: Launcher<'_> = Launcher::new(ThreadPool::default_pool(-1), range);
        launcher.perform(begin, end, |_, index| functor(index))
    }
    /// Calls `functor(item)` for every item on the default pool, with at most
    /// `nb_items` threads (-1: the pool default); forced or single-item runs stay in the
    /// caller; `Parallel::for_each(values.iter_mut(), |value| *value = 0.0, false, -1)?`.
    pub fn for_each<I>(
        items: I,
        functor: impl Fn(I::Item) + Sync,
        is_force_single_thread_execution: bool,
        nb_items: i32,
    ) -> Result<(), Failure>
    where
        I: Iterator + Send,
        I::Item: Send,
    {
        if is_force_single_thread_execution || nb_items == 1 {
            items.for_each(functor);
            return Ok(());
        }
        let pool: &ThreadPool = ThreadPool::default_pool(-1);
        let nb_threads: i32 = if nb_items == -1 {
            -1
        } else {
            nb_items.min(pool.nb_default_threads_to_launch())
        };
        Launcher::new(pool, nb_threads).perform_iter(items, |_, item| functor(item))
    }
}

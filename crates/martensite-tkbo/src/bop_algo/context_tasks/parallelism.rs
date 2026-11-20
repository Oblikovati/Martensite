//! How many executor workers an operation's context tasks may use.
use std::num::NonZeroUsize;

/// Serial execution, or the executor with at most a number of workers.
/// Example: `Parallelism::limited(NonZeroUsize::MIN).worker_cap(100) == 1`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Parallelism {
    // Zero is serial; `usize::MAX` leaves the worker count to the task count.
    max_workers: usize,
}

impl Parallelism {
    /// Every task runs on the caller; `Parallelism::SERIAL.enabled() == false`.
    pub(crate) const SERIAL: Self = Self { max_workers: 0 };
    /// One worker per task, up to the pool size; `Parallelism::UNLIMITED.enabled()`.
    pub(crate) const UNLIMITED: Self = Self {
        max_workers: usize::MAX,
    };

    /// At most `max_workers` workers; `Parallelism::limited(NonZeroUsize::new(4).unwrap())`.
    pub(crate) fn limited(max_workers: NonZeroUsize) -> Self {
        Self {
            max_workers: max_workers.get(),
        }
    }

    /// The native flag without a worker bound; `Parallelism::from_flag(options.run_parallel())`.
    pub(crate) fn from_flag(parallel: bool) -> Self {
        if parallel {
            Self::UNLIMITED
        } else {
            Self::SERIAL
        }
    }

    /// Whether tasks run on the executor; `options.parallelism().enabled()`.
    pub(crate) fn enabled(self) -> bool {
        self.max_workers > 0
    }

    /// A single task runs on the caller, as the native code does; `parallel.for_tasks(work.len())`.
    pub(crate) fn for_tasks(self, count: usize) -> Self {
        if count == 1 { Self::SERIAL } else { self }
    }

    /// The launcher's worker hint: no more workers than tasks, and within `i32`.
    /// Example: `Parallelism::UNLIMITED.worker_cap(usize::MAX) == i32::MAX`.
    pub(crate) fn worker_cap(self, task_count: usize) -> i32 {
        let workers = task_count.min(self.max_workers).min(i32::MAX as usize);
        // Within `i32::MAX` by the clamp above, so the conversion cannot fail.
        i32::try_from(workers).unwrap_or(i32::MAX)
    }
}

#[cfg(test)]
#[path = "parallelism_tests.rs"]
mod tests;

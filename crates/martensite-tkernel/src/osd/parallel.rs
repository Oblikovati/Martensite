//! Threads, the kernel thread pool and parallel loops: threads, the thread pool,
//! its launcher and the parallel loops (without TBB).
//!
//! The pool runs on `std::thread`; thread indices, the locking
//! of idle workers and the "busy pool runs in the caller" fallback are fixed contracts.
mod event;
mod iterator_interface;
mod job;
mod launcher;
mod loops;
mod thread;
mod thread_pool;
mod universal_iterator;
mod worker;

pub use iterator_interface::{ParallelIteratorInterface, destroy_parallel_iterator};
pub use launcher::Launcher;
pub use loops::Parallel;
pub use thread::{OsdThread, OsdThreadCallback, ThreadFunction};
pub use thread_pool::ThreadPool;

pub use universal_iterator::ParallelUniversalIterator;

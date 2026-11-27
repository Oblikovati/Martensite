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
mod platform_thread;
mod thread;
mod thread_pool;
mod universal_iterator;
mod worker;

pub use iterator_interface::{ParallelIteratorInterface, destroy_parallel_iterator};
pub use launcher::Launcher;
pub use loops::Parallel;
pub use platform_thread::wasm_worker_pool_size;
#[cfg(target_arch = "wasm32")]
pub use platform_thread::{
    WasmWorkerPool, init_wasm_worker_pool, init_wasm_worker_pool_from_url,
    martensite_wasm_worker_entry, martensite_wasm_worker_register,
};
pub use thread::{OsdThread, OsdThreadCallback};
pub use thread_pool::ThreadPool;

pub use universal_iterator::ParallelUniversalIterator;

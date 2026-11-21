//! Returning freed heap pages to the operating system (#580).
//!
//! glibc keeps freed memory in per-thread arenas. A parallel phase that follows a
//! memory-heavy serial phase allocates in the worker threads' arenas and cannot reuse
//! the free pages of the main arena, so peak RSS grows by the parallel working set.
//! Releasing those pages first keeps the peak at the serial level.
#![allow(unsafe_code)]

/// Gives the operating system the memory the allocator holds but no allocation uses.
/// Example: `NativeHeapRelease.release_free_pages()`.
pub trait HeapRelease {
    /// Returns whether the allocator gave memory back; `heap.release_free_pages()`.
    fn release_free_pages(&self) -> bool;
}

/// The process allocator: glibc's `malloc_trim` on Linux, nothing elsewhere.
/// Example: `NativeHeapRelease.release_free_pages()`.
#[derive(Clone, Copy, Debug, Default)]
pub struct NativeHeapRelease;

impl HeapRelease for NativeHeapRelease {
    fn release_free_pages(&self) -> bool {
        release_free_pages()
    }
}

#[cfg(all(target_os = "linux", target_env = "gnu"))]
fn release_free_pages() -> bool {
    // SAFETY: `malloc_trim` takes no pointer, is thread-safe in glibc and only returns
    // pages that no live allocation uses; a pad of zero keeps no extra heap top.
    unsafe { libc::malloc_trim(0) != 0 }
}

#[cfg(not(all(target_os = "linux", target_env = "gnu")))]
fn release_free_pages() -> bool {
    // Other allocators return memory on their own schedule; there is nothing to call.
    false
}

#[cfg(test)]
#[path = "heap_release_tests.rs"]
mod tests;

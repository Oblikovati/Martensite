//! Typed allocation borrows allocator sources while physical leases preserve safe Rust storage.
use super::raw_heap::heap_free;
use super::{AllocatorKind, BaseAllocator, Layout, locked};
use std::ptr::NonNull;
impl BaseAllocator {
    /// Typed allocation for RAII owners and collections, honouring the layout alignment.
    pub(super) fn allocate_typed(&self, layout: Layout) -> Option<NonNull<u8>> {
        if !self.is_heap_kind() && !self.uses_custom_source() {
            return self.allocate_builtin_arena(layout);
        }
        if self.may_hold_record(layout) {
            return self.allocate_virtual_typed(layout);
        }
        // A built-in heap policy allocates directly with `std::alloc` so the returned
        // pointer's provenance matches `layout` without a prepended header (#584, #63).
        let direct = self.direct_heap_layout(layout)?;
        // SAFETY: `direct.size() > 0` is enforced by `Allocator::allocate`.
        NonNull::new(unsafe { std::alloc::alloc_zeroed(direct) })
    }

    pub(super) fn direct_heap_layout(&self, layout: Layout) -> Option<Layout> {
        match &*self.fields.inner {
            AllocatorKind::Common | AllocatorKind::Heap => Some(layout),
            AllocatorKind::Aligned(alignment) => {
                let min_align = crate::standard::memory::raw_heap::mm_malloc_alignment(*alignment)?;
                Layout::from_size_align(layout.size(), layout.align().max(min_align)).ok()
            }
            _ => None,
        }
    }

    /// Releases a direct built-in heap typed allocation allocated with `direct_heap_layout`.
    ///
    /// # Safety
    /// `address` must come from `allocate_typed` on this direct built-in heap path with `layout`.
    pub(super) unsafe fn free_direct_heap_typed(&self, address: NonNull<u8>, layout: Layout) {
        let direct = self
            .direct_heap_layout(layout)
            .expect("direct heap typed allocation must retain a valid layout");
        // SAFETY: `address` was allocated via `std::alloc::alloc_zeroed(direct)`.
        unsafe { std::alloc::dealloc(address.as_ptr(), direct) };
    }
    fn allocate_builtin_arena(&self, layout: Layout) -> Option<NonNull<u8>> {
        match &*self.fields.inner {
            AllocatorKind::Incremental(engine) => {
                let mut engine = locked(engine);
                let address = engine.allocate_aligned(layout.size(), layout.align())?;
                engine.acquire();
                Some(address)
            }
            AllocatorKind::Accumulating(engine) => {
                locked(engine).allocate_aligned(layout.size(), layout.align())
            }
            _ => unreachable!("heap typed allocation uses virtual provenance records"),
        }
    }

    /// Releases a raw-policy owner or a typed arena allocation.
    ///
    /// # Safety
    /// `address` must come from this allocator's raw policy or typed arena and not be freed.
    /// Layout-bearing heap releases restore the separately recorded original allocation pointer.
    pub(super) unsafe fn free_typed(&self, address: NonNull<u8>) {
        // A built-in source releases through its policy whether or not it has retired (#584).
        if self.ever_custom() && (self.storage_source_retired() || self.uses_custom_source()) {
            // SAFETY: live virtual sources or independent backing accept original raw byte storage.
            unsafe { self.free_raw(address) }
                .expect("owned raw storage must be accepted by its live provider");
            self.release_virtual_lease();
            return;
        }
        // SAFETY: original arena leases and heap raw provenance remain valid.
        unsafe { self.free_builtin_typed(address) };
    }
    unsafe fn free_builtin_typed(&self, address: NonNull<u8>) {
        match &*self.fields.inner {
            AllocatorKind::Incremental(engine) => locked(engine).release(),
            AllocatorKind::Accumulating(engine) => {
                // Owned layouts can exceed lookup windows; actual block extents retain them.
                locked(engine)
                    .free_owned(address.as_ptr().addr())
                    .expect("owned accumulating allocation must retain its counted block");
            }
            // SAFETY: forwarded from this function's contract.
            _ => unsafe { heap_free(address) },
        }
    }
}

#[cfg(test)]
#[path = "typed_fast_path_tests.rs"]
mod typed_fast_path_tests;

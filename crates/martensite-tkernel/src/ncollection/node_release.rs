//! Typed linked-node destruction must release allocation even when the item destructor unwinds.
use super::BaseAllocator;
use crate::standard::memory::raw_heap::{heap_allocate, heap_free};
use allocator_api2::alloc::Allocator;
use std::{alloc::Layout, mem::ManuallyDrop, ptr::NonNull};
struct NodeAllocationRelease<F: FnOnce()> {
    release: Option<F>,
}
impl<F: FnOnce()> Drop for NodeAllocationRelease<F> {
    fn drop(&mut self) {
        // Exactly one cleanup consumes allocation provenance, including during unwinding.
        self.release.take().expect("owned node cleanup runs once")();
    }
}
pub(super) unsafe fn release_node_payload<T>(pointer: *mut T, release: impl FnOnce()) {
    let allocation = NodeAllocationRelease {
        release: Some(release),
    };
    // SAFETY: the caller guarantees `pointer` is a live, uniquely owned value that is dropped once. allocator-api2 0.4.0 Box::drop lacks unwind deallocation. Retain the allocation
    // guard while invoking user destruction, with the original allocator and layout.
    unsafe { std::ptr::drop_in_place(pointer) };
    drop(allocation);
}
pub(super) unsafe fn release_typed_node<T>(pointer: *mut T, allocator: &BaseAllocator) {
    let allocation = NonNull::new(pointer.cast()).expect("owned node is nonnull");
    let layout = Layout::new::<T>();
    // SAFETY: caller retains the original typed allocation and its exact layout.
    unsafe {
        release_node_payload(pointer, || allocator.deallocate(allocation, layout));
    }
}

pub(super) fn allocate_common_node<T>(construct: impl FnOnce() -> T) -> *mut T {
    let layout = Layout::new::<T>();
    let allocation = heap_allocate(layout.size(), layout.align(), false)
        .unwrap_or_else(|| std::alloc::handle_alloc_error(layout));
    let release = NodeAllocationRelease {
        // SAFETY: this guard exclusively owns the default heap allocation until initialization.
        release: Some(|| unsafe { heap_free(allocation) }),
    };
    let value = construct();
    // SAFETY: the allocation has the exact size and alignment and is not yet initialized.
    unsafe { allocation.cast::<T>().write(value) };
    // Transfer the allocation from the unwind guard to the initialized node.
    let _ = ManuallyDrop::new(release);
    allocation.cast().as_ptr()
}

pub(super) unsafe fn release_owned_node<T>(pointer: *mut T, allocation: Option<&BaseAllocator>) {
    if let Some(allocator) = allocation {
        // SAFETY: caller transfers its live typed node and original allocator.
        unsafe { release_typed_node(pointer, allocator) };
        return;
    }
    // SAFETY: callers transfer a live typed node from allocate_common_node.
    let pointer = unsafe { NonNull::new_unchecked(pointer) };
    // SAFETY: its allocation header is still live when the destructor completes or unwinds.
    unsafe { release_node_payload(pointer.as_ptr(), || heap_free(pointer.cast())) };
}

#[cfg(test)]
mod tests {
    use super::allocate_common_node;

    #[test]
    fn common_node_constructor_unwind_releases_heap_reservation() {
        // Miri checks that the allocation is reclaimed even though no node is returned.
        let result = std::panic::catch_unwind(|| {
            allocate_common_node::<u64>(|| panic!("node construction failed"))
        });
        assert!(result.is_err());
    }
}

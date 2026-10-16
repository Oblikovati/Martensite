//! Buckets use the common optimal raw policy; nodes retain independent global ownership.
use crate::ncollection::{AllocatorInvocation, AllocatorObject};
use std::alloc::{Layout, handle_alloc_error};
use std::ptr::NonNull;
#[cfg(test)]
pub(in super::super) mod observation;

pub(super) fn reserve(layout: Layout) -> NonNull<u8> {
    // The common optimal allocation uses the C heap; keep this allocation
    // separate from global node Box allocation while preserving malloc alignment.
    // Borrow the actual static handle; constructing an owning façade adds an extra heap event.
    #[cfg(test)]
    observation::allocation(layout.size());
    AllocatorInvocation::from_handle(AllocatorObject::common_base_allocator())
        .expect("common allocator pointer=null; expected static live owner")
        .allocate_optimal(layout.size())
        .unwrap_or_else(|| handle_alloc_error(layout))
}

/// The caller supplies this policy's live allocation, or the null table.
pub(super) unsafe fn release(address: *mut u8) {
    #[cfg(test)]
    observation::release(address.is_null());
    // Freeing null must not lazily allocate a common-allocator handle.
    if address.is_null() {
        return;
    }
    // SAFETY: the table retains the exact pointer returned by reserve until this final release.
    unsafe {
        AllocatorInvocation::from_handle(AllocatorObject::common_base_allocator())
            .expect("common allocator pointer=null; expected static live owner")
            .free(address)
    }
    .expect("common bucket allocation: expected matching raw release");
}

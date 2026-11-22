//! Qualified parent storage access ends before inherited virtual forwarding can destroy the receiver.
#![allow(unsafe_code)] // Explicit pointer-release and callback-free receiver-capture contracts are tested.
use super::{AllocatorInvocation, AllocatorPolicyInvocation, AllocatorSource};
use crate::standard::{Failure, raise_native_failure};
use std::ptr::NonNull;
pub(super) fn default_policy(source: &dyn AllocatorSource) -> AllocatorPolicyInvocation {
    // Shared physical policy is independent of the containing object's logical lifetime.
    source
        .allocator_object_base()
        .resource()
        .policy_invocation()
}
fn forwards_optimal(source: &dyn AllocatorSource) -> bool {
    source.allocator_object_base().allocate_calls_optimal()
}
fn forwards_allocate(source: &dyn AllocatorSource) -> bool {
    source.allocator_object_base().optimal_calls_allocate()
}
pub(super) fn allocate_default(
    invocation: &AllocatorInvocation,
    size: usize,
) -> Option<NonNull<u8>> {
    if raise_native_failure(invocation.with_source(forwards_optimal)) {
        return invocation.allocate_optimal(size);
    }
    let policy = raise_native_failure(invocation.parent_policy());
    policy.allocate(size, true)
}
pub(super) fn allocate_optimal_default(
    invocation: &AllocatorInvocation,
    size: usize,
) -> Option<NonNull<u8>> {
    if raise_native_failure(invocation.with_source(forwards_allocate)) {
        return invocation.allocate(size);
    }
    let policy = raise_native_failure(invocation.parent_policy());
    policy.allocate_optimal(size)
}
pub(super) unsafe fn free_default(
    invocation: &AllocatorInvocation,
    pointer: *mut u8,
) -> Result<(), Failure> {
    let policy = invocation.parent_policy()?;
    // SAFETY: the caller of free_default guarantees that `pointer` is null or a live block of
    // this allocator's policy, which is the contract of free_nullable.
    unsafe { policy.free_nullable(pointer) }
}

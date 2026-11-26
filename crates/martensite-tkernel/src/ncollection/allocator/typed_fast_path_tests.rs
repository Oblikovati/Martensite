//! Built-in allocators skip runtime dispatch and original-pointer records (#584).
#![allow(unsafe_code)] // The tests release the exact typed blocks they allocate.
use super::super::BaseAllocator;
use allocator_api2::alloc::Allocator;
use std::{alloc::Layout, sync::Arc};

#[test]
fn a_malloc_aligned_heap_block_keeps_no_record() {
    for allocator in [BaseAllocator::common(), BaseAllocator::global_heap()] {
        let layout = Layout::new::<[u64; 2]>();
        let block = Allocator::allocate(&allocator, layout)
            .unwrap()
            .cast::<u8>();
        assert!(!allocator.has_virtual_allocation(block));
        // SAFETY: the block came from this allocator with this layout.
        unsafe { Allocator::deallocate(&allocator, block, layout) };
    }
}

#[test]
fn an_over_aligned_heap_block_keeps_its_original_pointer() {
    let allocator = BaseAllocator::common();
    let layout = Layout::from_size_align(64, 64).unwrap();
    let block = Allocator::allocate(&allocator, layout)
        .unwrap()
        .cast::<u8>();
    assert_eq!(block.as_ptr().addr() % 64, 0);
    assert!(allocator.has_virtual_allocation(block));
    // SAFETY: the block came from this allocator with this layout.
    unsafe { Allocator::deallocate(&allocator, block, layout) };
    assert!(!allocator.has_virtual_allocation(block));
}

#[test]
fn every_lease_of_a_builtin_facade_shares_one_facade() {
    let allocator = BaseAllocator::incremental(4096);
    let first = allocator.storage_lease();
    let second = allocator.storage_lease();
    assert!(Arc::ptr_eq(&first.fields, &second.fields));
    assert!(Arc::ptr_eq(&first.fields, &first.storage_lease().fields));
    assert!(Arc::ptr_eq(&first.fields, &first.clone().fields));
}

#[test]
fn a_direct_heap_block_releases_through_another_current_allocator() {
    let original = BaseAllocator::fresh_common();
    let current = BaseAllocator::common();
    assert_ne!(original, current);
    let layout = Layout::new::<u32>();
    let block = Allocator::allocate(&original, layout).unwrap().cast::<u8>();
    // SAFETY: the live block came from a common policy, which `current` also releases.
    unsafe { original.deallocate_using(block, layout, &current) };
}

#[test]
fn incremental_values_still_count_against_reset() {
    let allocator = BaseAllocator::incremental(4096);
    let value = allocator.allocate_value(7_u64);
    assert!(allocator.reset(false).is_err());
    drop(value);
    assert!(allocator.reset(false).is_ok());
}

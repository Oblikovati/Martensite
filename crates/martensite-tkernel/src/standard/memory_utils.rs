//! Shared and unique owning pointers whose
//! storage comes from kernel allocators.
//!
//! Rust constructs the value first and moves it in.
//! Shared pointers are `Arc` and unique pointers are `Box`. The global
//! standard allocator is Rust's global allocator (the system allocator).
use crate::ncollection::{AllocatedBox, AllocatorVector, BaseAllocator, TypedAllocator};
use std::ops::Deref;
use std::sync::Arc;

/// Shared ownership through the global allocator; `*make_shared(5) == 5`.
pub fn make_shared<T>(value: T) -> Arc<T> {
    Arc::new(value)
}

/// Unique ownership through the global allocator; `*make_unique(String::new()) == ""`.
pub fn make_unique<T>(value: T) -> Box<T> {
    Box::new(value)
}

/// Shared ownership of a value stored by `allocator`;
/// `*make_oshared(&BaseAllocator::incremental(1024), 7) == 7`. The value lives in
/// the allocator's memory; the reference count lives beside it in the global heap.
pub fn make_oshared<T>(allocator: &BaseAllocator, value: T) -> Arc<AllocatedBox<T>> {
    Arc::new(allocator.allocate_value(value))
}

/// Shared ownership of a value stored through a typed allocator view (both
/// typed allocator forms: a borrowed allocator is copied, a moved one is
/// consumed, which Rust expresses the same way); `*make_oshared_in(&TypedAllocator::default(), 1)`.
pub fn make_oshared_in<T>(allocator: &TypedAllocator<T>, value: T) -> AllocatorShared<T> {
    let mut storage = allocator.vector();
    storage.push(value);
    AllocatorShared {
        storage: Arc::new(storage),
    }
}

/// A shared value kept in allocator-owned storage; `*shared == value`.
pub struct AllocatorShared<T> {
    storage: Arc<AllocatorVector<T>>,
}
impl<T> Deref for AllocatorShared<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.storage[0]
    }
}
impl<T> Clone for AllocatorShared<T> {
    fn clone(&self) -> Self {
        Self {
            storage: Arc::clone(&self.storage),
        }
    }
}
impl<T> AllocatorShared<T> {
    /// Number of owners, like `shared_ptr::use_count`; `shared.use_count() == 1`.
    pub fn use_count(&self) -> usize {
        Arc::strong_count(&self.storage)
    }
}

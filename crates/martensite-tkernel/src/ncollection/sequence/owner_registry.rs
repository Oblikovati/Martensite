//! The owner index lives behind one raw allocation pointer so the controlled-clear liveness
//! check can keep reading it while the sequence later inserts or removes owners.
//!
//! A `Box` field would not do: every `&mut` access through the box re-asserts unique
//! ownership of the whole map, which invalidates the pointer the clear scope saved.
//! Deriving every reference from the same raw pointer keeps that saved pointer valid.
use super::owner::SequenceNodeOwners;
use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
    ptr::NonNull,
};

/// Heap-allocated owner index with a stable address; `SequenceOwnerRegistry::<i32>::default()`.
pub(super) struct SequenceOwnerRegistry<T> {
    owners: NonNull<SequenceNodeOwners<T>>,
    // Owns the map exactly as `Box<SequenceNodeOwners<T>>` does, for drop check and variance.
    map: PhantomData<SequenceNodeOwners<T>>,
}
// SAFETY: the registry uniquely owns its map like a Box, so it may move to another thread
// whenever the map itself may.
unsafe impl<T> Send for SequenceOwnerRegistry<T> where SequenceNodeOwners<T>: Send {}
// SAFETY: shared registry access only hands out `&SequenceNodeOwners<T>`, as `&Box` does.
unsafe impl<T> Sync for SequenceOwnerRegistry<T> where SequenceNodeOwners<T>: Sync {}
impl<T> Default for SequenceOwnerRegistry<T> {
    fn default() -> Self {
        let owners = Box::into_raw(Box::<SequenceNodeOwners<T>>::default());
        // Keep the original allocation provenance; Box::into_non_null exceeds our MSRV.
        let owners = NonNull::new(owners).unwrap_or_else(|| {
            std::alloc::handle_alloc_error(std::alloc::Layout::new::<SequenceNodeOwners<T>>())
        });
        Self {
            owners,
            map: PhantomData,
        }
    }
}
impl<T> SequenceOwnerRegistry<T> {
    /// The address the controlled-clear scope reads; it shares every access's provenance.
    pub(super) fn registry_address(&self) -> *const SequenceNodeOwners<T> {
        self.owners.as_ptr().cast_const()
    }
    /// Moves the map out, releasing its allocation; `registry.into_owners().into_values()`.
    pub(super) fn into_owners(mut self) -> SequenceNodeOwners<T> {
        std::mem::take(&mut *self)
    }
}
impl<T> Deref for SequenceOwnerRegistry<T> {
    type Target = SequenceNodeOwners<T>;
    fn deref(&self) -> &Self::Target {
        // SAFETY: the pointer came from Box::into_raw and stays live until Drop; `&self`
        // excludes every mutable access for this borrow.
        unsafe { self.owners.as_ref() }
    }
}
impl<T> DerefMut for SequenceOwnerRegistry<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: the pointer came from Box::into_raw and stays live until Drop; `&mut self`
        // excludes every other access for this borrow.
        unsafe { self.owners.as_mut() }
    }
}
impl<T> Drop for SequenceOwnerRegistry<T> {
    fn drop(&mut self) {
        // SAFETY: the pointer came from Box::into_raw and is released only here, once.
        drop(unsafe { Box::from_raw(self.owners.as_ptr()) });
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn checked_registry_pointer_retains_identity_through_mutable_access() {
        let mut registry = super::SequenceOwnerRegistry::<i32>::default();
        let original = registry.registry_address();
        assert!(!original.is_null());
        registry.clear();
        assert_eq!(registry.registry_address(), original);
        assert!(registry.into_owners().is_empty());
    }
}

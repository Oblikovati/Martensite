//! Shared lifetime for one collection entry's object handle.
use super::Handle;
use std::{ops::Deref, sync::Arc};

/// One owning handle in a collection entry; `HandleSlot::new(handle)`.
/// A lease keeps the same entry's handle alive across an unlocked callback.
pub struct HandleSlot<T: ?Sized>(Arc<Handle<T>>);

impl<T: ?Sized> HandleSlot<T> {
    /// Move ownership into a slot; `HandleSlot::new(handle)`.
    pub fn new(handle: Handle<T>) -> Self {
        Self(Arc::new(handle))
    }

    /// Retain the slot without adding an object owner; `let entry = slot.lease();`.
    pub fn lease(&self) -> Arc<Handle<T>> {
        Arc::clone(&self.0)
    }
}

impl<T: ?Sized> Clone for HandleSlot<T> {
    fn clone(&self) -> Self {
        // A copied collection takes its own object reference for every copied entry.
        Self::new(Handle::clone(self))
    }
}

impl<T: ?Sized> Deref for HandleSlot<T> {
    type Target = Handle<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lease_keeps_one_entry_alive_and_clone_creates_an_independent_owner() {
        let value = Handle::new(42);
        let slot = HandleSlot::new(value.clone());
        let lease = slot.lease();
        assert!(std::ptr::eq(&*slot, &*lease));
        assert_eq!(value.ref_count(), 2);
        let copied = slot.clone();
        assert_eq!(value.ref_count(), 3);
        drop((slot, copied));
        assert_eq!(value.ref_count(), 2);
        drop(lease);
        assert_eq!(value.ref_count(), 1);
    }
}

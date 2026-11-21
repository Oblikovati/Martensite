use super::Handle;
use std::sync::{Arc, Weak};

/// Non-owning pointer to a concrete or trait object; `handle.downgrade()`.
#[repr(transparent)]
pub struct WeakHandle<T: ?Sized>(Option<Weak<T>>);

impl<T: ?Sized> Default for WeakHandle<T> {
    fn default() -> Self {
        Self(None)
    }
}

impl<T: ?Sized> Clone for WeakHandle<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: ?Sized> WeakHandle<T> {
    /// Check whether a pointer was never assigned; expiry is independent; `pointer.is_null()`.
    pub fn is_null(&self) -> bool {
        self.0.is_none()
    }

    /// Acquire an owner, or a null handle if the object has expired; `pointer.upgrade()`.
    pub fn upgrade(&self) -> Handle<T> {
        Handle(self.0.as_ref().and_then(Weak::upgrade))
    }

    /// Read the live-owner count without upgrading the weak pointer; `pointer.ref_count()`.
    pub fn ref_count(&self) -> usize {
        self.0.as_ref().map_or(0, Weak::strong_count)
    }

    /// Stable identity for both live and expired targets; `pointer.object_address()`.
    pub fn object_address(&self) -> *const () {
        self.0
            .as_ref()
            .map_or(std::ptr::null(), |owner| owner.as_ptr().cast())
    }
}

impl<T: ?Sized> Handle<T> {
    /// Store object identity without owning it; `let pointer = handle.downgrade()`.
    pub fn downgrade(&self) -> WeakHandle<T> {
        WeakHandle(self.0.as_ref().map(Arc::downgrade))
    }
}

impl<T: ?Sized> PartialEq for WeakHandle<T> {
    fn eq(&self, other: &Self) -> bool {
        self.object_address() == other.object_address()
    }
}

impl<T: ?Sized> Eq for WeakHandle<T> {}

impl<T: ?Sized> std::fmt::Debug for WeakHandle<T> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_tuple("WeakHandle")
            .field(&self.object_address())
            .finish()
    }
}

impl<T: ?Sized> crate::ncollection::HashKey for WeakHandle<T> {
    fn kernel_hash(&self) -> usize {
        self.object_address() as usize
    }
}

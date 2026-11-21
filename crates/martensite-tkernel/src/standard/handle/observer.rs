use super::Handle;
use std::ops::Deref;

/// A read of a handle slot. An immutable slot is borrowed. A slot that can be
/// reassigned while its value is read returns a short-lived owning snapshot.
pub enum HandleObserver<'a, T: ?Sized> {
    /// Borrow from an immutable slot.
    Borrowed(&'a Handle<T>),
    /// Hold a snapshot from a mutable or weak slot.
    Snapshot(Handle<T>),
}

impl<T: ?Sized> Handle<T> {
    /// Borrow an immutable slot without adding ownership; `handle.observe()`.
    pub fn observe(&self) -> HandleObserver<'_, T> {
        HandleObserver::Borrowed(self)
    }
}

impl<T: ?Sized> HandleObserver<'_, T> {
    /// Take the observed value as a new owner; `slot.observe().into_handle()`.
    pub fn into_handle(self) -> Handle<T> {
        match self {
            Self::Borrowed(handle) => handle.clone(),
            Self::Snapshot(handle) => handle,
        }
    }
}

impl<T: ?Sized> Deref for HandleObserver<'_, T> {
    type Target = Handle<T>;
    fn deref(&self) -> &Self::Target {
        match self {
            Self::Borrowed(handle) => handle,
            Self::Snapshot(handle) => handle,
        }
    }
}

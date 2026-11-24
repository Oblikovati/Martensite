use super::{Handle, Transient, TransientHandleTarget};
use std::sync::Arc;

/// A typed borrow of a handle's object. A view never changes the strong count.
pub struct HandleView<'a, T: ?Sized> {
    target: Option<&'a T>,
    source: &'a dyn ViewOwner,
}

trait ViewOwner {
    fn clone_transient(&self) -> Option<Arc<dyn Transient>>;
}

impl<T: ?Sized + TransientHandleTarget> ViewOwner for Handle<T> {
    fn clone_transient(&self) -> Option<Arc<dyn Transient>> {
        self.0.clone().map(T::into_transient)
    }
}

impl<'a, T: ?Sized + TransientHandleTarget> HandleView<'a, T> {
    pub(super) fn new<U: ?Sized + TransientHandleTarget>(owner: &'a Handle<U>) -> Self {
        let target = owner
            .get()
            .and_then(|value| T::borrow_from_transient(value.as_transient()));
        Self {
            target,
            source: owner,
        }
    }

    /// True for an absent object or a failed cast; `view.is_null()`.
    pub fn is_null(&self) -> bool {
        self.target.is_none()
    }

    /// Borrow the typed view of the object; `view.get()`.
    pub fn get(&self) -> Option<&'a T> {
        self.target
    }

    /// Read the object address without changing ownership; `view.object_address()`.
    pub fn object_address(&self) -> *const () {
        self.target
            .map_or(std::ptr::null(), |value| std::ptr::from_ref(value).cast())
    }

    /// Borrow the original object through its root interface; `view.transient()`.
    pub fn transient(&self) -> Option<&'a dyn Transient> {
        self.target.map(T::as_transient)
    }

    pub(super) fn clone_owner(&self) -> Option<Arc<dyn Transient>> {
        self.target?;
        self.source.clone_transient()
    }
}

impl<T: TransientHandleTarget> HandleView<'_, T> {
    /// Observe the typed pointer, or null for an absent object; `view.as_ptr()`.
    pub fn as_ptr(&self) -> *const T {
        self.get().map_or(std::ptr::null(), std::ptr::from_ref)
    }
}

impl<T: ?Sized + TransientHandleTarget, U: ?Sized> PartialEq<Handle<U>> for HandleView<'_, T> {
    fn eq(&self, other: &Handle<U>) -> bool {
        self.object_address() == other.object_address()
    }
}

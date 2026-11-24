//! Nullable shared ownership of a concrete object or an actual Rust trait object.
use super::{RuntimeType, RuntimeTyped, Transient, TransientHandleTarget};
use std::{any::TypeId, sync::Arc};

mod casts;
mod observer;
mod traits;
mod view;
mod weak;
pub use casts::HandleUpcast;
pub use observer::HandleObserver;
pub use view::HandleView;
pub use weak::WeakHandle;

/// A shared pointer with identity equality; `Handle::new(value)`.
#[repr(transparent)]
pub struct Handle<T: ?Sized>(Option<Arc<T>>);

impl<T: ?Sized> Default for Handle<T> {
    fn default() -> Self {
        Self(None)
    }
}

impl<T: ?Sized> Clone for Handle<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }

    fn clone_from(&mut self, source: &Self) {
        if self != source {
            self.0.clone_from(&source.0);
        }
    }
}

impl<T> Handle<T> {
    /// Allocate and own a concrete value; `Handle::new(3)`.
    pub fn new(value: T) -> Self {
        Self(Some(Arc::new(value)))
    }

    /// Borrow the pointer, returning null for an absent value; `handle.as_ptr()`.
    pub fn as_ptr(&self) -> *const T {
        self.0.as_ref().map_or(std::ptr::null(), Arc::as_ptr)
    }
}

impl<T: ?Sized> Handle<T> {
    /// Share an existing allocation; `Handle::from_arc(Arc::new(3))`.
    pub fn from_arc(owner: Arc<T>) -> Self {
        Self(Some(owner))
    }

    /// Transfer ownership to Rust's shared pointer, preserving null state; `handle.into_arc()`.
    pub fn into_arc(self) -> Option<Arc<T>> {
        self.0
    }

    /// Check for absent ownership; `Handle::<i32>::default().is_null()`.
    pub fn is_null(&self) -> bool {
        self.0.is_none()
    }

    /// Check for a live owner; `Handle::new(3).is_some()`.
    pub fn is_some(&self) -> bool {
        self.0.is_some()
    }

    /// Borrow the original value without a cast or clone; `Handle::new(3).get()`.
    pub fn get(&self) -> Option<&T> {
        self.0.as_deref()
    }

    /// Read allocation identity across concrete and trait handles; `handle.object_address()`.
    pub fn object_address(&self) -> *const () {
        self.0
            .as_ref()
            .map_or(std::ptr::null(), |owner| Arc::as_ptr(owner).cast())
    }

    /// Count strong owners, including temporary snapshots; `Handle::new(3).ref_count() == 1`.
    pub fn ref_count(&self) -> usize {
        self.0.as_ref().map_or(0, Arc::strong_count)
    }

    /// Release ownership and become null; `handle.nullify()`.
    pub fn nullify(&mut self) {
        *self = Self::default();
    }

    /// Hash identity, with zero for null; `handle.hash_code()`.
    pub fn hash_code(&self) -> usize {
        self.object_address() as usize
    }
}

impl<T: TransientHandleTarget + ?Sized> Handle<T> {
    /// Dispatch the object's class descriptor; `handle.dynamic_type().map(RuntimeType::name)`.
    pub fn dynamic_type(&self) -> Option<&'static RuntimeType> {
        self.get()
            .map(|target| target.as_transient().dynamic_type())
    }

    /// Inspect the actual Rust type, independently of reported ancestry; `handle.rust_type_id()`.
    pub fn rust_type_id(&self) -> Option<TypeId> {
        self.get().map(|target| target.as_transient().type_id())
    }

    /// Check dynamic ancestry, false for a null handle; `handle.is_kind(Base::static_type())`.
    pub fn is_kind(&self, base: &RuntimeType) -> bool {
        self.get()
            .is_some_and(|target| target.as_transient().is_kind(base))
    }

    /// Check dynamic ancestry by name; `handle.is_kind_name("Standard_Transient")`.
    pub fn is_kind_name(&self, name: &str) -> bool {
        self.get()
            .is_some_and(|target| target.as_transient().is_kind_name(name))
    }

    /// Check exact descriptor identity, false for a null handle; `handle.is_instance(Class::static_type())`.
    pub fn is_instance(&self, exact: &RuntimeType) -> bool {
        self.dynamic_type()
            .is_some_and(|kind| std::ptr::eq(kind, exact))
    }

    /// Check exact class name, false for a null handle; `handle.is_instance_name("Standard_Transient")`.
    pub fn is_instance_name(&self, name: &str) -> bool {
        self.get()
            .is_some_and(|target| target.as_transient().is_instance_name(name))
    }
}

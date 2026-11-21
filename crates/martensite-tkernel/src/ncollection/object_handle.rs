//! Reference-counted handles to objects that are not transients.
//!
//! The object is wrapped in a private transient adaptor held by a standard `Handle`;
//! the adaptor has no runtime type of its own, so its dynamic type is
//! the base transient type.
use crate::standard::{Handle, RuntimeType, RuntimeTyped, StandardTransient, Transient};

/// The adaptor: a transient owning one object.
struct HandleAdaptor<T> {
    object: T,
}

impl<T: Send + Sync + 'static> RuntimeTyped for HandleAdaptor<T> {
    // No runtime type of its own: it reports the base transient's name and descriptor.
    const TYPE_NAME: &'static str = StandardTransient::TYPE_NAME;
    fn static_type() -> &'static RuntimeType {
        StandardTransient::static_type() // `Ptr` declares no RTTI of its own
    }
}

impl<T: Send + Sync + 'static> Transient for HandleAdaptor<T> {
    fn dynamic_type(&self) -> &'static RuntimeType {
        StandardTransient::static_type()
    }
}

/// A shared handle to any object; `ObjectHandle::new(vec![1, 2])`.
pub struct ObjectHandle<T: Send + Sync + 'static> {
    handle: Handle<HandleAdaptor<T>>,
}

impl<T: Send + Sync + 'static> ObjectHandle<T> {
    /// Takes ownership of a new object.
    ///
    /// Example: `ObjectHandle::new(5).get() == Some(&5)`.
    pub fn new(object: T) -> Self {
        Self {
            handle: Handle::new(HandleAdaptor { object }),
        }
    }

    /// Borrows the object, `None` for a null handle.
    ///
    /// Shared ownership only grants shared borrows in Rust; mutate through `T`'s interior
    /// mutability. Example: `handle.get().map(|v| v.len())`.
    pub fn get(&self) -> Option<&T> {
        self.handle.get().map(|adaptor| &adaptor.object)
    }

    /// True when no object is held; `ObjectHandle::<i32>::default().is_null()`.
    pub fn is_null(&self) -> bool {
        self.handle.is_null()
    }

    /// The same object as a `Handle<dyn Transient>`, the base class.
    ///
    /// Example: `ObjectHandle::new(1).transient().ref_count() == 2`.
    pub fn transient(&self) -> Handle<dyn Transient> {
        self.handle.upcast()
    }

    /// Recovers the typed handle when `other` holds an adaptor of `T`, else null.
    ///
    /// Example: `ObjectHandle::<i32>::down_cast(&h.transient()).get() == Some(&1)`.
    pub fn down_cast(other: &Handle<dyn Transient>) -> Self {
        Self {
            handle: other.downcast(),
        }
    }
}

impl<T: Send + Sync + 'static> Default for ObjectHandle<T> {
    /// The null handle.
    fn default() -> Self {
        Self {
            handle: Handle::default(),
        }
    }
}

impl<T: Send + Sync + 'static> Clone for ObjectHandle<T> {
    fn clone(&self) -> Self {
        Self {
            handle: self.handle.clone(),
        }
    }
}

impl<T: Send + Sync + 'static> PartialEq for ObjectHandle<T> {
    /// Identity of the shared object, as the inherited handle comparison.
    fn eq(&self, other: &Self) -> bool {
        self.handle == other.handle
    }
}

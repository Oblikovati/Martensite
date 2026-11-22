//! Ownership and reference conversions at the object-model boundary.
use super::{Any, Arc, Transient};

/// This class's direct Rust coercions to a particular interface.
pub struct InterfaceCaster<I: ?Sized> {
    borrow: fn(&dyn Transient) -> Option<&I>,
    own: fn(Arc<dyn Transient>) -> Option<Arc<I>>,
}

impl<I: ?Sized> InterfaceCaster<I> {
    /// Pair the borrowed and owning conversions for an interface;
    /// `InterfaceCaster::new(borrow_geometry, own_geometry)`.
    pub const fn new(
        borrow: fn(&dyn Transient) -> Option<&I>,
        own: fn(Arc<dyn Transient>) -> Option<Arc<I>>,
    ) -> Self {
        Self { borrow, own }
    }
}

/// Type-erased storage for one explicitly declared interface conversion.
pub struct InterfaceEntry {
    pub(super) caster: Box<dyn Any + Send + Sync>,
}

impl InterfaceEntry {
    /// Store one interface's typed conversion; `InterfaceEntry::new(caster)`.
    pub fn new<I: ?Sized + 'static>(caster: InterfaceCaster<I>) -> Self {
        Self {
            caster: Box::new(caster),
        }
    }
}

/// An object whose concrete or interface type can be recovered across erased boundaries.
pub trait TransientHandleTarget: 'static {
    /// Coerce a borrowed object to its root interface; `target.as_transient()`.
    fn as_transient(&self) -> &dyn Transient;
    /// Move an Arc to the root interface; `T::into_transient(owner)`.
    fn into_transient(owner: Arc<Self>) -> Arc<dyn Transient>;
    /// Check the root object's type or interface without acquiring ownership;
    /// `T::borrow_from_transient(object)`.
    fn borrow_from_transient(object: &dyn Transient) -> Option<&Self>;
    /// Recover a typed owner when the root object supports this target;
    /// `T::from_transient(owner)`.
    fn from_transient(owner: Arc<dyn Transient>) -> Option<Arc<Self>>;
}

impl<T: Transient> TransientHandleTarget for T {
    fn as_transient(&self) -> &dyn Transient {
        self
    }
    fn into_transient(owner: Arc<Self>) -> Arc<dyn Transient> {
        owner
    }
    fn borrow_from_transient(object: &dyn Transient) -> Option<&Self> {
        project_concrete(object)
    }
    fn from_transient(owner: Arc<dyn Transient>) -> Option<Arc<Self>> {
        let erased: Arc<dyn Any + Send + Sync> = owner;
        Arc::downcast(erased).ok()
    }
}

impl TransientHandleTarget for dyn Transient {
    fn as_transient(&self) -> &dyn Transient {
        self
    }
    fn into_transient(owner: Arc<Self>) -> Arc<dyn Transient> {
        owner
    }
    fn borrow_from_transient(object: &dyn Transient) -> Option<&Self> {
        Some(object)
    }
    fn from_transient(owner: Arc<dyn Transient>) -> Option<Arc<Self>> {
        Some(owner)
    }
}

/// Borrow the Rust object of concrete type `T`; `project_concrete::<Line>(object)`.
pub fn project_concrete<T: Any>(object: &dyn Transient) -> Option<&T> {
    let erased: &dyn Any = object;
    erased.downcast_ref()
}

/// Borrow an explicitly registered interface; `project_interface::<dyn Curve>(object)`.
pub fn project_interface<I: ?Sized + 'static>(object: &dyn Transient) -> Option<&I> {
    let caster = object.rust_interface_descriptor().interface_caster::<I>()?;
    (caster.borrow)(object)
}

/// Recover an explicitly registered interface owner; `own_interface::<dyn Curve>(owner)`.
pub fn own_interface<I: ?Sized + 'static>(owner: Arc<dyn Transient>) -> Option<Arc<I>> {
    let caster = owner.rust_interface_descriptor().interface_caster::<I>()?;
    (caster.own)(owner)
}

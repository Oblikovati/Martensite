//! The legacy root of persistent classes.
use super::StandardTransient;
use std::sync::atomic::AtomicI32;
/// Transient with the storage type and reference numbers; `StandardPersistent::default()`.
#[repr(C)]
#[derive(Default)]
pub struct StandardPersistent {
    // Atomics give a mutable reference number through a shared handle, which Rust lends only as `&`.
    type_number: AtomicI32,
    reference_number: AtomicI32,
}
crate::impl_transient!(
    StandardPersistent,
    "Standard_Persistent",
    StandardTransient,
    interfaces: [dyn Persistent]
);
impl StandardPersistent {
    /// The storage type number, 0 after construction, as a mutable `int`;
    /// `object.type_num().store(3, Ordering::Relaxed)`.
    pub fn type_num(&self) -> &AtomicI32 {
        &self.type_number
    }
    /// The storage reference number, 0 after construction (meant for the
    /// storage schema); `object.ref_num().load(Ordering::Relaxed)`.
    pub fn ref_num(&self) -> &AtomicI32 {
        &self.reference_number
    }
}

/// A persistent object lends its storage type and reference numbers.
pub trait Persistent: super::Transient {
    /// Borrow the object's persistence fields; `object.persistent_state().type_num()`.
    fn persistent_state(&self) -> &StandardPersistent;

    /// The type number stored by a schema; `object.type_num().load(Ordering::Relaxed)`.
    fn type_num(&self) -> &AtomicI32 {
        self.persistent_state().type_num()
    }

    /// The reference number stored by a schema; `object.ref_num().load(Ordering::Relaxed)`.
    fn ref_num(&self) -> &AtomicI32 {
        self.persistent_state().ref_num()
    }
}

impl Persistent for StandardPersistent {
    fn persistent_state(&self) -> &StandardPersistent {
        self
    }
}

impl super::RuntimeTyped for dyn Persistent {
    const TYPE_NAME: &'static str = <StandardPersistent as super::RuntimeTyped>::TYPE_NAME;
    fn static_type() -> &'static super::RuntimeType {
        <StandardPersistent as super::RuntimeTyped>::static_type()
    }
}

crate::impl_transient_interface_target!(dyn Persistent);

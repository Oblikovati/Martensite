//! The legacy root of persistent classes.
use super::StandardTransient;
use std::sync::atomic::AtomicI32;
/// Transient with the storage type and reference numbers; `StandardPersistent::default()`.
#[repr(C)]
#[derive(Default)]
pub struct StandardPersistent {
    base: StandardTransient,
    // Atomics give a mutable reference number through a shared handle, which Rust lends only as `&`.
    type_number: AtomicI32,
    reference_number: AtomicI32,
}
crate::impl_transient!(
    StandardPersistent,
    "Standard_Persistent",
    StandardTransient,
    base
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

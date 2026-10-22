// Rust port of ApplicationFramework/TKTObj/TObj/TObj_ObjectIterator.hxx and .cxx
// at 3d097a0328e71b826377d4814ab05ec3c3d23871.

//! `TObj_ObjectIterator`: the interface of the iterators on TObj objects.
use super::TObjObject;
use martensite_tkernel::{
    impl_transient_interface,
    standard::{Failure, Handle, StandardTransient, Transient},
};

/// `TObj_ObjectIterator`'s virtual members; an iterator moves through `&self` (its
/// position is interior state), as natively through a handle.
pub trait TObjObjectIterator: Transient {
    /// `More`: false by default.
    fn more(&self) -> bool {
        false
    }
    /// `Next`: does nothing by default.
    fn next(&self) -> Result<(), Failure> {
        Ok(())
    }
    /// Read the current object with typed failures from derived iterators.
    /// Example: `let object = iterator.checked_value()?;`.
    fn checked_value(&self) -> Result<Handle<dyn TObjObject>, Failure> {
        Ok(self.value())
    }
    /// Convenience value access; use `checked_value` to retain derived failure semantics.
    /// Example: `iterator.value().is_null()`.
    fn value(&self) -> Handle<dyn TObjObject> {
        Handle::default()
    }
}
impl_transient_interface!(
    dyn TObjObjectIterator,
    "TObj_ObjectIterator",
    StandardTransient
);

/// `TObj_ObjectIterator` itself: the empty iterator.
///
/// ```
/// use martensite_tktobj::tobj::{TObjObjectIterator, TObjStandardObjectIterator};
/// let iterator = TObjStandardObjectIterator::default();
/// assert!(!iterator.more() && iterator.value().is_null());
/// ```
#[derive(Default)]
pub struct TObjStandardObjectIterator {
    transient: StandardTransient,
}
super::canonical_runtime_type::canonical_tobj_type!(
    TObjStandardObjectIterator,
    dyn TObjObjectIterator,
    transient,
    [dyn TObjObjectIterator]
);

impl TObjObjectIterator for TObjStandardObjectIterator {}

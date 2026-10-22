//! `martensite_object_model_ObjectIterator`: the interface of the iterators on ObjectModel objects.
use super::ModelObject;
use martensite_tkernel::{
    impl_transient_interface,
    standard::{Failure, Handle, StandardTransient, Transient},
};

/// `martensite_object_model_ObjectIterator`'s virtual members; an iterator moves through `&self` (its
/// position is interior state), as by the model through a handle.
pub trait ObjectIterator: Transient {
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
    fn checked_value(&self) -> Result<Handle<dyn ModelObject>, Failure> {
        Ok(self.value())
    }
    /// Convenience value access; use `checked_value` to retain derived failure semantics.
    /// Example: `iterator.value().is_null()`.
    fn value(&self) -> Handle<dyn ModelObject> {
        Handle::default()
    }
}
impl_transient_interface!(
    dyn ObjectIterator,
    "martensite_object_model_ObjectIterator",
    StandardTransient
);

/// `martensite_object_model_ObjectIterator` itself: the empty iterator.
///
/// ```
/// use martensite_tktobj::object_model::{ObjectIterator, StandardObjectIterator};
/// let iterator = StandardObjectIterator::default();
/// assert!(!iterator.more() && iterator.value().is_null());
/// ```
#[derive(Default)]
pub struct StandardObjectIterator {
    transient: StandardTransient,
}
super::canonical_runtime_type::canonical_object_model_type!(
    StandardObjectIterator,
    dyn ObjectIterator,
    transient,
    [dyn ObjectIterator]
);

impl ObjectIterator for StandardObjectIterator {}

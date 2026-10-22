//! `martensite_object_model_TypedSequenceIterator`: an iterator on a shared sequence of objects.
use super::{BackReferences, ModelObject, ObjectIterator};
use martensite_tkernel::{
    impl_transient,
    standard::{Failure, Handle, RuntimeType, SharedState, StandardTransient},
};

/// `martensite_object_model_TypedSequenceIterator`; it sees later changes of the shared sequence, as by the model.
///
/// ```
/// use martensite_tktobj::object_model::{ObjectIterator, TypedSequenceIterator};
/// let iterator = TypedSequenceIterator::empty();
/// assert!(!iterator.more());
/// ```
pub struct TypedSequenceIterator {
    transient: StandardTransient,
    index: SharedState<usize>,
    kind: Option<&'static RuntimeType>,
    objects: Handle<BackReferences>,
}
impl_transient!(TypedSequenceIterator, "martensite_object_model_TypedSequenceIterator", dyn ObjectIterator,
    transient, interfaces: [dyn ObjectIterator]);

impl TypedSequenceIterator {
    /// `martensite_object_model_TypedSequenceIterator()`: an empty iterator.
    pub fn empty() -> Self {
        Self::new(&Handle::default(), None)
    }
    /// `martensite_object_model_TypedSequenceIterator(theObjects, theType)`: the objects of that type (any when
    /// `None`).
    pub fn new(objects: &Handle<BackReferences>, kind: Option<&'static RuntimeType>) -> Self {
        Self {
            transient: StandardTransient::default(),
            index: SharedState::new(1),
            kind,
            objects: objects.clone(),
        }
    }
    /// `myIndex` (protected): the 1-based current index.
    pub fn index(&self) -> usize {
        *self.index.read()
    }
    /// `myType` (protected): the type of the iterated objects, `None` for any.
    pub fn kind(&self) -> Option<&'static RuntimeType> {
        self.kind
    }
    /// `myObjects` (protected): the shared sequence.
    pub fn objects(&self) -> Handle<BackReferences> {
        self.objects.clone()
    }
    /// The object at the current index; null past the end or on a null entry.
    fn current(&self) -> Handle<dyn ModelObject> {
        let index = *self.index.read();
        let Some(objects) = self.objects.get() else {
            return Handle::default();
        };
        objects.sequence(|items| items.value(index).cloned().unwrap_or_default())
    }
}

impl ObjectIterator for TypedSequenceIterator {
    /// `More`: a non-null current object; an object of another type is skipped (the
    /// runtime const member moves the iterator).
    fn more(&self) -> bool {
        loop {
            let current = self.current();
            if current.is_null() {
                return false;
            }
            if self.kind.is_none_or(|kind| current.is_kind(kind)) {
                return true;
            }
            *self.index.write() += 1;
        }
    }
    /// `Next`.
    fn next(&self) -> Result<(), Failure> {
        *self.index.write() += 1;
        Ok(())
    }
    /// Read the current sequence slot, preserving a null entry and reimplementation invalid indices.
    /// Example: `let current = iterator.checked_value()?;`.
    fn checked_value(&self) -> Result<Handle<dyn ModelObject>, Failure> {
        let objects = martensite_tkcdf::live(
            &self.objects,
            "martensite_object_model_TypedSequenceIterator::Value",
        )?;
        let index = self.index();
        objects.sequence(|items| items.value(index).cloned())
    }
    /// `Value`: the current object (null where the runtime `Value` raises
    /// `Standard_OutOfRange` or dereferences a null sequence).
    fn value(&self) -> Handle<dyn ModelObject> {
        self.current()
    }
}

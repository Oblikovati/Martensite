//! `martensite_object_model_LabelIterator`: the base of the iterators that walk document data framework labels.
use super::{ModelObject, ObjectIterator};
use martensite_tkernel::{
    impl_transient_interface,
    standard::{Failure, Handle, SharedState},
};
use martensite_tklcaf::tdf::{TdfChildIterator, TdfLabel};

/// The protected fields `myNode`, `myIterator` and `myObject`.
#[derive(Default)]
pub struct LabelIteratorBase {
    node: SharedState<TdfLabel>,
    iterator: SharedState<TdfChildIterator>,
    object: SharedState<Handle<dyn ModelObject>>,
}

impl LabelIteratorBase {
    /// `martensite_object_model_LabelIterator()`: an empty iterator.
    pub fn new() -> Self {
        Self::default()
    }
    /// `martensite_object_model_LabelIterator(theLabel, isRecursive)`; `Standard_NullObject` for a null label.
    pub fn with_label(label: &TdfLabel, is_recursive: bool) -> Result<Self, Failure> {
        let base = Self::new();
        base.init(label, is_recursive)?;
        Ok(base)
    }
    /// `Init(theLabel, isRecursive)` (protected).
    pub fn init(&self, label: &TdfLabel, is_recursive: bool) -> Result<(), Failure> {
        self.iterator.write().initialize(label, is_recursive)
    }
    /// `myNode`.
    pub fn node(&self) -> TdfLabel {
        self.node.read().clone()
    }
    /// `myObject`.
    pub fn object(&self) -> Handle<dyn ModelObject> {
        self.object.read().clone()
    }
    /// Sets the current item (`myObject` and `myNode`).
    pub fn set_current(&self, object: &Handle<dyn ModelObject>, node: &TdfLabel) {
        *self.object.write() = object.clone();
        *self.node.write() = node.clone();
    }
    /// Sets `myObject` alone.
    pub fn set_object(&self, object: &Handle<dyn ModelObject>) {
        *self.object.write() = object.clone();
    }
    /// Sets `myNode` alone.
    pub fn set_node(&self, node: &TdfLabel) {
        *self.node.write() = node.clone();
    }
    /// A copy of `myIterator`; a step works on the copy and stores it back.
    pub fn child_iterator(&self) -> TdfChildIterator {
        self.iterator.read().clone()
    }
    /// Stores `myIterator` back.
    pub fn set_child_iterator(&self, iterator: TdfChildIterator) {
        *self.iterator.write() = iterator;
    }
}

/// `martensite_object_model_LabelIterator`'s members; the `martensite_object_model_ObjectIterator` members of a label iterator
/// are filled by [`crate::object_model_label_iterator_members`].
pub trait LabelIterator: ObjectIterator {
    /// The embedded base with the runtime protected fields.
    fn label_iterator_base(&self) -> &LabelIteratorBase;
    /// `MakeStep` (pure virtual): shifts the iterator to the next object.
    fn make_step(&self) -> Result<(), Failure>;
    /// `LabelValue`: the label of the current item.
    fn label_value(&self) -> TdfLabel {
        self.label_iterator_base().node()
    }
}
impl_transient_interface!(
    dyn LabelIterator,
    "martensite_object_model_LabelIterator",
    dyn ObjectIterator
);

/// `martensite_object_model_LabelIterator::More`: a current node.
pub fn label_iterator_more<T: LabelIterator + ?Sized>(iterator: &T) -> bool {
    !iterator.label_iterator_base().node().is_null()
}

/// `martensite_object_model_LabelIterator::Next`: clears the current item, then steps.
pub fn label_iterator_next<T: LabelIterator + ?Sized>(iterator: &T) -> Result<(), Failure> {
    iterator
        .label_iterator_base()
        .set_current(&Handle::default(), &TdfLabel::default());
    iterator.make_step()
}

/// Fills the `martensite_object_model_ObjectIterator` impl of a `martensite_object_model_LabelIterator` with its `More`, `Next`
/// and `Value`.
#[macro_export]
macro_rules! object_model_label_iterator_members {
    () => {
        fn more(&self) -> bool {
            $crate::object_model::label_iterator_more(self)
        }
        fn next(&self) -> Result<(), martensite_tkernel::standard::Failure> {
            $crate::object_model::label_iterator_next(self)
        }
        fn value(
            &self,
        ) -> martensite_tkernel::standard::Handle<dyn $crate::object_model::ModelObject> {
            $crate::object_model::LabelIterator::label_iterator_base(self).object()
        }
    };
}

//! `martensite_object_model_ReferenceIterator`: the objects referred by the `martensite_object_model_ReferenceAttribute` attributes
//! under a label.
use super::{LabelIterator, LabelIteratorBase, ObjectIterator, ReferenceAttribute};
use martensite_tkernel::{
    impl_transient,
    standard::{Failure, Handle, RuntimeType, StandardTransient},
};
use martensite_tklcaf::tdf::TdfLabel;

/// `martensite_object_model_ReferenceIterator`; `ReferenceIterator::new(&label, None, true)`.
pub struct ReferenceIterator {
    transient: StandardTransient,
    base: LabelIteratorBase,
    kind: Option<&'static RuntimeType>,
}
impl_transient!(ReferenceIterator, "martensite_object_model_ReferenceIterator", dyn LabelIterator,
    transient, interfaces: [dyn ObjectIterator, dyn LabelIterator]);

impl ReferenceIterator {
    /// `martensite_object_model_ReferenceIterator(theLabel, theType, theRecursive)`.
    pub fn new(
        label: &TdfLabel,
        kind: Option<&'static RuntimeType>,
        recursive: bool,
    ) -> Result<Handle<ReferenceIterator>, Failure> {
        let iterator = Handle::new(Self {
            transient: StandardTransient::default(),
            base: LabelIteratorBase::with_label(label, recursive)?,
            kind,
        });
        iterator.make_step()?;
        Ok(iterator)
    }
    /// `myType` (protected): the type of the referred objects, `None` for any.
    pub fn kind(&self) -> Option<&'static RuntimeType> {
        self.kind
    }
}

impl ObjectIterator for ReferenceIterator {
    crate::object_model_label_iterator_members!();
}

impl LabelIterator for ReferenceIterator {
    fn label_iterator_base(&self) -> &LabelIteratorBase {
        &self.base
    }
    /// `MakeStep`: the next label with a reference to an object of the type (a null
    /// referred object is reported too, as by the model).
    fn make_step(&self) -> Result<(), Failure> {
        let mut children = self.base.child_iterator();
        while children.more() && self.base.node().is_null() {
            let label = children.value();
            let mut reference: Handle<ReferenceAttribute> = Handle::default();
            if label.find_attribute_as(&ReferenceAttribute::get_id(), &mut reference)? {
                let object = reference.get_object()?;
                self.base.set_object(&object);
                let filtered = self
                    .kind
                    .is_some_and(|kind| object.is_some() && !object.is_kind(kind));
                if !filtered {
                    self.base.set_node(&label);
                }
            }
            children.next();
        }
        self.base.set_child_iterator(children);
        Ok(())
    }
}

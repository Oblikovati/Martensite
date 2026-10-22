//! `martensite_object_model_ObjectTreeIterator`: the ObjectModel objects on the sub-labels of a label.
use super::ModelObjectBase;
use super::{LabelIterator, LabelIteratorBase, ObjectIterator};
use martensite_tkernel::{
    impl_transient,
    standard::{Failure, Handle, RuntimeType, StandardTransient},
};
use martensite_tklcaf::tdf::TdfLabel;

/// `martensite_object_model_ObjectTreeIterator`; `ObjectTreeIterator::new(&label, None, false, false)`.
pub struct ObjectTreeIterator {
    transient: StandardTransient,
    base: LabelIteratorBase,
    kind: Option<&'static RuntimeType>,
    all_sub_children: bool,
}
impl_transient!(ObjectTreeIterator, "martensite_object_model_ObjectTreeIterator", dyn LabelIterator,
    transient, interfaces: [dyn ObjectIterator, dyn LabelIterator]);

impl ObjectTreeIterator {
    /// `martensite_object_model_ObjectTreeIterator(theLabel, theType, theRecursive, theAllSubChildren)`: the
    /// objects of that type (any when `None`) on the sub-labels (at all depths when
    /// recursive), stopping at the first object level unless `all_sub_children`.
    pub fn new(
        label: &TdfLabel,
        kind: Option<&'static RuntimeType>,
        recursive: bool,
        all_sub_children: bool,
    ) -> Result<Handle<ObjectTreeIterator>, Failure> {
        let iterator = Handle::new(Self {
            transient: StandardTransient::default(),
            base: LabelIteratorBase::with_label(label, recursive)?,
            kind,
            all_sub_children,
        });
        iterator.make_step()?;
        Ok(iterator)
    }
    /// `myType` (protected): the type of the iterated objects, `None` for any.
    pub fn kind(&self) -> Option<&'static RuntimeType> {
        self.kind
    }
    /// `myAllSubChildren` (protected): the walk does not stop at the first object level.
    pub fn all_sub_children(&self) -> bool {
        self.all_sub_children
    }
}

impl ObjectIterator for ObjectTreeIterator {
    crate::object_model_label_iterator_members!();
}

impl LabelIterator for ObjectTreeIterator {
    fn label_iterator_base(&self) -> &LabelIteratorBase {
        &self.base
    }
    /// `MakeStep`: an object label stops the walk below it (unless all sub-children are
    /// wanted); any other label is entered.
    fn make_step(&self) -> Result<(), Failure> {
        let mut children = self.base.child_iterator();
        while children.more() && self.base.node().is_null() {
            let label = children.value();
            let mut object = Handle::default();
            if !ModelObjectBase::get_obj(&label, &mut object, false)? {
                children.next();
                continue;
            }
            if self.kind.is_none_or(|kind| object.is_kind(kind)) {
                self.base.set_current(&object, &label);
            }
            if self.all_sub_children {
                children.next();
            } else {
                children.next_brother();
            }
        }
        self.base.set_child_iterator(children);
        Ok(())
    }
}

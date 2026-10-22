//! `martensite_object_model_ModelIterator`: every object of a model, depth first from the root.
use super::{DocumentModel, ModelObject, ObjectIterator};
use martensite_tkernel::{
    impl_transient,
    standard::{Failure, Handle, SharedState, StandardTransient},
};

/// `martensite_object_model_ModelIterator`; `ModelIterator::new(&model)`.
pub struct ModelIterator {
    transient: StandardTransient,
    object: SharedState<Handle<dyn ModelObject>>,
    iterators: SharedState<Vec<Handle<dyn ObjectIterator>>>,
}
impl_transient!(ModelIterator, "martensite_object_model_ModelIterator", dyn ObjectIterator, transient,
    interfaces: [dyn ObjectIterator]);

impl ModelIterator {
    /// `martensite_object_model_ModelIterator(theModel)`: starts at the root of the model.
    pub fn new(model: &Handle<dyn DocumentModel>) -> Result<Handle<ModelIterator>, Failure> {
        let root = martensite_tkcdf::live(
            model,
            "martensite_object_model_ModelIterator::martensite_object_model_ModelIterator",
        )?
        .get_root()?;
        let iterator = Handle::new(Self {
            transient: StandardTransient::default(),
            object: SharedState::new(root.clone()),
            iterators: SharedState::new(Vec::new()),
        });
        if !root.is_null() {
            iterator.add_iterator(&root)?;
        }
        Ok(iterator)
    }
    /// `myObject` (protected): the current object.
    pub fn object(&self) -> Handle<dyn ModelObject> {
        self.object.read().clone()
    }
    /// `myIterSeq` (protected): the stack of child iterators, innermost last.
    pub fn iterators(&self) -> Vec<Handle<dyn ObjectIterator>> {
        self.iterators.read().clone()
    }
    /// `addIterator(theObj)` (protected): pushes the iterator on the object's children.
    pub fn add_iterator(&self, object: &Handle<dyn ModelObject>) -> Result<(), Failure> {
        let children =
            martensite_tkcdf::live(object, "martensite_object_model_ModelIterator::addIterator")?
                .get_children(None)?;
        if children.is_null() {
            return Ok(()); // object has no children
        }
        self.iterators.write().push(children);
        Ok(())
    }
}

impl ObjectIterator for ModelIterator {
    /// `More`: a current object.
    fn more(&self) -> bool {
        !self.object.read().is_null()
    }
    /// `Next`: the next object of the innermost iterator that has one (its children are
    /// visited next).
    fn next(&self) -> Result<(), Failure> {
        *self.object.write() = Handle::default();
        loop {
            let last = self.iterators.read().last().cloned();
            let Some(last) = last else {
                return Ok(());
            };
            let iterator =
                martensite_tkcdf::live(&last, "martensite_object_model_ModelIterator::Next")?;
            if iterator.more() {
                let object = iterator.value();
                *self.object.write() = object.clone();
                iterator.next()?;
                return self.add_iterator(&object);
            }
            self.iterators.write().pop();
        }
    }
    /// `Value`: the current object.
    fn value(&self) -> Handle<dyn ModelObject> {
        self.object.read().clone()
    }
}

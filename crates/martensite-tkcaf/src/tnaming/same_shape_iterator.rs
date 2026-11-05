//! `TNamingSameShapeIterator`: every evolution that uses one shape, on either side.
use super::named_shape::used_shapes_of;
use super::node::same_pointer;
use super::{TNamingNode, TNamingNodePtr, TNamingUsedShapes};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::{Failure, Handle, exceptions};
use martensite_tklcaf::live;
use martensite_tklcaf::tdf::TdfLabel;
use std::sync::Arc;

/// `TNamingSameShapeIterator`: the labels whose named shapes use a shape.
///
/// ```
/// use martensite_tkbrep::topods::Shape;
/// use martensite_tkcaf::tnaming::TNamingSameShapeIterator;
/// use martensite_tklcaf::tdf::TdfData;
/// let data = TdfData::new();
/// let iterator = TNamingSameShapeIterator::from_label(&Shape::default(), &data.root()).unwrap();
/// assert!(!iterator.more());
/// ```
pub struct TNamingSameShapeIterator {
    node: TNamingNodePtr,
    is_new: bool,
}

impl TNamingSameShapeIterator {
    /// The constructor `(aShape, Shapes)`: `StandardNoSuchObject` when the shape is not
    /// in the map (`ChangeFind`).
    pub fn from_used_shapes(
        shape: &Shape,
        shapes: &Handle<TNamingUsedShapes>,
    ) -> Result<Self, Failure> {
        let who = "TNamingSameShapeIterator";
        let reference = live(shapes, who)?.map().find(shape)?.clone();
        let node = reference.first_use().ok_or_else(|| {
            exceptions::null_object(
                "TNamingSameShapeIterator: the shape has no first use; expected a used shape",
            )
        })?;
        let is_new = same_pointer(&node.new_shape(), &Some(reference));
        Ok(Self {
            node: Some(node),
            is_new,
        })
    }
    /// The constructor `(aShape, access)`; without a root `TNamingUsedShapes` the native
    /// iterator is left uninitialized, the implementation starts at the end.
    pub fn from_label(shape: &Shape, access: &TdfLabel) -> Result<Self, Failure> {
        let used = used_shapes_of(access)?;
        if used.is_null() {
            return Ok(Self {
                node: None,
                is_new: false,
            });
        }
        Self::from_used_shapes(shape, &used)
    }
    /// `More`.
    pub fn more(&self) -> bool {
        self.node.is_some()
    }
    /// `Next`: the next node of the shape's chain (`StandardNoMoreObject` at the end, a
    /// crash natively).
    #[allow(clippy::should_implement_trait)] // A More/Next iteration protocol, fallible.
    pub fn next(&mut self) -> Result<(), Failure> {
        let node = self.current("TNamingSameShapeIterator::Next", exceptions::no_more_object)?;
        let reference = if self.is_new {
            node.new_shape()
        } else {
            node.old()
        };
        self.node = node.next_same_shape(&reference);
        if let Some(next) = &self.node {
            self.is_new = same_pointer(&next.new_shape(), &reference);
        }
        Ok(())
    }
    /// `Label`: the label of the current evolution.
    pub fn label(&self) -> Result<TdfLabel, Failure> {
        self.current(
            "TNamingSameShapeIterator::Label",
            exceptions::no_such_object,
        )?
        .label()
    }
    fn current(&self, who: &str, error: fn(&str) -> Failure) -> Result<Arc<TNamingNode>, Failure> {
        self.node.clone().ok_or_else(|| {
            error(&format!(
                "{who}: the iterator is at its end; expected More()"
            ))
        })
    }
}

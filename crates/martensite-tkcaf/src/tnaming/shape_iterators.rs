//! The iterators that walk the evolution graph from one shape: to the shapes it became
//! (`TNamingNewShapeIterator`) or to the shapes it came from (`TNamingOldShapeIterator`).
use super::iterator::node_evolution;
use super::named_shape::used_shapes_of;
use super::node_chains::select_same_shape;
use super::{
    TNamingEvolution, TNamingIterator, TNamingNamedShape, TNamingNode, TNamingNodePtr,
    TNamingUsedShapes,
};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::{Failure, Handle, exceptions};
use martensite_tklcaf::live;
use martensite_tklcaf::tdf::TdfLabel;
use std::sync::Arc;

/// The common start of the constructors on a shape: the first node of the shape's chain
/// that uses it on the given side; `StandardNoSuchObject` when the shape is not named.
pub(crate) fn start_on_shape(
    shape: &Shape,
    used: &Handle<TNamingUsedShapes>,
    old: bool,
    transaction: i32,
    who: &str,
) -> Result<TNamingNodePtr, Failure> {
    let reference = live(used, who)?.map().seek(shape).cloned();
    let Some(reference) = reference else {
        return Err(exceptions::no_such_object(&format!(
            "{who} aShape: the shape is not in the TNamingUsedShapes map; expected a named shape"
        )));
    };
    let first = reference.first_use();
    select_same_shape(first, old, &Some(reference), transaction)
}

/// The constructors on a label: the root's `TNamingUsedShapes`. Without one the native
/// iterator is left uninitialized; the implementation starts at the end.
fn start_on_label(
    shape: &Shape,
    access: &TdfLabel,
    old: bool,
    transaction: i32,
    who: &str,
) -> Result<TNamingNodePtr, Failure> {
    let used = used_shapes_of(access)?;
    if used.is_null() {
        return Ok(None);
    }
    start_on_shape(shape, &used, old, transaction, who)
}

/// The descent constructors: from the shape `side` of the current node, restart at its
/// first use (`StandardNoSuchObject` at the end).
fn descend(
    node: &TNamingNodePtr,
    side: fn(&TNamingNode) -> super::TNamingRefShapePtr,
    old: bool,
    transaction: i32,
    who: &str,
) -> Result<TNamingNodePtr, Failure> {
    let node = node.clone().ok_or_else(|| {
        exceptions::no_such_object(&format!(
            "{who}: the iterator is at its end; expected More()"
        ))
    })?;
    let Some(reference) = side(&node) else {
        // No descendant.
        return Ok(None);
    };
    select_same_shape(reference.first_use(), old, &Some(reference), transaction)
}

/// The members both iterators share; `old` is the side the walked shape is on.
macro_rules! shape_iterator {
    ($name:ident, $class_name:literal, $old:expr, $walked:ident, $reached:ident) => {
        impl $name {
            /// The constructor `(aShape, Trans, Shapes)`.
            pub fn from_used_shapes_in_transaction(
                shape: &Shape,
                transaction: i32,
                shapes: &Handle<TNamingUsedShapes>,
            ) -> Result<Self, Failure> {
                let node = start_on_shape(shape, shapes, $old, transaction, $class_name)?;
                Ok(Self { node, transaction })
            }
            /// The constructor `(aShape, Trans, access)`.
            pub fn from_label_in_transaction(
                shape: &Shape,
                transaction: i32,
                access: &TdfLabel,
            ) -> Result<Self, Failure> {
                let node = start_on_label(shape, access, $old, transaction, $class_name)?;
                Ok(Self { node, transaction })
            }
            /// The constructor `(aShape, Shapes)` (current validity).
            pub fn from_used_shapes(
                shape: &Shape,
                shapes: &Handle<TNamingUsedShapes>,
            ) -> Result<Self, Failure> {
                Self::from_used_shapes_in_transaction(shape, -1, shapes)
            }
            /// The constructor `(aShape, access)` (current validity).
            pub fn from_label(shape: &Shape, access: &TdfLabel) -> Result<Self, Failure> {
                Self::from_label_in_transaction(shape, -1, access)
            }
            /// The constructor `(const TNamingIterator&)`: the evolutions of the iterator's
            /// new shape.
            pub fn from_iterator(iterator: &TNamingIterator) -> Result<Self, Failure> {
                let transaction = iterator.transaction;
                let node = descend(
                    &iterator.node,
                    TNamingNode::new_shape,
                    $old,
                    transaction,
                    $class_name,
                )?;
                Ok(Self { node, transaction })
            }
            /// `More`.
            pub fn more(&self) -> bool {
                self.node.is_some()
            }
            /// `Next`: the next evolution of the walked shape (`StandardNoMoreObject` at
            /// the end, a crash natively).
            #[allow(clippy::should_implement_trait)] // A More/Next iteration protocol, fallible.
            pub fn next(&mut self) -> Result<(), Failure> {
                let node = self.current(concat!($class_name, "::Next"))?;
                let reference = node.$walked();
                self.node = select_same_shape(
                    node.next_same_shape(&reference),
                    $old,
                    &reference,
                    self.transaction,
                )?;
                Ok(())
            }
            /// `Label`: the label of the current evolution.
            pub fn label(&self) -> Result<TdfLabel, Failure> {
                self.current(concat!($class_name, "::Label"))?.label()
            }
            /// `NamedShape`: the named shape of the current evolution.
            pub fn named_shape(&self) -> Result<Handle<TNamingNamedShape>, Failure> {
                Ok(self.current(concat!($class_name, "::Label"))?.attribute())
            }
            /// `Shape`: the shape reached by the current evolution.
            pub fn shape(&self) -> Result<Shape, Failure> {
                let node = self.current(concat!($class_name, "::Shape"))?;
                let reference = node.$reached().ok_or_else(|| {
                    exceptions::null_object(concat!(
                        $class_name,
                        "::Shape: the evolution has no such shape; expected a selected evolution"
                    ))
                })?;
                Ok(reference.shape())
            }
            /// `IsModification`: the current evolution is `MODIFY` or `DELETE`.
            pub fn is_modification(&self) -> Result<bool, Failure> {
                let node = self.current(concat!($class_name, "::IsModification"))?;
                let evolution = node_evolution(&node, concat!($class_name, "::IsModification"))?;
                Ok(matches!(
                    evolution,
                    TNamingEvolution::Modify | TNamingEvolution::Delete
                ))
            }
            fn current(&self, who: &str) -> Result<Arc<TNamingNode>, Failure> {
                self.node.clone().ok_or_else(|| {
                    exceptions::no_such_object(&format!(
                        "{who}: the iterator is at its end; expected More()"
                    ))
                })
            }
        }
    };
}

/// `TNamingNewShapeIterator`: the evolutions that use a shape as their old shape, so the
/// shapes it was modified or generated into.
///
/// ```
/// use martensite_tkbrep::topods::Shape;
/// use martensite_tkcaf::tnaming::TNamingNewShapeIterator;
/// use martensite_tklcaf::tdf::TdfData;
/// let data = TdfData::new();
/// // Without a TNamingUsedShapes on the root there is nothing to walk.
/// let iterator = TNamingNewShapeIterator::from_label(&Shape::default(), &data.root()).unwrap();
/// assert!(!iterator.more());
/// ```
pub struct TNamingNewShapeIterator {
    node: TNamingNodePtr,
    transaction: i32,
}
shape_iterator!(
    TNamingNewShapeIterator,
    "TNamingNewShapeIterator",
    true,
    old,
    new_shape
);

impl TNamingNewShapeIterator {
    /// The constructor `(const TNamingNewShapeIterator&)`: natively the copy constructor,
    /// which descends to the evolutions of the current new shape (it is not a copy).
    pub fn from_new_shape_iterator(iterator: &TNamingNewShapeIterator) -> Result<Self, Failure> {
        let transaction = iterator.transaction;
        let node = descend(
            &iterator.node,
            TNamingNode::new_shape,
            true,
            transaction,
            "TNamingNewShapeIterator",
        )?;
        Ok(Self { node, transaction })
    }
}

/// `TNamingOldShapeIterator`: the evolutions that use a shape as their new shape, so the
/// shapes it was made from.
///
/// ```
/// use martensite_tkbrep::topods::Shape;
/// use martensite_tkcaf::tnaming::TNamingOldShapeIterator;
/// use martensite_tklcaf::tdf::TdfData;
/// let data = TdfData::new();
/// let iterator = TNamingOldShapeIterator::from_label(&Shape::default(), &data.root()).unwrap();
/// assert!(!iterator.more());
/// ```
pub struct TNamingOldShapeIterator {
    node: TNamingNodePtr,
    transaction: i32,
}
shape_iterator!(
    TNamingOldShapeIterator,
    "TNamingOldShapeIterator",
    false,
    new_shape,
    old
);

impl TNamingOldShapeIterator {
    /// The constructor `(const TNamingOldShapeIterator&)`: natively the copy constructor,
    /// which ascends to the evolutions of the current old shape (it is not a copy).
    pub fn from_old_shape_iterator(iterator: &TNamingOldShapeIterator) -> Result<Self, Failure> {
        let transaction = iterator.transaction;
        let node = descend(
            &iterator.node,
            TNamingNode::old,
            false,
            transaction,
            "TNamingOldShapeIterator",
        )?;
        Ok(Self { node, transaction })
    }
}

//! `TNamingBuilder`: records shape evolutions in the named shape of a label.
use super::node_chains::append_to_chain;
use super::{
    TNamingEvolution, TNamingNamedShape, TNamingNode, TNamingRefShape, TNamingShapeRefMap,
    TNamingUsedShapes,
};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::{Failure, Handle, exceptions};
use martensite_tklcaf::live;
use martensite_tklcaf::tdf::{TdfAttributeState, TdfLabel};
use std::sync::Arc;

/// `TNamingBuilder`: the constructor finds or creates the root `TNamingUsedShapes` and the
/// label's `TNamingNamedShape`; an existing named shape is backed up, cleared and gets a
/// new version. Each member then adds one evolution of the builder's kind.
///
/// ```
/// use martensite_tkcaf::tnaming::{TNamingBuilder, TNamingEvolution};
/// use martensite_tklcaf::tdf::TdfData;
/// let data = TdfData::new();
/// let builder = TNamingBuilder::new(&data.root().find_child(1, true).unwrap()).unwrap();
/// let named = builder.named_shape();
/// assert!(named.is_empty());
/// assert_eq!(named.evolution(), TNamingEvolution::Primitive);
/// ```
pub struct TNamingBuilder {
    shapes: Handle<TNamingUsedShapes>,
    attribute: Handle<TNamingNamedShape>,
}

impl TNamingBuilder {
    /// `TNamingBuilder(L)`; `StandardNullObject` for a null label, the framework errors of
    /// `AddAttribute` and `Backup` otherwise.
    pub fn new(label: &TdfLabel) -> Result<Self, Failure> {
        let shapes = find_or_add_used_shapes(label)?;
        let mut attribute: Handle<TNamingNamedShape> = Handle::default();
        if !label.find_attribute_as(&TNamingNamedShape::get_id(), &mut attribute)? {
            attribute = Handle::new(TNamingNamedShape::new());
            label.add_attribute(&attribute.cast(), true)?;
        } else {
            let named = live(&attribute, "TNamingBuilder")?;
            named.backup()?;
            named.clear()?;
            named.state.write().version += 1;
        }
        Ok(Self { shapes, attribute })
    }
    /// `NamedShape`: the attribute being built.
    pub fn named_shape(&self) -> Handle<TNamingNamedShape> {
        self.attribute.clone()
    }
    /// `Generated(newShape)`: a primitive evolution. `StandardConstructionError` for
    /// another kind than `PRIMITIVE`, or when the shape is already the first use of this
    /// attribute.
    pub fn generated(&self, new_shape: &Shape) -> Result<(), Failure> {
        let named = self.start(TNamingEvolution::Primitive)?;
        let mut map = live(&self.shapes, "TNamingBuilder")?.map();
        let Some(reference) = map.seek(new_shape).cloned() else {
            let reference = TNamingRefShape::shared(new_shape);
            let node = TNamingNode::new(None, Some(reference.clone()));
            reference.set_first_use(&Some(node.clone()));
            map.bind(new_shape.clone(), reference);
            named.add(&self.attribute.downgrade(), &node);
            return Ok(());
        };
        self.refuse_own_first_use(&reference)?;
        let node = TNamingNode::new(None, Some(reference.clone()));
        named.add(&self.attribute.downgrade(), &node);
        append_to_chain(&reference, &node)
    }
    /// `Delete(oldShape)`: the old shape becomes a null shape (a new null-shape entry
    /// replaces the previous one in the map, as natively).
    pub fn delete(&self, old_shape: &Shape) -> Result<(), Failure> {
        let named = self.start(TNamingEvolution::Delete)?;
        let mut map = live(&self.shapes, "TNamingBuilder")?.map();
        let old = find_or_bind(&mut map, old_shape);
        let null_shape = Shape::default();
        let new = TNamingRefShape::shared(&null_shape);
        map.bind(null_shape, new.clone());
        self.link(named, &old, &new)
    }
    /// `Generated(oldShape, newShape)`; nothing when the shapes are the same.
    pub fn generated_from(&self, old_shape: &Shape, new_shape: &Shape) -> Result<(), Failure> {
        self.add_pair(TNamingEvolution::Generated, old_shape, new_shape)
    }
    /// `Modify(oldShape, newShape)`; nothing when the shapes are the same.
    pub fn modify(&self, old_shape: &Shape, new_shape: &Shape) -> Result<(), Failure> {
        self.add_pair(TNamingEvolution::Modify, old_shape, new_shape)
    }
    /// `Select(S, InS)`: the selected shape `S` in its context `InS` (old shape `InS`, new
    /// shape `S`).
    pub fn select(&self, shape: &Shape, in_shape: &Shape) -> Result<(), Failure> {
        let named = self.start(TNamingEvolution::Selected)?;
        let mut map = live(&self.shapes, "TNamingBuilder")?.map();
        let old = find_or_bind(&mut map, in_shape);
        let new = find_or_bind(&mut map, shape);
        self.link(named, &old, &new)
    }
    /// The common body of `Generated(old, new)` and `Modify`.
    fn add_pair(
        &self,
        evolution: TNamingEvolution,
        old_shape: &Shape,
        new_shape: &Shape,
    ) -> Result<(), Failure> {
        let named = self.start(evolution)?;
        if old_shape.is_same(new_shape) {
            return Ok(());
        }
        let mut map = live(&self.shapes, "TNamingBuilder")?.map();
        let old = find_or_bind(&mut map, old_shape);
        let new = find_or_bind(&mut map, new_shape);
        self.link(named, &old, &new)
    }
    /// The evolution check at the start of each member: the first evolution sets the
    /// kind, the next ones must have the same.
    fn start(&self, evolution: TNamingEvolution) -> Result<&TNamingNamedShape, Failure> {
        let named = live(&self.attribute, "TNamingBuilder")?;
        let mut state = named.state.write();
        if state.node.is_none() {
            state.evolution = evolution;
        } else if state.evolution != evolution {
            return Err(exceptions::construction_error(&format!(
                "TNamingBuilder : not same evolution ({evolution:?}; expected {:?})",
                state.evolution
            )));
        }
        Ok(named)
    }
    /// A new node (old, new) added to the attribute and to both shape chains.
    fn link(
        &self,
        named: &TNamingNamedShape,
        old: &Arc<TNamingRefShape>,
        new: &Arc<TNamingRefShape>,
    ) -> Result<(), Failure> {
        let node = TNamingNode::new(Some(old.clone()), Some(new.clone()));
        named.add(&self.attribute.downgrade(), &node);
        append_to_chain(old, &node)?;
        append_to_chain(new, &node)
    }
    /// `pns->FirstUse()->myAtt == myAtt`: a shape cannot be generated twice by one
    /// attribute.
    fn refuse_own_first_use(&self, reference: &TNamingRefShape) -> Result<(), Failure> {
        let first = reference.first_use().ok_or_else(|| {
            exceptions::null_object(
                "TNamingBuilder::Generate: the bound shape has no first use; expected a used shape",
            )
        })?;
        if first.links().att.object_address() == self.attribute.object_address() {
            return Err(exceptions::construction_error(
                "TNamingBuilder::Generate: the shape is already generated by this attribute; expected a new shape",
            ));
        }
        Ok(())
    }
}

/// The root's `TNamingUsedShapes`, added when it is missing.
fn find_or_add_used_shapes(label: &TdfLabel) -> Result<Handle<TNamingUsedShapes>, Failure> {
    let root = label.root()?;
    let mut shapes: Handle<TNamingUsedShapes> = Handle::default();
    if !root.find_attribute_as(&TNamingUsedShapes::get_id(), &mut shapes)? {
        shapes = Handle::new(TNamingUsedShapes::new());
        root.add_attribute(&shapes.cast(), true)?;
    }
    Ok(shapes)
}

/// The ref shape bound to `shape`, created and bound when it is missing.
fn find_or_bind(map: &mut TNamingShapeRefMap, shape: &Shape) -> Arc<TNamingRefShape> {
    if let Some(reference) = map.seek(shape) {
        return reference.clone();
    }
    let reference = TNamingRefShape::shared(shape);
    map.bind(shape.clone(), reference.clone());
    reference
}

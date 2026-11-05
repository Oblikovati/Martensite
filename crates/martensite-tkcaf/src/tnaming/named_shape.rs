//! `TNamingNamedShape`: the attribute that records the shape evolutions of a label
//! (ocaf-design.md section 10).
use super::node_chains::remove_nodes;
use super::{TNamingEvolution, TNamingNode, TNamingNodePtr, TNamingTool, TNamingUsedShapes};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::{
    impl_transient,
    standard::{Failure, Guid, Handle, SharedState, StandardTransient, WeakHandle},
};
use martensite_tklcaf::guid_from_text;
use martensite_tklcaf::tdf::{TdfAttribute, TdfAttributeBase, TdfLabel};
use std::sync::Arc;

/// `TNamingNamedShape::GetID()`.
const NAMED_SHAPE_ID: Guid = guid_from_text("c4ef4200-568f-11d1-8940-080009dc3333");

/// The fields of `TNamingNamedShape`.
#[derive(Clone, Default)]
pub(crate) struct NamedShapeState {
    pub(crate) node: TNamingNodePtr,
    pub(crate) evolution: TNamingEvolution,
    pub(crate) version: i32,
}

/// `TNamingNamedShape`: the evolutions (old shape, new shape) of one label, all of one
/// [`TNamingEvolution`]. A `TNamingBuilder` fills it.
///
/// ```
/// use martensite_tkcaf::tnaming::{TNamingEvolution, TNamingNamedShape};
/// let named = TNamingNamedShape::new();
/// assert!(named.is_empty());
/// assert_eq!(named.evolution(), TNamingEvolution::Primitive);
/// assert_eq!(named.version(), 0);
/// ```
pub struct TNamingNamedShape {
    transient: StandardTransient,
    pub(crate) base: TdfAttributeBase,
    pub(crate) state: SharedState<NamedShapeState>,
}
impl_transient!(TNamingNamedShape, "TNamingNamedShape", dyn TdfAttribute, transient, interfaces: [dyn TdfAttribute]);

impl Default for TNamingNamedShape {
    fn default() -> Self {
        Self::new()
    }
}

impl TNamingNamedShape {
    /// `GetID`: the GUID of named shapes.
    pub fn get_id() -> Guid {
        NAMED_SHAPE_ID
    }
    /// `TNamingNamedShape()`: no evolution, `PRIMITIVE`, version 0.
    pub fn new() -> Self {
        Self {
            transient: StandardTransient::default(),
            base: TdfAttributeBase::default(),
            state: SharedState::new(NamedShapeState::default()),
        }
    }
    /// `IsEmpty`: no evolution.
    pub fn is_empty(&self) -> bool {
        self.first_node().is_none()
    }
    /// `Get`: the new shapes (a compound when there are several, null when empty);
    /// `TNamingTool::GetShape`.
    pub fn get(&self) -> Result<Shape, Failure> {
        TNamingTool::shape_of(self)
    }
    /// `Evolution`.
    pub fn evolution(&self) -> TNamingEvolution {
        self.state.read().evolution
    }
    /// `Version`.
    pub fn version(&self) -> i32 {
        self.state.read().version
    }
    /// `SetVersion`.
    pub fn set_version(&self, version: i32) {
        self.state.write().version = version;
    }
    /// `Clear`: removes every evolution from the shape chains of the `TNamingUsedShapes`
    /// map (a shape no other evolution uses leaves the map) and deletes them. An
    /// attribute without a label is left as it is, as natively.
    pub fn clear(&self) -> Result<(), Failure> {
        let label = self.base.label();
        if label.is_null() {
            return Ok(());
        }
        let first = self.first_node();
        let used = used_shapes_of(&label)?;
        remove_nodes(used.get(), &first);
        self.state.write().node = None;
        Ok(())
    }
    /// The private `Add(pdn)`: the node becomes the first evolution of `this`.
    pub(crate) fn add(&self, this: &WeakHandle<TNamingNamedShape>, node: &Arc<TNamingNode>) {
        let mut state = self.state.write();
        node.edit(|links| {
            links.att = this.clone();
            if state.node.is_some() {
                links.next_same_attribute = state.node.clone();
            }
        });
        state.node = Some(node.clone());
    }
    /// `myNode`, read by the friend iterators.
    pub(crate) fn first_node(&self) -> TNamingNodePtr {
        self.state.read().node.clone()
    }
    /// The handle `this` converts to natively (bound once the attribute is on a label).
    pub(crate) fn this_handle(&self) -> Result<Handle<TNamingNamedShape>, Failure> {
        Ok(self.base.this()?.cast::<TNamingNamedShape>())
    }
}

impl Drop for TNamingNamedShape {
    /// `~TNamingNamedShape`: `Clear()`. A destructor cannot report a failure; the
    /// evolutions are released anyway.
    fn drop(&mut self) {
        let _ = self.clear();
    }
}

/// The `TNamingUsedShapes` of the label's root (`Label().Root().FindAttribute`), null when
/// the root has none.
pub(crate) fn used_shapes_of(label: &TdfLabel) -> Result<Handle<TNamingUsedShapes>, Failure> {
    let mut used: Handle<TNamingUsedShapes> = Handle::default();
    label
        .root()?
        .find_attribute_as(&TNamingUsedShapes::get_id(), &mut used)?;
    Ok(used)
}

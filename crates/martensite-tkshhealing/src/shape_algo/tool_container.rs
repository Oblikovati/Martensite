//! `ShapeAlgoToolContainer`: the factory of the tools the algorithm container uses.
use crate::shape_fix::{ShapeFixEdgeProjAux, ShapeFixShape};
use martensite_tkernel::impl_transient;
use martensite_tkernel::standard::{Handle, StandardTransient};

/// `ShapeAlgoToolContainer`: returns new Shape Healing tools. The members are
/// virtual, but no class overrides them.
///
/// ```
/// use martensite_tkshhealing::shape_algo::ShapeAlgoToolContainer;
/// let tools = ShapeAlgoToolContainer::new();
/// assert!(!tools.fix_shape().is_null());
/// assert!(!tools.edge_proj_aux().is_null());
/// ```
#[derive(Default)]
pub struct ShapeAlgoToolContainer {
    parent: StandardTransient,
}
impl_transient!(
    ShapeAlgoToolContainer,
    "ShapeAlgo_ToolContainer",
    StandardTransient,
    parent
);

impl ShapeAlgoToolContainer {
    /// `ShapeAlgoToolContainer()`; `ShapeAlgoToolContainer::new()`.
    pub fn new() -> Self {
        Self::default()
    }

    /// `FixShape()`: a new empty `ShapeFixShape`; `tools.fix_shape()`.
    pub fn fix_shape(&self) -> Handle<ShapeFixShape> {
        Handle::new(ShapeFixShape::empty())
    }

    /// `EdgeProjAux()`: a new empty `ShapeFixEdgeProjAux`; `tools.edge_proj_aux()`.
    pub fn edge_proj_aux(&self) -> Handle<ShapeFixEdgeProjAux> {
        Handle::new(ShapeFixEdgeProjAux::new())
    }
}

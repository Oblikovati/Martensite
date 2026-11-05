//! Shared read access complements the operation-local mutable extension view.
use super::TopOpeBRepBuildShapeSetBase;
use martensite_tkbrep::{topexp::Explorer, topods::Shape};
use martensite_tkernel::ncollection::{List, ListCursor};

/// Shared retained-state access; `base.extension_state_ref().shapes().len()`.
/// This view never locks, clones list nodes, or permits mutation through shared access.
pub struct ShapeSetExtensionStateRef<'a> {
    base: &'a TopOpeBRepBuildShapeSetBase,
}

impl TopOpeBRepBuildShapeSetBase {
    /// Borrows extension state for an immutable operation; `let view = base.extension_state_ref();`.
    pub fn extension_state_ref(&self) -> ShapeSetExtensionStateRef<'_> {
        ShapeSetExtensionStateRef { base: self }
    }
}

impl ShapeSetExtensionStateRef<'_> {
    /// Reads the stored shapes independently of any rebound cursor source; `view.shapes().len()`.
    pub fn shapes(&self) -> &List<Shape> {
        &self.base.my_shapes
    }

    /// Reads the actual subshape search; `view.explorer().depth()`.
    pub fn explorer(&self) -> &Explorer {
        &self.base.traversal.explorer
    }

    /// Reads the retained shape cursor; `view.shapes_cursor().more()`.
    pub fn shapes_cursor(&self) -> &ListCursor {
        &self.base.traversal.shapes
    }

    /// Reads the retained start cursor; `view.starts_cursor().more()`.
    pub fn starts_cursor(&self) -> &ListCursor {
        &self.base.traversal.starts
    }

    /// Reads the retained incidence cursor; `view.incident_cursor().more()`.
    pub fn incident_cursor(&self) -> &ListCursor {
        &self.base.traversal.incident
    }
}

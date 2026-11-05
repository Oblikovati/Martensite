//! The set retains cursor owners while extension code has exclusive traversal access.
use super::{IncidentSource, TopOpeBRepBuildShapeSetBase};
use martensite_tkbrep::{topexp::Explorer, topods::Shape};
use martensite_tkernel::ncollection::{List, ListCursor};

/// Exclusive traversal and source-list access; `view.shapes_mut().clear()`.
///
/// Rebinding moves the source list into this operation, preserving its nodes.
/// Cursor positions only resolve against retained owners; a foreign or deleted
/// node is never dereferenced. The shape values retain their ordinary shared
/// geometry semantics, not deep geometry isolation.
pub struct ShapeSetExtensionState<'a> {
    base: &'a mut TopOpeBRepBuildShapeSetBase,
}

impl TopOpeBRepBuildShapeSetBase {
    /// Borrows live traversal state; `let view = base.extension_state();`.
    pub fn extension_state(&mut self) -> ShapeSetExtensionState<'_> {
        ShapeSetExtensionState { base: self }
    }
}

impl ShapeSetExtensionState<'_> {
    /// Edits the stored shape list without updating deduplication indexes; `view.shapes_mut().clear()`.
    pub fn shapes_mut(&mut self) -> &mut List<Shape> {
        &mut self.base.my_shapes
    }

    /// Edits the current subshape search, retaining its depth; `view.explorer_mut().clear()`.
    pub fn explorer_mut(&mut self) -> &mut Explorer {
        &mut self.base.traversal.explorer
    }

    /// Edits the shape node position; `view.shapes_cursor_mut().init(&retained_list)`.
    pub fn shapes_cursor_mut(&mut self) -> &mut ListCursor {
        &mut self.base.traversal.shapes
    }

    /// Edits the start-element node position; `view.starts_cursor_mut().init(&retained_list)`.
    pub fn starts_cursor_mut(&mut self) -> &mut ListCursor {
        &mut self.base.traversal.starts
    }

    /// Edits the current incident node position; `view.incident_cursor_mut().init(&retained_list)`.
    pub fn incident_cursor_mut(&mut self) -> &mut ListCursor {
        &mut self.base.traversal.incident
    }

    /// Retains an independently owned shape source without copying nodes; `view.bind_shapes(source)`.
    pub fn bind_shapes(&mut self, source: List<Shape>) {
        self.base.traversal.shapes = source.cursor();
        self.base.traversal.shapes_owner = Some(source);
    }

    /// Retains an independently owned start source without changing start elements; `view.bind_starts(source)`.
    pub fn bind_starts(&mut self, source: List<Shape>) {
        self.base.traversal.starts = source.cursor();
        self.base.traversal.starts_owner = Some(source);
    }

    /// Retains an independently owned incident source; `view.bind_incident(source)`.
    pub fn bind_incident(&mut self, source: List<Shape>) {
        self.base.traversal.incident = source.cursor();
        self.base.traversal.owned = source;
        self.base.traversal.source = IncidentSource::Owned;
    }

    /// Edits the currently bound shape source; `view.shape_source_mut().append(shape)`.
    pub fn shape_source_mut(&mut self) -> &mut List<Shape> {
        self.base
            .traversal
            .shapes_owner
            .as_mut()
            .unwrap_or(&mut self.base.my_shapes)
    }

    /// Edits the currently bound start source; `view.start_source_mut().append(shape)`.
    pub fn start_source_mut(&mut self) -> &mut List<Shape> {
        self.base
            .traversal
            .starts_owner
            .as_mut()
            .unwrap_or(&mut self.base.my_start_shapes)
    }

    /// Edits the explicitly bound incident owner; `view.bound_incident_source_mut().clear()`.
    /// External borrowed providers continue to own their own source separately.
    pub fn bound_incident_source_mut(&mut self) -> &mut List<Shape> {
        &mut self.base.traversal.owned
    }
}

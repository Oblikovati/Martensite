//! The retained element list as GlFrameStats::updateStructures walks GlGroup nodes.
use super::{Graphic3dDrawGroup, GroupCommand, GroupGeometry};
use crate::element_stats::{CountedElement, GroupElementStats, bound_count};

impl Graphic3dDrawGroup {
    /// Counted elements in native order; aspect, flipping and stencil nodes count nothing.
    /// Example: `group.element_stats().len()`.
    pub(crate) fn element_stats(&self) -> Vec<CountedElement> {
        self.submissions
            .read()
            .commands
            .iter()
            .filter_map(command_stats)
            .collect()
    }
}

fn command_stats(command: &GroupCommand) -> Option<CountedElement> {
    match command {
        GroupCommand::Geometry(geometry) => geometry_stats(geometry).map(CountedElement::BuiltIn),
        GroupCommand::Element(element) => Some(CountedElement::User(element.clone())),
        _ => None,
    }
}

fn geometry_stats(geometry: &GroupGeometry) -> Option<GroupElementStats> {
    match geometry {
        GroupGeometry::Text(_, upload, _) => Some(GroupElementStats::Text(upload.current())),
        // GlGroup::AddPrimitiveArray creates no element for null attributes.
        GroupGeometry::Primitives(primitive) if primitive.attributes.is_null() => None,
        GroupGeometry::Primitives(primitive) => Some(GroupElementStats::Primitive {
            kind: primitive.kind,
            bounds: bound_count(&primitive.bounds),
            upload: primitive.upload.current(),
        }),
    }
}

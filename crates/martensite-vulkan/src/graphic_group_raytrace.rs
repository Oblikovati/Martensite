//! The element nodes GlView::addRaytraceGroups walks: aspects and primitive arrays.
use super::{GroupCommand, GroupGeometry};
use crate::primitive::PrimitiveSubmission;
use martensite_tkernel::standard::Handle;
use martensite_tkservice::graphic3d::Graphic3dAspectsSource;

/// One ray-tracing element node of a group, in submission order.
#[derive(Clone)]
pub(crate) enum RaytraceGroupCommand {
    /// An GlAspects node: a new material for the arrays after it.
    Aspects(Handle<dyn Graphic3dAspectsSource>),
    /// An GlPrimitiveArray node.
    Primitive(PrimitiveSubmission),
}

impl super::Graphic3dDrawGroup {
    /// The group aspects (`GlAspects()`, null when unset) and the aspect and primitive-array
    /// nodes; `group.raytrace_commands().1.len()`.
    pub(crate) fn raytrace_commands(
        &self,
    ) -> (
        Handle<dyn Graphic3dAspectsSource>,
        Vec<RaytraceGroupCommand>,
    ) {
        let state = self.submissions.read();
        let commands = state.commands.iter().filter_map(|command| match command {
            GroupCommand::Aspects(aspects) => Some(RaytraceGroupCommand::Aspects(aspects.clone())),
            GroupCommand::Geometry(GroupGeometry::Primitives(primitive)) => {
                Some(RaytraceGroupCommand::Primitive(primitive.clone()))
            }
            _ => None,
        });
        (state.aspects.clone(), commands.collect())
    }
}

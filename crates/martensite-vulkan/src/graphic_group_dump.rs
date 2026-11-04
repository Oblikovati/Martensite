//! `GlGroup::DumpJson` and the group's ray-tracing flag (`myIsRaytracable`):
//! a group becomes ray-traceable when it receives a primitive array drawn with at least
//! `GL_TRIANGLES` while it has no transformation persistence, and stays so until cleared.
use super::{Graphic3dDrawGroup, GroupCommand, GroupGeometry};
use crate::driver_dump::{address_of, aspects_element, element_begin, primitive_array, text};
use martensite_tkernel::standard::dump::DumpStream;
use martensite_tkservice::graphic3d::*;
use martensite_tkservice::json_dump::{base_class, class_begin, field_dumped, field_integer};
use std::io;

impl Graphic3dDrawGroup {
    /// `IsRaytracable`: whether the group holds ray-traced geometry.
    pub(crate) fn is_raytracable(&self) -> bool {
        self.submissions.read().raytracable
    }
    /// The tail of `GlGroup::AddElement` for a ray-traced element.
    pub(super) fn note_raytraced_element(&self) {
        if self.has_persistence() {
            return;
        }
        self.submissions.write().raytracable = true;
        self.update_raytracable_structure();
    }
    /// `HasPersistence`: the group's or its structure's transformation persistence.
    fn has_persistence(&self) -> bool {
        if !self.base.fields.read().trsf_pers.is_null() {
            return true;
        }
        self.structure().get().is_some_and(|structure| {
            !structure
                .cstructure()
                .cstructure_base()
                .fields
                .read()
                .trsf_pers
                .is_null()
        })
    }
    /// `GlStruct()->UpdateStateIfRaytracable(false)` when the group is ray-traceable.
    pub(super) fn update_raytracable_structure(&self) {
        if !self.is_raytracable() {
            return;
        }
        let Some(structure) = self.structure().get().map(|s| s.cstructure()) else {
            return;
        };
        if let Some(structure) = structure.cast::<crate::Graphic3dRenderStructure>().get() {
            structure.update_state_if_raytracable(false);
        }
    }
    /// `GlGroup::DumpJson`: the Graphic3d group, the group aspects, every element in
    /// order and the ray-tracing flag.
    pub(super) fn dump_group(&self, stream: &mut DumpStream, depth: i32) -> io::Result<()> {
        class_begin(stream, "GlGroup")?;
        base_class(stream, depth, &|o, d| {
            crate::driver_dump::graphic3d_group(self, o, d)
        })?;
        let state = self.submissions.read();
        if !state.aspects.is_null() {
            let address = address_of(&state.aspects);
            field_dumped(stream, depth, "myAspects", &|o, d| {
                aspects_element(o, d, address, &state.aspects)
            })?;
        }
        for command in &state.commands {
            let address = address_of(command);
            field_dumped(stream, depth, "anElement", &|o, d| {
                dump_command(command, address, o, d)
            })?;
        }
        field_integer(stream, "myIsRaytracable", i32::from(state.raytracable))
    }
}

/// The `DumpJson` of the native element a group command stands for.
fn dump_command(
    command: &GroupCommand,
    address: usize,
    stream: &mut DumpStream,
    depth: i32,
) -> io::Result<()> {
    match command {
        GroupCommand::Aspects(aspect) => aspects_element(stream, depth, address, aspect),
        GroupCommand::Flipping(_) => element_begin(stream, depth, "GlFlipper", address),
        GroupCommand::Stencil(enabled) => {
            element_begin(stream, depth, "GlStencilTest", address)?;
            field_integer(stream, "myIsEnabled", i32::from(*enabled))
        }
        GroupCommand::Geometry(GroupGeometry::Primitives(primitive)) => {
            primitive_array(stream, depth, address, primitive, primitive.uid)
        }
        GroupCommand::Geometry(GroupGeometry::Text(_, upload, _)) => {
            text(stream, depth, address, upload.current())
        }
        GroupCommand::Element(element) => element.dump_json(stream, depth),
    }
}

//! `GlStructure::DumpJson` and the structure's ray-tracing state.
use super::Graphic3dRenderStructure;
use crate::Graphic3dDrawGroup;
use martensite_tkernel::standard::dump::DumpStream;
use martensite_tkservice::graphic3d::*;
use martensite_tkservice::json_dump::{base_class, class_begin, field_integer, field_pointer};
use std::io;

impl Graphic3dRenderStructure {
    /// `IsRaytracable`: groups with ray-traced geometry and no transformation persistence, or
    /// a ray-traceable instanced structure.
    pub(crate) fn is_raytracable(&self) -> bool {
        let own = {
            let fields = self.base.fields.read();
            !fields.groups.is_empty() && self.raytrace.read().flag && fields.trsf_pers.is_null()
        };
        own || self
            .instanced_structure()
            .is_some_and(|instanced| instanced.is_raytracable())
    }
    /// `UpdateStateIfRaytracable`: refresh the flag (from the groups when `to_check`) and count
    /// a modification when the structure is ray-traceable.
    pub(crate) fn update_state_if_raytracable(&self, to_check: bool) {
        let flag = !to_check
            || self.groups().iter().any(|group| {
                group
                    .cast::<Graphic3dDrawGroup>()
                    .get()
                    .is_some_and(Graphic3dDrawGroup::is_raytracable)
            });
        self.raytrace.write().flag = flag;
        self.count_raytracable_modification();
    }
    /// The native `++myModificationState` guarded by `IsRaytracable`.
    pub(super) fn count_raytracable_modification(&self) {
        if self.is_raytracable() {
            self.raytrace.write().modification += 1;
        }
    }
    /// `ResetModificationState` when the counter is positive, as `toUpdateStructure` does for a
    /// structure that lost its ray-traced geometry; whether it was reset.
    pub(crate) fn reset_counted_modification(&self) -> bool {
        let mut state = self.raytrace.write();
        let counted = state.modification > 0;
        state.modification = 0;
        counted
    }
    /// The ray tracer's `StructState`: this counter and the instanced structure's counter.
    pub(crate) fn raytrace_struct_state(&self) -> (usize, Option<usize>) {
        let instanced = self.instanced_structure();
        let instanced = instanced.map(|structure| structure.raytrace.read().modification);
        (self.raytrace.read().modification, instanced)
    }
    /// `InstancedStructure`: the connected structure, or the parent of a shadow.
    fn instanced_structure(&self) -> Option<martensite_tkernel::standard::Handle<Self>> {
        self.geometry_reference().ok().flatten()
    }
    /// `GlStructure::DumpJson`: the Graphic3d structure, the instanced structure, the
    /// ray-tracing state and whether the transformation mirrors.
    pub(super) fn dump_structure(&self, stream: &mut DumpStream, depth: i32) -> io::Result<()> {
        class_begin(stream, "GlStructure")?;
        base_class(stream, depth, &|o, d| {
            crate::driver_dump::graphic3d_cstructure(self, o, d)
        })?;
        let instanced = self
            .instanced_structure()
            .map_or(0, |instanced| instanced.object_address() as usize);
        field_pointer(stream, "myInstancedStructure", instanced)?;
        let state = *self.raytrace.read();
        field_integer(stream, "myIsRaytracable", i32::from(state.flag))?;
        martensite_tkservice::json_dump::field_unsigned(
            stream,
            "myModificationState",
            state.modification as u64,
        )?;
        field_integer(stream, "myIsMirrored", i32::from(self.is_mirrored()))
    }
    /// `myIsMirrored`: the transformation's 3x3 determinant is negative.
    fn is_mirrored(&self) -> bool {
        let trsf = self.base.fields.read().trsf.clone();
        let Some(datum) = trsf.get() else {
            return false;
        };
        let t = datum.transformation();
        let v = |r: i32, c: i32| t.value(r, c).unwrap_or_default();
        let det = v(1, 1) * (v(2, 2) * v(3, 3) - v(3, 2) * v(2, 3))
            - v(1, 2) * (v(2, 1) * v(3, 3) - v(3, 1) * v(2, 3))
            + v(1, 3) * (v(2, 1) * v(3, 2) - v(3, 1) * v(2, 2));
        det < 0.
    }
}

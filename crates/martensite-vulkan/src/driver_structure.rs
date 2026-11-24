//! Renderer-created groups make ordinary Graphic3d submissions reach the draw stream.
use crate::driver_device_lost::DriverObjects;
use crate::text_layout::TextProvider;
use crate::{Graphic3dDrawGroup, RenderError};
use martensite_tkernel::standard::{Failure, Handle, SharedState, StandardTransient, WeakHandle};
use martensite_tkservice::graphic3d::Graphic3dClipPlaneSequenceSource;
use martensite_tkservice::graphic3d::*;
/// The Vulkan driver structure (`Graphic3dCStructure` of the driver): it owns the
/// structure's draw groups, connections, highlight state and shadow links, and lowers them
/// to draw records when a view extracts a frame.
///
/// Created by `Graphic3dRenderDriver::create_structure`; e.g.
/// `driver.create_structure(&manager)?.cast::<Graphic3dRenderStructure>`.
pub struct Graphic3dRenderStructure {
    base: Graphic3dCStructureBase,
    connection: RevisedState<Option<WeakHandle<Self>>>,
    this: SharedState<WeakHandle<Self>>,
    parent: Option<Handle<Self>>,
    text_source: TextProvider,
    raytrace: SharedState<RaytraceState>,
    registry: Option<std::sync::Weak<DriverObjects>>,
}
/// Native `GlStructure` ray-tracing bookkeeping: `myIsRaytracable` and
/// `myModificationState`, which the native ray tracer compares between frames.
#[derive(Clone, Copy, Default)]
struct RaytraceState {
    flag: bool,
    modification: usize,
}
martensite_tkernel::impl_transient!(Graphic3dRenderStructure, "Martensite_RenderStructure", StandardTransient, interfaces:[dyn Graphic3dCStructure]);
#[path = "structure_replay.rs"]
pub(crate) mod structure_replay;
impl Graphic3dRenderStructure {
    pub(crate) fn has_only_local_geometry(&self, driver: &std::sync::Arc<DriverObjects>) -> bool {
        self.parent.is_none()
            && self
                .registry
                .as_ref()
                .is_some_and(|owner| owner.ptr_eq(&std::sync::Arc::downgrade(driver)))
            && self.connection.read().is_none()
    }

    /// A structure of `manager` whose groups lay out text with `text_source`; e.g.
    /// `Graphic3dRenderStructure::new(&manager, Arc::new(FreeTypeTextSource::default()))?`.
    pub fn new(
        manager: &Handle<Graphic3dStructureManager>,
        text_source: std::sync::Arc<dyn crate::TextLayoutSource>,
    ) -> Result<Handle<Self>, Failure> {
        Self::with_parent(manager, None, TextProvider::Injected(text_source), None)
    }

    pub(crate) fn registered(
        manager: &Handle<Graphic3dStructureManager>,
        text_source: TextProvider,
        registry: &std::sync::Arc<DriverObjects>,
    ) -> Result<Handle<dyn Graphic3dCStructure>, Failure> {
        let structure = Self::with_parent(
            manager,
            None,
            text_source,
            Some(std::sync::Arc::downgrade(registry)),
        )?
        .cast();
        registry.register_structure(&structure);
        Ok(structure)
    }

    fn with_parent(
        manager: &Handle<Graphic3dStructureManager>,
        parent: Option<Handle<Self>>,
        text_source: TextProvider,
        registry: Option<std::sync::Weak<DriverObjects>>,
    ) -> Result<Handle<Self>, Failure> {
        let object = Handle::new(Self {
            base: Graphic3dCStructureBase::new(manager)?,
            connection: RevisedState::default(),
            this: SharedState::default(),
            parent,
            text_source,
            raytrace: SharedState::default(),
            registry,
        });
        *object.this.write() = object.downgrade();
        object.inherit_shadow_state();
        Ok(object)
    }
    fn inherit_shadow_state(&self) {
        let Some(parent) = &self.parent else {
            return;
        };
        let source = parent.base.fields.read();
        let mut target = self.base.fields.write();
        target.is_infinite = source.is_infinite;
        target.bnd_box = source.bnd_box;
        target.trsf = source.trsf.clone();
        target.trsf_pers = source.trsf_pers.clone();
    }
    pub(crate) fn draws(
        &self,
        camera: &Graphic3dCamera,
        view_id: i32,
        extent: [u32; 2],
        rendering: &Graphic3dRenderingParams,
        clipping: &Handle<dyn Graphic3dClipPlaneSequenceSource>,
        elements: &crate::render_element_pass::RenderElementPass,
    ) -> Result<crate::stencil_program::StencilProgram, RenderError> {
        let mut program = crate::stencil_program::StencilProgram::default();
        let scene = (camera, extent, rendering, clipping, elements);
        self.draws_into(view_id, scene, &mut program)?;
        Ok(program)
    }

    /// Append the structure's program for this pass to `target`: its groups' records
    /// replayed in place when all hold, otherwise its groups lowered;
    /// `structure.draws_into(view_id, scene, &mut layer_program)?`.
    pub(crate) fn draws_into(
        &self,
        view_id: i32,
        scene: structure_replay::StructureFrame<'_>,
        target: &mut crate::stencil_program::StencilProgram,
    ) -> Result<(), RenderError> {
        if !self.is_visible_in_view(view_id) || self.is_culled() {
            return Ok(());
        }
        #[cfg(feature = "draw-record-verification")]
        {
            let mut replayed = crate::stencil_program::StencilProgram::default();
            let replay = self.replayed_groups(scene, &mut replayed);
            let lowered = self.lowered_draws(scene)?;
            if replay.is_some() {
                structure_replay::verify(&replayed, &lowered);
            }
            target.append(lowered);
            Ok(())
        }
        #[cfg(not(feature = "draw-record-verification"))]
        {
            if let Some(skipped) = self.replayed_groups(scene, target) {
                scene.4.add_skipped(skipped);
                return Ok(());
            }
            target.append(self.lowered_draws(scene)?);
            Ok(())
        }
    }

    /// The structure's groups lowered (or replayed one by one) through the capping scene.
    fn lowered_draws(
        &self,
        (camera, extent, rendering, clipping, elements): structure_replay::StructureFrame<'_>,
    ) -> Result<crate::stencil_program::StencilProgram, RenderError> {
        let (fields, local_groups) = self.base.fields.read().snapshot_with_group_handles();
        if crate::retained_clipping::structure_is_clipped(clipping, &fields)? {
            return Ok(Default::default());
        }
        let local = elements
            .layer_origin()
            .and_then(|origin| origin.structure_frame(camera, &fields));
        let (camera, fields) = match &local {
            Some((camera, fields)) => (camera, fields),
            None => (camera, &fields),
        };
        let mut program = crate::retained_capping::CappingScene {
            camera,
            extent,
            fields,
            rendering,
            clipping,
            elements,
        }
        .draws(self.geometry_batches(local_groups)?)?;
        #[cfg(feature = "vertex-output-diagnostics")]
        vertex_output_scope::retain_main_capture(
            &mut program.draws,
            self.parent.as_ref().map(|parent| {
                [
                    self as *const Self as usize,
                    parent.object_address() as usize,
                ]
            }),
        );
        // The transparent revisit draws the same box again; one submission is equivalent.
        if elements.lowers_group_geometry()
            && let Some(draw) = crate::bound_box_highlight::bound_box_draw(
                fields,
                camera,
                Some(crate::render_scale::persistence_extent(extent, rendering)),
            )?
        {
            program.draws.push(draw);
        }
        Ok(program)
    }
    /// Groups of this structure, then of each instanced structure, for frame statistics.
    /// An invalid reference path keeps only the structure's own groups, which it still owns.
    /// Example: `structure.instanced_group_batches()[0].len()`.
    pub(crate) fn instanced_group_batches(&self) -> Vec<Vec<Handle<dyn Graphic3dGroup>>> {
        self.geometry_batches(self.groups())
            .map(|batches| batches.into_batches().collect())
            .unwrap_or_else(|_| vec![self.groups()])
    }
}
impl Drop for Graphic3dRenderStructure {
    fn drop(&mut self) {
        if let Some(registry) = self.registry.as_ref().and_then(std::sync::Weak::upgrade) {
            registry.forget_structure(self.this.read().object_address().addr());
        }
    }
}

impl Graphic3dCStructure for Graphic3dRenderStructure {
    fn cstructure_base(&self) -> &Graphic3dCStructureBase {
        &self.base
    }
    fn on_visibility_changed(&self) {
        self.count_raytracable_modification();
    }
    fn set_transformation(&self, trsf: &Handle<martensite_tkmath::toploc::Datum3>) {
        self.base.fields.write().trsf = trsf.clone();
        self.count_raytracable_modification();
    }
    fn set_transform_persistence(&self, trsf_pers: &Handle<dyn Graphic3dTransformPersSource>) {
        let changed = {
            let mut fields = self.base.fields.write();
            let changed = (fields.trsf_pers.is_null() || trsf_pers.is_null())
                && fields.trsf_pers != *trsf_pers;
            fields.trsf_pers = trsf_pers.clone();
            changed
        };
        if changed {
            self.raytrace.write().modification += 1;
        }
    }
    fn dump_json(
        &self,
        stream: &mut martensite_tkernel::standard::dump::DumpStream,
        depth: i32,
    ) -> std::io::Result<()> {
        self.dump_structure(stream, depth)
    }
    fn clear(&self) {
        let raytracable_deleted = self.groups().iter().any(|group| {
            group
                .cast::<Graphic3dDrawGroup>()
                .get()
                .is_some_and(Graphic3dDrawGroup::is_raytracable)
        });
        if raytracable_deleted {
            self.raytrace.write().flag = false;
        }
        for group in self.groups() {
            group.clear(false);
        }
        let mut fields = self.base.fields.write();
        fields.groups.clear(None);
        fields.is_2d_text = false;
        fields.is_for_highlight = false;
    }
    fn connect(&self, structure: &dyn Graphic3dCStructure) {
        // Preserve typed failures across the non-fallible driver hook.
        martensite_tkernel::standard::raise_native_failure(self.connect_geometry(structure));
    }
    fn disconnect(&self, structure: &dyn Graphic3dCStructure) {
        // Match the same void-hook exception boundary as connection.
        martensite_tkernel::standard::raise_native_failure(self.disconnect_geometry(structure));
    }
    fn graphic_highlight(&self, style: &Handle<dyn Graphic3dPresentationAttributesSource>) {
        let mut fields = self.base.fields.write();
        fields.highlight_style = style.clone();
        fields.highlight = true;
    }
    fn graphic_unhighlight(&self) {
        let mut fields = self.base.fields.write();
        fields.highlight_style = Handle::default();
        fields.highlight = false;
    }
    fn shadow_link(
        &self,
        manager: &Handle<Graphic3dStructureManager>,
    ) -> Result<Handle<dyn Graphic3dCStructure>, Failure> {
        // GlStructureShadow at flattens a shadow of a shadow to the original parent.
        let parent = self
            .parent
            .clone()
            .unwrap_or_else(|| self.this.read().upgrade());
        Ok(Self::with_parent(manager, Some(parent), self.text_source.clone(), None)?.cast())
    }
    fn new_group(
        &self,
        structure: &Handle<dyn Graphic3dStructureSource>,
    ) -> Result<Handle<dyn Graphic3dGroup>, Failure> {
        let group: Handle<dyn Graphic3dGroup> =
            Graphic3dDrawGroup::with_provider(structure, self.text_source.clone()).cast();
        self.base
            .fields
            .write()
            .groups
            .append(martensite_tkernel::standard::HandleSlot::new(group.clone()));
        Ok(group)
    }
    fn remove_group(&self, group: &Handle<dyn Graphic3dGroup>) {
        if let Some(value) = group.get() {
            value.clear(false);
        }
        // Remove every entry retaining this group, as the former vector retain did.
        let mut fields = self.base.fields.write();
        let groups = &mut fields.groups;
        let mut cursor = groups.cursor();
        while cursor.more() {
            if cursor
                .value(groups)
                .is_some_and(|stored| &**stored == group)
            {
                martensite_tkernel::standard::raise_native_failure(
                    groups.remove_cursor(&mut cursor),
                );
            } else {
                cursor.next(groups);
            }
        }
    }
}

#[path = "connected_geometry.rs"]
mod connected_geometry;
pub(crate) use connected_geometry::{GeometryBatches, GroupBatch};
#[path = "driver_structure_dump.rs"]
mod dump;

#[cfg(feature = "vertex-output-diagnostics")]
#[path = "shadow_vertex_output.rs"]
mod vertex_output_scope;

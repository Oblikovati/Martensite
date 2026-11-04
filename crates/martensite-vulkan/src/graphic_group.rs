//! Retained Graphic3d commands distinguish the group style from later primitive styles.
use crate::line_style::primitive_style_in_pipeline;
use crate::{PrimitiveDraw, RenderError, primitive::PrimitiveSubmission};
use martensite_tkernel::{
    ncollection::Vec4,
    standard::{Handle, SharedState, StandardTransient},
};
use martensite_tkmath::gp::Axis2;
use martensite_tkservice::graphic3d::*;
use std::sync::Arc;
#[path = "graphic_group_commands.rs"]
mod commands;
#[path = "graphic_group_dump.rs"]
mod dump;
#[path = "graphic_group_elements.rs"]
mod elements;
#[path = "graphic_group_extract.rs"]
mod extraction;
#[path = "graphic_group_primitive_lowering.rs"]
mod primitive_lowering;
#[path = "graphic_group_raytrace.rs"]
mod raytrace;
#[path = "graphic_group_stats.rs"]
mod statistics;
use primitive_lowering::lower_styled_primitive;
#[cfg(feature = "vertex-output-diagnostics")]
#[path = "graphic_group_vertex_output.rs"]
mod vertex_output;
use commands::{GroupCommand, GroupGeometry};
pub(crate) use raytrace::RaytraceGroupCommand;
#[derive(Default)]
struct GroupSubmissionState {
    #[cfg(feature = "vertex-output-diagnostics")]
    vertex_output: Option<(usize, std::num::NonZeroU64)>,
    #[cfg(feature = "vertex-output-diagnostics")]
    vertex_output_text: Option<(usize, std::num::NonZeroU64)>,
    aspects: Handle<dyn Graphic3dAspectsSource>,
    commands: Vec<GroupCommand>,
    /// Native `myIsRaytracable`; reset by `Clear` with the commands.
    raytracable: bool,
}
/// A retained group feeding Martensite draw extraction; `Graphic3dDrawGroup::new(&structure)`.
pub struct Graphic3dDrawGroup {
    transient: StandardTransient,
    base: Graphic3dGroupBase,
    submissions: SharedState<GroupSubmissionState>,
    text_source: Arc<dyn crate::TextLayoutSource>,
    mesh_edge_pipeline: crate::RenderMeshEdgePipeline,
    pub(crate) texture_images: Arc<crate::retained_texture_images::RetainedTextureImages>,
    pub(crate) aspect_textures: crate::retained_aspect_textures::RetainedAspectTextures,
    /// The driver's precompiled user programs (`crate::retained_user_program`).
    pub(crate) user_programs: Arc<crate::RenderUserProgramLibrary>,
}
martensite_tkernel::impl_transient!(Graphic3dDrawGroup,"Martensite_Graphic3dDrawGroup",StandardTransient,transient,interfaces:[dyn Graphic3dGroup]);
impl Graphic3dDrawGroup {
    /// Attach to a live Graphic3d structure; `Graphic3dDrawGroup::new(&structure)`.
    pub fn new(structure: &Handle<dyn Graphic3dStructureSource>) -> Handle<Self> {
        Self::with_text_source(structure, Arc::new(crate::FreeTypeTextSource::default()))
    }
    /// Inject font I/O and layout; `Graphic3dDrawGroup::with_text_source(&structure, fonts)`.
    pub fn with_text_source(
        structure: &Handle<dyn Graphic3dStructureSource>,
        text_source: Arc<dyn crate::TextLayoutSource>,
    ) -> Handle<Self> {
        Handle::new(Self {
            transient: StandardTransient::default(),
            base: Graphic3dGroupBase::new(structure),
            submissions: SharedState::default(),
            text_source,
            mesh_edge_pipeline: crate::retained_mesh_edges::driver_mesh_edge_pipeline(structure),
            texture_images: crate::retained_surface_texture::driver_texture_images(structure),
            aspect_textures: Default::default(),
            user_programs: crate::retained_user_program::driver_user_programs(structure),
        })
    }
    /// Extract visible primitives using current styles, buffers and camera; `group.primitive_draws(&camera)?`.
    pub fn primitive_draws(
        &self,
        camera: &Graphic3dCamera,
    ) -> Result<Vec<PrimitiveDraw>, RenderError> {
        if self.is_deleted() || !self.structure().is_visible() {
            return Ok(vec![]);
        }
        self.draws_in_structure(
            camera,
            None,
            &self
                .structure()
                .cstructure()
                .cstructure_base()
                .fields
                .read(),
            &Graphic3dRenderingParams::new().expect("default rendering parameters"),
            &Handle::default(),
        )
    }
    /// Extract using the actual pixel viewport, including persistence; `group.primitive_draws_for_view(&camera, [800, 600])?`.
    pub fn primitive_draws_for_view(
        &self,
        camera: &Graphic3dCamera,
        extent: [u32; 2],
    ) -> Result<Vec<PrimitiveDraw>, RenderError> {
        if self.is_deleted() || !self.structure().is_visible() {
            return Ok(vec![]);
        }
        self.draws_in_structure(
            camera,
            Some(extent),
            &self
                .structure()
                .cstructure()
                .cstructure_base()
                .fields
                .read(),
            &Graphic3dRenderingParams::new().expect("default rendering parameters"),
            &Handle::default(),
        )
    }
    /// Compatibility extraction name; `group.triangle_draws(&camera)?` also supports line primitives.
    pub fn triangle_draws(
        &self,
        camera: &Graphic3dCamera,
    ) -> Result<Vec<PrimitiveDraw>, RenderError> {
        self.primitive_draws(camera)
    }
    /// Compatibility viewport extraction name; `group.triangle_draws_for_view(&camera, [64,64])?`.
    pub fn triangle_draws_for_view(
        &self,
        camera: &Graphic3dCamera,
        extent: [u32; 2],
    ) -> Result<Vec<PrimitiveDraw>, RenderError> {
        self.primitive_draws_for_view(camera, extent)
    }
    pub(crate) fn draws_in_structure(
        &self,
        camera: &Graphic3dCamera,
        extent: Option<[u32; 2]>,
        structure: &Graphic3dCStructureFields,
        rendering: &Graphic3dRenderingParams,
        global_clipping: &Handle<Graphic3dSequenceOfHClipPlane>,
    ) -> Result<Vec<PrimitiveDraw>, RenderError> {
        Ok(self
            .program_in_structure(
                camera,
                extent,
                structure,
                rendering,
                global_clipping,
                (
                    &crate::render_element_pass::RenderElementPass::unfiltered(),
                    &mut Default::default(),
                ),
            )?
            .draws)
    }
    /// Native group arrays are GlPrimitiveArray elements, each taking a driver id.
    fn take_primitive_array_uid(&self) -> usize {
        // A detached group (no structure or manager) has no driver to take an id from.
        let structure = self.structure();
        let Some(manager) = structure
            .get()
            .map(|structure| structure.structure_manager())
        else {
            return 0;
        };
        let Some(driver) = manager.get().map(|manager| manager.graphic_driver()) else {
            return 0;
        };
        driver
            .cast::<crate::Graphic3dRenderDriver>()
            .get()
            .map_or(0, |driver| driver.next_primitive_array_uid())
    }
    /// Retain an array the driver draws itself rather than a group element (native
    /// GlPrimitiveArray members of GlGraduatedTrihedron): it takes no driver id.
    /// Example: `group.add_driver_primitive_array(&array)`.
    pub(crate) fn add_driver_primitive_array(&self, array: &Graphic3dArrayOfPrimitives) {
        let primitive = PrimitiveSubmission {
            kind: array.primitive_type(),
            indices: array.indices_ref().clone(),
            attributes: array.attributes_ref().clone(),
            bounds: array.bounds_ref().clone(),
            upload: Default::default(),
            uid: 0,
        };
        self.retain_primitive(primitive, false);
    }
    fn retain_primitive(&self, primitive: PrimitiveSubmission, evaluate_bounds: bool) {
        if self.is_deleted() {
            return;
        }
        if evaluate_bounds && let Ok(positions) = primitive.positions() {
            let mut fields = self.base.fields.write();
            // Graphic3dGroup at bounds Vec4 inputs by raw xyz, ignoring w.
            for [x, y, z, _] in positions {
                fields.bounds.add(&Vec4::new(x, y, z, 1.));
            }
        }
        self.submissions
            .write()
            .commands
            .push(GroupCommand::Geometry(GroupGeometry::Primitives(primitive)));
        self.update();
    }
}
impl Graphic3dGroup for Graphic3dDrawGroup {
    fn group_base(&self) -> &Graphic3dGroupBase {
        &self.base
    }
    fn clear(&self, update: bool) {
        self.aspect_textures.synchronize();
        self.base.fields.write().bounds.clear();
        *self.submissions.write() = GroupSubmissionState::default();
        if update {
            self.update();
        }
    }
    fn aspects(&self) -> Handle<dyn Graphic3dAspectsSource> {
        self.submissions.read().aspects.clone()
    }
    fn set_group_primitives_aspect(
        &self,
        aspect: &dyn martensite_tkernel::standard::HandleReference<dyn Graphic3dAspectsSource>,
    ) {
        if self.is_deleted() {
            return;
        }
        self.aspect_textures.invalidate(&aspect.observe());
        self.submissions.write().aspects = aspect.observe().clone();
        self.update_raytracable_structure();
        self.update();
    }
    fn set_primitives_aspect(
        &self,
        aspect: &dyn martensite_tkernel::standard::HandleReference<dyn Graphic3dAspectsSource>,
    ) {
        if self.aspects().is_null() {
            self.set_group_primitives_aspect(aspect);
            return;
        }
        if self.is_deleted() {
            return;
        }
        self.aspect_textures.invalidate(&aspect.observe());
        self.submissions
            .write()
            .commands
            .push(GroupCommand::Aspects(aspect.observe().clone()));
        self.update();
    }
    fn synchronize_aspects(&self) {
        self.aspect_textures.synchronize();
        self.synchronize_elements();
        if !self.submissions.read().aspects.is_null() {
            self.update_raytracable_structure();
        }
        self.update();
    }
    fn replace_aspects(&self, map: &Graphic3dAspectsReplaceMap) {
        self.aspect_textures.synchronize();
        let mut state = self.submissions.write();
        let replaced = replace_style(&mut state.aspects, map);
        for command in &mut state.commands {
            if let GroupCommand::Aspects(style) = command {
                replace_style(style, map);
            }
        }
        drop(state);
        if replaced {
            self.update_raytracable_structure();
        }
    }
    fn dump_json(
        &self,
        stream: &mut martensite_tkernel::standard::dump::DumpStream,
        depth: i32,
    ) -> std::io::Result<()> {
        self.dump_group(stream, depth)
    }
    fn add_primitive_array_buffers(
        &self,
        kind: Graphic3dTypeOfPrimitiveArray,
        indices: &Handle<dyn Graphic3dIndexBufferSource>,
        attributes: &Handle<dyn Graphic3dBufferSource>,
        bounds: &Handle<Graphic3dBoundBuffer>,
        evaluate_bounds: bool,
    ) {
        if self.is_deleted() || attributes.is_null() {
            // GlGroup::AddPrimitiveArray returns before creating an element.
            return;
        }
        let uid = self.take_primitive_array_uid();
        let primitive = PrimitiveSubmission {
            kind,
            indices: indices.clone(),
            attributes: attributes.clone(),
            bounds: bounds.clone(),
            upload: Default::default(),
            uid,
        };
        let raytraced = crate::driver_dump::is_raytraced_primitive(&primitive);
        self.retain_primitive(primitive, evaluate_bounds);
        if raytraced {
            self.note_raytraced_element();
        }
    }
    fn add_text(&self, text: &Handle<Graphic3dText>, evaluate_bounds: bool) {
        if self.is_deleted() || text.is_null() {
            return;
        }
        // GlGroup::AddText mutates undersized text to the driver's default height.
        if text.height() < 2. {
            text.set_height(16.);
        }
        self.submissions
            .write()
            .commands
            .push(GroupCommand::Geometry(GroupGeometry::Text(
                text.clone(),
                Default::default(),
                crate::render_element::TextPlacement::Scene,
            )));
        self.record_text_bounds(text, evaluate_bounds);
    }
    fn set_stencil_test_options(&self, enabled: bool) {
        if self.is_deleted() {
            return;
        }
        self.submissions
            .write()
            .commands
            .push(GroupCommand::Stencil(enabled));
    }
    fn set_flipping_options(&self, enabled: bool, plane: &Axis2) {
        if self.is_deleted() {
            return;
        }
        self.submissions
            .write()
            .commands
            .push(GroupCommand::Flipping(enabled.then_some(*plane)));
        if enabled {
            self.structure().cstructure().set_group_flipping(true);
            self.base.fields.write().flipper = Handle::new(Graphic3dFlipper::new(*plane));
        }
    }
}
fn apply_command_clipping(
    draws: &mut [PrimitiveDraw],
    flips: &crate::group_flipping::GroupFlipStack,
    clipping: Option<&crate::RenderClipping>,
) -> Result<(), RenderError> {
    let clipping = command_clipping(flips, clipping)?;
    for draw in draws {
        // The standard font shader advertises zero clipping planes.
        if draw.textured_style.is_none() {
            draw.clipping = clipping.clone();
        }
    }
    Ok(())
}
fn command_clipping(
    flips: &crate::group_flipping::GroupFlipStack,
    clipping: Option<&crate::RenderClipping>,
) -> Result<Option<crate::RenderClipping>, RenderError> {
    let Some(clipping) = clipping else {
        return Ok(None);
    };
    let Some(model) = flips.override_model() else {
        return Ok(Some(clipping.clone()));
    };
    crate::RenderClipping::new(clipping.chains().to_vec(), model.data()).map(Some)
}
/// Replace a mapped style; whether it was replaced.
fn replace_style(
    style: &mut Handle<dyn Graphic3dAspectsSource>,
    map: &Graphic3dAspectsReplaceMap,
) -> bool {
    let Some((_, replacement)) = map.get(&(style.object_address() as usize)) else {
        return false;
    };
    *style = replacement.clone();
    true
}

fn lower_hatched_surface(
    primitive: &PrimitiveSubmission,
    aspects: &Handle<dyn Graphic3dAspectsSource>,
    closed: bool,
    matrix: [f32; 16],
    highlighted: bool,
    context: &crate::text_submission::TextSceneContext<'_>,
) -> Result<Vec<PrimitiveDraw>, RenderError> {
    let mut draws = lower_group_surface(primitive, aspects, closed, matrix, highlighted, context)?;
    crate::retained_hatch::apply_hatch(&mut draws, aspects)?;
    Ok(draws)
}

fn lower_group_surface(
    primitive: &PrimitiveSubmission,
    aspects: &Handle<dyn Graphic3dAspectsSource>,
    closed: bool,
    matrix: [f32; 16],
    highlighted: bool,
    context: &crate::text_submission::TextSceneContext<'_>,
) -> Result<Vec<PrimitiveDraw>, RenderError> {
    let program =
        crate::retained_user_program::resolve_user_program(&context.group.user_programs, aspects)?;
    let lighting_pipeline =
        crate::lighting_pipeline::structure_lighting_pipeline(&context.group.structure());
    let mut lowered = lower_styled_primitive(
        primitive,
        aspects,
        closed,
        matrix,
        highlighted,
        lighting_pipeline,
        crate::lighting_pipeline::structure_unlit_face_policy(&context.group.structure()),
    )?;
    crate::retained_surface_texture::apply_surface_texture(
        &mut lowered,
        primitive,
        aspects,
        context,
    )?;
    for draw in &mut lowered {
        draw.source_line_strip = primitive.kind == Graphic3dTypeOfPrimitiveArray::Polylines;
        draw.source_color_array = !highlighted
            && primitive
                .attributes
                .attribute_data(Graphic3dTypeOfAttribute::Color)
                .is_some();
    }
    crate::retained_lighting::apply_surface_lighting(&mut lowered, primitive, aspects, context)?;
    crate::retained_user_program::apply_user_program(
        &mut lowered,
        program,
        primitive,
        aspects,
        context,
    )?;
    lower_group_edges(&mut lowered, aspects, context)?;
    crate::retained_hollow::apply_hollow_interior(&mut lowered, &**aspects, context.rendering)?;
    append_silhouettes(&mut lowered, aspects, context)?;
    crate::polygon_point_raster::apply_polygon_points(&mut lowered, &**aspects)?;
    crate::retained_surface_texture::refuse_builtin_volume_textures(&lowered)?;
    Ok(lowered)
}
/// The outline after its face draw, as GlPrimitiveArray::Render draws it.
fn append_silhouettes(
    draws: &mut Vec<PrimitiveDraw>,
    aspects: &Handle<dyn Graphic3dAspectsSource>,
    context: &crate::text_submission::TextSceneContext<'_>,
) -> Result<(), RenderError> {
    if !aspects.to_draw_silhouette() {
        return Ok(());
    }
    let scale =
        crate::retained_mesh_edges::native_line_width_scale(context.rendering.resolution_ratio());
    let params = crate::silhouette_outline::SilhouetteParams::of(
        &**aspects,
        context.camera,
        context.extent,
        scale,
    )?;
    let mut index = 0;
    while index < draws.len() {
        if let Some(outline) =
            crate::silhouette_outline::outline_draw(&draws[index], &**aspects, params)?
        {
            index += 1;
            draws.insert(index, outline);
        }
        index += 1;
    }
    Ok(())
}
fn lower_group_edges(
    draws: &mut Vec<PrimitiveDraw>,
    aspects: &Handle<dyn Graphic3dAspectsSource>,
    context: &crate::text_submission::TextSceneContext<'_>,
) -> Result<(), RenderError> {
    crate::retained_mesh_edges::apply_retained_mesh_edges(
        draws,
        &**aspects,
        context.group.mesh_edge_pipeline,
        context.texture_coordinates(&**aspects),
    )?;
    crate::retained_mesh_edges::apply_graphic3d_raster_widths(draws, context.rendering)?;
    crate::retained_eye_edge_matrix::apply_eye_edge_matrices(
        draws,
        crate::persistent_transform::group_projection_only(context)?,
    )
}

//! Resolve visible Graphic3d layers into the same immutable submissions used by offscreen tests.
use crate::{
    RenderError, RenderFrame, RenderLayer, driver_structure::Graphic3dRenderStructure,
    error::rejected, render_element_pass::RenderElementPass,
};
use martensite_tkernel::standard::{Handle, HandleObserver, HandleReference};
use martensite_tkservice::graphic3d::*;
pub(crate) fn frame_for_view(
    view: &dyn Graphic3dCView,
    extent: [u32; 2],
    clipping: &dyn HandleReference<dyn Graphic3dClipPlaneSequenceSource>,
    extras: ViewFrameExtras<'_>,
) -> Result<RenderFrame, RenderError> {
    validate_view(view)?;
    crate::retained_texture_images::prune_view_texture_snapshots(view)?;
    let camera = crate::view_render_camera::owned_render_camera(view);
    // GlView::render ray-traces at the viewport size when the scene has ray-traced data.
    crate::raytrace::admit_view_adaptive_sampling(view, extras.limits);
    let (raytrace, environment) = crate::view_raytrace::view_raytrace(view, extent, &extras)?;
    let inputs = (clipping, extras.stats, raytrace.is_some(), extras.limits);
    let (mut passes, rendering, immediate) =
        view_layer_passes(view, extent, &camera, inputs, extras.frame_steps, None)?;
    // A render scale renders the scene into its own scaled framebuffer (see crate::render_scale).
    let extent = scene_extent(view, extent)?;
    apply_view_culling(&mut passes, view.backfacing_model());
    crate::retained_lighting::apply_view_lights(&mut passes, view)?;
    let (main_layers, colored_quad) =
        main_pass_layers(view, &mut passes, extent, (&extras, immediate))?;
    let grid = extras.grid_block(&camera, main_layers);
    let raytrace = raytrace.map(|mut raytrace| {
        // After the raster pass of the last ray-traced layer, before the upper layers.
        raytrace.span = passes
            .iter()
            .rposition(|layer| layer.raytraced)
            .map_or(0, |i| i + 1);
        std::sync::Arc::new(raytrace)
    });
    let clear = view_clear_color(view);
    let mut frame = RenderFrame::with_layers_and_grid(extent[0], extent[1], clear, passes, grid)?;
    frame.colored_quad = colored_quad;
    frame.raytrace = raytrace;
    frame.environment = environment;
    frame.raster = crate::retained_transparency::view_raster_settings(view, &rendering)?;
    // GlView::redraw clears alpha to 1 with buffersOpaqueAlpha, else to 0.
    frame.clear_color[3] = if frame.raster.alpha_writes { 0. } else { 1. };
    crate::compatibility_text_color::capture_text_color_target(&mut frame);
    Ok(frame)
}
/// The depth-only scene of one shadow map (GlView::renderShadowMap): the non-immediate
/// layers seen from the light's `camera` into a `resolution` square, culled for the light
/// camera over the `window` extent, without transform-persistent groups.
pub(crate) fn shadow_frame_for_view(
    view: &dyn Graphic3dCView,
    (resolution, window): (u32, [u32; 2]),
    camera: &Handle<Graphic3dCamera>,
    (clipping, limits): ShadowPassInputs<'_>,
) -> Result<RenderFrame, RenderError> {
    let extent = [resolution, resolution];
    let shadow = Some((camera, window));
    let (mut passes, _, immediate) = view_layer_passes(
        view,
        extent,
        camera,
        (clipping, None, false, limits),
        None,
        shadow,
    )?;
    passes.truncate(passes.len() - immediate);
    apply_view_culling(&mut passes, view.backfacing_model());
    RenderFrame::with_layers_and_grid(resolution, resolution, [0., 0., 0., 1.], passes, None)
}
/// The clip planes and device limits of a shadow-map pass.
pub(crate) type ShadowPassInputs<'a> = (
    &'a dyn HandleReference<dyn Graphic3dClipPlaneSequenceSource>,
    Option<crate::RenderDeviceLimits>,
);
/// The view state beyond its layers: statistics, their overlay and the shader grid.
#[derive(Clone, Copy, Default)]
pub(crate) struct ViewFrameExtras<'a> {
    pub stats: Option<&'a crate::RenderFrameStats>,
    pub overlay: Option<&'a crate::frame_stats_overlay::FrameStatsOverlay>,
    pub trihedron: Option<&'a crate::RenderGraduatedTrihedron>,
    pub grid:
        Option<&'a martensite_tkernel::standard::SharedState<crate::shader_grid::RenderShaderGrid>>,
    /// The view whose frame-step hooks build the layers; direct extraction builds them inline.
    pub frame_steps: Option<&'a crate::Graphic3dRenderView>,
    /// The device limits elements see through their context state; `None` without a device.
    pub limits: Option<crate::RenderDeviceLimits>,
}
impl ViewFrameExtras<'_> {
    /// The grid block of this frame, after recording the frame's world view for background grids.
    fn grid_block(
        &self,
        camera: &Graphic3dCamera,
        main_layers: usize,
    ) -> Option<(usize, crate::shader_grid::RenderShaderGridBlock)> {
        let mut grid = self.grid?.write();
        let world_view = camera.orientation_matrix_f();
        grid.observe_world_view(world_view);
        grid.is_drawn()
            .then(|| (main_layers, grid.uniforms(camera, &world_view)))
    }
}
/// Insert the graduated trihedron, prepend the background and append the statistics overlay;
/// returns the main-pass layer count and the corner gradient background.
fn main_pass_layers(
    view: &dyn Graphic3dCView,
    passes: &mut Vec<RenderLayer>,
    extent: [u32; 2],
    (extras, immediate): (&ViewFrameExtras<'_>, usize),
) -> Result<
    (
        usize,
        Option<crate::colored_quad_background::ColoredQuadBackground>,
    ),
    RenderError,
> {
    let mut main = passes.len() - immediate;
    // GlView::renderScene draws the trihedron after the shader grid, before immediate layers;
    // the grid block stays at `main`, so it precedes the trihedron layer.
    if let Some(trihedron) = extras.trihedron {
        passes.insert(main, trihedron.layer(view, extent)?);
    }
    if let (Some(stats), Some(overlay)) = (extras.stats, extras.overlay) {
        passes.extend(overlay.layer(view, stats, extent)?);
    }
    let before = passes.len();
    let colored_quad = crate::gradient_background::prepend_background(view, extent, passes)?;
    main += passes.len() - before;
    Ok((main, colored_quad))
}
/// The framebuffer extent of the view's scene: the target, or its scaled framebuffer.
pub(crate) fn scene_extent(
    view: &dyn Graphic3dCView,
    extent: [u32; 2],
) -> Result<[u32; 2], RenderError> {
    match crate::render_scale::active_render_scale(&view.rendering_params()) {
        Some(scale) => crate::render_scale::scaled_extent(extent, scale),
        None => Ok(extent),
    }
}
struct ViewLayerInputs<'a> {
    camera: &'a Graphic3dCamera,
    clipping: HandleObserver<Graphic3dSequenceOfHClipPlane>,
    rendering: Graphic3dRenderingParams,
    extent: [u32; 2],
    view_id: i32,
    /// myTextureEnv of GlView.
    environment: Handle<Graphic3dTextureEnv>,
    /// The shadow-map pass skips transform-persistent groups.
    skip_trsf_persistence: bool,
    /// The ray-traced layers rasterize only what ray tracing does not render.
    raytraced: bool,
    /// The frame's context state; each layer adds its polygon offset.
    context: crate::RenderContextState,
}
impl<'a> ViewLayerInputs<'a> {
    fn new(
        view: &dyn Graphic3dCView,
        extent: [u32; 2],
        camera: &'a Graphic3dCamera,
        (clipping, limits): ShadowPassInputs<'_>,
    ) -> Result<Self, RenderError> {
        let raster = scene_extent(view, extent)?;
        let rendering =
            crate::render_scale::scaled_rendering(&view.rendering_params(), extent, raster);
        // Culling updates before the clip planes reset; capture this owned field afterward.
        Ok(Self {
            camera,
            clipping: clipping
                .observe()
                .observe_as::<Graphic3dSequenceOfHClipPlane>(),
            context: frame_context_state(view, raster, &rendering, limits),
            rendering,
            extent: raster,
            view_id: view.identification(),
            environment: view.cview_base().fields.read().texture_env_data.clone(),
            skip_trsf_persistence: false,
            raytraced: false,
        })
    }
    fn layer(&self, layer: &Graphic3dLayer) -> Result<RenderLayer, RenderError> {
        layer_pass(
            layer,
            self.camera,
            self.view_id,
            self.extent,
            &self.rendering,
            &self.clipping,
            (
                &self.environment,
                self.skip_trsf_persistence,
                self.raytraced,
                &self.context,
            ),
        )
    }
}
/// The context state of a frame: `SetResolution` with the
/// render scale, `ResizeViewport` to the scene framebuffer, the line feather, the view's
/// backfacing model and alpha-to-coverage allowed only for a multisampled scene.
fn frame_context_state(
    view: &dyn Graphic3dCView,
    raster: [u32; 2],
    rendering: &Graphic3dRenderingParams,
    limits: Option<crate::RenderDeviceLimits>,
) -> crate::RenderContextState {
    let params = view.rendering_params();
    let scale = crate::render_scale::active_render_scale(&params).unwrap_or(1.);
    let fixed = crate::retained_transparency::fixed_function_view(view);
    let pbr = !fixed && limits.is_some();
    let mut state = crate::RenderContextState {
        limits,
        camera: view.camera(),
        window_size: view.window().get().map_or([0; 2], |w| w.size().into()),
        // GlView::render: PBRShadingModelFallback(model, checkPBRAvailability).
        shading_model: crate::RenderContextState::pbr_shading_model_fallback(
            params.shading_model,
            pbr,
        ),
        has_pbr: pbr,
        fixed_function_lighting: fixed,
        srgb: !fixed && limits.is_some_and(|limits| limits.has_srgb()),
        line_feather: params.line_feather,
        view_backfacing: view.backfacing_model(),
        allow_alpha_to_coverage: params.to_enable_alpha_to_coverage
            && rendering.nb_msaa_samples > 0,
        ..crate::RenderContextState::default()
    };
    state.set_resolution(params.resolution, params.resolution_ratio(), scale);
    state.resize_viewport(raster);
    state
}
type ViewPassInputs<'a> = (
    &'a dyn HandleReference<dyn Graphic3dClipPlaneSequenceSource>,
    Option<&'a crate::RenderFrameStats>,
    bool,
    Option<crate::RenderDeviceLimits>,
);
/// The culling camera and window extent of a shadow-map pass (GlView::renderShadowMap).
type ShadowCulling<'a> = Option<(&'a Handle<Graphic3dCamera>, [u32; 2])>;
fn view_layer_passes(
    view: &dyn Graphic3dCView,
    extent: [u32; 2],
    camera: &Graphic3dCamera,
    (clipping, stats, raytraced, limits): ViewPassInputs<'_>,
    steps: Option<&crate::Graphic3dRenderView>,
    shadow: ShadowCulling<'_>,
) -> Result<(Vec<RenderLayer>, Graphic3dRenderingParams, usize), RenderError> {
    let mut layers = view.layers();
    // GlView::renderStructs draws no structure in a subview composer.
    if view.is_subview_composer() {
        layers.clear();
    }
    // GlFilteredIndexedLayerIterator renders normal layers before immediate ones.
    layers.sort_by_key(|layer| layer.is_immediate());
    let mut culling = || match shadow {
        Some((camera, window)) => update_culling(view, &layers, camera, window),
        None => update_culling(
            view,
            &layers,
            &crate::view_render_camera::owned_render_camera(view),
            extent,
        ),
    };
    match stats {
        // GlLayerList::UpdateCulling measures this traversal as CpuCulling.
        Some(stats) => stats.time_culling(&mut culling)?,
        None => culling()?,
    }
    // Culling keeps the target size, as GlView passes the window size to its BVH selector.
    let mut inputs = ViewLayerInputs::new(view, extent, camera, (clipping, limits))?;
    inputs.skip_trsf_persistence = shadow.is_some();
    if let Some((light_camera, _)) = shadow {
        // GlView::renderShadowMap sets the light camera on the context.
        inputs.context.camera = light_camera.clone();
    }
    inputs.raytraced = raytraced;
    let build = |immediate: bool| {
        crate::view_layer_clear::source_passes(
            layers
                .iter()
                .filter(|layer| layer.is_immediate() == immediate)
                .filter(|layer| layer_selected(view, layer.layer_id())),
            |layer| inputs.layer(layer),
        )
    };
    let (mut passes, immediate) = match steps {
        Some(view) => view.run_frame_steps(camera.projection_type(), &build)?,
        None => (build(false)?, build(true)?),
    };
    crate::view_layer_clear::initialize_normal_passes(&mut passes);
    let count = immediate.len();
    passes.extend(immediate);
    let (passes, count) = crate::depth_prepass::expand_view_layers(
        passes,
        count,
        inputs.rendering.to_enable_depth_prepass,
    );
    Ok((passes, inputs.rendering, count))
}
fn view_clear_color(view: &dyn Graphic3dCView) -> [f32; 4] {
    let color = view.cview_base().fields.read().bg_color;
    [
        color.rgb().red() as f32,
        color.rgb().green() as f32,
        color.rgb().blue() as f32,
        color.alpha(),
    ]
}

fn update_culling(
    view: &dyn Graphic3dCView,
    layers: &[Handle<Graphic3dLayer>],
    camera: &Handle<Graphic3dCamera>,
    extent: [u32; 2],
) -> Result<(), RenderError> {
    let params = view.rendering_params();
    let selector = culling_selector(view, camera, extent);
    for layer in layers {
        layer
            .update_culling(
                view.identification(),
                &selector,
                params.ray_tracing.frustum_culling_state,
            )
            .map_err(|error| rejected(error, "a valid native layer culling traversal"))?;
    }
    Ok(())
}
fn culling_selector(
    view: &dyn Graphic3dCView,
    camera: &Handle<Graphic3dCamera>,
    extent: [u32; 2],
) -> Graphic3dCullingTool {
    let mut selector = Graphic3dCullingTool::default();
    selector.set_view_volume(camera, &martensite_tkernel::ncollection::Mat4::identity());
    selector.set_viewport_size(
        extent[0] as i32,
        extent[1] as i32,
        f64::from(view.rendering_params().resolution_ratio()),
    );
    selector.cache_clip_pts_projections();
    selector
}
fn layer_selected(view: &dyn Graphic3dCView, id: Graphic3dZLayerId) -> bool {
    if view.z_layer_redraw_mode() {
        return id == view.z_layer_target();
    }
    id >= view.z_layer_target()
}
fn layer_pass(
    layer: &Graphic3dLayer,
    camera: &Graphic3dCamera,
    view_id: i32,
    extent: [u32; 2],
    rendering: &Graphic3dRenderingParams,
    clipping: &Handle<Graphic3dSequenceOfHClipPlane>,
    (environment, skip_trsf_persistence, raytraced, context): LayerPassFrame<'_>,
) -> Result<RenderLayer, RenderError> {
    let settings = layer.layer_settings();
    let raytraced = raytraced && settings.is_raytracable() && !layer.is_immediate();
    // GlLayerList::renderLayer sets the layer polygon offset before its structures.
    let context = std::sync::Arc::new(crate::RenderContextState {
        polygon_offset: settings.polygon_offset(),
        // GlLayerList::renderLayer: SetLocalOrigin(aLayerSettings.Origin).
        local_origin: crate::layer_origin::LayerOrigin::of(&settings)
            .map_or([0.; 3], |o| o.coords()),
        ..context.clone()
    });
    let inputs = StructureInputs {
        camera,
        view_id,
        extent,
        rendering,
        clipping,
    };
    // GlLayerList::renderLayer clears the workspace environment for layers without it.
    let environment = settings
        .use_environment_texture()
        .then(|| environment.clone());
    // The layer origin moves into the world view (GlLayerList::renderLayer), as the float
    // clip chain needs for objects far from the world origin (v3d/viewcube/largedist).
    let opaque = RenderElementPass::opaque(settings.to_enable_depth_write())
        .with_environment(environment)
        .with_layer_origin(crate::layer_origin::LayerOrigin::of(&settings))
        .with_skip_trsf_persistence(skip_trsf_persistence)
        .with_skip_raytraced(raytraced)
        .with_context_state(context);
    let mut program = layer_program(layer, &inputs, &opaque)?;
    if opaque.needs_transparent_pass() {
        // GlLayerList::renderTransparent revisits layers whose opaque pass skipped elements.
        program.append(layer_program(layer, &inputs, &opaque.transparent())?);
    }
    let offset = settings.polygon_offset();
    let mut pass = RenderLayer::from_stencil_program(
        program,
        settings.to_enable_depth_test(),
        settings.to_enable_depth_write(),
        settings.to_clear_depth(),
    )
    .with_depth_prepass_membership(settings.to_render_in_depth_prepass())
    .with_local_origin(opaque.layer_origin())
    .with_polygon_offset(crate::RenderPolygonOffset::new(
        offset.mode.0,
        offset.factor,
        offset.units,
    )?);
    pass.raytraced = raytraced;
    Ok(pass)
}
/// The environment texture, shadow-pass persistence filter, ray-traced flag and context state
/// of a layer pass.
type LayerPassFrame<'a> = (
    &'a Handle<Graphic3dTextureEnv>,
    bool,
    bool,
    &'a crate::RenderContextState,
);
struct StructureInputs<'a> {
    camera: &'a Graphic3dCamera,
    view_id: i32,
    extent: [u32; 2],
    rendering: &'a Graphic3dRenderingParams,
    clipping: &'a Handle<Graphic3dSequenceOfHClipPlane>,
}
fn layer_program(
    layer: &Graphic3dLayer,
    inputs: &StructureInputs<'_>,
    pass: &RenderElementPass,
) -> Result<crate::stencil_program::StencilProgram, RenderError> {
    let mut program = crate::stencil_program::StencilProgram::default();
    for priority in layer.array_of_structures() {
        for weak in priority.iter() {
            program.append(structure_draws(&weak.upgrade(), inputs, pass)?);
        }
    }
    Ok(program)
}
fn structure_draws(
    structure: &Handle<dyn Graphic3dCStructure>,
    inputs: &StructureInputs<'_>,
    pass: &RenderElementPass,
) -> Result<crate::stencil_program::StencilProgram, RenderError> {
    if structure.is_null() {
        return Ok(Default::default());
    }
    let retained = structure.cast::<Graphic3dRenderStructure>();
    retained
        .get()
        .ok_or_else(|| rejected(structure.object_address(), "a Martensite driver structure"))?
        .draws(
            inputs.camera,
            inputs.view_id,
            inputs.extent,
            inputs.rendering,
            inputs.clipping,
            pass,
        )
}
fn validate_raster_params(view: &dyn Graphic3dCView) -> Result<(), RenderError> {
    let params = view.rendering_params();
    // Ray tracing renders the ray-traced layers itself (crate::raytrace), the rest rasterizes.
    if !matches!(
        params.method,
        Graphic3dRenderingMode::Rasterization | Graphic3dRenderingMode::Raytracing
    ) {
        return Err(rejected(
            (params.method, params.transparency_method),
            "rasterization or ray tracing",
        ));
    }
    Ok(())
}
fn validate_view(view: &dyn Graphic3dCView) -> Result<(), RenderError> {
    validate_raster_params(view)?;
    // Computed (hidden-line) mode needs nothing from the renderer: Graphic3dCView swaps the
    // displayed structures for their computed ones, as GlView draws whatever is displayed.
    // A stereo camera reaches here per eye (driver_view_stereo.rs); elsewhere it draws mono.
    Ok(())
}

fn apply_view_culling(layers: &mut [RenderLayer], mode: Graphic3dTypeOfBackfacingModel) {
    if mode == Graphic3dTypeOfBackfacingModel::Auto {
        return;
    }
    for draw in layers.iter_mut().flat_map(|layer| &mut layer.draws) {
        draw.face_culling = crate::face_culling::resolve_culling(mode, false);
    }
}

#[cfg(test)]
#[path = "ffp_texcoord_tests.rs"]
mod ffp_texcoord_tests;

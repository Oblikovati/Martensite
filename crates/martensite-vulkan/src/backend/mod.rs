//! Only this private boundary uses Ash or Vulkan handles.
mod alpha_write_pipelines;
mod depth_bias_device;
mod depth_bias_record;
mod depth_equivalence;
mod device_lifecycle;
mod device_limits_query;
mod device_retirement;
mod diagnostic_facts;
mod factorized_line_upload;
mod image_memory;
mod layer_depth_policy;
mod line_clip_device;
mod line_clip_shader;
mod line_clip_stage;
mod line_source_upload;
mod owned_frame_submission;
mod owned_window_submission;
mod portability_admission;
mod portability_runtime;
mod raytrace_adaptive;
mod raytrace_adaptive_record;
mod raytrace_environment;
mod raytrace_fsaa;
mod raytrace_fsaa_record;
#[cfg(feature = "raytrace-hit-diagnostics")]
mod raytrace_hit_control;
#[cfg(feature = "raytrace-hit-diagnostics")]
mod raytrace_hit_frame;
#[cfg(feature = "raytrace-hit-diagnostics")]
mod raytrace_hit_records;
#[cfg(feature = "raytrace-hit-diagnostics")]
mod raytrace_hit_scene;
#[cfg(feature = "raytrace-hit-diagnostics")]
mod raytrace_hit_scope;
mod raytrace_image;
#[cfg(feature = "raytrace-input-diagnostics")]
mod raytrace_input_diagnostic;
mod raytrace_pass;
mod raytrace_pathtrace;
mod raytrace_pipeline;
mod raytrace_resources;
mod renderer_close;
mod retained_submission;
mod retirement_attempt;
mod runtime;
mod scene_color_scope;
mod shader_grid_pass;
mod submission_calls;
mod submission_protocol;
mod surface_platform;
mod user_program_draw;
mod user_program_pipeline;
mod user_program_textures;
mod validation;
mod validation_layer;
mod world_clip_device;
mod world_clip_grid12_shader;
mod world_clip_interpolation_upload;
mod world_clip_shader;
use crate::{FrameCapture, RenderBackend, RenderError, RenderFrame};
pub use runtime::VulkanOptions;
use runtime::VulkanRuntime;
pub use validation_layer::{
    VERIFIED_VALIDATION_LAYER, ValidationLayerVersion, active_validation_layer,
};
/// Vulkan 1.3 offscreen renderer; `VulkanRenderer::new(VulkanOptions::default())?`.
pub struct VulkanRenderer {
    runtime: std::sync::Arc<VulkanRuntime>,
    retirements: retained_submission::RetirementStore,
    pipeline: pipeline_variants::RasterPipelines,
    capping_pipeline: Option<pipeline_variants::RasterPipelines>,
    fragment_alpha: crate::legacy_fragment_alpha::LegacyFragmentAlpha,
    stencil: crate::RenderStencilState,
    grid_pipelines: shader_grid_pass::GridPipelineCache,
    raytrace_pipelines: raytrace_pass::RaytraceCaches,
    environments: view_environments::ViewEnvironments,
}
impl VulkanRenderer {
    /// Load Vulkan and select a capable graphics device; `VulkanRenderer::new(options)?`.
    pub fn new(options: VulkanOptions) -> Result<Self, RenderError> {
        let runtime = VulkanRuntime::create(options)?;
        Self::initialized_renderer(runtime)
    }
    /// Require explicit raster admission; `Self::new_with_raster_policy(options, policy)?`.
    pub fn new_with_raster_policy(
        options: VulkanOptions,
        policy: crate::RenderRasterPolicy,
    ) -> Result<Self, RenderError> {
        Self::new_with_conventions(options, policy, Default::default())
    }
    /// Compose attachment mapping with raster behavior; `Self::new_with_conventions(options, raster, coordinates)?`.
    pub fn new_with_conventions(
        options: VulkanOptions,
        raster: crate::RenderRasterPolicy,
        coordinates: crate::RenderCoordinateConvention,
    ) -> Result<Self, RenderError> {
        let runtime = VulkanRuntime::create_with_conventions(options, raster, coordinates)?;
        Self::initialized_renderer(runtime)
    }
    /// Select an explicit versioned line-clipping experiment without changing raster policy.
    /// Example: `Self::new_with_line_clipping(options, raster, coordinates, RenderLineClippingBehavior::PlatformNativeV2)?`.
    pub fn new_with_line_clipping(
        options: VulkanOptions,
        raster: crate::RenderRasterPolicy,
        coordinates: crate::RenderCoordinateConvention,
        behavior: crate::RenderLineClippingBehavior,
    ) -> Result<Self, RenderError> {
        let runtime =
            VulkanRuntime::create_with_line_clipping(options, raster, coordinates, behavior)?;
        Self::initialized_renderer(runtime)
    }
    /// Read the immutable replay selection; `renderer.line_clipping_behavior()`.
    pub fn line_clipping_behavior(&self) -> crate::RenderLineClippingBehavior {
        self.runtime.line_clipping
    }
    /// Read the immutable raster-stage world-clipping selection.
    /// Example: `renderer.world_clipping_behavior()` records replay version 1, 2, or 3.
    pub fn world_clipping_behavior(&self) -> crate::RenderWorldClippingBehavior {
        self.runtime.world_clipping
    }
    /// Record the fragment world-clip interpolation contract; `renderer.world_clip_interpolation_behavior()` returns `HardwareV1` by default.
    pub fn world_clip_interpolation_behavior(&self) -> crate::RenderWorldClipInterpolationBehavior {
        self.runtime.world_clip_interpolation
    }
    /// Test control without mirrored sample positions; `Self::new_with_standard_samples(options)?`.
    #[cfg(test)]
    pub(crate) fn new_with_standard_samples(options: VulkanOptions) -> Result<Self, RenderError> {
        let runtime = VulkanRuntime::create_with_standard_samples(options)?;
        Self::initialized_renderer(runtime)
    }
    /// Read the immutable mapping; both versions return logical top-left captures.
    /// Example: `renderer.coordinate_convention()` records the selected behavior.
    pub fn coordinate_convention(&self) -> crate::RenderCoordinateConvention {
        // Frame resources and presentation read this same runtime-owned value.
        self.runtime.attachment_coordinates
    }
    /// Inspect the immutable admitted policy; `renderer.raster_policy()` records replay behavior.
    pub fn raster_policy(&self) -> crate::RenderRasterPolicy {
        // Replay inspection must not re-run admission against changing device availability.
        self.runtime.raster_policy
    }
    fn initialized_renderer(runtime: std::sync::Arc<VulkanRuntime>) -> Result<Self, RenderError> {
        // Formats and topology are fixed; viewport and layer depth policies are dynamic.
        let pipeline = pipeline_variants::RasterPipelines::new(&runtime, false)?;
        Ok(Self {
            runtime,
            retirements: retained_submission::RetirementStore::new()?,
            pipeline,
            capping_pipeline: None,
            fragment_alpha: Default::default(),
            stencil: Default::default(),
            grid_pipelines: Default::default(),
            raytrace_pipelines: Default::default(),
            environments: Default::default(),
        })
    }
    pub(crate) fn capture_limit(&self) -> u32 {
        let limits = &self.runtime.limits;
        limits
            .max_framebuffer_width
            .min(limits.max_framebuffer_height)
            .min(limits.max_image_dimension2_d)
    }
    /// Inspect the selected device name; `renderer.device_name()`.
    pub fn device_name(&self) -> &str {
        &self.runtime.name
    }
    /// The depth a capture reports for a window depth, after the attachment's storage
    /// (24-bit normalized where the device offers it, as OpenGL's DEPTH24_STENCIL8; float
    /// otherwise); `renderer.stored_depth(0.5)`.
    pub fn stored_depth(&self, window_depth: f32) -> f32 {
        depth_equivalence::stored_depth(self.runtime.depth.formats.plain, window_depth)
    }
    /// Read exact selected Vulkan implementation provenance; `renderer.driver_identity()`.
    pub fn driver_identity(&self) -> crate::RenderDriverIdentity {
        driver_identity::query_driver_identity(&self.runtime.instance, self.runtime.physical)
    }
    /// Testing only: the raw `VkInstance` handle; `renderer.raw_instance()`.
    pub fn raw_instance(&self) -> u64 {
        ash::vk::Handle::as_raw(self.runtime.instance.handle())
    }
    /// Require complete validation coverage; `renderer.validation_errors()?` refuses disabled or poisoned observations.
    pub fn validation_errors(
        &self,
    ) -> Result<Vec<String>, crate::RenderValidationObservationFailure> {
        self.runtime.validation.errors()
    }
}
mod attachment_coordinates;
mod attachment_orientation;
mod buffer;
#[cfg(feature = "command-state-diagnostics")]
mod command_state_diagnostic;
mod driver_identity;
mod factorized_projection;
mod fixed_lighting_pipeline;
mod fixed_vertex_layout;
mod fixed_vertex_sources;
mod frame;
mod frame_draw_presentation;
#[cfg(feature = "command-state-diagnostics")]
mod geometry_command_diagnostic;
mod image;
mod light_dot_renderer;
mod light_dot_shader;
mod light_dot_shader_table;
#[cfg(feature = "lit-only-command-diagnostics")]
pub(crate) mod lit_only_commands;
mod memory_arena;
mod memory_selection;
mod pipeline;
mod pipeline_shader_sources;
mod pipeline_shader_stages;
mod pipeline_state;
mod pipeline_variants;
mod record;
mod record_draw_bindings;
#[cfg(feature = "command-state-diagnostics")]
mod replay_attachment_diagnostic;
#[cfg(feature = "command-state-diagnostics")]
mod replay_command_diagnostic;
#[cfg(feature = "command-state-diagnostics")]
mod replay_device_diagnostic;
#[cfg(any(
    feature = "vertex-output-diagnostics",
    feature = "command-state-diagnostics"
))]
mod replay_pipeline_diagnostic;
#[cfg(any(
    feature = "vertex-output-diagnostics",
    feature = "command-state-diagnostics"
))]
mod replay_pipeline_state;
#[cfg(feature = "command-state-diagnostics")]
mod replay_raster_commands;
#[cfg(feature = "command-state-diagnostics")]
mod replay_resource_diagnostic;
mod submission;
mod transform_constants;
mod validation_settings;
#[cfg(feature = "vertex-output-diagnostics")]
mod vertex_output;
mod window_depth_completion;
mod window_depth_device;
mod window_depth_fragment;
mod window_depth_parameters;
mod window_depth_pipeline;
mod window_depth_record;
mod window_depth_shader;
mod window_depth_status;
mod window_depth_status_protocol;
mod window_depth_status_state;
mod window_depth_transforms;
mod window_depth_upload;
impl RenderBackend for VulkanRenderer {
    fn render(&mut self, frame: &RenderFrame) -> Result<FrameCapture, RenderError> {
        self.runtime.lifecycle.admit("render frame")?;
        let outcome = self.render_owned_frame(frame);
        self.runtime.lifecycle.observe(outcome)
    }
}
impl VulkanRenderer {
    fn render_owned_frame(&mut self, frame: &RenderFrame) -> Result<FrameCapture, RenderError> {
        let resources = self.completed_owned_frame(frame)?;
        resources.capture()
    }
    /// Inspect persistent admission; `renderer.device_state()` never polls execution progress.
    pub fn device_state(&self) -> crate::RenderDeviceState {
        // A successful unrelated operation cannot reset a device-lost observation.
        self.runtime.lifecycle.state()
    }
}
mod draw_state;
mod glyph_atlas_image;
mod gui_native_extent;
mod line_device;
mod point_sprite;
mod subview_upload;
mod window_blit;
mod window_chain;
mod window_fence_retirement;
mod window_gui_contract;
mod window_gui_render;
mod window_submission;
mod window_support;
mod window_surface;
mod window_surface_lease;
mod window_sync;
mod window_sync_retirement;
pub(crate) use window_surface::WindowPresenter;

impl VulkanRenderer {
    pub(crate) fn window_presenter(
        &self,
        window: &martensite_tkernel::standard::Handle<
            dyn martensite_tkservice::aspect::AspectWindow,
        >,
    ) -> Result<WindowPresenter, RenderError> {
        self.runtime.lifecycle.admit("create window presenter")?;
        self.runtime
            .lifecycle
            .observe(WindowPresenter::new(&self.runtime, window))
    }
}
mod capping_state;
mod storage_descriptor;
impl VulkanRenderer {
    fn commit_fragment_alpha(
        &mut self,
        frame: &RenderFrame,
        ending: crate::legacy_fragment_alpha::LegacyFragmentAlpha,
    ) {
        if let Some(snapshot) = &frame.fragment_alpha_context {
            snapshot.commit(ending);
            return;
        }
        self.fragment_alpha = ending;
    }
    fn commit_stencil(&mut self, frame: &RenderFrame, ending: crate::RenderStencilState) {
        if let Some(snapshot) = &frame.stencil_context {
            snapshot.commit(ending);
            return;
        }
        self.stencil = ending;
    }
    fn frame_resources(
        &mut self,
        frame: &RenderFrame,
    ) -> Result<frame::FrameResources, RenderError> {
        let initial_stencil = frame.initial_stencil(self.stencil);
        let capped = needs_stencil_pipeline(frame, initial_stencil);
        self.prepare_capping_pipeline(capped)?;
        let environment = self.environments.prepare(&self.runtime, frame)?;
        let pipeline = self.frame_pipeline(capped);
        pipeline.use_frame_pbr_environment(environment.pbr.clone())?;
        let shadow_maps = frame
            .shadow_maps
            .as_deref()
            .map(|maps| shadow_maps::ShadowMapSet::new(&self.runtime, Some(maps)))
            .transpose()?;
        pipeline.use_frame_shadow_maps(shadow_maps)?;
        let created = frame::FrameResources::new(
            &self.runtime,
            frame,
            pipeline,
            frame.initial_fragment_alpha(self.fragment_alpha),
            initial_stencil,
            &self.retirements,
        );
        pipeline.use_frame_pbr_environment(None)?;
        pipeline.use_frame_shadow_maps(None)?;
        let mut resources = created?;
        let raytrace_cube = environment
            .background
            .as_ref()
            .map(|(cube, _)| cube.clone())
            .or_else(|| environment.raytrace_cube.clone());
        resources.background = environment
            .background
            .map(|background| {
                self.environments
                    .background_draw(&self.runtime, &resources, background)
                    .map(frame::FrameBackgroundDraw::Cube)
            })
            .transpose()?;
        if let Some(quad) = frame
            .colored_quad
            .as_ref()
            .filter(|_| resources.background.is_none())
        {
            let draw = self
                .environments
                .colored_quad_draw(&self.runtime, &resources, quad)?;
            resources.background = Some(frame::FrameBackgroundDraw::ColoredQuad(draw));
        }
        if let Some(pair) = frame.stereo_pair.as_ref() {
            let draw = self.environments.stereo_pair_draw(
                &self.runtime,
                &resources,
                pair,
                &self.retirements,
            )?;
            resources.background = Some(frame::FrameBackgroundDraw::StereoPair(draw));
        }
        resources.grid =
            shader_grid_pass::prepare_grid(&self.runtime, &self.grid_pipelines, frame, &resources)?;
        resources.raytrace = raytrace_pass::prepare_raytrace(
            &self.runtime,
            &self.raytrace_pipelines,
            frame,
            (
                &resources,
                raytrace_pass::RaytraceImageSources {
                    pipelines: &self.pipeline,
                    retirements: &self.retirements,
                    cube: raytrace_cube,
                },
            ),
        )?;
        self.runtime.image_memory.sample_frame();
        Ok(resources)
    }
    /// Image bytes for frame statistics; `renderer.image_memory().sampled_image_bytes`.
    pub fn image_memory(&self) -> crate::RenderImageMemory {
        let ledger = &self.runtime.image_memory;
        crate::RenderImageMemory {
            frame_attachment_bytes: ledger.frame_target_bytes() as usize,
            sampled_image_bytes: ledger.sampled_bytes() as usize,
        }
    }
}

fn needs_stencil_pipeline(frame: &RenderFrame, initial_stencil: crate::RenderStencilState) -> bool {
    frame.stencil_clear_value != 0
        || initial_stencil.is_enabled()
        || frame
            .layers
            .iter()
            .any(|layer| !layer.stencil_boundaries.is_empty())
        || frame
            .draws
            .iter()
            .any(|draw| draw.capping_pass != crate::RenderCappingPass::None)
}
impl VulkanRenderer {
    fn prepare_capping_pipeline(&mut self, capped: bool) -> Result<(), RenderError> {
        if capped && self.capping_pipeline.is_none() {
            self.capping_pipeline = Some(pipeline_variants::RasterPipelines::new(
                &self.runtime,
                true,
            )?);
        }
        Ok(())
    }
    fn frame_pipeline(&self, capped: bool) -> &pipeline_variants::RasterPipelines {
        if capped {
            self.capping_pipeline
                .as_ref()
                .expect("created stencil pipelines")
        } else {
            &self.pipeline
        }
    }
}
mod capping_record;
mod colored_quad_pass;
mod coordinate_shader;
mod coordinate_shader_table;
mod cube_seams;
mod device_configuration;
mod environment_cube;
mod environment_pipeline;
mod fragment_alpha_plan;
mod fragment_alpha_upload;
mod fragment_variant_registry;
mod frame_raster_plan;
mod image_float_atomics;
mod material_map_descriptor;
mod material_map_pipeline;
mod memory_query;
mod mesh_edge_pipeline;
mod mirrored_sample_locations;
mod multisample_targets;
mod native_alpha_shader;
mod native_alpha_shader_table;
mod oit_barriers;
mod oit_blend;
mod oit_composite;
mod oit_descriptor;
mod oit_frame;
mod oit_pipeline;
mod oit_pipeline_cache;
mod oit_plan;
mod oit_record;
mod oit_render_commands;
mod oit_shader_table;
mod oit_support;
mod oit_targets;
mod oit_upload;
mod ordered_stencil_record;
mod pbr_environment;
mod pbr_pipeline;
mod pipeline_layout;
mod raster_limits;
mod raster_pipeline_policy;
mod raster_policy_device;
mod raster_samples;
mod raytrace_scene_images;
mod raytrace_texture_indexing;
mod render_target_record;
mod sampled_surface_image;
mod sampled_surface_upload;
mod shadow_maps;
mod stereo_pair_pass;
mod surface_image_cache;
mod surface_image_descriptor;
mod surface_image_format;
mod surface_sampler;
mod surface_texture_pipeline;
mod view_environments;

#[cfg(test)]
pub(crate) mod diagnostic_executable;
mod fixed_grid_pipeline;
mod fixed_grid_record;
mod fixed_grid_shader;
mod fixed_grid_sprite;
mod fixed_grid_sprite_shader;
mod fixed_grid_upload;
mod fixed_grid_viewport;
mod phong_interpolation_configuration;
mod phong_interpolation_device;
mod phong_interpolation_math;
mod phong_interpolation_record;
mod phong_interpolation_shader;
mod phong_interpolation_upload;
mod phong_projected_axis;
mod phong_projected_fill;
mod point_raster_configuration;
mod text_flat_corners;
mod text_flat_corners_layout;
mod text_grid_interpolation_device;
mod text_grid_interpolation_math;
mod text_grid_interpolation_shader;
mod text_grid_interpolation_upload;
mod thin_axis_configuration;
mod thin_axis_upload;
mod triangle_grid_coverage_device;
mod triangle_grid_coverage_shader;
mod triangle_grid_coverage_upload;
mod triangle_grid_projection_intervals;
mod v4_depth_admission;
mod v4_geometry_transport;

#[cfg(any(
    feature = "vertex-output-diagnostics",
    feature = "command-state-diagnostics"
))]
mod vertex_pipeline_diagnostic;
#[cfg(test)]
include!("test_modules.rs");

#[cfg(test)]
mod text_depth_composition_gpu_tests;

mod clipped_depth_device;
mod clipped_depth_uniforms;

mod clipped_depth_upload;

mod clipped_depth_shader;

#[cfg(test)]
mod clipped_depth_gpu_tests;

mod clipped_depth_composition;

#[cfg(feature = "command-state-diagnostics")]
mod clipped_depth_incidence;

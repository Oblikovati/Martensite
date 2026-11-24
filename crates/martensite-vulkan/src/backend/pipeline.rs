//! The initial unlit pipeline consumes real vertex/index buffers and a per-draw transform.
#[cfg(test)]
use super::diagnostic_executable::{ExecutableGap, ExecutableModuleInput};
#[cfg(test)]
use super::pipeline_layout::descriptor_layout_handles;
use super::pipeline_layout::{create_layout, fixed_marker_push_range, push_range};
use super::pipeline_shader_sources::lighting_shaders;
pub(super) use super::raster_pipeline_policy::create_pipeline;
use super::runtime::{VulkanRuntime, vk_error};
use crate::{RenderError, error::rejected};
use ash::vk;
use std::sync::Arc;
#[path = "pipeline_creation.rs"]
mod creation;
#[path = "pipeline_draw_table.rs"]
mod draw_table;
#[cfg(test)]
pub(super) use draw_table::indexed_storage_limits;
#[path = "pipeline_stage_selection.rs"]
mod stage_selection;
#[path = "pipeline_stock_layout.rs"]
mod stock_layout;
pub(super) use draw_table::DrawTablePipelines;
use stock_layout::{lighting_layout, stock_base_vertex_stride};
#[path = "pipeline_light_dot.rs"]
mod light_dot;
#[path = "pipeline_mesh_vertex_twins.rs"]
mod mesh_vertex_twins;
#[path = "pipeline_v4_geometry.rs"]
mod v4_geometry;
#[path = "pipeline_vertex_output.rs"]
mod vertex_output;
#[path = "pipeline_window_depth.rs"]
mod window_depth;
pub(super) use mesh_vertex_twins::MeshVertexTwins;

pub(super) struct DrawPipeline {
    pub phong_interpolation_shader: bool,
    pub triangle_grid_coverage_shader: bool,
    pub world_clip_grid12_shader: bool,
    pub text_grid_interpolation_shader: bool,
    pub(super) stock_base_vertex_stride: Option<u64>,
    #[cfg(feature = "vertex-output-diagnostics")]
    pub vertex_output_shader: bool,
    runtime: Arc<VulkanRuntime>,
    pub handle: vk::Pipeline,
    pub layout: vk::PipelineLayout,
    pub push_stages: vk::ShaderStageFlags,
    pub sprite_set: u32,
    pub kind: super::pipeline_variants::PipelineVariant,
    pub oit_layout: Option<Arc<super::oit_descriptor::OitDescriptorLayout>>,
    shaders: Vec<vk::ShaderModule>,
    /// The bytes of the vertex stage, after every substitution.
    vertex_bytes: &'static [u8],
    fragment_bytes: &'static [u8],
    /// The sample count and alpha coverage of an ordinary single-pass pipeline, from which its
    /// draw-table twin is built again; `None` for OIT, native-alpha and directional pipelines.
    table_inputs: Option<(vk::SampleCountFlags, bool)>,
    /// The draw table of an indirect run, at set 7 (pipeline_draw_table.rs).
    pub draw_table_layout: Option<super::storage_descriptor::StorageDescriptorLayout>,
    /// `has_plain_lit_stages`, once it was asked after the pipeline was built.
    plain_lit_stages: std::sync::OnceLock<bool>,
    #[cfg(test)]
    executable_bytes: Vec<&'static [u8]>,
    pub clipping_layout: Option<super::storage_descriptor::StorageDescriptorLayout>,
    pub lighting_layout: Option<super::storage_descriptor::StorageDescriptorLayout>,
    pub sprite_layout: Option<super::storage_descriptor::StorageDescriptorLayout>,
    /// The SphereMapMatrices block of the environment-texture vertex stage, at set 2.
    pub sphere_layout: Option<super::storage_descriptor::StorageDescriptorLayout>,
    pub surface_layout: Option<super::surface_image_descriptor::SurfaceImageLayout>,
    pub pbr_layout: Option<super::pbr_environment::PbrEnvironmentLayout>,
    pub material_map_layout: Option<super::material_map_descriptor::MaterialMapLayout>,
    /// The shadow maps of the lit fragments, at set 6.
    pub window_depth_status_layout: Option<super::storage_descriptor::StorageDescriptorLayout>,
    pub shadow_layout: Option<super::shadow_maps::ShadowMapLayout>,
}
impl DrawPipeline {
    pub(super) fn meshlet_fragment_module(&self) -> Option<vk::ShaderModule> {
        let directional = self.vertex_bytes == super::directional_phong::SHADERS[0]
            && self.fragment_bytes == super::directional_phong::SHADERS[1];
        (self.shaders.len() == 2 && (directional || self.has_plain_lit_stages()))
            .then(|| self.shaders[1])
    }

    #[cfg(feature = "lit-only-command-diagnostics")]
    pub(super) fn lit_only_fragment_bytes(&self) -> &'static [u8] {
        self.fragment_bytes
    }
    pub fn new(
        runtime: &Arc<VulkanRuntime>,
        kind: super::pipeline_variants::PipelineVariant,
    ) -> Result<Self, RenderError> {
        Self::build(
            runtime,
            kind,
            None,
            super::window_depth_shader::seed_samples(kind),
            false,
            None,
            false,
        )
    }
    pub fn new_oit(
        runtime: &Arc<VulkanRuntime>,
        kind: super::pipeline_variants::PipelineVariant,
        target: super::oit_pipeline::OitPipelineTarget,
        layout: &Arc<super::oit_descriptor::OitDescriptorLayout>,
    ) -> Result<Self, RenderError> {
        Self::build(
            runtime,
            kind,
            Some((target, layout.clone())),
            target.samples,
            false,
            None,
            false,
        )
    }
    pub fn new_multisample(
        runtime: &Arc<VulkanRuntime>,
        kind: super::pipeline_variants::PipelineVariant,
        samples: vk::SampleCountFlags,
        alpha_coverage: bool,
    ) -> Result<Self, RenderError> {
        Self::build(runtime, kind, None, samples, alpha_coverage, None, false)
    }
    pub fn new_native_alpha(
        runtime: &Arc<VulkanRuntime>,
        kind: super::pipeline_variants::PipelineVariant,
        samples: vk::SampleCountFlags,
        alpha_coverage: bool,
        layout: &Arc<super::oit_descriptor::OitDescriptorLayout>,
    ) -> Result<Self, RenderError> {
        Self::build(
            runtime,
            kind,
            None,
            samples,
            alpha_coverage,
            Some(layout.clone()),
            false,
        )
    }
    pub(super) fn new_directional_phong(
        runtime: &Arc<VulkanRuntime>,
        kind: super::pipeline_variants::PipelineVariant,
    ) -> Result<Self, RenderError> {
        Self::build(
            runtime,
            kind,
            None,
            vk::SampleCountFlags::TYPE_1,
            false,
            None,
            true,
        )
    }

    fn build(
        runtime: &Arc<VulkanRuntime>,
        kind: super::pipeline_variants::PipelineVariant,
        oit: Option<(
            super::oit_pipeline::OitPipelineTarget,
            Arc<super::oit_descriptor::OitDescriptorLayout>,
        )>,
        samples: vk::SampleCountFlags,
        alpha_coverage: bool,
        alpha_layout: Option<Arc<super::oit_descriptor::OitDescriptorLayout>>,
        directional_phong: bool,
    ) -> Result<Self, RenderError> {
        let super::pipeline_variants::PipelineVariant {
            clamp: _,
            line,
            point,
            text,
            text_blend: _,
            clipped,
            lighting,
            ..
        } = kind;
        super::window_depth_shader::admit_kind(kind, oit.is_some(), samples)?;
        let mut pipeline = Self {
            phong_interpolation_shader: false,
            triangle_grid_coverage_shader: false,
            world_clip_grid12_shader: false,
            text_grid_interpolation_shader: false,
            stock_base_vertex_stride: None,
            #[cfg(feature = "vertex-output-diagnostics")]
            vertex_output_shader: false,
            runtime: runtime.clone(),
            kind,
            oit_layout: oit
                .as_ref()
                .map(|(_, layout)| layout.clone())
                .or_else(|| alpha_layout.clone()),
            handle: vk::Pipeline::null(),
            layout: vk::PipelineLayout::null(),
            push_stages: push_range(
                line || kind.polygon_edges
                    || point
                    || text
                    || kind.surface_texture
                    || kind.surface_alpha,
                kind.mesh_edges,
            )
            .stage_flags,
            sprite_set: if lighting && (point || kind.surface_texture) {
                2
            } else {
                0
            },
            shaders: vec![],
            vertex_bytes: &[],
            fragment_bytes: &[],
            table_inputs: (oit.is_none() && alpha_layout.is_none() && !directional_phong)
                .then_some((samples, alpha_coverage)),
            draw_table_layout: None,
            plain_lit_stages: std::sync::OnceLock::new(),
            #[cfg(test)]
            executable_bytes: Vec::new(),
            clipping_layout: if clipped {
                Some(super::storage_descriptor::StorageDescriptorLayout::new(
                    runtime,
                    vk::ShaderStageFlags::VERTEX | vk::ShaderStageFlags::FRAGMENT,
                )?)
            } else {
                None
            },
            lighting_layout: lighting_layout(runtime, kind)?,
            // Glyph atlases bind as combined image samplers, like surface textures.
            surface_layout: if kind.surface_texture || text {
                Some(super::surface_image_descriptor::SurfaceImageLayout::new(
                    runtime,
                )?)
            } else {
                None
            },
            material_map_layout: if kind.material_maps {
                Some(super::material_map_descriptor::MaterialMapLayout::new(
                    runtime,
                )?)
            } else {
                None
            },
            window_depth_status_layout: None,
            shadow_layout: if lighting {
                Some(super::shadow_maps::ShadowMapLayout::new(runtime)?)
            } else {
                None
            },
            pbr_layout: if kind.pbr {
                Some(super::pbr_environment::PbrEnvironmentLayout::new(runtime)?)
            } else {
                None
            },
            sphere_layout: if kind.sphere_map {
                Some(super::storage_descriptor::StorageDescriptorLayout::new(
                    runtime,
                    vk::ShaderStageFlags::VERTEX,
                )?)
            } else {
                None
            },
            sprite_layout: if kind.uses_marker_sampler() {
                Some(super::storage_descriptor::StorageDescriptorLayout::new(
                    runtime,
                    vk::ShaderStageFlags::FRAGMENT,
                )?)
            } else {
                None
            },
        };
        if kind.fixed_lighting {
            let shaders = if kind.mesh_edges {
                super::fixed_lighting_pipeline::fixed_mesh_triple(kind).to_vec()
            } else {
                super::fixed_lighting_pipeline::fixed_lighting_pair(kind).to_vec()
            };
            for bytes in shaders {
                pipeline.add_shader(bytes)?;
            }
        } else if kind.fixed_line_stipple && !kind.lighting {
            for bytes in super::fixed_lighting_pipeline::fixed_unlit_line_pair(kind.clipped) {
                pipeline.add_shader(bytes)?;
            }
        } else if kind.point && kind.encoded_surface && !kind.lighting {
            for bytes in super::fixed_lighting_pipeline::fixed_unlit_point_pair(kind.clipped) {
                pipeline.add_shader(bytes)?;
            }
        } else if kind.mesh_edges {
            for bytes in super::mesh_edge_pipeline::edge_shader_bytes(kind) {
                pipeline.add_shader(bytes)?;
            }
        } else if kind.surface_texture || kind.surface_alpha {
            for bytes in super::surface_texture_pipeline::surface_shaders(kind) {
                pipeline.add_shader(bytes)?;
            }
        } else if kind.compact_vertices {
            pipeline.load_compact_mesh_shaders(directional_phong)?;
        } else if directional_phong {
            for bytes in super::directional_phong::SHADERS {
                pipeline.add_shader(bytes)?;
            }
        } else {
            pipeline.load_shaders(line || kind.polygon_edges, point, text, clipped, lighting)?;
        }
        if kind.pbr && oit.is_none() {
            pipeline.replace_fragment(super::pbr_pipeline::pbr_fragment(kind))?;
        }
        if kind.material_maps && oit.is_none() {
            let bytes = super::material_map_pipeline::material_map_fragment(kind);
            pipeline.replace_fragment(bytes)?;
        }
        if let Some((target, _)) = &oit {
            pipeline.replace_oit_fragment(*target)?;
        }
        if alpha_layout.is_some() {
            pipeline.replace_fragment(super::native_alpha_shader::native_alpha_fragment(
                pipeline.fragment_bytes,
            )?)?;
        }
        if !directional_phong {
            pipeline.apply_coordinate_fragment()?;
        }
        pipeline.apply_light_dot_behavior()?;
        pipeline.apply_fixed_reciprocal_fragment()?;
        pipeline.apply_sprite_fragment()?;
        pipeline.apply_window_depth_geometry()?;
        pipeline.add_v4_geometry_transport()?;
        pipeline.apply_draw_table()?;
        let mut transform_push = super::factorized_projection::push_range(
            runtime.projection_behavior,
            kind,
            super::fixed_grid_pipeline::fixed_grid_push_range(
                fixed_marker_push_range(
                    line || kind.polygon_edges
                        || point
                        || text
                        || kind.surface_texture
                        || kind.surface_alpha,
                    kind.mesh_edges,
                    point && kind.encoded_surface,
                ),
                kind.fixed_grid_point,
            ),
        );
        if kind.line_tint.uses_draw_color() {
            transform_push.size = 96;
        }
        if kind.compact_vertices {
            transform_push.size += super::mesh_vertex_codec::DECODE_PUSH_BYTES;
        }
        pipeline.layout = create_layout(
            runtime,
            transform_push,
            pipeline
                .sprite_layout
                .as_ref()
                .map(|v| v.handle)
                .or_else(|| pipeline.surface_layout.as_ref().map(|v| v.handle)),
            pipeline.lighting_layout.as_ref(),
            pipeline.phong_interpolation_shader,
            pipeline.triangle_grid_coverage_shader,
            super::fixed_grid_record::logical_facing_active(
                runtime.triangle_coverage,
                pipeline.triangle_grid_coverage_shader,
            ),
            pipeline.text_grid_interpolation_shader,
            [
                pipeline.window_depth_status_layout.as_ref(),
                pipeline.draw_table_layout.as_ref(),
            ],
            pipeline.clipping_layout.as_ref(),
            [
                pipeline.sphere_layout.as_ref().map(|layout| layout.handle),
                pipeline.oit_layout.as_ref().map(|layout| layout.handle),
            ],
            [
                pipeline.pbr_layout.as_ref().map(|layout| layout.handle),
                pipeline
                    .material_map_layout
                    .as_ref()
                    .map(|layout| layout.handle),
                pipeline.shadow_layout.as_ref().map(|layout| layout.handle),
            ],
        )?;
        pipeline.create(
            kind,
            oit.as_ref().map(|(target, _)| *target),
            samples,
            alpha_coverage,
        )?;
        Ok(pipeline)
    }
    #[cfg(feature = "command-state-diagnostics")]
    pub(super) fn diagnostic_modules(&self) -> Vec<u64> {
        use ash::vk::Handle;
        self.shaders.iter().map(|shader| shader.as_raw()).collect()
    }
    fn add_shader(&mut self, bytes: &'static [u8]) -> Result<(), RenderError> {
        let bytes = self.selected_builtin_vertex(bytes)?;
        let actual = self.capture_shader_source(bytes)?;
        if self.shaders.is_empty() {
            self.stock_base_vertex_stride = stock_base_vertex_stride(self.kind, actual);
            self.vertex_bytes = actual;
        }
        if self.shaders.len() == 1 {
            self.fragment_bytes = bytes;
        }
        self.shaders.push(load_shader(&self.runtime, actual)?);
        #[cfg(test)]
        if self.runtime.executable_profile.is_some() {
            self.executable_bytes.push(actual);
        }
        Ok(())
    }
    fn replace_fragment(&mut self, bytes: &'static [u8]) -> Result<(), RenderError> {
        let actual = self.capture_shader_source(bytes)?;
        let replacement = load_shader(&self.runtime, actual)?;
        // SAFETY: the handle belongs to this device and its owner releases it only after the GPU work that used it has completed.
        unsafe {
            self.runtime
                .device()
                .destroy_shader_module(self.shaders[1], None);
        }
        self.shaders[1] = replacement;
        self.fragment_bytes = bytes;
        #[cfg(test)]
        if !self.executable_bytes.is_empty() {
            self.executable_bytes[1] = actual;
        }
        Ok(())
    }
    fn create(
        &mut self,
        kind: super::pipeline_variants::PipelineVariant,
        oit: Option<super::oit_pipeline::OitPipelineTarget>,
        samples: vk::SampleCountFlags,
        alpha_coverage: bool,
    ) -> Result<(), RenderError> {
        let request = creation::PipelineCreationRequest {
            kind,
            oit,
            samples,
            alpha_coverage,
        };
        self.handle = creation::create(self, request)?;
        Ok(())
    }
}
impl Drop for DrawPipeline {
    fn drop(&mut self) {
        // SAFETY: the handle belongs to this device and its owner releases it only after the GPU work that used it has completed.
        unsafe {
            let device = self.runtime.device();
            if self.handle != vk::Pipeline::null() {
                device.destroy_pipeline(self.handle, None);
            }
            if self.layout != vk::PipelineLayout::null() {
                device.destroy_pipeline_layout(self.layout, None);
            }
            for shader in self.shaders.drain(..) {
                device.destroy_shader_module(shader, None);
            }
        }
    }
}
pub(super) fn load_shader(
    runtime: &VulkanRuntime,
    bytes: &[u8],
) -> Result<vk::ShaderModule, RenderError> {
    if !bytes.len().is_multiple_of(4) {
        return Err(rejected(bytes.len(), "SPIR-V words aligned to four bytes"));
    }
    let words: Vec<u32> = bytes
        .as_chunks::<4>()
        .0
        .iter()
        .map(|word| u32::from_le_bytes(*word))
        .collect();
    // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
    let shader = unsafe {
        runtime
            .device()
            .create_shader_module(&vk::ShaderModuleCreateInfo::default().code(&words), None)
    }
    .map_err(|e| vk_error("create shader module", e))?;
    #[cfg(feature = "command-state-diagnostics")]
    {
        super::geometry_command_diagnostic::shader_created(shader, bytes);
        super::replay_device_diagnostic::shader(runtime.device().handle(), shader);
    }
    Ok(shader)
}
#[cfg(test)]
pub(super) fn shader_stages(
    shaders: &[vk::ShaderModule],
) -> Vec<vk::PipelineShaderStageCreateInfo<'static>> {
    super::pipeline_shader_stages::shader_stages(shaders)
}
#[cfg(test)]
include!("pipeline_tests.rs");
#[cfg(test)]
include!("pipeline_executable_test_helpers.rs");

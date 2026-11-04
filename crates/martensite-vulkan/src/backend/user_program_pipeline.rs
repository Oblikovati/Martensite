//! Graphics pipelines of registered user programs (GlShaderProgram linked from the
//! Graphic3dShaderProgram stages): the program's SPIR-V stages with the fixed state of the face
//! draw's built-in variant, the native attribute locations (occVertex 0, occNormal 1,
//! occTexCoord 2, occVertColor 3) from one interleaved buffer, samplers in set 0 and the
//! default uniform block in set 1. Tessellated programs draw three-vertex patches.
use super::{
    oit_pipeline::{OitPipelineTarget, geometry_blend_attachments},
    pipeline::create_pipeline,
    pipeline_state::PipelineState,
    pipeline_variants::PipelineVariant,
    runtime::{VulkanRuntime, vk_error},
};
use crate::user_program::{UserProgramKey, UserProgramModules, UserStage};
use crate::{RenderError, error::rejected};
use ash::vk;
use std::sync::{Arc, Mutex};
#[cfg(all(test, target_os = "linux"))]
#[path = "owned_pipeline_capture.rs"]
mod owned_capture;
#[cfg(all(test, target_os = "linux"))]
pub(super) use owned_capture::OwnedPipelineCapture;

/// Bytes per vertex: occVertex, occNormal (padded), occTexCoord and occVertColor as vec4.
pub(super) const USER_VERTEX_STRIDE: u32 = 64;
/// The native attribute names by location.
const ATTRIBUTE_NAMES: [&str; 4] = ["occVertex", "occNormal", "occTexCoord", "occVertColor"];

/// One linked user program for one built-in variant's fixed state.
pub(super) struct UserProgramPipeline {
    runtime: Arc<VulkanRuntime>,
    pub handle: vk::Pipeline,
    pub layout: vk::PipelineLayout,
    /// Set 0 (samplers) and set 1 (default uniform block).
    pub set_layouts: [vk::DescriptorSetLayout; 2],
    /// The samplers of set 0 by ascending binding.
    pub samplers: Vec<UserSampler>,
    shaders: Vec<vk::ShaderModule>,
    #[cfg(all(test, target_os = "linux"))]
    pub owned_capture: Option<OwnedPipelineCapture>,
}

/// The fixed state a user program is linked with: the built-in variant of the face draw, its
/// address (the cache identity of that variant), the frame's sample count, whether alpha
/// to coverage applies, the OIT attachments the program draws into and whether its one colour
/// is written into each of them (native `gl_FragColor`, `broadcasts_frag_color`).
/// Example: `UserPipelineState::single(kind, variant)`.
#[derive(Clone, Copy)]
pub(super) struct UserPipelineState {
    pub kind: PipelineVariant,
    pub variant: usize,
    pub samples: vk::SampleCountFlags,
    pub alpha_coverage: bool,
    pub oit: Option<OitPipelineTarget>,
    pub broadcast: bool,
}
impl UserPipelineState {
    /// The single-sampled state of `kind`.
    pub fn single(kind: PipelineVariant, variant: usize) -> Self {
        Self {
            kind,
            variant,
            samples: vk::SampleCountFlags::TYPE_1,
            alpha_coverage: false,
            oit: None,
            broadcast: false,
        }
    }
    fn key(&self, program: UserProgramKey) -> UserPipelineKey {
        let oit = self
            .oit
            .map(|target| (target.formats.map(|format| format.as_raw()), target.count));
        let oit = oit.map(|(formats, count)| (formats, count, self.broadcast));
        (
            program,
            self.variant,
            self.samples.as_raw(),
            self.alpha_coverage,
            oit,
        )
    }
    /// The colour attachment formats and blends: the scene colour, or the OIT targets.
    fn attachments(
        &self,
    ) -> (
        Vec<vk::Format>,
        Option<Vec<vk::PipelineColorBlendAttachmentState>>,
    ) {
        match self.oit {
            None => (vec![vk::Format::R8G8B8A8_SRGB], None),
            Some(target) => (
                target.formats[..target.count].to_vec(),
                Some(geometry_blend_attachments(target, self.kind)),
            ),
        }
    }
}

impl UserProgramPipeline {
    fn new(
        runtime: &Arc<VulkanRuntime>,
        modules: &UserProgramModules,
        fixed: UserPipelineState,
    ) -> Result<Self, RenderError> {
        validate_device(runtime, modules)?;
        let mut pipeline = Self {
            runtime: runtime.clone(),
            handle: vk::Pipeline::null(),
            layout: vk::PipelineLayout::null(),
            set_layouts: [vk::DescriptorSetLayout::null(); 2],
            samplers: user_samplers(modules),
            shaders: Vec::new(),
            #[cfg(all(test, target_os = "linux"))]
            owned_capture: None,
        };
        for (reflection, words) in &modules.stages {
            let words = stage_words(reflection, words, fixed)?;
            pipeline.shaders.push(create_module(runtime, &words)?);
        }
        pipeline.create_layouts()?;
        pipeline.create(modules, fixed)?;
        Ok(pipeline)
    }
    fn create_layouts(&mut self) -> Result<(), RenderError> {
        let samplers: Vec<_> = self
            .samplers
            .iter()
            .map(|sampler| {
                layout_binding(sampler.binding, vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
            })
            .collect();
        let block = [layout_binding(0, vk::DescriptorType::UNIFORM_BUFFER)];
        self.set_layouts[0] = create_set_layout(&self.runtime, &samplers)?;
        self.set_layouts[1] = create_set_layout(&self.runtime, &block)?;
        let info = vk::PipelineLayoutCreateInfo::default().set_layouts(&self.set_layouts);
        // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
        self.layout = unsafe { self.runtime.device().create_pipeline_layout(&info, None) }
            .map_err(|e| vk_error("create user program layout", e))?;
        Ok(())
    }
    fn create(
        &mut self,
        modules: &UserProgramModules,
        fixed: UserPipelineState,
    ) -> Result<(), RenderError> {
        let kind = fixed.kind;
        // Constant 7 (kRemapClipDepth) of the last pre-rasterization stage: remap OpenGL clip
        // depth unless VK_EXT_depth_clip_control applies it (the recording tool).
        let remap = u32::from(!self.runtime.depth.clip_control).to_ne_bytes();
        let entries = [vk::SpecializationMapEntry::default()
            .constant_id(7)
            .offset(0)
            .size(4)];
        let specialization = vk::SpecializationInfo::default()
            .map_entries(&entries)
            .data(&remap);
        let stages: Vec<_> = stage_infos(modules, &self.shaders)
            .into_iter()
            .map(|stage| stage.specialization_info(&specialization))
            .collect();
        let inputs = vertex_attributes(modules)?;
        let bindings = [vk::VertexInputBindingDescription::default()
            .stride(USER_VERTEX_STRIDE)
            .input_rate(vk::VertexInputRate::VERTEX)];
        let vertex = vk::PipelineVertexInputStateCreateInfo::default()
            .vertex_binding_descriptions(&bindings)
            .vertex_attribute_descriptions(&inputs);
        let tessellated = modules.stage(UserStage::TessEvaluation).is_some();
        let assembly =
            vk::PipelineInputAssemblyStateCreateInfo::default().topology(if tessellated {
                vk::PrimitiveTopology::PATCH_LIST
            } else {
                vk::PrimitiveTopology::TRIANGLE_LIST
            });
        let tessellation =
            vk::PipelineTessellationStateCreateInfo::default().patch_control_points(3);
        let mut state = PipelineState::new(kind);
        state.set_multisampling(fixed.samples, fixed.alpha_coverage);
        let (formats, blends) = fixed.attachments();
        let blend = vk::PipelineColorBlendStateCreateInfo::default()
            .attachments(blends.as_deref().unwrap_or(&[]));
        let depth = kind.depth_format(self.runtime.depth.formats);
        let stencil = if kind.stencil {
            depth
        } else {
            vk::Format::UNDEFINED
        };
        let mut rendering = vk::PipelineRenderingCreateInfo::default()
            .color_attachment_formats(&formats)
            .depth_attachment_format(depth)
            .stencil_attachment_format(stencil);
        let mut info = state
            .description(self.layout, &stages)
            .vertex_input_state(&vertex)
            .input_assembly_state(&assembly)
            .push_next(&mut rendering);
        if tessellated {
            info = info.tessellation_state(&tessellation);
        }
        if blends.is_some() {
            info = info.color_blend_state(&blend);
        }
        self.handle = create_pipeline(&self.runtime, info, &state.raster_description())?;
        #[cfg(all(test, target_os = "linux"))]
        {
            self.owned_capture = Some(owned_capture::snapshot(
                &self.runtime,
                &info,
                remap,
                depth,
                stencil,
            ));
        }
        Ok(())
    }
}

impl Drop for UserProgramPipeline {
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
            for layout in self.set_layouts {
                if layout != vk::DescriptorSetLayout::null() {
                    device.destroy_descriptor_set_layout(layout, None);
                }
            }
            for shader in self.shaders.drain(..) {
                device.destroy_shader_module(shader, None);
            }
        }
    }
}

/// Tessellation and geometry stages need their device features.
fn validate_device(
    runtime: &VulkanRuntime,
    modules: &UserProgramModules,
) -> Result<(), RenderError> {
    let tessellated = modules.stage(UserStage::TessControl).is_some()
        || modules.stage(UserStage::TessEvaluation).is_some();
    let geometry = modules.stage(UserStage::Geometry).is_some();
    let supported = (
        !tessellated || runtime.tessellation_shader,
        !geometry || runtime.geometry_shader,
    );
    if supported != (true, true) {
        return Err(rejected(
            supported,
            "tessellationShader and geometryShader features for those stages",
        ));
    }
    Ok(())
}

/// The stage's SPIR-V; a broadcasting fragment stage writes its colour into every OIT target.
fn stage_words(
    reflection: &crate::user_program::StageReflection,
    words: &[u32],
    fixed: UserPipelineState,
) -> Result<Vec<u32>, RenderError> {
    let attachments = fixed.oit.map_or(1, |target| target.count as u32);
    if reflection.stage != UserStage::Fragment || !fixed.broadcast {
        return Ok(words.to_vec());
    }
    crate::user_program::broadcast_frag_color(words, attachments)
}

fn create_module(runtime: &VulkanRuntime, words: &[u32]) -> Result<vk::ShaderModule, RenderError> {
    // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
    unsafe {
        runtime
            .device()
            .create_shader_module(&vk::ShaderModuleCreateInfo::default().code(words), None)
    }
    .map_err(|e| vk_error("create user program shader module", e))
}

fn layout_binding(
    binding: u32,
    kind: vk::DescriptorType,
) -> vk::DescriptorSetLayoutBinding<'static> {
    vk::DescriptorSetLayoutBinding::default()
        .binding(binding)
        .descriptor_type(kind)
        .descriptor_count(1)
        .stage_flags(vk::ShaderStageFlags::ALL_GRAPHICS)
}

fn create_set_layout(
    runtime: &VulkanRuntime,
    bindings: &[vk::DescriptorSetLayoutBinding<'_>],
) -> Result<vk::DescriptorSetLayout, RenderError> {
    let info = vk::DescriptorSetLayoutCreateInfo::default().bindings(bindings);
    // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
    unsafe { runtime.device().create_descriptor_set_layout(&info, None) }
        .map_err(|e| vk_error("create user program set layout", e))
}

/// One set-0 sampler of a linked program and the texture unit it reads.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct UserSampler {
    pub binding: u32,
    /// `Graphic3dTextureUnit` of an `occSamplerN` name; `None` for another name, which reads
    /// an unbound unit.
    pub unit: Option<usize>,
    /// A `sampler3D`: it reads the unit's 3D texture (GL_TEXTURE_3D target) only.
    pub volume: bool,
}

/// The distinct set-0 samplers of every stage by ascending binding.
fn user_samplers(modules: &UserProgramModules) -> Vec<UserSampler> {
    let mut samplers: Vec<UserSampler> = modules
        .stages
        .iter()
        .flat_map(|(reflection, _)| reflection.samplers.iter())
        .map(|sampler| UserSampler {
            binding: sampler.binding,
            unit: sampler_unit(&sampler.name),
            volume: sampler.dim == crate::user_program::SPIRV_DIM_3D,
        })
        .collect();
    samplers.sort_unstable_by_key(|sampler| sampler.binding);
    samplers.dedup();
    samplers
}

/// GlShaderProgram::Initialize sets every `occSamplerN` uniform to texture unit N
/// (`occActiveSampler` and `occSamplerBaseColor` are macros of `occSampler0`);
/// `sampler_unit("occSampler2") == Some(2)`.
pub(super) fn sampler_unit(name: &str) -> Option<usize> {
    let digits = name.strip_prefix("occSampler")?;
    let canonical = !digits.is_empty() && !(digits.len() > 1 && digits.starts_with('0'));
    canonical.then(|| digits.parse().ok()).flatten()
}

fn stage_flag(stage: UserStage) -> vk::ShaderStageFlags {
    match stage {
        UserStage::Vertex => vk::ShaderStageFlags::VERTEX,
        UserStage::TessControl => vk::ShaderStageFlags::TESSELLATION_CONTROL,
        UserStage::TessEvaluation => vk::ShaderStageFlags::TESSELLATION_EVALUATION,
        UserStage::Geometry => vk::ShaderStageFlags::GEOMETRY,
        UserStage::Fragment => vk::ShaderStageFlags::FRAGMENT,
    }
}

fn stage_infos(
    modules: &UserProgramModules,
    shaders: &[vk::ShaderModule],
) -> Vec<vk::PipelineShaderStageCreateInfo<'static>> {
    let stages = modules.stages.iter().zip(shaders);
    stages
        .map(|((reflection, _), &module)| {
            vk::PipelineShaderStageCreateInfo::default()
                .stage(stage_flag(reflection.stage))
                .module(module)
                .name(c"main")
        })
        .collect()
}

/// The attributes the vertex stage reads; any input other than the native four is refused.
fn vertex_attributes(
    modules: &UserProgramModules,
) -> Result<Vec<vk::VertexInputAttributeDescription>, RenderError> {
    let vertex = modules
        .stage(UserStage::Vertex)
        .expect("validated vertex stage");
    vertex
        .inputs
        .iter()
        .map(|(name, &location)| {
            let known = ATTRIBUTE_NAMES.iter().position(|known| known == name);
            match known {
                Some(slot) if slot as u32 == location => Ok(attribute(location)),
                _ => Err(rejected(
                    (name.clone(), location),
                    "vertex inputs occVertex@0, occNormal@1, occTexCoord@2, occVertColor@3",
                )),
            }
        })
        .collect()
}

fn attribute(location: u32) -> vk::VertexInputAttributeDescription {
    let format = if location == 1 {
        vk::Format::R32G32B32_SFLOAT
    } else {
        vk::Format::R32G32B32A32_SFLOAT
    };
    vk::VertexInputAttributeDescription::default()
        .location(location)
        .binding(0)
        .format(format)
        .offset(16 * location)
}

/// A program key, the address of the built-in variant whose fixed state it takes, the sample
/// count and alpha to coverage.
type UserPipelineKey = (
    UserProgramKey,
    usize,
    u32,
    bool,
    Option<([i32; 3], usize, bool)>,
);

/// Linked user programs per program key and built-in variant, kept with the renderer.
#[derive(Default)]
pub(super) struct UserProgramPipelineCache {
    pipelines: Mutex<Vec<(UserPipelineKey, Arc<UserProgramPipeline>)>>,
}

impl UserProgramPipelineCache {
    /// The pipeline of `modules` with the `fixed` state;
    /// `cache.pipeline(&runtime, &modules, UserPipelineState::single(kind, variant))?`.
    pub fn pipeline(
        &self,
        runtime: &Arc<VulkanRuntime>,
        modules: &UserProgramModules,
        fixed: UserPipelineState,
    ) -> Result<Arc<UserProgramPipeline>, RenderError> {
        let mut pipelines = self.pipelines.lock().map_err(|error| {
            rejected(
                error.to_string(),
                "an unpoisoned user program pipeline cache",
            )
        })?;
        let key = fixed.key(modules.key);
        if let Some((_, pipeline)) = pipelines.iter().find(|(known, _)| *known == key) {
            return Ok(pipeline.clone());
        }
        let pipeline = Arc::new(UserProgramPipeline::new(runtime, modules, fixed)?);
        pipelines.push((key, pipeline.clone()));
        Ok(pipeline)
    }
}

#[cfg(test)]
#[path = "user_program_light_dot_tests.rs"]
mod light_dot_unapplied_tests;

//! The ray-tracing programs of GlView::initRaytraceResources as Vulkan pipelines: program 0
//! (raytrace_render.frag), the FSAA program 1 (raytrace_smooth.frag) and the output program
//! (raytrace_display.frag), each over the screen quad of raytrace.vert, without blending.
use super::{
    pipeline::load_shader,
    pipeline_shader_stages::shader_stages,
    raster_pipeline_policy::create_pipeline,
    raytrace_resources::RaytraceLayout,
    runtime::{VulkanRuntime, vk_error},
    shader_grid_pass::{GridFixedState, GridTarget},
};
use crate::RenderError;
use crate::raytrace::RaytraceConstants;
use ash::vk;
use std::sync::{Arc, Mutex};

const VERTEX: &[u8] = include_bytes!("../../shaders/raytrace.vert.spv");

/// One ray-tracing program.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum RaytraceProgram {
    /// Program 0: the ray-traced image and its depth (depth-tested, written).
    Render,
    /// Program 1: one smoothing pass over the previous image (no depth).
    Smooth,
    /// myOutImageProgram: the smoothed image and its depth into the frame.
    Display,
    /// Program 0 with PATH_TRACING: one frame accumulated into the other target (no depth).
    RenderPathtrace,
    /// The output program with PATH_TRACING: exposure, tone mapping, the averaged depth.
    DisplayPathtrace,
    /// Program 0 with ADAPTIVE_SAMPLING: one pass accumulating into the output image, without
    /// attachments (multi-pass variant, tile samples).
    RenderAdaptive,
    /// The same with ADAPTIVE_SAMPLING_ATOMIC: one pass over the offset tiles.
    RenderAdaptiveAtomic,
    /// The output program with ADAPTIVE_SAMPLING: averages the output image, adds the visual
    /// error per tile.
    DisplayAdaptive,
}
impl RaytraceProgram {
    fn fragment(self) -> &'static [u8] {
        match self {
            Self::Render => include_bytes!("../../shaders/raytrace_render.frag.spv"),
            Self::Smooth => include_bytes!("../../shaders/raytrace_smooth.frag.spv"),
            Self::Display => include_bytes!("../../shaders/raytrace_display.frag.spv"),
            Self::RenderPathtrace => include_bytes!("../../shaders/raytrace_render.pt.frag.spv"),
            Self::DisplayPathtrace => include_bytes!("../../shaders/raytrace_display.pt.frag.spv"),
            Self::RenderAdaptive => include_bytes!("../../shaders/raytrace_render.iss.frag.spv"),
            Self::RenderAdaptiveAtomic => {
                include_bytes!("../../shaders/raytrace_render.iss_atomic.frag.spv")
            }
            Self::DisplayAdaptive => include_bytes!("../../shaders/raytrace_display.iss.frag.spv"),
        }
    }
    /// The USE_TEXTURES binary of the scene programs; the output programs have none.
    fn textured_fragment(self) -> &'static [u8] {
        match self {
            Self::Render => include_bytes!("../../shaders/raytrace_render.tex.frag.spv"),
            Self::Smooth => include_bytes!("../../shaders/raytrace_smooth.tex.frag.spv"),
            Self::RenderPathtrace => {
                include_bytes!("../../shaders/raytrace_render.pt.tex.frag.spv")
            }
            Self::RenderAdaptive => {
                include_bytes!("../../shaders/raytrace_render.iss.tex.frag.spv")
            }
            Self::RenderAdaptiveAtomic => {
                include_bytes!("../../shaders/raytrace_render.iss_atomic.tex.frag.spv")
            }
            Self::Display | Self::DisplayPathtrace | Self::DisplayAdaptive => self.fragment(),
        }
    }
    /// The output programs read two images (set 0) instead of the scene.
    fn is_output(self) -> bool {
        matches!(
            self,
            Self::Display | Self::DisplayPathtrace | Self::DisplayAdaptive
        )
    }
    /// The adaptive passes of program 0 write only storage images: no colour attachment.
    fn is_adaptive_render(self) -> bool {
        matches!(self, Self::RenderAdaptive | Self::RenderAdaptiveAtomic)
    }
    /// The smoothing and path-tracing passes render without depth (GL_DEPTH_TEST disabled).
    fn without_depth(self) -> bool {
        matches!(self, Self::Smooth | Self::RenderPathtrace) || self.is_adaptive_render()
    }
    /// Push-constant bytes: `uFsaaOffset, uSamples`, the Display.fs block (with the tile size
    /// and variance scale when adaptive) or the adaptive pass seed and viewport.
    fn push_bytes(self) -> u32 {
        match self {
            Self::Display | Self::DisplayPathtrace => 16,
            Self::DisplayAdaptive => 28,
            Self::RenderAdaptive | Self::RenderAdaptiveAtomic => 16,
            _ => 12,
        }
    }
}

/// Specialization constants of the ray-tracing fragment programs.
const SPECIALIZATION_CONSTANTS: usize = 10;

/// The specialization data of constants 0 to 9 of the ray-tracing fragment programs.
fn specialization_bytes(constants: &RaytraceConstants) -> [u8; 4 * SPECIALIZATION_CONSTANTS] {
    let values = [
        constants.stack_size as u32,
        constants.bounces as u32,
        u32::from(constants.zero_to_one_depth),
        u32::from(constants.transparent_shadows),
        // THE_SHIFT_sRGB: the scene attachments render sRGB (GlContext::ToRenderSRGB).
        0,
        u32::from(constants.two_sided_bxdf),
        u32::from(constants.filmic),
        u32::from(constants.ignore_normal_map),
        u32::from(constants.depth_of_field),
        u32::from(constants.cubemap_background),
    ];
    let mut bytes = [0; 4 * SPECIALIZATION_CONSTANTS];
    for (index, value) in values.iter().enumerate() {
        bytes[4 * index..][..4].copy_from_slice(&value.to_ne_bytes());
    }
    bytes
}

/// The program, attachments and constants a pipeline is built for.
/// The last member is whether alpha is written (GlContext's colour mask: no alpha with
/// `buffersOpaqueAlpha`, so the float targets keep their earlier alpha).
pub(super) type RaytracePipelineKey = (RaytraceProgram, GridTarget, RaytraceConstants, bool);

pub(super) struct RaytracePipeline {
    runtime: Arc<VulkanRuntime>,
    pub handle: vk::Pipeline,
    pub layout: vk::PipelineLayout,
    /// Set 0 of programs 0 and 1 (the scene); unused by the output program.
    pub scene: RaytraceLayout,
    set_layouts: Vec<vk::DescriptorSetLayout>,
    shaders: Vec<vk::ShaderModule>,
    #[cfg(feature = "raytrace-hit-diagnostics")]
    hit_layout: Option<super::storage_descriptor::StorageDescriptorLayout>,
    #[cfg(feature = "raytrace-input-diagnostics")]
    pub input_observation: Option<crate::raytrace_input_observation::RayPipelineInputs>,
}
impl RaytracePipeline {
    fn new(runtime: &Arc<VulkanRuntime>, key: RaytracePipelineKey) -> Result<Self, RenderError> {
        let (program, target, constants, alpha_writes) = key;
        let mut pipeline = Self {
            runtime: runtime.clone(),
            handle: vk::Pipeline::null(),
            layout: vk::PipelineLayout::null(),
            scene: RaytraceLayout::new(runtime, constants.textures)?,
            set_layouts: Vec::new(),
            #[cfg(feature = "raytrace-hit-diagnostics")]
            hit_layout: None,
            #[cfg(feature = "raytrace-input-diagnostics")]
            input_observation: None,
            shaders: vec![
                load_shader(runtime, VERTEX)?,
                load_shader(
                    runtime,
                    selected_fragment(runtime, program, constants.textures),
                )?,
            ],
        };
        pipeline.create_layout(program)?;
        pipeline.create(program, target, (&constants, alpha_writes))?;
        Ok(pipeline)
    }
    fn create_layout(&mut self, program: RaytraceProgram) -> Result<(), RenderError> {
        let adaptive = program == RaytraceProgram::DisplayAdaptive || program.is_adaptive_render();
        let samplers = if program.is_output() { 2 } else { 1 };
        self.set_layouts.push(match adaptive {
            true => super::raytrace_adaptive::storage_layout(&self.runtime)?,
            false => super::raytrace_fsaa::sampler_layout(&self.runtime, samplers)?,
        });
        let layouts = match program.is_output() {
            true => vec![self.set_layouts[0]],
            false => vec![self.scene.handle, self.set_layouts[0]],
        };
        #[cfg(feature = "raytrace-hit-diagnostics")]
        let layouts = self.with_hit_layout(program, layouts)?;
        let ranges = [vk::PushConstantRange::default()
            .stage_flags(vk::ShaderStageFlags::FRAGMENT)
            .size(program.push_bytes())];
        let info = vk::PipelineLayoutCreateInfo::default()
            .set_layouts(&layouts)
            .push_constant_ranges(&ranges);
        // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
        self.layout = unsafe { self.runtime.device().create_pipeline_layout(&info, None) }
            .map_err(|e| vk_error("create ray-tracing pipeline layout", e))?;
        Ok(())
    }
    #[cfg(feature = "raytrace-hit-diagnostics")]
    fn with_hit_layout(
        &mut self,
        program: RaytraceProgram,
        mut layouts: Vec<vk::DescriptorSetLayout>,
    ) -> Result<Vec<vk::DescriptorSetLayout>, RenderError> {
        if self.runtime.ray_hit_control.is_some() && program == RaytraceProgram::Render {
            let layout = super::storage_descriptor::StorageDescriptorLayout::new(
                &self.runtime,
                vk::ShaderStageFlags::FRAGMENT,
            )?;
            layouts.push(layout.handle);
            self.hit_layout = Some(layout);
        }
        Ok(layouts)
    }
    fn create(
        &mut self,
        program: RaytraceProgram,
        target: GridTarget,
        (constants, alpha_writes): (&RaytraceConstants, bool),
    ) -> Result<(), RenderError> {
        let data = specialization_bytes(constants);
        let entries: Vec<_> = (0..SPECIALIZATION_CONSTANTS as u32)
            .map(|id| {
                vk::SpecializationMapEntry::default()
                    .constant_id(id)
                    .offset(4 * id)
                    .size(4)
            })
            .collect();
        let specialization = vk::SpecializationInfo::default()
            .map_entries(&entries)
            .data(&data);
        let mut stages = shader_stages(&self.shaders);
        stages[1] = stages[1].specialization_info(&specialization);
        // GlView::redraw sets glDepthFunc(GL_LEQUAL) for the frame, ray tracing included.
        let state = GridFixedState::new(target).with_depth_write();
        let mut mask =
            vk::ColorComponentFlags::R | vk::ColorComponentFlags::G | vk::ColorComponentFlags::B;
        if alpha_writes {
            mask |= vk::ColorComponentFlags::A;
        }
        let attachments = usize::from(!program.is_adaptive_render());
        let blends = [vk::PipelineColorBlendAttachmentState::default().color_write_mask(mask)];
        let blend =
            vk::PipelineColorBlendStateCreateInfo::default().attachments(&blends[..attachments]);
        let formats = [target.color];
        let formats = &formats[..attachments];
        let stencil = if target.stencil {
            target.depth
        } else {
            vk::Format::UNDEFINED
        };
        let depth = if program.without_depth() {
            vk::Format::UNDEFINED
        } else {
            target.depth
        };
        let mut rendering = vk::PipelineRenderingCreateInfo::default()
            .color_attachment_formats(formats)
            .depth_attachment_format(depth)
            .stencil_attachment_format(if program.without_depth() {
                vk::Format::UNDEFINED
            } else {
                stencil
            });
        let info = state
            .description(self.layout, &stages)
            .color_blend_state(&blend)
            .push_next(&mut rendering);
        self.handle = create_pipeline(&self.runtime, info, &state.raster)?;
        #[cfg(feature = "raytrace-input-diagnostics")]
        {
            let fragment = selected_fragment(&self.runtime, program, constants.textures);
            self.input_observation = Some(super::raytrace_input_diagnostic::pipeline(
                (self.handle, self.layout, self.scene.handle),
                &self.shaders,
                program,
                (&data, &entries),
                (VERTEX, fragment),
            ));
        }
        Ok(())
    }
}
impl Drop for RaytracePipeline {
    fn drop(&mut self) {
        // SAFETY: the handle belongs to this device and its owner releases it only after the GPU work that used it has completed.
        unsafe {
            let device = self.runtime.device();
            device.destroy_pipeline(self.handle, None);
            device.destroy_pipeline_layout(self.layout, None);
            for layout in self.set_layouts.drain(..) {
                device.destroy_descriptor_set_layout(layout, None);
            }
            for shader in self.shaders.drain(..) {
                device.destroy_shader_module(shader, None);
            }
        }
    }
}

/// Pipelines per program, attachment configuration and constants, created on first use.
#[derive(Default)]
pub(super) struct RaytracePipelineCache {
    pipelines: Mutex<Vec<(RaytracePipelineKey, Arc<RaytracePipeline>)>>,
}
impl RaytracePipelineCache {
    pub fn pipeline(
        &self,
        runtime: &Arc<VulkanRuntime>,
        key: RaytracePipelineKey,
    ) -> Result<Arc<RaytracePipeline>, RenderError> {
        let mut pipelines = self.pipelines.lock().map_err(|error| {
            crate::error::rejected(
                error.to_string(),
                "an unpoisoned ray-tracing pipeline cache",
            )
        })?;
        if let Some((_, pipeline)) = pipelines.iter().find(|(stored, _)| *stored == key) {
            return Ok(pipeline.clone());
        }
        let pipeline = Arc::new(RaytracePipeline::new(runtime, key)?);
        pipelines.push((key, pipeline.clone()));
        Ok(pipeline)
    }
}

fn selected_fragment(
    runtime: &VulkanRuntime,
    program: RaytraceProgram,
    textured: bool,
) -> &'static [u8] {
    #[cfg(feature = "raytrace-hit-diagnostics")]
    if let Some(control) = &runtime.ray_hit_control {
        if program == RaytraceProgram::Render {
            return control.shader_bytes();
        }
    }
    let _ = runtime;
    if textured {
        program.textured_fragment()
    } else {
        program.fragment()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn specialization_data_carries_stack_bounces_and_flags_in_order() {
        let constants = crate::raytrace::RaytraceConstants {
            stack_size: 14,
            bounces: 3,
            zero_to_one_depth: false,
            transparent_shadows: true,
            two_sided_bxdf: false,
            filmic: true,
            ignore_normal_map: false,
            depth_of_field: true,
            textures: false,
            cubemap_background: true,
        };
        let bytes = super::specialization_bytes(&constants);
        let word = |i: usize| u32::from_ne_bytes(bytes[4 * i..][..4].try_into().unwrap());
        assert_eq!(
            [word(0), word(1), word(2), word(3), word(4)],
            [14, 3, 0, 1, 0]
        );
        assert_eq!([word(5), word(6), word(7), word(8)], [0, 1, 0, 1]);
        assert_eq!(word(9), 1);
    }
}

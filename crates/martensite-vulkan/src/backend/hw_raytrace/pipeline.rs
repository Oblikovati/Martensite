//! Compute pipelines and descriptor set layouts for the OpenPBR compute path tracer
//! (`pathtrace_hw.comp`, `pathtrace_sw.comp`), 3-pass SVGF denoiser (`svgf_reproject.comp`,
//! `svgf_atrous.comp`, `svgf_compose_tonemap.comp`), and standalone HDR tone mapper (`tonemap.comp`).
use super::super::pbr_environment::PbrEnvironmentLayout;
use super::super::runtime::{VulkanRuntime, vk_error};
use crate::RenderError;
use ash::vk;
use std::sync::Arc;

const PATHTRACE_HW_SPV: &[u8] = include_bytes!("../../../shaders/openpbr/pathtrace_hw.comp.spv");
const PATHTRACE_SW_SPV: &[u8] = include_bytes!("../../../shaders/openpbr/pathtrace_sw.comp.spv");
const SVGF_REPROJECT_SPV: &[u8] =
    include_bytes!("../../../shaders/openpbr/svgf_reproject.comp.spv");
const SVGF_ATROUS_SPV: &[u8] = include_bytes!("../../../shaders/openpbr/svgf_atrous.comp.spv");
const SVGF_COMPOSE_SPV: &[u8] =
    include_bytes!("../../../shaders/openpbr/svgf_compose_tonemap.comp.spv");
const TONEMAP_SPV: &[u8] = include_bytes!("../../../shaders/openpbr/tonemap.comp.spv");

/// Descriptor set layouts shared across the compute path tracer, SVGF passes, and tone mapper.
pub(in super::super) struct ComputeRaytraceLayouts {
    runtime: Arc<VulkanRuntime>,
    pub scene_sw: vk::DescriptorSetLayout,
    pub scene_hw: Option<vk::DescriptorSetLayout>,
    pub gbuffer: vk::DescriptorSetLayout,
    pub env: PbrEnvironmentLayout,
    pub reproject: vk::DescriptorSetLayout,
    pub atrous: vk::DescriptorSetLayout,
    pub compose: vk::DescriptorSetLayout,
    pub tonemap: vk::DescriptorSetLayout,
}

impl ComputeRaytraceLayouts {
    pub fn new(runtime: &Arc<VulkanRuntime>) -> Result<Self, RenderError> {
        let sw_bindings = scene_bindings(false);
        let scene_sw = create_set_layout(runtime, &sw_bindings)?;
        let scene_hw = if runtime.hw_raytrace.is_some() {
            let hw_bindings = scene_bindings(true);
            Some(create_set_layout(runtime, &hw_bindings)?)
        } else {
            None
        };
        let gbuffer = create_storage_image_layout(runtime, 5)?;
        let env = PbrEnvironmentLayout::new(runtime)?;
        let reproject = create_storage_image_layout(runtime, 8)?;
        let atrous = create_storage_image_layout(runtime, 3)?;
        let compose = create_storage_image_layout(runtime, 5)?;
        let tonemap = create_storage_image_layout(runtime, 2)?;
        Ok(Self {
            runtime: runtime.clone(),
            scene_sw,
            scene_hw,
            gbuffer,
            env,
            reproject,
            atrous,
            compose,
            tonemap,
        })
    }
}

impl Drop for ComputeRaytraceLayouts {
    fn drop(&mut self) {
        let dev = self.runtime.device();
        // SAFETY: layouts belong to this device and drop only after pipelines using them drop.
        unsafe {
            dev.destroy_descriptor_set_layout(self.scene_sw, None);
            if let Some(hw) = self.scene_hw {
                dev.destroy_descriptor_set_layout(hw, None);
            }
            dev.destroy_descriptor_set_layout(self.gbuffer, None);
            dev.destroy_descriptor_set_layout(self.reproject, None);
            dev.destroy_descriptor_set_layout(self.atrous, None);
            dev.destroy_descriptor_set_layout(self.compose, None);
            dev.destroy_descriptor_set_layout(self.tonemap, None);
        }
    }
}

/// One compiled Vulkan compute pipeline and its layout.
pub(in super::super) struct ComputePipeline {
    runtime: Arc<VulkanRuntime>,
    pub layout: vk::PipelineLayout,
    pub handle: vk::Pipeline,
}

impl ComputePipeline {
    fn new(
        runtime: &Arc<VulkanRuntime>,
        set_layouts: &[vk::DescriptorSetLayout],
        push_bytes: u32,
        spv: &[u8],
    ) -> Result<Self, RenderError> {
        let push_ranges = if push_bytes > 0 {
            vec![
                vk::PushConstantRange::default()
                    .stage_flags(vk::ShaderStageFlags::COMPUTE)
                    .offset(0)
                    .size(push_bytes),
            ]
        } else {
            Vec::new()
        };
        let layout_info = vk::PipelineLayoutCreateInfo::default()
            .set_layouts(set_layouts)
            .push_constant_ranges(&push_ranges);
        // SAFETY: layout_info borrows live slices on this device.
        let layout = unsafe { runtime.device().create_pipeline_layout(&layout_info, None) }
            .map_err(|e| vk_error("create compute pipeline layout", e))?;
        let module = match create_shader_module(runtime, spv) {
            Ok(module) => module,
            Err(err) => {
                // SAFETY: layout belongs to this device and is unused.
                unsafe { runtime.device().destroy_pipeline_layout(layout, None) };
                return Err(err);
            }
        };
        let stage = vk::PipelineShaderStageCreateInfo::default()
            .stage(vk::ShaderStageFlags::COMPUTE)
            .module(module)
            .name(c"main");
        let info = vk::ComputePipelineCreateInfo::default()
            .stage(stage)
            .layout(layout);
        let result = runtime.pipeline_cache.with_handle(|cache| {
            // SAFETY: module and layout are live through create_compute_pipelines.
            unsafe {
                runtime
                    .device()
                    .create_compute_pipelines(cache, &[info], None)
            }
        });
        // SAFETY: shader module can be destroyed immediately after pipeline creation.
        unsafe { runtime.device().destroy_shader_module(module, None) };
        let handle = match result {
            Ok(mut pipes) => pipes.remove(0),
            Err((_, err)) => {
                // SAFETY: layout belongs to this device.
                unsafe { runtime.device().destroy_pipeline_layout(layout, None) };
                return Err(vk_error("create compute pipeline", err));
            }
        };
        Ok(Self {
            runtime: runtime.clone(),
            layout,
            handle,
        })
    }
}

impl Drop for ComputePipeline {
    fn drop(&mut self) {
        let dev = self.runtime.device();
        // SAFETY: pipeline and layout belong to this device and drop after GPU completion.
        unsafe {
            dev.destroy_pipeline(self.handle, None);
            dev.destroy_pipeline_layout(self.layout, None);
        }
    }
}

/// Complete collection of OpenPBR compute ray-tracing, SVGF, and tone-mapping pipelines.
pub(in super::super) struct ComputeRaytracePipelines {
    pub pathtrace_sw: ComputePipeline,
    pub pathtrace_hw: Option<ComputePipeline>,
    pub svgf_reproject: ComputePipeline,
    pub svgf_atrous: ComputePipeline,
    pub svgf_compose: ComputePipeline,
    pub tonemap: ComputePipeline,
    pub layouts: ComputeRaytraceLayouts,
}

impl ComputeRaytracePipelines {
    pub fn new(runtime: &Arc<VulkanRuntime>) -> Result<Self, RenderError> {
        let layouts = ComputeRaytraceLayouts::new(runtime)?;
        let pathtrace_sw = ComputePipeline::new(
            runtime,
            &[layouts.scene_sw, layouts.gbuffer, layouts.env.handle],
            0,
            PATHTRACE_SW_SPV,
        )?;
        let pathtrace_hw = match layouts.scene_hw {
            Some(scene_hw) => Some(ComputePipeline::new(
                runtime,
                &[scene_hw, layouts.gbuffer, layouts.env.handle],
                0,
                PATHTRACE_HW_SPV,
            )?),
            None => None,
        };
        let svgf_reproject =
            ComputePipeline::new(runtime, &[layouts.reproject], 16, SVGF_REPROJECT_SPV)?;
        let svgf_atrous = ComputePipeline::new(runtime, &[layouts.atrous], 24, SVGF_ATROUS_SPV)?;
        let svgf_compose = ComputePipeline::new(runtime, &[layouts.compose], 32, SVGF_COMPOSE_SPV)?;
        let tonemap = ComputePipeline::new(runtime, &[layouts.tonemap], 24, TONEMAP_SPV)?;
        Ok(Self {
            pathtrace_sw,
            pathtrace_hw,
            svgf_reproject,
            svgf_atrous,
            svgf_compose,
            tonemap,
            layouts,
        })
    }
}

fn scene_bindings(include_tlas: bool) -> Vec<vk::DescriptorSetLayoutBinding<'static>> {
    let mut list = vec![
        vk::DescriptorSetLayoutBinding::default()
            .binding(0)
            .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
            .descriptor_count(1)
            .stage_flags(vk::ShaderStageFlags::COMPUTE),
    ];
    for b in 1..=5u32 {
        list.push(
            vk::DescriptorSetLayoutBinding::default()
                .binding(b)
                .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
                .descriptor_count(1)
                .stage_flags(vk::ShaderStageFlags::COMPUTE),
        );
    }
    if include_tlas {
        list.push(
            vk::DescriptorSetLayoutBinding::default()
                .binding(6)
                .descriptor_type(vk::DescriptorType::ACCELERATION_STRUCTURE_KHR)
                .descriptor_count(1)
                .stage_flags(vk::ShaderStageFlags::COMPUTE),
        );
    }
    list
}

fn create_storage_image_layout(
    runtime: &VulkanRuntime,
    count: u32,
) -> Result<vk::DescriptorSetLayout, RenderError> {
    let bindings: Vec<_> = (0..count)
        .map(|binding| {
            vk::DescriptorSetLayoutBinding::default()
                .binding(binding)
                .descriptor_type(vk::DescriptorType::STORAGE_IMAGE)
                .descriptor_count(1)
                .stage_flags(vk::ShaderStageFlags::COMPUTE)
        })
        .collect();
    create_set_layout(runtime, &bindings)
}

fn create_set_layout(
    runtime: &VulkanRuntime,
    bindings: &[vk::DescriptorSetLayoutBinding<'_>],
) -> Result<vk::DescriptorSetLayout, RenderError> {
    let info = vk::DescriptorSetLayoutCreateInfo::default().bindings(bindings);
    // SAFETY: info borrows live bindings slice on this device.
    unsafe { runtime.device().create_descriptor_set_layout(&info, None) }
        .map_err(|e| vk_error("create compute descriptor set layout", e))
}

fn create_shader_module(
    runtime: &VulkanRuntime,
    spv: &[u8],
) -> Result<vk::ShaderModule, RenderError> {
    let words: Vec<u32> = spv
        .as_chunks::<4>()
        .0
        .iter()
        .map(|&c| u32::from_le_bytes(c))
        .collect();
    let info = vk::ShaderModuleCreateInfo::default().code(&words);
    // SAFETY: words holds valid aligned SPIR-V u32 bytecode for this call.
    unsafe { runtime.device().create_shader_module(&info, None) }
        .map_err(|e| vk_error("create compute shader module", e))
}

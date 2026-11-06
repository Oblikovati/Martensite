//! Descriptor set 6 of the Phong-family and PBR fragment programs: the view's shadow maps
//! (GlShadowMapArray) as the layers of one R32F array image sampled with the native
//! depth-texture state (Graphic3dTOTF_NEAREST without repeat), and the storage buffer of
//! `occShadowMapSizeBias` and `occShadowMapMatrices`. A frame without shadow maps binds the
//! empty set (no maps), which leaves every light unshadowed.
use super::{
    buffer::DeviceBuffer,
    memory_selection::HostBufferAccess,
    runtime::{VulkanRuntime, vk_error},
    submission::CommandSubmission,
};
use crate::{RenderError, shadow_maps::RenderShadowMaps};
use ash::vk;
use std::sync::Arc;

const FORMAT: vk::Format = vk::Format::R32_SFLOAT;

/// The two-binding layout of set 6; `ShadowMapLayout::new(&runtime)?`.
pub(super) struct ShadowMapLayout {
    runtime: Arc<VulkanRuntime>,
    pub handle: vk::DescriptorSetLayout,
}
impl ShadowMapLayout {
    pub fn new(runtime: &Arc<VulkanRuntime>) -> Result<Self, RenderError> {
        let bindings = [
            vk::DescriptorSetLayoutBinding::default()
                .binding(0)
                .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
                .descriptor_count(1)
                .stage_flags(vk::ShaderStageFlags::FRAGMENT),
            vk::DescriptorSetLayoutBinding::default()
                .binding(1)
                .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
                .descriptor_count(1)
                .stage_flags(vk::ShaderStageFlags::VERTEX | vk::ShaderStageFlags::FRAGMENT),
        ];
        let info = vk::DescriptorSetLayoutCreateInfo::default().bindings(&bindings);
        // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
        let handle = unsafe { runtime.device().create_descriptor_set_layout(&info, None) }
            .map_err(|e| vk_error("create shadow map layout", e))?;
        #[cfg(feature = "command-state-diagnostics")]
        super::replay_resource_diagnostic::descriptor_layout(handle, &info);
        Ok(Self {
            runtime: runtime.clone(),
            handle,
        })
    }
}
impl Drop for ShadowMapLayout {
    fn drop(&mut self) {
        // SAFETY: the handle belongs to this device and its owner releases it only after the GPU work that used it has completed.
        unsafe {
            self.runtime
                .device()
                .destroy_descriptor_set_layout(self.handle, None);
        }
    }
}

/// One frame's shadow maps and their bound set; kept alive by every draw that samples them.
pub(super) struct ShadowMapSet {
    runtime: Arc<VulkanRuntime>,
    image: vk::Image,
    view: vk::ImageView,
    sampler: vk::Sampler,
    pool: vk::DescriptorPool,
    _parameters: DeviceBuffer,
    memory: Option<super::memory_arena::MemoryLease>,
    allocation: u64,
    layout: ShadowMapLayout,
    pub set: vk::DescriptorSet,
}
impl ShadowMapSet {
    /// The set of `maps`, or the empty set for `None`; `ShadowMapSet::new(rt, &layout, maps)?`.
    pub fn new(
        runtime: &Arc<VulkanRuntime>,
        maps: Option<&RenderShadowMaps>,
    ) -> Result<Arc<Self>, RenderError> {
        let contents = ShadowMapContents::of(maps)?;
        let mut set = Self {
            runtime: runtime.clone(),
            image: vk::Image::null(),
            view: vk::ImageView::null(),
            sampler: vk::Sampler::null(),
            pool: vk::DescriptorPool::null(),
            _parameters: parameter_buffer(runtime, &contents.parameters)?,
            memory: None,
            allocation: 0,
            layout: ShadowMapLayout::new(runtime)?,
            set: vk::DescriptorSet::null(),
        };
        set.create_image(&contents)?;
        set.upload(&contents)?;
        set.sampler = create_sampler(runtime)?;
        set.pool = create_pool(runtime)?;
        set.set = set.allocate_set()?;
        #[cfg(feature = "command-state-diagnostics")]
        super::replay_command_diagnostic::buffer_written(
            set._parameters.handle,
            &contents.parameters,
        );
        Ok(Arc::new(set))
    }
    fn create_image(&mut self, contents: &ShadowMapContents) -> Result<(), RenderError> {
        let info = vk::ImageCreateInfo::default()
            .image_type(vk::ImageType::TYPE_2D)
            .format(FORMAT)
            .extent(vk::Extent3D {
                width: contents.size,
                height: contents.size,
                depth: 1,
            })
            .mip_levels(1)
            .array_layers(contents.layers)
            .samples(vk::SampleCountFlags::TYPE_1)
            .tiling(vk::ImageTiling::OPTIMAL)
            .usage(vk::ImageUsageFlags::SAMPLED | vk::ImageUsageFlags::TRANSFER_DST);
        let device = self.runtime.device();
        // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
        self.image = unsafe { device.create_image(&info, None) }
            .map_err(|e| vk_error("create shadow map image", e))?;
        let lease = super::memory_arena::bind_device_image(&self.runtime, self.image)?;
        self.allocation = lease.size();
        self.runtime.image_memory.allocate_sampled(self.allocation);
        self.memory = Some(lease);
        let view = vk::ImageViewCreateInfo::default()
            .image(self.image)
            .view_type(vk::ImageViewType::TYPE_2D_ARRAY)
            .format(FORMAT)
            .subresource_range(range(contents.layers));
        // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
        self.view = unsafe { device.create_image_view(&view, None) }
            .map_err(|e| vk_error("create shadow map view", e))?;
        Ok(())
    }
    fn upload(&self, contents: &ShadowMapContents) -> Result<(), RenderError> {
        let staging = DeviceBuffer::new(
            &self.runtime,
            contents.texels.len() as u64,
            vk::BufferUsageFlags::TRANSFER_SRC,
            HostBufferAccess::Upload,
        )?;
        staging.write(&contents.texels)?;
        let submission = CommandSubmission::new(&self.runtime)?;
        record_upload(
            self.runtime.device(),
            submission.command,
            (staging.handle, self.image),
            contents,
        );
        let outcome = submission.finish();
        if submission.retirement_incomplete() {
            // The device may still read the staging buffer: never free it.
            std::mem::forget(staging);
            std::mem::forget(submission);
        }
        outcome
    }
    fn allocate_set(&self) -> Result<vk::DescriptorSet, RenderError> {
        let layouts = [self.layout.handle];
        let info = vk::DescriptorSetAllocateInfo::default()
            .descriptor_pool(self.pool)
            .set_layouts(&layouts);
        // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
        let set = unsafe { self.runtime.device().allocate_descriptor_sets(&info) }
            .map_err(|e| vk_error("allocate shadow map set", e))?[0];
        #[cfg(feature = "command-state-diagnostics")]
        super::replay_resource_diagnostic::allocated(set, self.layout.handle);
        let image = [vk::DescriptorImageInfo::default()
            .sampler(self.sampler)
            .image_view(self.view)
            .image_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)];
        let buffer = [vk::DescriptorBufferInfo::default()
            .buffer(self._parameters.handle)
            .range(vk::WHOLE_SIZE)];
        let writes = [
            vk::WriteDescriptorSet::default()
                .dst_set(set)
                .dst_binding(0)
                .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
                .image_info(&image),
            vk::WriteDescriptorSet::default()
                .dst_set(set)
                .dst_binding(1)
                .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
                .buffer_info(&buffer),
        ];
        // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
        unsafe { self.runtime.device().update_descriptor_sets(&writes, &[]) };
        #[cfg(feature = "command-state-diagnostics")]
        super::replay_command_diagnostic::descriptor_writes(&writes);
        Ok(set)
    }
}
impl Drop for ShadowMapSet {
    fn drop(&mut self) {
        // SAFETY: the handle belongs to this device and its owner releases it only after the GPU work that used it has completed.
        unsafe {
            let device = self.runtime.device();
            if self.pool != vk::DescriptorPool::null() {
                device.destroy_descriptor_pool(self.pool, None);
            }
            if self.sampler != vk::Sampler::null() {
                device.destroy_sampler(self.sampler, None);
            }
            if self.view != vk::ImageView::null() {
                device.destroy_image_view(self.view, None);
            }
            if self.image != vk::Image::null() {
                device.destroy_image(self.image, None);
            }
        }
        self.runtime.image_memory.release_sampled(self.allocation);
    }
}

impl super::pipeline_variants::RasterPipelines {
    pub(super) fn release_shadow_maps(&mut self) {
        for slot in [&mut self.empty_shadow_maps, &mut self.frame_shadow_maps] {
            *slot
                .get_mut()
                .unwrap_or_else(std::sync::PoisonError::into_inner) = None;
        }
    }
    /// Bind the frame's shadow maps to the lit draws uploaded until reset to `None`;
    /// `pipelines.use_frame_shadow_maps(Some(maps))?`.
    pub(super) fn use_frame_shadow_maps(
        &self,
        maps: Option<Arc<ShadowMapSet>>,
    ) -> Result<(), RenderError> {
        *self.frame_shadow_maps.lock().map_err(|e| {
            crate::error::rejected(e.to_string(), "an unpoisoned frame shadow-map set")
        })? = maps;
        Ok(())
    }
    /// The frame's shadow maps, else the empty set created on first use;
    /// `pipelines.shadow_maps(&runtime)?`.
    pub(super) fn shadow_maps(
        &self,
        runtime: &Arc<VulkanRuntime>,
    ) -> Result<Arc<ShadowMapSet>, RenderError> {
        let poisoned = |e: String| crate::error::rejected(e, "an unpoisoned shadow-map set");
        if let Some(maps) = self
            .frame_shadow_maps
            .lock()
            .map_err(|e| poisoned(e.to_string()))?
            .as_ref()
        {
            return Ok(maps.clone());
        }
        let mut slot = self
            .empty_shadow_maps
            .lock()
            .map_err(|e| poisoned(e.to_string()))?;
        if let Some(maps) = slot.as_ref() {
            return Ok(maps.clone());
        }
        let maps = ShadowMapSet::new(runtime, None)?;
        *slot = Some(maps.clone());
        Ok(maps)
    }
}

/// The texels (layer after layer) and the storage-buffer bytes of one set.
struct ShadowMapContents {
    size: u32,
    layers: u32,
    texels: Vec<u8>,
    parameters: Vec<u8>,
}
impl ShadowMapContents {
    fn of(maps: Option<&RenderShadowMaps>) -> Result<Self, RenderError> {
        let Some(maps) = maps.filter(|maps| !maps.maps.is_empty()) else {
            // No map: one far-plane texel and a zero map count.
            return Ok(Self {
                size: 1,
                layers: 1,
                texels: 1f32.to_ne_bytes().to_vec(),
                parameters: [0f32; 4].into_iter().flat_map(f32::to_ne_bytes).collect(),
            });
        };
        let size = maps.resolution;
        let texel_count = size as usize * size as usize;
        let mut texels = Vec::with_capacity(texel_count * 4 * maps.maps.len());
        for map in &maps.maps {
            if map.depth.len() != texel_count {
                return Err(crate::error::rejected(
                    map.depth.len(),
                    "a shadow map of resolution x resolution depth texels",
                ));
            }
            texels.extend(map.depth.iter().flat_map(|depth| depth.to_ne_bytes()));
        }
        // GlShaderManager::pushLightSourceState: 1 / SizeX of the first map and its bias.
        let mut parameters: Vec<u8> = [1. / size as f32, maps.bias, maps.maps.len() as f32, 0.]
            .into_iter()
            .flat_map(f32::to_ne_bytes)
            .collect();
        for map in &maps.maps {
            parameters.extend(
                map.light_matrix
                    .iter()
                    .flat_map(|value| value.to_ne_bytes()),
            );
        }
        Ok(Self {
            size,
            layers: maps.maps.len() as u32,
            texels,
            parameters,
        })
    }
}

fn range(layers: u32) -> vk::ImageSubresourceRange {
    vk::ImageSubresourceRange::default()
        .aspect_mask(vk::ImageAspectFlags::COLOR)
        .level_count(1)
        .layer_count(layers)
}
fn parameter_buffer(
    runtime: &Arc<VulkanRuntime>,
    bytes: &[u8],
) -> Result<DeviceBuffer, RenderError> {
    let buffer = DeviceBuffer::new(
        runtime,
        bytes.len() as u64,
        vk::BufferUsageFlags::STORAGE_BUFFER,
        HostBufferAccess::Upload,
    )?;
    buffer.write(bytes)?;
    Ok(buffer)
}
/// The native depth-texture sampler: Graphic3dTOTF_NEAREST, clamped (no repeat).
fn create_sampler(runtime: &VulkanRuntime) -> Result<vk::Sampler, RenderError> {
    let clamp = vk::SamplerAddressMode::CLAMP_TO_EDGE;
    let info = vk::SamplerCreateInfo::default()
        .mag_filter(vk::Filter::NEAREST)
        .min_filter(vk::Filter::NEAREST)
        .mipmap_mode(vk::SamplerMipmapMode::NEAREST)
        .address_mode_u(clamp)
        .address_mode_v(clamp)
        .address_mode_w(clamp)
        .min_lod(0.)
        .max_lod(0.);
    // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
    unsafe { runtime.device().create_sampler(&info, None) }
        .map_err(|e| vk_error("create shadow map sampler", e))
}
fn create_pool(runtime: &VulkanRuntime) -> Result<vk::DescriptorPool, RenderError> {
    let sizes = [
        vk::DescriptorPoolSize {
            ty: vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
            descriptor_count: 1,
        },
        vk::DescriptorPoolSize {
            ty: vk::DescriptorType::STORAGE_BUFFER,
            descriptor_count: 1,
        },
    ];
    let info = vk::DescriptorPoolCreateInfo::default()
        .max_sets(1)
        .pool_sizes(&sizes);
    // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
    unsafe { runtime.device().create_descriptor_pool(&info, None) }
        .map_err(|e| vk_error("create shadow map pool", e))
}
fn record_upload(
    device: &ash::Device,
    command: vk::CommandBuffer,
    (source, image): (vk::Buffer, vk::Image),
    contents: &ShadowMapContents,
) {
    let barrier = |old, new, (source_stage, source_access), (target_stage, target_access)| {
        vk::ImageMemoryBarrier2::default()
            .image(image)
            .subresource_range(range(contents.layers))
            .old_layout(old)
            .new_layout(new)
            .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
            .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
            .src_stage_mask(source_stage)
            .src_access_mask(source_access)
            .dst_stage_mask(target_stage)
            .dst_access_mask(target_access)
    };
    let transfer = (
        vk::PipelineStageFlags2::TRANSFER,
        vk::AccessFlags2::TRANSFER_WRITE,
    );
    let to_transfer = [barrier(
        vk::ImageLayout::UNDEFINED,
        vk::ImageLayout::TRANSFER_DST_OPTIMAL,
        (vk::PipelineStageFlags2::NONE, vk::AccessFlags2::NONE),
        transfer,
    )];
    let region = vk::BufferImageCopy::default()
        .image_subresource(
            vk::ImageSubresourceLayers::default()
                .aspect_mask(vk::ImageAspectFlags::COLOR)
                .layer_count(contents.layers),
        )
        .image_extent(vk::Extent3D {
            width: contents.size,
            height: contents.size,
            depth: 1,
        });
    let to_sampled = [barrier(
        vk::ImageLayout::TRANSFER_DST_OPTIMAL,
        vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
        transfer,
        (
            vk::PipelineStageFlags2::FRAGMENT_SHADER,
            vk::AccessFlags2::SHADER_SAMPLED_READ,
        ),
    )];
    // SAFETY: the command buffer is recording on this thread and every handle it names outlives the submission.
    unsafe {
        device.cmd_pipeline_barrier2(
            command,
            &vk::DependencyInfo::default().image_memory_barriers(&to_transfer),
        );
        device.cmd_copy_buffer_to_image(
            command,
            source,
            image,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            &[region],
        );
        device.cmd_pipeline_barrier2(
            command,
            &vk::DependencyInfo::default().image_memory_barriers(&to_sampled),
        );
    }
}

//! Storage-image targets and pre-allocated descriptor sets for the unified OpenPBR compute
//! path tracer and 3-pass SVGF spatiotemporal denoiser.
use super::super::runtime::{VulkanRuntime, vk_error};
use super::super::submission::CommandSubmission;
use super::pipeline::ComputeRaytraceLayouts;
use crate::RenderError;
use ash::vk;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

/// One 2D `GENERAL`-layout compute storage image owned by `PathtraceTargets`.
pub(in super::super) struct StorageImage {
    runtime: Arc<VulkanRuntime>,
    pub handle: vk::Image,
    pub view: vk::ImageView,
    _lease: super::super::memory_arena::MemoryLease,
}

impl StorageImage {
    pub fn new(
        runtime: &Arc<VulkanRuntime>,
        extent: vk::Extent2D,
        format: vk::Format,
        extra_usage: vk::ImageUsageFlags,
    ) -> Result<Self, RenderError> {
        let info = vk::ImageCreateInfo::default()
            .image_type(vk::ImageType::TYPE_2D)
            .format(format)
            .extent(vk::Extent3D {
                width: extent.width.max(1),
                height: extent.height.max(1),
                depth: 1,
            })
            .mip_levels(1)
            .array_layers(1)
            .samples(vk::SampleCountFlags::TYPE_1)
            .tiling(vk::ImageTiling::OPTIMAL)
            .usage(vk::ImageUsageFlags::STORAGE | vk::ImageUsageFlags::TRANSFER_DST | extra_usage)
            .initial_layout(vk::ImageLayout::UNDEFINED);
        // SAFETY: info borrows live values and device stays valid for runtime's lifetime.
        let handle = unsafe { runtime.device().create_image(&info, None) }
            .map_err(|e| vk_error("create raytrace storage image", e))?;
        let lease = match super::super::memory_arena::bind_device_image(runtime, handle) {
            Ok(lease) => lease,
            Err(err) => {
                // SAFETY: handle belongs to this device and was not bound.
                unsafe { runtime.device().destroy_image(handle, None) };
                return Err(err);
            }
        };
        let view_info = vk::ImageViewCreateInfo::default()
            .image(handle)
            .view_type(vk::ImageViewType::TYPE_2D)
            .format(format)
            .subresource_range(color_range());
        // SAFETY: handle is bound and live on this device.
        let view = match unsafe { runtime.device().create_image_view(&view_info, None) } {
            Ok(view) => view,
            Err(err) => {
                // SAFETY: handle belongs to this device and has no live views.
                unsafe { runtime.device().destroy_image(handle, None) };
                return Err(vk_error("create raytrace storage image view", err));
            }
        };
        Ok(Self {
            runtime: runtime.clone(),
            handle,
            view,
            _lease: lease,
        })
    }
}

impl Drop for StorageImage {
    fn drop(&mut self) {
        // SAFETY: view and image belong to this device and drop after GPU completion.
        unsafe {
            self.runtime.device().destroy_image_view(self.view, None);
            self.runtime.device().destroy_image(self.handle, None);
        }
    }
}

/// Complete set of G-buffer, SVGF ping-pong history, progressive accumulator, and LDR output
/// storage images for one eye at a fixed extent.
pub(in super::super) struct PathtraceTargets {
    runtime: Arc<VulkanRuntime>,
    pub extent: vk::Extent2D,
    _cur_radiance: StorageImage,
    _albedo_roughness: StorageImage,
    _normal_depth: [StorageImage; 2],
    _motion_id: StorageImage,
    _emission: StorageImage,
    _history_color: [StorageImage; 2],
    _moments_hist: [StorageImage; 2],
    _atrous_scratch: [StorageImage; 2],
    _accum_hdr: StorageImage,
    pub out_ldr: StorageImage,
    pool: vk::DescriptorPool,
    /// Path tracer Set 1 (G-buffer outputs) for history slot 0 and 1.
    pub pathtrace_gbuffer_sets: [vk::DescriptorSet; 2],
    /// SVGF Pass 1 (reproject) Set 0 for history slot 0 and 1.
    pub reproject_sets: [vk::DescriptorSet; 2],
    /// SVGF Pass 2 (A-trous):
    /// `[slot][0]` reads `history_color[slot]` -> `atrous_scratch[0]`,
    /// `[slot][1]` reads `atrous_scratch[0]` -> `atrous_scratch[1]`,
    /// `[slot][2]` reads `atrous_scratch[1]` -> `atrous_scratch[0]`.
    pub atrous_sets: [[vk::DescriptorSet; 3]; 2],
    /// SVGF Pass 3 (compose + tonemap):
    /// `[0]` reads `cur_radiance` (denoiser off),
    /// `[1]` reads `atrous_scratch[0]` (odd A-trous iterations),
    /// `[2]` reads `atrous_scratch[1]` (even A-trous iterations),
    /// `[3]` reads `history_color[0]` (0 A-trous iterations, slot 0),
    /// `[4]` reads `history_color[1]` (0 A-trous iterations, slot 1).
    pub compose_sets: [vk::DescriptorSet; 5],
    slot_cursor: AtomicUsize,
}

impl PathtraceTargets {
    pub fn new(
        runtime: &Arc<VulkanRuntime>,
        extent: vk::Extent2D,
        layouts: &ComputeRaytraceLayouts,
    ) -> Result<Self, RenderError> {
        let f16 = vk::Format::R16G16B16A16_SFLOAT;
        let f32 = vk::Format::R32G32B32A32_SFLOAT;
        let cur_radiance = StorageImage::new(runtime, extent, f16, vk::ImageUsageFlags::empty())?;
        let albedo_roughness =
            StorageImage::new(runtime, extent, f16, vk::ImageUsageFlags::empty())?;
        let normal_depth = [
            StorageImage::new(runtime, extent, f32, vk::ImageUsageFlags::empty())?,
            StorageImage::new(runtime, extent, f32, vk::ImageUsageFlags::empty())?,
        ];
        let motion_id = StorageImage::new(runtime, extent, f32, vk::ImageUsageFlags::empty())?;
        let emission = StorageImage::new(runtime, extent, f16, vk::ImageUsageFlags::empty())?;
        let history_color = [
            StorageImage::new(runtime, extent, f16, vk::ImageUsageFlags::empty())?,
            StorageImage::new(runtime, extent, f16, vk::ImageUsageFlags::empty())?,
        ];
        let moments_hist = [
            StorageImage::new(runtime, extent, f32, vk::ImageUsageFlags::empty())?,
            StorageImage::new(runtime, extent, f32, vk::ImageUsageFlags::empty())?,
        ];
        let atrous_scratch = [
            StorageImage::new(runtime, extent, f16, vk::ImageUsageFlags::empty())?,
            StorageImage::new(runtime, extent, f16, vk::ImageUsageFlags::empty())?,
        ];
        let accum_hdr = StorageImage::new(runtime, extent, f32, vk::ImageUsageFlags::empty())?;
        let out_ldr = StorageImage::new(
            runtime,
            extent,
            vk::Format::R8G8B8A8_UNORM,
            vk::ImageUsageFlags::TRANSFER_SRC,
        )?;

        initialize_all_images_general(
            runtime,
            &[
                cur_radiance.handle,
                albedo_roughness.handle,
                normal_depth[0].handle,
                normal_depth[1].handle,
                motion_id.handle,
                emission.handle,
                history_color[0].handle,
                history_color[1].handle,
                moments_hist[0].handle,
                moments_hist[1].handle,
                atrous_scratch[0].handle,
                atrous_scratch[1].handle,
                accum_hdr.handle,
                out_ldr.handle,
            ],
        )?;

        let pool_sizes = [vk::DescriptorPoolSize {
            ty: vk::DescriptorType::STORAGE_IMAGE,
            descriptor_count: 80,
        }];
        let pool_info = vk::DescriptorPoolCreateInfo::default()
            .max_sets(16)
            .pool_sizes(&pool_sizes);
        // SAFETY: pool_info borrows live slice; device stays valid for runtime's lifetime.
        let pool = unsafe { runtime.device().create_descriptor_pool(&pool_info, None) }
            .map_err(|e| vk_error("create raytrace targets descriptor pool", e))?;

        let set_layouts = [
            layouts.gbuffer,
            layouts.gbuffer,
            layouts.reproject,
            layouts.reproject,
            layouts.atrous,
            layouts.atrous,
            layouts.atrous,
            layouts.atrous,
            layouts.atrous,
            layouts.atrous,
            layouts.compose,
            layouts.compose,
            layouts.compose,
            layouts.compose,
            layouts.compose,
        ];
        let alloc_info = vk::DescriptorSetAllocateInfo::default()
            .descriptor_pool(pool)
            .set_layouts(&set_layouts);
        // SAFETY: pool and layouts are live on this device.
        let sets = match unsafe { runtime.device().allocate_descriptor_sets(&alloc_info) } {
            Ok(sets) => sets,
            Err(err) => {
                // SAFETY: pool belongs to this device.
                unsafe { runtime.device().destroy_descriptor_pool(pool, None) };
                return Err(vk_error("allocate raytrace targets descriptor sets", err));
            }
        };

        let pathtrace_gbuffer_sets = [sets[0], sets[1]];
        let reproject_sets = [sets[2], sets[3]];
        let atrous_sets = [[sets[4], sets[5], sets[6]], [sets[7], sets[8], sets[9]]];
        let compose_sets = [sets[10], sets[11], sets[12], sets[13], sets[14]];

        for slot in 0..2 {
            let prev = 1 - slot;
            write_storage_set(
                runtime,
                pathtrace_gbuffer_sets[slot],
                &[
                    cur_radiance.view,
                    albedo_roughness.view,
                    normal_depth[slot].view,
                    motion_id.view,
                    emission.view,
                ],
            );
            write_storage_set(
                runtime,
                reproject_sets[slot],
                &[
                    cur_radiance.view,
                    normal_depth[slot].view,
                    motion_id.view,
                    history_color[prev].view,
                    normal_depth[prev].view,
                    moments_hist[prev].view,
                    history_color[slot].view,
                    moments_hist[slot].view,
                ],
            );
            write_storage_set(
                runtime,
                atrous_sets[slot][0],
                &[
                    history_color[slot].view,
                    normal_depth[slot].view,
                    atrous_scratch[0].view,
                ],
            );
            write_storage_set(
                runtime,
                atrous_sets[slot][1],
                &[
                    atrous_scratch[0].view,
                    normal_depth[slot].view,
                    atrous_scratch[1].view,
                ],
            );
            write_storage_set(
                runtime,
                atrous_sets[slot][2],
                &[
                    atrous_scratch[1].view,
                    normal_depth[slot].view,
                    atrous_scratch[0].view,
                ],
            );
        }

        let compose_sources = [
            cur_radiance.view,
            atrous_scratch[0].view,
            atrous_scratch[1].view,
            history_color[0].view,
            history_color[1].view,
        ];
        for (idx, src_view) in compose_sources.into_iter().enumerate() {
            write_storage_set(
                runtime,
                compose_sets[idx],
                &[
                    src_view,
                    albedo_roughness.view,
                    emission.view,
                    accum_hdr.view,
                    out_ldr.view,
                ],
            );
        }

        Ok(Self {
            runtime: runtime.clone(),
            extent,
            _cur_radiance: cur_radiance,
            _albedo_roughness: albedo_roughness,
            _normal_depth: normal_depth,
            _motion_id: motion_id,
            _emission: emission,
            _history_color: history_color,
            _moments_hist: moments_hist,
            _atrous_scratch: atrous_scratch,
            _accum_hdr: accum_hdr,
            out_ldr,
            pool,
            pathtrace_gbuffer_sets,
            reproject_sets,
            atrous_sets,
            compose_sets,
            slot_cursor: AtomicUsize::new(0),
        })
    }

    /// Advances the ping-pong history index and returns the current slot (`0` or `1`).
    pub fn next_slot(&self) -> usize {
        self.slot_cursor.fetch_add(1, Ordering::AcqRel) & 1
    }
}

impl Drop for PathtraceTargets {
    fn drop(&mut self) {
        // SAFETY: pool belongs to this device and drops after GPU work finishes.
        unsafe {
            self.runtime
                .device()
                .destroy_descriptor_pool(self.pool, None)
        };
    }
}

pub(super) fn color_range() -> vk::ImageSubresourceRange {
    vk::ImageSubresourceRange::default()
        .aspect_mask(vk::ImageAspectFlags::COLOR)
        .level_count(1)
        .layer_count(1)
}

fn initialize_all_images_general(
    runtime: &Arc<VulkanRuntime>,
    images: &[vk::Image],
) -> Result<(), RenderError> {
    let submission = CommandSubmission::new(runtime)?;
    let to_clear: Vec<_> = images
        .iter()
        .map(|&image| {
            vk::ImageMemoryBarrier2::default()
                .image(image)
                .subresource_range(color_range())
                .old_layout(vk::ImageLayout::UNDEFINED)
                .new_layout(vk::ImageLayout::TRANSFER_DST_OPTIMAL)
                .dst_stage_mask(vk::PipelineStageFlags2::CLEAR)
                .dst_access_mask(vk::AccessFlags2::TRANSFER_WRITE)
        })
        .collect();
    let to_general: Vec<_> = images
        .iter()
        .map(|&image| {
            vk::ImageMemoryBarrier2::default()
                .image(image)
                .subresource_range(color_range())
                .old_layout(vk::ImageLayout::TRANSFER_DST_OPTIMAL)
                .new_layout(vk::ImageLayout::GENERAL)
                .src_stage_mask(vk::PipelineStageFlags2::CLEAR)
                .src_access_mask(vk::AccessFlags2::TRANSFER_WRITE)
                .dst_stage_mask(vk::PipelineStageFlags2::COMPUTE_SHADER)
                .dst_access_mask(
                    vk::AccessFlags2::SHADER_STORAGE_READ | vk::AccessFlags2::SHADER_STORAGE_WRITE,
                )
        })
        .collect();
    let zero = vk::ClearColorValue { float32: [0.0; 4] };
    let range = [color_range()];
    // SAFETY: command buffer is recording; all image handles are live until submission finishes.
    unsafe {
        let dev = runtime.device();
        dev.cmd_pipeline_barrier2(
            submission.command,
            &vk::DependencyInfo::default().image_memory_barriers(&to_clear),
        );
        for &img in images {
            dev.cmd_clear_color_image(
                submission.command,
                img,
                vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                &zero,
                &range,
            );
        }
        dev.cmd_pipeline_barrier2(
            submission.command,
            &vk::DependencyInfo::default().image_memory_barriers(&to_general),
        );
    }
    submission.finish()
}

pub(super) fn write_storage_set(
    runtime: &VulkanRuntime,
    set: vk::DescriptorSet,
    views: &[vk::ImageView],
) {
    let infos: Vec<_> = views
        .iter()
        .map(|&view| {
            [vk::DescriptorImageInfo::default()
                .image_view(view)
                .image_layout(vk::ImageLayout::GENERAL)]
        })
        .collect();
    let writes: Vec<_> = infos
        .iter()
        .enumerate()
        .map(|(binding, info)| {
            vk::WriteDescriptorSet::default()
                .dst_set(set)
                .dst_binding(binding as u32)
                .descriptor_type(vk::DescriptorType::STORAGE_IMAGE)
                .image_info(info)
        })
        .collect();
    // SAFETY: set and image views are live on this device.
    unsafe { runtime.device().update_descriptor_sets(&writes, &[]) };
}

//! Unified OpenPBR Surface v1.1 compute path tracer, Vulkan hardware ray tracing
//! (`VK_KHR_acceleration_structure` + `VK_KHR_ray_query`), 3-pass SVGF spatiotemporal denoiser,
//! and HDR AgX/ACES/Filmic tone mapper.
pub(super) mod accel;
pub(super) mod device;
pub(super) mod pipeline;
pub(super) mod targets;

use super::buffer::DeviceBuffer;
use super::frame::FrameResources;
use super::memory_selection::HostBufferAccess;
use super::pbr_environment::PbrEnvironment;
use super::runtime::{VulkanRuntime, vk_error};
use super::submission::CommandSubmission;
use crate::raytrace::pt_scene::{
    PathtraceCameraUbo, PtSceneData, encode_bvh_bytes, encode_camera_ubo_bytes,
    encode_lights_bytes, encode_materials_bytes, encode_triangles_bytes, encode_vertices_bytes,
};
use crate::{RenderError, error::rejected};
use accel::{BlasCache, BottomLevelAccel, TlasInstanceInput, TopLevelAccel};
use ash::vk;
use device::ResolvedRaytraceBackend;
use martensite_tkservice::graphic3d::{Graphic3dRayTracingBackend, Graphic3dToneMappingMethod};
use pipeline::ComputeRaytracePipelines;
use std::sync::{Arc, Mutex};
use targets::{PathtraceTargets, StorageImage, color_range, write_storage_set};

/// Persistent renderer-owned caches for OpenPBR compute path tracing and SVGF targets.
#[derive(Default)]
pub(in super::super) struct ComputeRaytraceCaches {
    pipelines: Mutex<Option<Arc<ComputeRaytracePipelines>>>,
    blas_cache: BlasCache,
    targets: Mutex<[Option<Arc<PathtraceTargets>>; 2]>,
}

impl ComputeRaytraceCaches {
    pub fn pipelines(
        &self,
        runtime: &Arc<VulkanRuntime>,
    ) -> Result<Arc<ComputeRaytracePipelines>, RenderError> {
        let mut guard = self
            .pipelines
            .lock()
            .map_err(|e| rejected(e.to_string(), "unpoisoned compute raytrace pipelines"))?;
        if let Some(p) = guard.as_ref() {
            return Ok(p.clone());
        }
        let created = Arc::new(ComputeRaytracePipelines::new(runtime)?);
        *guard = Some(created.clone());
        Ok(created)
    }

    pub fn targets(
        &self,
        runtime: &Arc<VulkanRuntime>,
        extent: vk::Extent2D,
        eye: usize,
        pipelines: &ComputeRaytracePipelines,
    ) -> Result<Arc<PathtraceTargets>, RenderError> {
        let mut guard = self
            .targets
            .lock()
            .map_err(|e| rejected(e.to_string(), "unpoisoned compute raytrace targets"))?;
        let slot = &mut guard[eye.min(1)];
        if let Some(existing) = slot.as_ref().filter(|t| t.extent == extent) {
            return Ok(existing.clone());
        }
        let created = Arc::new(PathtraceTargets::new(runtime, extent, &pipelines.layouts)?);
        *slot = Some(created.clone());
        Ok(created)
    }
}

/// Uploaded GPU buffers, acceleration structures, and descriptor set for one ray-traced frame.
struct UploadedSceneSet {
    runtime: Arc<VulkanRuntime>,
    pool: vk::DescriptorPool,
    pub set: vk::DescriptorSet,
    _ubo: DeviceBuffer,
    _vertices: DeviceBuffer,
    _triangles: DeviceBuffer,
    _materials: DeviceBuffer,
    _lights: DeviceBuffer,
    _bvh: DeviceBuffer,
    _blas: Option<Arc<BottomLevelAccel>>,
    _tlas: Option<TopLevelAccel>,
}

impl Drop for UploadedSceneSet {
    fn drop(&mut self) {
        // SAFETY: pool belongs to this device and drops only after GPU submission completes.
        unsafe {
            self.runtime
                .device()
                .destroy_descriptor_pool(self.pool, None)
        };
    }
}

/// Prepared OpenPBR compute path-tracing + SVGF + tone-mapping pass for one frame.
pub(in super::super) struct ComputeRaytraceDraw {
    pub span: usize,
    pub backend: ResolvedRaytraceBackend,
    pipelines: Arc<ComputeRaytracePipelines>,
    targets: Arc<PathtraceTargets>,
    scene: UploadedSceneSet,
    pbr_env: Arc<PbrEnvironment>,
    history_slot: usize,
    pub extent: vk::Extent2D,
    pub denoiser_enabled: bool,
    pub denoiser_atrous_iterations: u32,
    pub denoiser_temporal_alpha: f32,
    pub reset_history: bool,
    pub tone_mapping: u32,
    pub exposure: f32,
    pub white_point: f32,
    pub accum_frames: u32,
}

impl ComputeRaytraceDraw {
    /// Kept for interface parity with `FrameResources::completed_readback`.
    pub fn publish_feedback(&self) -> Result<(), RenderError> {
        Ok(())
    }
}

/// Parameters for one compute path-tracing frame (from either a `RaytraceFrame` or direct test).
pub(in super::super) struct ComputeRaytraceInputs<'a> {
    pub span: usize,
    pub eye_targets: usize,
    pub extent: vk::Extent2D,
    pub requested_backend: Graphic3dRayTracingBackend,
    pub ubo: PathtraceCameraUbo,
    pub scene: &'a PtSceneData,
    pub pbr_env: Arc<PbrEnvironment>,
    pub denoiser_enabled: bool,
    pub denoiser_atrous_iterations: u32,
    pub denoiser_temporal_alpha: f32,
    pub reset_history: bool,
    pub tone_mapping: Graphic3dToneMappingMethod,
    pub exposure: f32,
    pub white_point: f32,
    pub accum_frames: u32,
}

/// Prepares GPU buffers, BLAS/TLAS (for HW RT), and descriptor sets for one compute path-trace pass.
pub(in super::super) fn prepare_compute_raytrace(
    runtime: &Arc<VulkanRuntime>,
    caches: &ComputeRaytraceCaches,
    inputs: ComputeRaytraceInputs<'_>,
) -> Result<ComputeRaytraceDraw, RenderError> {
    let backend = device::admit_backend(inputs.requested_backend, runtime.hw_raytrace.is_some())?;
    let pipelines = caches.pipelines(runtime)?;
    let targets = caches.targets(runtime, inputs.extent, inputs.eye_targets, &pipelines)?;
    let history_slot = targets.next_slot();

    let ubo_bytes = encode_camera_ubo_bytes(&inputs.ubo);
    let ubo = upload_buffer(runtime, vk::BufferUsageFlags::UNIFORM_BUFFER, &ubo_bytes)?;
    let vertices = upload_buffer(
        runtime,
        vk::BufferUsageFlags::STORAGE_BUFFER,
        &encode_vertices_bytes(&inputs.scene.vertices),
    )?;
    let triangles = upload_buffer(
        runtime,
        vk::BufferUsageFlags::STORAGE_BUFFER,
        &encode_triangles_bytes(&inputs.scene.triangles),
    )?;
    let materials = upload_buffer(
        runtime,
        vk::BufferUsageFlags::STORAGE_BUFFER,
        &encode_materials_bytes(&inputs.scene.materials),
    )?;
    let lights = upload_buffer(
        runtime,
        vk::BufferUsageFlags::STORAGE_BUFFER,
        &encode_lights_bytes(&inputs.scene.lights),
    )?;
    let bvh = upload_buffer(
        runtime,
        vk::BufferUsageFlags::STORAGE_BUFFER,
        &encode_bvh_bytes(&inputs.scene.bvh_nodes),
    )?;

    let (blas, tlas) = if backend == ResolvedRaytraceBackend::HardwareRayQuery {
        build_hw_accel(runtime, &caches.blas_cache, inputs.scene)?
    } else {
        (None, None)
    };

    let scene = allocate_scene_set(
        runtime,
        &pipelines,
        backend,
        (ubo, vertices, triangles, materials, lights, bvh),
        (blas, tlas),
    )?;

    Ok(ComputeRaytraceDraw {
        span: inputs.span,
        backend,
        pipelines,
        targets,
        scene,
        pbr_env: inputs.pbr_env,
        history_slot,
        extent: inputs.extent,
        denoiser_enabled: inputs.denoiser_enabled,
        denoiser_atrous_iterations: inputs.denoiser_atrous_iterations.min(5),
        denoiser_temporal_alpha: inputs.denoiser_temporal_alpha.clamp(0.01, 1.0),
        reset_history: inputs.reset_history,
        tone_mapping: encode_tonemap_method(inputs.tone_mapping),
        exposure: inputs.exposure,
        white_point: inputs.white_point,
        accum_frames: inputs.accum_frames,
    })
}

/// Records compute path tracing, optional 3-pass SVGF denoising, HDR tone mapping, and copies
/// the result into `resources.color`.
pub(in super::super) fn record_compute_raytrace(
    runtime: &VulkanRuntime,
    command: vk::CommandBuffer,
    resources: &FrameResources,
    draw: &ComputeRaytraceDraw,
) {
    let dev = runtime.device();
    let gx = draw.extent.width.div_ceil(8);
    let gy = draw.extent.height.div_ceil(8);

    // 1. Unified OpenPBR compute path tracer (HW rayQueryEXT or SW BVH).
    let pt_pipe = match draw.backend {
        ResolvedRaytraceBackend::HardwareRayQuery => draw
            .pipelines
            .pathtrace_hw
            .as_ref()
            .unwrap_or(&draw.pipelines.pathtrace_sw),
        ResolvedRaytraceBackend::SoftwareComputeBvh => &draw.pipelines.pathtrace_sw,
    };
    let pt_sets = [
        draw.scene.set,
        draw.targets.pathtrace_gbuffer_sets[draw.history_slot],
        draw.pbr_env.set,
    ];
    // SAFETY: command buffer is recording; pipelines and descriptor sets outlive submission.
    unsafe {
        dev.cmd_bind_pipeline(command, vk::PipelineBindPoint::COMPUTE, pt_pipe.handle);
        dev.cmd_bind_descriptor_sets(
            command,
            vk::PipelineBindPoint::COMPUTE,
            pt_pipe.layout,
            0,
            &pt_sets,
            &[],
        );
        dev.cmd_dispatch(command, gx, gy, 1);
    }
    compute_barrier(dev, command);

    // 2. Optional 3-pass SVGF spatiotemporal denoiser.
    let compose_set_idx = if draw.denoiser_enabled {
        // Pass 1: Temporal reprojection & variance estimation.
        let reproj_push = pack_reproject_push(
            draw.extent.width,
            draw.extent.height,
            draw.denoiser_temporal_alpha,
            u32::from(draw.reset_history),
        );
        // SAFETY: command buffer is recording; pipeline and descriptor sets outlive submission.
        unsafe {
            dev.cmd_bind_pipeline(
                command,
                vk::PipelineBindPoint::COMPUTE,
                draw.pipelines.svgf_reproject.handle,
            );
            dev.cmd_bind_descriptor_sets(
                command,
                vk::PipelineBindPoint::COMPUTE,
                draw.pipelines.svgf_reproject.layout,
                0,
                &[draw.targets.reproject_sets[draw.history_slot]],
                &[],
            );
            dev.cmd_push_constants(
                command,
                draw.pipelines.svgf_reproject.layout,
                vk::ShaderStageFlags::COMPUTE,
                0,
                &reproj_push,
            );
            dev.cmd_dispatch(command, gx, gy, 1);
        }
        compute_barrier(dev, command);

        // Pass 2: Multi-scale 5x5 A-trous wavelet iterations.
        let iters = draw.denoiser_atrous_iterations;
        for iter in 0..iters {
            let set_idx = if iter == 0 {
                0
            } else if iter % 2 == 1 {
                1
            } else {
                2
            };
            let atrous_push = pack_atrous_push(
                draw.extent.width,
                draw.extent.height,
                1i32 << iter,
                4.0,
                128.0,
                0.02,
            );
            // SAFETY: command buffer is recording; handles outlive submission.
            unsafe {
                dev.cmd_bind_pipeline(
                    command,
                    vk::PipelineBindPoint::COMPUTE,
                    draw.pipelines.svgf_atrous.handle,
                );
                dev.cmd_bind_descriptor_sets(
                    command,
                    vk::PipelineBindPoint::COMPUTE,
                    draw.pipelines.svgf_atrous.layout,
                    0,
                    &[draw.targets.atrous_sets[draw.history_slot][set_idx]],
                    &[],
                );
                dev.cmd_push_constants(
                    command,
                    draw.pipelines.svgf_atrous.layout,
                    vk::ShaderStageFlags::COMPUTE,
                    0,
                    &atrous_push,
                );
                dev.cmd_dispatch(command, gx, gy, 1);
            }
            compute_barrier(dev, command);
        }

        if iters == 0 {
            3 + draw.history_slot
        } else if iters % 2 == 1 {
            1
        } else {
            2
        }
    } else {
        0
    };

    // 3. Albedo remodulation, emission/background composition, progressive accumulation, & HDR tone map.
    let compose_push = pack_compose_push(
        draw.extent.width,
        draw.extent.height,
        draw.tone_mapping,
        draw.exposure,
        draw.white_point,
        u32::from(draw.denoiser_enabled),
        draw.accum_frames,
        1,
    );
    // SAFETY: command buffer is recording; handles outlive submission.
    unsafe {
        dev.cmd_bind_pipeline(
            command,
            vk::PipelineBindPoint::COMPUTE,
            draw.pipelines.svgf_compose.handle,
        );
        dev.cmd_bind_descriptor_sets(
            command,
            vk::PipelineBindPoint::COMPUTE,
            draw.pipelines.svgf_compose.layout,
            0,
            &[draw.targets.compose_sets[compose_set_idx]],
            &[],
        );
        dev.cmd_push_constants(
            command,
            draw.pipelines.svgf_compose.layout,
            vk::ShaderStageFlags::COMPUTE,
            0,
            &compose_push,
        );
        dev.cmd_dispatch(command, gx, gy, 1);
    }

    // 4. Copy sRGB-encoded `out_ldr` (`R8G8B8A8_UNORM`) directly into `resources.color` (`R8G8B8A8_SRGB`).
    copy_ldr_to_frame_color(
        dev,
        command,
        &draw.targets.out_ldr,
        &resources.color,
        draw.extent,
    );

    // Clear depth to 1.0 in ATTACHMENT_OPTIMAL if this raytrace pass starts the frame so any
    // subsequent raster overlay layers (trihedron, viewcube, annotations) depth-test cleanly.
    if draw.span == 0 && resources.background.is_none() {
        let depth_clear = vk::RenderingAttachmentInfo::default()
            .image_view(resources.render_depth().view)
            .image_layout(vk::ImageLayout::ATTACHMENT_OPTIMAL)
            .load_op(vk::AttachmentLoadOp::CLEAR)
            .store_op(vk::AttachmentStoreOp::STORE)
            .clear_value(vk::ClearValue {
                depth_stencil: vk::ClearDepthStencilValue {
                    depth: 1.0,
                    stencil: 0,
                },
            });
        let render_area = vk::Rect2D {
            offset: vk::Offset2D { x: 0, y: 0 },
            extent: resources.attachment_extent,
        };
        let info = vk::RenderingInfo::default()
            .render_area(render_area)
            .layer_count(1)
            .depth_attachment(&depth_clear);
        // SAFETY: command buffer is recording; depth attachment is in ATTACHMENT_OPTIMAL.
        unsafe {
            dev.cmd_begin_rendering(command, &info);
            dev.cmd_end_rendering(command);
        }
    }
}

/// Runs the standalone GPU HDR tone-mapping compute shader (`tonemap.comp`) on a linear
/// `width × height` RGBA32F pixel slice and returns sRGB-encoded RGBA8 bytes.
pub(in super::super) fn run_standalone_tonemap(
    runtime: &Arc<VulkanRuntime>,
    caches: &ComputeRaytraceCaches,
    extent: (u32, u32),
    hdr_rgba: &[[f32; 4]],
    method: Graphic3dToneMappingMethod,
    exposure: f32,
    white_point: f32,
) -> Result<Vec<u8>, RenderError> {
    let (width, height) = extent;
    let expected = (width as usize).saturating_mul(height as usize);
    if hdr_rgba.len() != expected || expected == 0 {
        return Err(rejected(
            (hdr_rgba.len(), width, height),
            "non-empty RGBA HDR buffer matching width * height",
        ));
    }
    let pipelines = caches.pipelines(runtime)?;
    let extent = vk::Extent2D { width, height };
    let hdr_img = StorageImage::new(
        runtime,
        extent,
        vk::Format::R16G16B16A16_SFLOAT,
        vk::ImageUsageFlags::TRANSFER_DST,
    )?;
    let ldr_img = StorageImage::new(
        runtime,
        extent,
        vk::Format::R8G8B8A8_UNORM,
        vk::ImageUsageFlags::TRANSFER_SRC,
    )?;

    let mut f16_bytes = Vec::with_capacity(expected * 8);
    for px in hdr_rgba {
        for &c in px {
            let h = crate::openpbr_luts::f32_to_f16_bits(c);
            f16_bytes.extend_from_slice(&h.to_le_bytes());
        }
    }
    let staging_in = DeviceBuffer::new(
        runtime,
        f16_bytes.len() as u64,
        vk::BufferUsageFlags::TRANSFER_SRC,
        HostBufferAccess::Upload,
    )?;
    staging_in.write(&f16_bytes)?;
    let readback = DeviceBuffer::new(
        runtime,
        (expected * 4) as u64,
        vk::BufferUsageFlags::TRANSFER_DST,
        HostBufferAccess::Readback,
    )?;

    let pool_sizes = [vk::DescriptorPoolSize {
        ty: vk::DescriptorType::STORAGE_IMAGE,
        descriptor_count: 2,
    }];
    let pool_info = vk::DescriptorPoolCreateInfo::default()
        .max_sets(1)
        .pool_sizes(&pool_sizes);
    // SAFETY: pool_info borrows live slice on this device.
    let pool = unsafe { runtime.device().create_descriptor_pool(&pool_info, None) }
        .map_err(|e| vk_error("create tonemap descriptor pool", e))?;
    let layouts = [pipelines.layouts.tonemap];
    let alloc = vk::DescriptorSetAllocateInfo::default()
        .descriptor_pool(pool)
        .set_layouts(&layouts);
    // SAFETY: pool and layout are live on this device.
    let set = match unsafe { runtime.device().allocate_descriptor_sets(&alloc) } {
        Ok(mut sets) => sets.remove(0),
        Err(err) => {
            // SAFETY: pool belongs to this device.
            unsafe { runtime.device().destroy_descriptor_pool(pool, None) };
            return Err(vk_error("allocate tonemap descriptor set", err));
        }
    };
    write_storage_set(runtime, set, &[hdr_img.view, ldr_img.view]);

    let push = pack_tonemap_push(
        width,
        height,
        encode_tonemap_method(method),
        exposure,
        white_point,
        1,
    );
    let sub = CommandSubmission::new(runtime)?;
    let dev = runtime.device();
    let copy_region = vk::BufferImageCopy::default()
        .image_subresource(vk::ImageSubresourceLayers {
            aspect_mask: vk::ImageAspectFlags::COLOR,
            mip_level: 0,
            base_array_layer: 0,
            layer_count: 1,
        })
        .image_extent(vk::Extent3D {
            width,
            height,
            depth: 1,
        });
    // SAFETY: command buffer is recording; all buffers, images, and set outlive sub.finish().
    unsafe {
        let prep = [
            vk::ImageMemoryBarrier2::default()
                .image(hdr_img.handle)
                .subresource_range(color_range())
                .old_layout(vk::ImageLayout::UNDEFINED)
                .new_layout(vk::ImageLayout::TRANSFER_DST_OPTIMAL)
                .dst_stage_mask(vk::PipelineStageFlags2::COPY)
                .dst_access_mask(vk::AccessFlags2::TRANSFER_WRITE),
            vk::ImageMemoryBarrier2::default()
                .image(ldr_img.handle)
                .subresource_range(color_range())
                .old_layout(vk::ImageLayout::UNDEFINED)
                .new_layout(vk::ImageLayout::GENERAL)
                .dst_stage_mask(vk::PipelineStageFlags2::COMPUTE_SHADER)
                .dst_access_mask(vk::AccessFlags2::SHADER_STORAGE_WRITE),
        ];
        dev.cmd_pipeline_barrier2(
            sub.command,
            &vk::DependencyInfo::default().image_memory_barriers(&prep),
        );
        dev.cmd_copy_buffer_to_image(
            sub.command,
            staging_in.handle,
            hdr_img.handle,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            &[copy_region],
        );
        let to_compute = [vk::ImageMemoryBarrier2::default()
            .image(hdr_img.handle)
            .subresource_range(color_range())
            .old_layout(vk::ImageLayout::TRANSFER_DST_OPTIMAL)
            .new_layout(vk::ImageLayout::GENERAL)
            .src_stage_mask(vk::PipelineStageFlags2::COPY)
            .src_access_mask(vk::AccessFlags2::TRANSFER_WRITE)
            .dst_stage_mask(vk::PipelineStageFlags2::COMPUTE_SHADER)
            .dst_access_mask(vk::AccessFlags2::SHADER_STORAGE_READ)];
        dev.cmd_pipeline_barrier2(
            sub.command,
            &vk::DependencyInfo::default().image_memory_barriers(&to_compute),
        );
        dev.cmd_bind_pipeline(
            sub.command,
            vk::PipelineBindPoint::COMPUTE,
            pipelines.tonemap.handle,
        );
        dev.cmd_bind_descriptor_sets(
            sub.command,
            vk::PipelineBindPoint::COMPUTE,
            pipelines.tonemap.layout,
            0,
            &[set],
            &[],
        );
        dev.cmd_push_constants(
            sub.command,
            pipelines.tonemap.layout,
            vk::ShaderStageFlags::COMPUTE,
            0,
            &push,
        );
        dev.cmd_dispatch(sub.command, width.div_ceil(8), height.div_ceil(8), 1);
        let to_readback = [vk::ImageMemoryBarrier2::default()
            .image(ldr_img.handle)
            .subresource_range(color_range())
            .old_layout(vk::ImageLayout::GENERAL)
            .new_layout(vk::ImageLayout::TRANSFER_SRC_OPTIMAL)
            .src_stage_mask(vk::PipelineStageFlags2::COMPUTE_SHADER)
            .src_access_mask(vk::AccessFlags2::SHADER_STORAGE_WRITE)
            .dst_stage_mask(vk::PipelineStageFlags2::COPY)
            .dst_access_mask(vk::AccessFlags2::TRANSFER_READ)];
        dev.cmd_pipeline_barrier2(
            sub.command,
            &vk::DependencyInfo::default().image_memory_barriers(&to_readback),
        );
        dev.cmd_copy_image_to_buffer(
            sub.command,
            ldr_img.handle,
            vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
            readback.handle,
            &[copy_region],
        );
    }
    let outcome = sub.finish();
    // SAFETY: pool belongs to this device and GPU work has completed.
    unsafe { dev.destroy_descriptor_pool(pool, None) };
    outcome?;
    readback.read(expected * 4)
}

pub(super) fn encode_tonemap_method(method: Graphic3dToneMappingMethod) -> u32 {
    match method {
        Graphic3dToneMappingMethod::Disabled => 0,
        Graphic3dToneMappingMethod::Filmic => 1,
        Graphic3dToneMappingMethod::AgX => 2,
        Graphic3dToneMappingMethod::Aces => 3,
    }
}

fn upload_buffer(
    runtime: &Arc<VulkanRuntime>,
    usage: vk::BufferUsageFlags,
    bytes: &[u8],
) -> Result<DeviceBuffer, RenderError> {
    let size = bytes.len().max(16) as u64;
    let buf = DeviceBuffer::new(runtime, size, usage, HostBufferAccess::Upload)?;
    buf.write(bytes)?;
    Ok(buf)
}

fn build_hw_accel(
    runtime: &Arc<VulkanRuntime>,
    blas_cache: &BlasCache,
    scene: &PtSceneData,
) -> Result<(Option<Arc<BottomLevelAccel>>, Option<TopLevelAccel>), RenderError> {
    if scene.triangles.is_empty() || scene.vertices.is_empty() {
        return Ok((None, None));
    }
    let blas = blas_cache.get_or_build(scene.geometry_hash, || {
        let pos_bytes = scene.blas_position_bytes();
        let idx_bytes = scene.blas_index_bytes();
        BottomLevelAccel::build_triangles(
            runtime,
            &pos_bytes,
            scene.vertices.len() as u32,
            &idx_bytes,
            scene.triangles.len() as u32,
        )
    })?;
    let identity_3x4 = [
        1.0, 0.0, 0.0, 0.0, // row 0
        0.0, 1.0, 0.0, 0.0, // row 1
        0.0, 0.0, 1.0, 0.0, // row 2
    ];
    let tlas = TopLevelAccel::build(
        runtime,
        &[TlasInstanceInput {
            transform_3x4: identity_3x4,
            custom_index: 0,
            blas_address: blas.device_address,
            opaque: true,
        }],
    )?;
    Ok((Some(blas), Some(tlas)))
}

fn allocate_scene_set(
    runtime: &Arc<VulkanRuntime>,
    pipelines: &ComputeRaytracePipelines,
    backend: ResolvedRaytraceBackend,
    buffers: (
        DeviceBuffer,
        DeviceBuffer,
        DeviceBuffer,
        DeviceBuffer,
        DeviceBuffer,
        DeviceBuffer,
    ),
    accel: (Option<Arc<BottomLevelAccel>>, Option<TopLevelAccel>),
) -> Result<UploadedSceneSet, RenderError> {
    let (ubo, vertices, triangles, materials, lights, bvh) = buffers;
    let (blas, tlas) = accel;
    let use_hw = backend == ResolvedRaytraceBackend::HardwareRayQuery && tlas.is_some();
    let mut pool_sizes = vec![
        vk::DescriptorPoolSize {
            ty: vk::DescriptorType::UNIFORM_BUFFER,
            descriptor_count: 1,
        },
        vk::DescriptorPoolSize {
            ty: vk::DescriptorType::STORAGE_BUFFER,
            descriptor_count: 5,
        },
    ];
    if use_hw {
        pool_sizes.push(vk::DescriptorPoolSize {
            ty: vk::DescriptorType::ACCELERATION_STRUCTURE_KHR,
            descriptor_count: 1,
        });
    }
    let pool_info = vk::DescriptorPoolCreateInfo::default()
        .max_sets(1)
        .pool_sizes(&pool_sizes);
    // SAFETY: pool_info borrows live slice on this device.
    let pool = unsafe { runtime.device().create_descriptor_pool(&pool_info, None) }
        .map_err(|e| vk_error("create compute raytrace scene pool", e))?;
    let layout = if use_hw {
        pipelines
            .layouts
            .scene_hw
            .unwrap_or(pipelines.layouts.scene_sw)
    } else {
        pipelines.layouts.scene_sw
    };
    let set_layouts = [layout];
    let alloc = vk::DescriptorSetAllocateInfo::default()
        .descriptor_pool(pool)
        .set_layouts(&set_layouts);
    // SAFETY: pool and layout are live on this device.
    let set = match unsafe { runtime.device().allocate_descriptor_sets(&alloc) } {
        Ok(mut sets) => sets.remove(0),
        Err(err) => {
            // SAFETY: pool belongs to this device.
            unsafe { runtime.device().destroy_descriptor_pool(pool, None) };
            return Err(vk_error("allocate compute raytrace scene set", err));
        }
    };

    let ubo_info = [vk::DescriptorBufferInfo::default()
        .buffer(ubo.handle)
        .offset(0)
        .range(vk::WHOLE_SIZE)];
    let ssbo_infos = [
        [vk::DescriptorBufferInfo::default()
            .buffer(vertices.handle)
            .range(vk::WHOLE_SIZE)],
        [vk::DescriptorBufferInfo::default()
            .buffer(triangles.handle)
            .range(vk::WHOLE_SIZE)],
        [vk::DescriptorBufferInfo::default()
            .buffer(materials.handle)
            .range(vk::WHOLE_SIZE)],
        [vk::DescriptorBufferInfo::default()
            .buffer(lights.handle)
            .range(vk::WHOLE_SIZE)],
        [vk::DescriptorBufferInfo::default()
            .buffer(bvh.handle)
            .range(vk::WHOLE_SIZE)],
    ];
    let mut writes = vec![
        vk::WriteDescriptorSet::default()
            .dst_set(set)
            .dst_binding(0)
            .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
            .buffer_info(&ubo_info),
    ];
    for (idx, info) in ssbo_infos.iter().enumerate() {
        writes.push(
            vk::WriteDescriptorSet::default()
                .dst_set(set)
                .dst_binding((idx + 1) as u32)
                .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
                .buffer_info(info),
        );
    }
    let as_handles = tlas.as_ref().map(|t| [t.handle]);
    let mut as_write = as_handles.as_ref().map(|handles| {
        vk::WriteDescriptorSetAccelerationStructureKHR::default().acceleration_structures(handles)
    });
    if let Some(ref mut accel_ext) = as_write
        && use_hw
    {
        writes.push(
            vk::WriteDescriptorSet::default()
                .dst_set(set)
                .dst_binding(6)
                .descriptor_count(1)
                .descriptor_type(vk::DescriptorType::ACCELERATION_STRUCTURE_KHR)
                .push_next(accel_ext),
        );
    }
    // SAFETY: all buffers, AS handles, and write structs stay alive through update_descriptor_sets.
    unsafe { runtime.device().update_descriptor_sets(&writes, &[]) };

    Ok(UploadedSceneSet {
        runtime: runtime.clone(),
        pool,
        set,
        _ubo: ubo,
        _vertices: vertices,
        _triangles: triangles,
        _materials: materials,
        _lights: lights,
        _bvh: bvh,
        _blas: blas,
        _tlas: tlas,
    })
}

fn compute_barrier(device: &ash::Device, command: vk::CommandBuffer) {
    let barrier = [vk::MemoryBarrier2::default()
        .src_stage_mask(vk::PipelineStageFlags2::COMPUTE_SHADER)
        .src_access_mask(vk::AccessFlags2::SHADER_STORAGE_WRITE)
        .dst_stage_mask(vk::PipelineStageFlags2::COMPUTE_SHADER)
        .dst_access_mask(
            vk::AccessFlags2::SHADER_STORAGE_READ | vk::AccessFlags2::SHADER_STORAGE_WRITE,
        )];
    // SAFETY: command buffer is recording on this thread.
    unsafe {
        device.cmd_pipeline_barrier2(
            command,
            &vk::DependencyInfo::default().memory_barriers(&barrier),
        );
    }
}

fn copy_ldr_to_frame_color(
    device: &ash::Device,
    command: vk::CommandBuffer,
    src: &StorageImage,
    dst: &super::image::TargetImage,
    extent: vk::Extent2D,
) {
    let before = [
        vk::ImageMemoryBarrier2::default()
            .image(src.handle)
            .subresource_range(color_range())
            .old_layout(vk::ImageLayout::GENERAL)
            .new_layout(vk::ImageLayout::TRANSFER_SRC_OPTIMAL)
            .src_stage_mask(vk::PipelineStageFlags2::COMPUTE_SHADER)
            .src_access_mask(vk::AccessFlags2::SHADER_STORAGE_WRITE)
            .dst_stage_mask(vk::PipelineStageFlags2::COPY)
            .dst_access_mask(vk::AccessFlags2::TRANSFER_READ),
        vk::ImageMemoryBarrier2::default()
            .image(dst.handle)
            .subresource_range(dst.range())
            .old_layout(vk::ImageLayout::ATTACHMENT_OPTIMAL)
            .new_layout(vk::ImageLayout::TRANSFER_DST_OPTIMAL)
            .src_stage_mask(vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT)
            .src_access_mask(vk::AccessFlags2::COLOR_ATTACHMENT_WRITE)
            .dst_stage_mask(vk::PipelineStageFlags2::COPY)
            .dst_access_mask(vk::AccessFlags2::TRANSFER_WRITE),
    ];
    let layers = vk::ImageSubresourceLayers {
        aspect_mask: vk::ImageAspectFlags::COLOR,
        mip_level: 0,
        base_array_layer: 0,
        layer_count: 1,
    };
    let region = [vk::ImageCopy::default()
        .src_subresource(layers)
        .dst_subresource(layers)
        .extent(vk::Extent3D {
            width: extent.width,
            height: extent.height,
            depth: 1,
        })];
    let after = [
        vk::ImageMemoryBarrier2::default()
            .image(src.handle)
            .subresource_range(color_range())
            .old_layout(vk::ImageLayout::TRANSFER_SRC_OPTIMAL)
            .new_layout(vk::ImageLayout::GENERAL)
            .src_stage_mask(vk::PipelineStageFlags2::COPY)
            .src_access_mask(vk::AccessFlags2::TRANSFER_READ)
            .dst_stage_mask(vk::PipelineStageFlags2::COMPUTE_SHADER)
            .dst_access_mask(vk::AccessFlags2::SHADER_STORAGE_WRITE),
        vk::ImageMemoryBarrier2::default()
            .image(dst.handle)
            .subresource_range(dst.range())
            .old_layout(vk::ImageLayout::TRANSFER_DST_OPTIMAL)
            .new_layout(vk::ImageLayout::ATTACHMENT_OPTIMAL)
            .src_stage_mask(vk::PipelineStageFlags2::COPY)
            .src_access_mask(vk::AccessFlags2::TRANSFER_WRITE)
            .dst_stage_mask(vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT)
            .dst_access_mask(
                vk::AccessFlags2::COLOR_ATTACHMENT_READ | vk::AccessFlags2::COLOR_ATTACHMENT_WRITE,
            ),
    ];
    // SAFETY: command buffer is recording; both images are live and have matching 32-bit formats.
    unsafe {
        device.cmd_pipeline_barrier2(
            command,
            &vk::DependencyInfo::default().image_memory_barriers(&before),
        );
        device.cmd_copy_image(
            command,
            src.handle,
            vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
            dst.handle,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            &region,
        );
        device.cmd_pipeline_barrier2(
            command,
            &vk::DependencyInfo::default().image_memory_barriers(&after),
        );
    }
}

fn pack_reproject_push(width: u32, height: u32, alpha: f32, reset: u32) -> [u8; 16] {
    let mut out = [0u8; 16];
    out[0..4].copy_from_slice(&width.to_ne_bytes());
    out[4..8].copy_from_slice(&height.to_ne_bytes());
    out[8..12].copy_from_slice(&alpha.to_ne_bytes());
    out[12..16].copy_from_slice(&reset.to_ne_bytes());
    out
}

fn pack_atrous_push(
    width: u32,
    height: u32,
    step: i32,
    phi_color: f32,
    phi_normal: f32,
    phi_depth: f32,
) -> [u8; 24] {
    let mut out = [0u8; 24];
    out[0..4].copy_from_slice(&width.to_ne_bytes());
    out[4..8].copy_from_slice(&height.to_ne_bytes());
    out[8..12].copy_from_slice(&step.to_ne_bytes());
    out[12..16].copy_from_slice(&phi_color.to_ne_bytes());
    out[16..20].copy_from_slice(&phi_normal.to_ne_bytes());
    out[20..24].copy_from_slice(&phi_depth.to_ne_bytes());
    out
}

#[allow(clippy::too_many_arguments)]
fn pack_compose_push(
    width: u32,
    height: u32,
    tonemap: u32,
    exposure: f32,
    white_point: f32,
    denoiser_enabled: u32,
    accum_frames: u32,
    encode_srgb: u32,
) -> [u8; 32] {
    let mut out = [0u8; 32];
    out[0..4].copy_from_slice(&width.to_ne_bytes());
    out[4..8].copy_from_slice(&height.to_ne_bytes());
    out[8..12].copy_from_slice(&tonemap.to_ne_bytes());
    out[12..16].copy_from_slice(&exposure.to_ne_bytes());
    out[16..20].copy_from_slice(&white_point.to_ne_bytes());
    out[20..24].copy_from_slice(&denoiser_enabled.to_ne_bytes());
    out[24..28].copy_from_slice(&accum_frames.to_ne_bytes());
    out[28..32].copy_from_slice(&encode_srgb.to_ne_bytes());
    out
}

fn pack_tonemap_push(
    width: u32,
    height: u32,
    method: u32,
    exposure: f32,
    white_point: f32,
    encode_srgb: u32,
) -> [u8; 24] {
    let mut out = [0u8; 24];
    out[0..4].copy_from_slice(&width.to_ne_bytes());
    out[4..8].copy_from_slice(&height.to_ne_bytes());
    out[8..12].copy_from_slice(&method.to_ne_bytes());
    out[12..16].copy_from_slice(&exposure.to_ne_bytes());
    out[16..20].copy_from_slice(&white_point.to_ne_bytes());
    out[20..24].copy_from_slice(&encode_srgb.to_ne_bytes());
    out
}

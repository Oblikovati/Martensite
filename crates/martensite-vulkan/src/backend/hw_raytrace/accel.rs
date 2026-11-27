//! Vulkan hardware acceleration structure manager: device-address buffers, compacted BLAS
//! cache, and per-frame TLAS builder (`VK_KHR_acceleration_structure`).
use super::super::buffer::DeviceBuffer;
use super::super::memory_selection::HostBufferAccess;
use super::super::runtime::{VulkanRuntime, vk_error};
use super::super::submission::CommandSubmission;
use crate::{RenderError, error::rejected};
use ash::vk;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Device-local Vulkan buffer allocated with `VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_BIT`.
pub(in super::super) struct DeviceAddressBuffer {
    runtime: Arc<VulkanRuntime>,
    pub handle: vk::Buffer,
    memory: vk::DeviceMemory,
    pub device_address: u64,
    pub size: u64,
}

impl DeviceAddressBuffer {
    /// Allocates a device-local buffer with device address capability (`size >= 16`).
    pub fn new(
        runtime: &Arc<VulkanRuntime>,
        size: u64,
        usage: vk::BufferUsageFlags,
        host_visible: bool,
    ) -> Result<Self, RenderError> {
        let size = size.max(16);
        let info = vk::BufferCreateInfo::default()
            .size(size)
            .usage(usage | vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS)
            .sharing_mode(vk::SharingMode::EXCLUSIVE);
        // SAFETY: info borrows live values and device stays valid for runtime's lifetime.
        let handle = unsafe { runtime.device().create_buffer(&info, None) }
            .map_err(|e| vk_error("create device-address buffer", e))?;
        // SAFETY: handle belongs to this device.
        let reqs = unsafe { runtime.device().get_buffer_memory_requirements(handle) };
        let props = if host_visible {
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT
        } else {
            vk::MemoryPropertyFlags::DEVICE_LOCAL
        };
        let memory_type = runtime.memory_type(reqs.memory_type_bits, props)?;
        let mut flags =
            vk::MemoryAllocateFlagsInfo::default().flags(vk::MemoryAllocateFlags::DEVICE_ADDRESS);
        let alloc = vk::MemoryAllocateInfo::default()
            .allocation_size(reqs.size)
            .memory_type_index(memory_type)
            .push_next(&mut flags);
        // SAFETY: alloc and flags stay alive through allocate_memory.
        let memory = match unsafe { runtime.device().allocate_memory(&alloc, None) } {
            Ok(mem) => mem,
            Err(e) => {
                // SAFETY: handle is unused and belongs to this device.
                unsafe { runtime.device().destroy_buffer(handle, None) };
                return Err(vk_error("allocate device-address buffer memory", e));
            }
        };
        // SAFETY: memory satisfies reqs and binds exclusively to handle.
        if let Err(e) = unsafe { runtime.device().bind_buffer_memory(handle, memory, 0) } {
            // SAFETY: both handles belong to this device and are unused.
            unsafe {
                runtime.device().destroy_buffer(handle, None);
                runtime.device().free_memory(memory, None);
            }
            return Err(vk_error("bind device-address buffer memory", e));
        }
        let addr_info = vk::BufferDeviceAddressInfo::default().buffer(handle);
        // SAFETY: handle was created with SHADER_DEVICE_ADDRESS and bound to DEVICE_ADDRESS memory.
        let device_address = unsafe { runtime.device().get_buffer_device_address(&addr_info) };
        Ok(Self {
            runtime: runtime.clone(),
            handle,
            memory,
            device_address,
            size,
        })
    }

    /// Uploads `bytes` into a host-visible device-address buffer.
    pub fn upload_host(
        runtime: &Arc<VulkanRuntime>,
        usage: vk::BufferUsageFlags,
        bytes: &[u8],
    ) -> Result<Self, RenderError> {
        let buf = Self::new(runtime, bytes.len() as u64, usage, true)?;
        if !bytes.is_empty() {
            // SAFETY: host-visible coherent memory mapped exclusively for this copy.
            unsafe {
                let ptr = runtime
                    .device()
                    .map_memory(
                        buf.memory,
                        0,
                        bytes.len() as u64,
                        vk::MemoryMapFlags::empty(),
                    )
                    .map_err(|e| vk_error("map device-address buffer", e))?;
                std::ptr::copy_nonoverlapping(bytes.as_ptr(), ptr.cast::<u8>(), bytes.len());
                runtime.device().unmap_memory(buf.memory);
            }
        }
        Ok(buf)
    }

    /// Uploads `bytes` into a device-local buffer via a staging copy.
    pub fn upload_device(
        runtime: &Arc<VulkanRuntime>,
        usage: vk::BufferUsageFlags,
        bytes: &[u8],
    ) -> Result<Self, RenderError> {
        let dst = Self::new(
            runtime,
            bytes.len() as u64,
            usage | vk::BufferUsageFlags::TRANSFER_DST,
            false,
        )?;
        if bytes.is_empty() {
            return Ok(dst);
        }
        let staging = DeviceBuffer::new(
            runtime,
            bytes.len() as u64,
            vk::BufferUsageFlags::TRANSFER_SRC,
            HostBufferAccess::Upload,
        )?;
        staging.write(bytes)?;
        let submission = CommandSubmission::new(runtime)?;
        let region = vk::BufferCopy::default().size(bytes.len() as u64);
        // SAFETY: command buffer is recording; staging and dst outlive submission.finish().
        unsafe {
            runtime.device().cmd_copy_buffer(
                submission.command,
                staging.handle,
                dst.handle,
                &[region],
            );
        }
        submission.finish()?;
        Ok(dst)
    }
}

impl Drop for DeviceAddressBuffer {
    fn drop(&mut self) {
        // SAFETY: resources belong to this device and drop only after GPU work completes.
        unsafe {
            self.runtime.device().destroy_buffer(self.handle, None);
            self.runtime.device().free_memory(self.memory, None);
        }
    }
}

/// Built & compacted Bottom-Level Acceleration Structure (BLAS) for one triangle mesh.
pub(in super::super) struct BottomLevelAccel {
    runtime: Arc<VulkanRuntime>,
    pub handle: vk::AccelerationStructureKHR,
    _buffer: DeviceAddressBuffer,
    pub device_address: u64,
    #[allow(dead_code)]
    pub original_bytes: u64,
    #[allow(dead_code)]
    pub compacted_bytes: u64,
}

impl BottomLevelAccel {
    /// Builds and compacts a triangle BLAS from tightly packed `[f32; 4]` positions (`stride = 16`)
    /// and `u32` triangle indices (`3 * primitive_count`).
    pub fn build_triangles(
        runtime: &Arc<VulkanRuntime>,
        positions_bytes: &[u8],
        vertex_count: u32,
        indices_bytes: &[u8],
        primitive_count: u32,
    ) -> Result<Self, RenderError> {
        let hw = runtime
            .hw_raytrace
            .as_ref()
            .ok_or_else(|| rejected("no hw_raytrace", "a hardware ray-tracing device"))?;
        let input_usage = vk::BufferUsageFlags::ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_KHR;
        let vbuf = DeviceAddressBuffer::upload_device(runtime, input_usage, positions_bytes)?;
        let ibuf = DeviceAddressBuffer::upload_device(runtime, input_usage, indices_bytes)?;

        let triangles = vk::AccelerationStructureGeometryTrianglesDataKHR::default()
            .vertex_format(vk::Format::R32G32B32_SFLOAT)
            .vertex_data(vk::DeviceOrHostAddressConstKHR {
                device_address: vbuf.device_address,
            })
            .vertex_stride(16)
            .max_vertex(vertex_count.saturating_sub(1))
            .index_type(vk::IndexType::UINT32)
            .index_data(vk::DeviceOrHostAddressConstKHR {
                device_address: ibuf.device_address,
            });
        let geometry = vk::AccelerationStructureGeometryKHR::default()
            .geometry_type(vk::GeometryTypeKHR::TRIANGLES)
            .flags(vk::GeometryFlagsKHR::OPAQUE)
            .geometry(vk::AccelerationStructureGeometryDataKHR { triangles });
        let geometries = [geometry];
        let build_info = vk::AccelerationStructureBuildGeometryInfoKHR::default()
            .ty(vk::AccelerationStructureTypeKHR::BOTTOM_LEVEL)
            .flags(
                vk::BuildAccelerationStructureFlagsKHR::PREFER_FAST_TRACE
                    | vk::BuildAccelerationStructureFlagsKHR::ALLOW_COMPACTION,
            )
            .mode(vk::BuildAccelerationStructureModeKHR::BUILD)
            .geometries(&geometries);

        let mut sizes = vk::AccelerationStructureBuildSizesInfoKHR::default();
        // SAFETY: hw.accel_struct belongs to this live device; build_info borrows live geometries.
        unsafe {
            hw.accel_struct.get_acceleration_structure_build_sizes(
                vk::AccelerationStructureBuildTypeKHR::DEVICE,
                &build_info,
                &[primitive_count],
                &mut sizes,
            );
        }

        let as_usage = vk::BufferUsageFlags::ACCELERATION_STRUCTURE_STORAGE_KHR;
        let raw_buf =
            DeviceAddressBuffer::new(runtime, sizes.acceleration_structure_size, as_usage, false)?;
        let raw_as = create_as(hw, vk::AccelerationStructureTypeKHR::BOTTOM_LEVEL, &raw_buf)?;
        let align = u64::from(hw.caps.min_as_scratch_alignment);
        let scratch = DeviceAddressBuffer::new(
            runtime,
            sizes.build_scratch_size + align,
            vk::BufferUsageFlags::STORAGE_BUFFER,
            false,
        )?;
        let scratch_addr = align_up(scratch.device_address, align);

        let query_pool = create_compaction_query_pool(runtime)?;
        let build_info = build_info.dst_acceleration_structure(raw_as).scratch_data(
            vk::DeviceOrHostAddressKHR {
                device_address: scratch_addr,
            },
        );
        let range =
            vk::AccelerationStructureBuildRangeInfoKHR::default().primitive_count(primitive_count);

        let submission = CommandSubmission::new(runtime)?;
        // SAFETY: command buffer is recording; all AS, buffers, and query pool outlive finish().
        unsafe {
            runtime
                .device()
                .cmd_reset_query_pool(submission.command, query_pool, 0, 1);
            hw.accel_struct.cmd_build_acceleration_structures(
                submission.command,
                &[build_info],
                &[&[range]],
            );
            record_as_barrier(
                runtime.device(),
                submission.command,
                vk::PipelineStageFlags2::ACCELERATION_STRUCTURE_BUILD_KHR,
                vk::AccessFlags2::ACCELERATION_STRUCTURE_WRITE_KHR,
                vk::PipelineStageFlags2::ACCELERATION_STRUCTURE_BUILD_KHR,
                vk::AccessFlags2::ACCELERATION_STRUCTURE_READ_KHR,
            );
            hw.accel_struct
                .cmd_write_acceleration_structures_properties(
                    submission.command,
                    &[raw_as],
                    vk::QueryType::ACCELERATION_STRUCTURE_COMPACTED_SIZE_KHR,
                    query_pool,
                    0,
                );
        }
        submission.finish()?;

        let mut compacted_size = [0u64; 1];
        // SAFETY: query pool completed before finish() returned.
        let read_res = unsafe {
            runtime.device().get_query_pool_results(
                query_pool,
                0,
                &mut compacted_size,
                vk::QueryResultFlags::TYPE_64 | vk::QueryResultFlags::WAIT,
            )
        };
        // SAFETY: query pool is no longer in use.
        unsafe { runtime.device().destroy_query_pool(query_pool, None) };
        read_res.map_err(|e| vk_error("read BLAS compacted size", e))?;

        let orig = sizes.acceleration_structure_size;
        let compact = compacted_size[0];
        if compact > 0 && compact < orig {
            let compact_buf = DeviceAddressBuffer::new(runtime, compact, as_usage, false)?;
            let compact_as = create_as(
                hw,
                vk::AccelerationStructureTypeKHR::BOTTOM_LEVEL,
                &compact_buf,
            )?;
            let copy_info = vk::CopyAccelerationStructureInfoKHR::default()
                .src(raw_as)
                .dst(compact_as)
                .mode(vk::CopyAccelerationStructureModeKHR::COMPACT);
            let sub2 = CommandSubmission::new(runtime)?;
            // SAFETY: both AS handles are live and outlive sub2.finish().
            unsafe {
                hw.accel_struct
                    .cmd_copy_acceleration_structure(sub2.command, &copy_info);
            }
            sub2.finish()?;
            // SAFETY: raw_as is no longer referenced after compaction copy completes.
            unsafe { hw.accel_struct.destroy_acceleration_structure(raw_as, None) };
            let addr = as_device_address(hw, compact_as);
            return Ok(Self {
                runtime: runtime.clone(),
                handle: compact_as,
                _buffer: compact_buf,
                device_address: addr,
                original_bytes: orig,
                compacted_bytes: compact,
            });
        }

        let addr = as_device_address(hw, raw_as);
        Ok(Self {
            runtime: runtime.clone(),
            handle: raw_as,
            _buffer: raw_buf,
            device_address: addr,
            original_bytes: orig,
            compacted_bytes: orig,
        })
    }
}

impl Drop for BottomLevelAccel {
    fn drop(&mut self) {
        if let Some(hw) = self.runtime.hw_raytrace.as_ref() {
            // SAFETY: AS belongs to this device and drops only after GPU work completes.
            unsafe {
                hw.accel_struct
                    .destroy_acceleration_structure(self.handle, None)
            };
        }
    }
}

/// One instance placed into a per-frame TLAS.
#[derive(Clone, Copy, Debug)]
pub(in super::super) struct TlasInstanceInput {
    pub transform_3x4: [f32; 12],
    pub custom_index: u32,
    pub blas_address: u64,
    pub opaque: bool,
}

/// Top-Level Acceleration Structure (TLAS) over all visible instances in one frame.
pub(in super::super) struct TopLevelAccel {
    runtime: Arc<VulkanRuntime>,
    pub handle: vk::AccelerationStructureKHR,
    _buffer: DeviceAddressBuffer,
}

impl TopLevelAccel {
    /// Builds a TLAS for `instances` synchronously and transitions it for compute/fragment reads.
    pub fn build(
        runtime: &Arc<VulkanRuntime>,
        instances: &[TlasInstanceInput],
    ) -> Result<Self, RenderError> {
        let hw = runtime
            .hw_raytrace
            .as_ref()
            .ok_or_else(|| rejected("no hw_raytrace", "a hardware ray-tracing device"))?;
        let vk_instances: Vec<vk::AccelerationStructureInstanceKHR> = instances
            .iter()
            .map(|inst| {
                let flags = if inst.opaque {
                    vk::GeometryInstanceFlagsKHR::TRIANGLE_FACING_CULL_DISABLE
                        | vk::GeometryInstanceFlagsKHR::FORCE_OPAQUE
                } else {
                    vk::GeometryInstanceFlagsKHR::TRIANGLE_FACING_CULL_DISABLE
                };
                vk::AccelerationStructureInstanceKHR {
                    transform: vk::TransformMatrixKHR {
                        matrix: inst.transform_3x4,
                    },
                    instance_custom_index_and_mask: vk::Packed24_8::new(inst.custom_index, 0xFF),
                    instance_shader_binding_table_record_offset_and_flags: vk::Packed24_8::new(
                        0,
                        flags.as_raw() as u8,
                    ),
                    acceleration_structure_reference: vk::AccelerationStructureReferenceKHR {
                        device_handle: inst.blas_address,
                    },
                }
            })
            .collect();
        let inst_bytes = instance_slice_to_bytes(&vk_instances);
        let inst_buf = DeviceAddressBuffer::upload_host(
            runtime,
            vk::BufferUsageFlags::ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_KHR,
            &inst_bytes,
        )?;
        let inst_data = vk::AccelerationStructureGeometryInstancesDataKHR::default()
            .array_of_pointers(false)
            .data(vk::DeviceOrHostAddressConstKHR {
                device_address: inst_buf.device_address,
            });
        let geometry = vk::AccelerationStructureGeometryKHR::default()
            .geometry_type(vk::GeometryTypeKHR::INSTANCES)
            .geometry(vk::AccelerationStructureGeometryDataKHR {
                instances: inst_data,
            });
        let geometries = [geometry];
        let build_info = vk::AccelerationStructureBuildGeometryInfoKHR::default()
            .ty(vk::AccelerationStructureTypeKHR::TOP_LEVEL)
            .flags(vk::BuildAccelerationStructureFlagsKHR::PREFER_FAST_TRACE)
            .mode(vk::BuildAccelerationStructureModeKHR::BUILD)
            .geometries(&geometries);

        let count = instances.len() as u32;
        let mut sizes = vk::AccelerationStructureBuildSizesInfoKHR::default();
        // SAFETY: hw.accel_struct belongs to this live device.
        unsafe {
            hw.accel_struct.get_acceleration_structure_build_sizes(
                vk::AccelerationStructureBuildTypeKHR::DEVICE,
                &build_info,
                &[count],
                &mut sizes,
            );
        }
        let tlas_buf = DeviceAddressBuffer::new(
            runtime,
            sizes.acceleration_structure_size,
            vk::BufferUsageFlags::ACCELERATION_STRUCTURE_STORAGE_KHR,
            false,
        )?;
        let handle = create_as(hw, vk::AccelerationStructureTypeKHR::TOP_LEVEL, &tlas_buf)?;
        let align = u64::from(hw.caps.min_as_scratch_alignment);
        let scratch = DeviceAddressBuffer::new(
            runtime,
            sizes.build_scratch_size + align,
            vk::BufferUsageFlags::STORAGE_BUFFER,
            false,
        )?;
        let scratch_addr = align_up(scratch.device_address, align);
        let build_info = build_info.dst_acceleration_structure(handle).scratch_data(
            vk::DeviceOrHostAddressKHR {
                device_address: scratch_addr,
            },
        );
        let range = vk::AccelerationStructureBuildRangeInfoKHR::default().primitive_count(count);
        let submission = CommandSubmission::new(runtime)?;
        // SAFETY: command buffer is recording; inst_buf, scratch, and handle outlive finish().
        unsafe {
            hw.accel_struct.cmd_build_acceleration_structures(
                submission.command,
                &[build_info],
                &[&[range]],
            );
            record_as_barrier(
                runtime.device(),
                submission.command,
                vk::PipelineStageFlags2::ACCELERATION_STRUCTURE_BUILD_KHR,
                vk::AccessFlags2::ACCELERATION_STRUCTURE_WRITE_KHR,
                vk::PipelineStageFlags2::COMPUTE_SHADER | vk::PipelineStageFlags2::FRAGMENT_SHADER,
                vk::AccessFlags2::ACCELERATION_STRUCTURE_READ_KHR,
            );
        }
        submission.finish()?;
        Ok(Self {
            runtime: runtime.clone(),
            handle,
            _buffer: tlas_buf,
        })
    }
}

impl Drop for TopLevelAccel {
    fn drop(&mut self) {
        if let Some(hw) = self.runtime.hw_raytrace.as_ref() {
            // SAFETY: AS belongs to this device and drops only after GPU work completes.
            unsafe {
                hw.accel_struct
                    .destroy_acceleration_structure(self.handle, None)
            };
        }
    }
}

/// Thread-safe cache of compacted BLAS handles keyed by mesh content digest.
#[derive(Default)]
pub(in super::super) struct BlasCache {
    entries: Mutex<HashMap<u64, Arc<BottomLevelAccel>>>,
}

impl BlasCache {
    pub fn get_or_build(
        &self,
        key: u64,
        build: impl FnOnce() -> Result<BottomLevelAccel, RenderError>,
    ) -> Result<Arc<BottomLevelAccel>, RenderError> {
        {
            let guard = self
                .entries
                .lock()
                .map_err(|e| rejected(e.to_string(), "an unpoisoned hardware BLAS cache"))?;
            if let Some(existing) = guard.get(&key) {
                return Ok(existing.clone());
            }
        }
        let built = Arc::new(build()?);
        let mut guard = self
            .entries
            .lock()
            .map_err(|e| rejected(e.to_string(), "an unpoisoned hardware BLAS cache"))?;
        Ok(guard.entry(key).or_insert_with(|| built.clone()).clone())
    }
}

fn create_as(
    hw: &super::device::HardwareRaytraceDevice,
    ty: vk::AccelerationStructureTypeKHR,
    buffer: &DeviceAddressBuffer,
) -> Result<vk::AccelerationStructureKHR, RenderError> {
    let info = vk::AccelerationStructureCreateInfoKHR::default()
        .buffer(buffer.handle)
        .size(buffer.size)
        .ty(ty);
    // SAFETY: buffer is live and allocated with ACCELERATION_STRUCTURE_STORAGE_KHR.
    unsafe { hw.accel_struct.create_acceleration_structure(&info, None) }
        .map_err(|e| vk_error("create acceleration structure", e))
}

fn as_device_address(
    hw: &super::device::HardwareRaytraceDevice,
    handle: vk::AccelerationStructureKHR,
) -> u64 {
    let info =
        vk::AccelerationStructureDeviceAddressInfoKHR::default().acceleration_structure(handle);
    // SAFETY: handle is a live acceleration structure on this device.
    unsafe {
        hw.accel_struct
            .get_acceleration_structure_device_address(&info)
    }
}

fn create_compaction_query_pool(runtime: &VulkanRuntime) -> Result<vk::QueryPool, RenderError> {
    let info = vk::QueryPoolCreateInfo::default()
        .query_type(vk::QueryType::ACCELERATION_STRUCTURE_COMPACTED_SIZE_KHR)
        .query_count(1);
    // SAFETY: info borrows live data on this device.
    unsafe { runtime.device().create_query_pool(&info, None) }
        .map_err(|e| vk_error("create BLAS compaction query pool", e))
}

fn align_up(value: u64, align: u64) -> u64 {
    let mask = align.max(1) - 1;
    (value + mask) & !mask
}

fn record_as_barrier(
    device: &ash::Device,
    command: vk::CommandBuffer,
    src_stage: vk::PipelineStageFlags2,
    src_access: vk::AccessFlags2,
    dst_stage: vk::PipelineStageFlags2,
    dst_access: vk::AccessFlags2,
) {
    let barrier = [vk::MemoryBarrier2::default()
        .src_stage_mask(src_stage)
        .src_access_mask(src_access)
        .dst_stage_mask(dst_stage)
        .dst_access_mask(dst_access)];
    let dep = vk::DependencyInfo::default().memory_barriers(&barrier);
    // SAFETY: command buffer is actively recording on this thread.
    unsafe { device.cmd_pipeline_barrier2(command, &dep) };
}

fn instance_slice_to_bytes(instances: &[vk::AccelerationStructureInstanceKHR]) -> Vec<u8> {
    let mut out = vec![0u8; std::mem::size_of_val(instances)];
    if !instances.is_empty() {
        // SAFETY: AccelerationStructureInstanceKHR is a C-layout POD struct of fixed size 64 bytes.
        unsafe {
            std::ptr::copy_nonoverlapping(
                instances.as_ptr().cast::<u8>(),
                out.as_mut_ptr(),
                out.len(),
            );
        }
    }
    out
}

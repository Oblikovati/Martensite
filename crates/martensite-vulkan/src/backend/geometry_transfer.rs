//! One frame owns its staging ranges until submission retirement.
use super::buffer::DeviceBuffer;
use super::frame_work_ledger::DeviceCount;
use super::geometry_buffer::GeometryBuffer;
use super::geometry_residency::{FrameGeometryResidency, GeometryStore};
use super::host_upload_pages::HostUploadPool;
use super::runtime::VulkanRuntime;
use crate::{RenderError, error::rejected};
use ash::vk;
use std::sync::Arc;

const STAGING_BLOCK_BYTES: usize = 4 << 20;

pub(super) fn geometry_staging_pool() -> HostUploadPool {
    HostUploadPool::new(STAGING_BLOCK_BYTES, vk::BufferUsageFlags::TRANSFER_SRC)
}

pub(super) struct GeometryTransfers {
    blocks: Vec<StagingBlock>,
    preserved: Vec<PreservedRanges>,
    writes_shared_page: bool,
    pub(super) residency: FrameGeometryResidency,
}

struct StagingBlock {
    buffer: Arc<DeviceBuffer>,
    capacity: usize,
    end: usize,
    copies: Vec<GeometryCopy>,
}

struct GeometryCopy {
    destination: Arc<GeometryBuffer>,
    region: vk::BufferCopy,
}

struct PreservedRanges {
    source: Arc<GeometryBuffer>,
    destination: Arc<GeometryBuffer>,
    regions: [vk::BufferCopy; 2],
    count: usize,
}

impl GeometryTransfers {
    pub(super) fn new(store: &Arc<GeometryStore>) -> Self {
        Self {
            blocks: Vec::new(),
            preserved: Vec::new(),
            writes_shared_page: false,
            residency: FrameGeometryResidency::new(store),
        }
    }
    /// Queue a transfer into a device-local allocation, such as a vertex or index stream.
    pub(super) fn upload(
        &mut self,
        runtime: &Arc<VulkanRuntime>,
        pool: &HostUploadPool,
        usage: vk::BufferUsageFlags,
        alignment: Option<u64>,
        bytes: &[u8],
    ) -> Result<Arc<GeometryBuffer>, RenderError> {
        if !bytes.len().is_multiple_of(4) {
            return Err(rejected(bytes.len(), "a four-byte-aligned geometry stream"));
        }
        let destination =
            self.residency
                .buffers()
                .allocate(runtime, usage, bytes.len() as u64, alignment)?;
        if !bytes.is_empty() {
            self.stage(runtime, pool, &destination, 0, bytes)?;
        }
        Ok(destination)
    }

    /// Copy unchanged ranges from an earlier upload and stage only the changed records.
    pub(super) fn patch_stream(
        &mut self,
        runtime: &Arc<VulkanRuntime>,
        pool: &HostUploadPool,
        usage: vk::BufferUsageFlags,
        previous: Arc<GeometryBuffer>,
        offset: u64,
        bytes: &[u8],
    ) -> Result<Arc<GeometryBuffer>, RenderError> {
        let size = previous.size();
        let range = patch_range(size, offset, bytes.len() as u64)?;
        if range.is_empty() {
            return Ok(previous);
        }
        let destination =
            self.residency
                .buffers()
                .allocate(runtime, usage, size, previous.alignment)?;
        self.stage(runtime, pool, &destination, offset, bytes)?;
        self.preserved.push(PreservedRanges::new(
            previous,
            Arc::clone(&destination),
            range,
        ));
        Ok(destination)
    }

    pub(super) fn stage(
        &mut self,
        runtime: &Arc<VulkanRuntime>,
        pool: &HostUploadPool,
        destination: &Arc<GeometryBuffer>,
        offset: u64,
        bytes: &[u8],
    ) -> Result<(), RenderError> {
        self.block(runtime, pool, bytes.len())?
            .write(bytes, destination, offset)?;
        self.writes_shared_page |= destination.is_shared();
        runtime
            .work
            .add(DeviceCount::GeometryUploadedBytes, bytes.len() as u64);
        Ok(())
    }

    fn block(
        &mut self,
        runtime: &Arc<VulkanRuntime>,
        pool: &HostUploadPool,
        bytes: usize,
    ) -> Result<&mut StagingBlock, RenderError> {
        if !self
            .blocks
            .last()
            .is_some_and(|block| bytes <= block.capacity - block.end)
        {
            self.blocks.push(StagingBlock {
                buffer: pool.acquire(runtime, bytes)?,
                capacity: pool.capacity_for(bytes),
                end: 0,
                copies: Vec::new(),
            });
        }
        self.blocks
            .last_mut()
            .ok_or_else(|| rejected(bytes, "an allocated staging block"))
    }

    /// Record all copies before the first draw that can read their destinations.
    pub(super) fn record(&self, runtime: &VulkanRuntime, command: vk::CommandBuffer) {
        if self.blocks.is_empty() {
            return;
        }
        let device = runtime.device();
        if self.writes_shared_page {
            // A previous frame can still read a vertex binding on this page. The binding
            // can span beyond that draw's range, so order vertex input before new copies.
            let before_copy = vk::MemoryBarrier2::default()
                .src_stage_mask(vk::PipelineStageFlags2::VERTEX_INPUT)
                .dst_stage_mask(vk::PipelineStageFlags2::COPY);
            record_barrier(device, command, before_copy);
        }
        for preserved in &self.preserved {
            preserved.record(device, command);
        }
        for block in &self.blocks {
            block.record(device, command);
        }
        record_barrier(device, command, vertex_input_barrier());
    }
}

fn record_barrier(
    device: &ash::Device,
    command: vk::CommandBuffer,
    barrier: vk::MemoryBarrier2<'_>,
) {
    let dependency = vk::DependencyInfo::default().memory_barriers(std::slice::from_ref(&barrier));
    // SAFETY: the frame owns a recording command buffer on this device, outside a render scope.
    unsafe { device.cmd_pipeline_barrier2(command, &dependency) };
}

impl StagingBlock {
    fn write(
        &mut self,
        bytes: &[u8],
        destination: &Arc<GeometryBuffer>,
        offset: u64,
    ) -> Result<(), RenderError> {
        self.buffer.write_at(self.end, bytes)?;
        let region = vk::BufferCopy::default()
            .src_offset(self.end as u64)
            .dst_offset(destination.offset + offset)
            .size(bytes.len() as u64);
        self.copies.push(GeometryCopy {
            destination: Arc::clone(destination),
            region,
        });
        self.end += bytes.len();
        Ok(())
    }

    fn record(&self, device: &ash::Device, command: vk::CommandBuffer) {
        for copy in &self.copies {
            // SAFETY: both owned buffers have the required transfer usage. The aligned range was
            // checked against the staging capacity and the destination was allocated for it.
            unsafe {
                device.cmd_copy_buffer(
                    command,
                    self.buffer.handle,
                    copy.destination.handle,
                    std::slice::from_ref(&copy.region),
                );
            }
        }
    }
}

/// The copies' writes, visible to draws and to later copies. Later copies include the
/// preserved ranges of later frames, which may read a stream while its frame is still in
/// flight: the queue keeps the submission order (#594).
fn vertex_input_barrier() -> vk::MemoryBarrier2<'static> {
    vk::MemoryBarrier2::default()
        .src_stage_mask(vk::PipelineStageFlags2::COPY)
        .src_access_mask(vk::AccessFlags2::TRANSFER_WRITE)
        .dst_stage_mask(vk::PipelineStageFlags2::VERTEX_INPUT | vk::PipelineStageFlags2::COPY)
        .dst_access_mask(
            vk::AccessFlags2::VERTEX_ATTRIBUTE_READ
                | vk::AccessFlags2::INDEX_READ
                | vk::AccessFlags2::TRANSFER_READ,
        )
}

fn patch_range(size: u64, offset: u64, length: u64) -> Result<std::ops::Range<u64>, RenderError> {
    if offset.is_multiple_of(4)
        && length.is_multiple_of(4)
        && let Some(end) = offset.checked_add(length).filter(|end| *end <= size)
    {
        return Ok(offset..end);
    }
    Err(rejected(
        (size, offset, length),
        "an aligned geometry patch inside the buffer",
    ))
}

impl PreservedRanges {
    fn new(
        source: Arc<GeometryBuffer>,
        destination: Arc<GeometryBuffer>,
        patch: std::ops::Range<u64>,
    ) -> Self {
        let from = |offset, size| {
            vk::BufferCopy::default()
                .src_offset(source.offset + offset)
                .dst_offset(destination.offset + offset)
                .size(size)
        };
        let mut regions = [
            from(0, patch.start),
            from(patch.end, source.size() - patch.end),
        ];
        regions.sort_by_key(|region| region.size == 0);
        let count = regions.iter().take_while(|region| region.size != 0).count();
        Self {
            source,
            destination,
            regions,
            count,
        }
    }

    fn record(&self, device: &ash::Device, command: vk::CommandBuffer) {
        if self.count == 0 {
            return;
        }
        // SAFETY: both owned buffers support transfers and belong to the recording device.
        // The leases reserve disjoint ranges even in the same buffer. The checked copies
        // are in bounds and do not overlap the patch.
        unsafe {
            device.cmd_copy_buffer(
                command,
                self.source.handle,
                self.destination.handle,
                &self.regions[..self.count],
            );
        }
    }
}

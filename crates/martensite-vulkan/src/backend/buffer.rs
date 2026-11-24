//! Buffer allocations are released on every construction, recording and readback failure.
use super::frame_work_ledger::DeviceCount;
use super::memory_arena::{MemoryLease, MemoryRequest, ResourceTiling};
use super::memory_selection::{HostBufferAccess, host_buffer_memory_type};
use super::runtime::{VulkanRuntime, vk_error};
use crate::error::{RenderError, rejected};
use ash::vk;
use std::sync::Arc;
pub(super) struct DeviceBuffer {
    runtime: Arc<VulkanRuntime>,
    pub handle: vk::Buffer,
    memory: Option<MemoryLease>,
    size: u64,
    transfer_shared: bool,
}
impl DeviceBuffer {
    pub fn new(
        runtime: &Arc<VulkanRuntime>,
        size: u64,
        usage: vk::BufferUsageFlags,
        access: HostBufferAccess,
    ) -> Result<Self, RenderError> {
        Self::allocate_buffer(runtime, size, usage, BufferMemory::Host(access), None)
    }
    /// A staging page can be reused by the graphics or dedicated transfer queue.
    pub(super) fn geometry_staging(
        runtime: &Arc<VulkanRuntime>,
        size: u64,
    ) -> Result<Self, RenderError> {
        Self::allocate_buffer(
            runtime,
            size,
            vk::BufferUsageFlags::TRANSFER_SRC,
            BufferMemory::Host(HostBufferAccess::Upload),
            super::dedicated_transfer::sharing_families(runtime),
        )
    }
    /// Allocate a device-local transfer destination on the graphics queue.
    pub(super) fn device_local(
        runtime: &Arc<VulkanRuntime>,
        size: u64,
        usage: vk::BufferUsageFlags,
    ) -> Result<Self, RenderError> {
        Self::device_local_geometry(runtime, size, usage, false)
    }

    /// A dedicated geometry buffer can share the copy and graphics queue families.
    pub(super) fn device_local_geometry(
        runtime: &Arc<VulkanRuntime>,
        size: u64,
        usage: vk::BufferUsageFlags,
        allow_transfer_queue: bool,
    ) -> Result<Self, RenderError> {
        let usage = usage | vk::BufferUsageFlags::TRANSFER_DST | vk::BufferUsageFlags::TRANSFER_SRC;
        let families = if allow_transfer_queue {
            super::dedicated_transfer::sharing_families(runtime)
        } else {
            None
        };
        Self::allocate_buffer(runtime, size, usage, BufferMemory::DeviceLocal, families)
    }
    fn allocate_buffer(
        runtime: &Arc<VulkanRuntime>,
        size: u64,
        usage: vk::BufferUsageFlags,
        placement: BufferMemory,
        families: Option<[u32; 2]>,
    ) -> Result<Self, RenderError> {
        let mut info = vk::BufferCreateInfo::default()
            .size(size.max(4))
            .usage(usage)
            .sharing_mode(vk::SharingMode::EXCLUSIVE);
        if let Some(families) = families.as_ref() {
            info = info
                .sharing_mode(vk::SharingMode::CONCURRENT)
                .queue_family_indices(families);
        }
        // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
        let handle = unsafe { runtime.device().create_buffer(&info, None) }
            .map_err(|e| vk_error("create buffer", e))?;
        runtime.work.add(DeviceCount::Buffers, 1);
        let mut buffer = Self {
            runtime: runtime.clone(),
            handle,
            memory: None,
            size: size.max(4),
            transfer_shared: families.is_some(),
        };
        buffer.allocate(placement)?;
        Ok(buffer)
    }
    fn allocate(&mut self, placement: BufferMemory) -> Result<(), RenderError> {
        // SAFETY: the instance, physical device and device handles stay valid while the runtime holds them, and the call only reads through out-parameters it owns.
        let requirements = unsafe {
            self.runtime
                .device()
                .get_buffer_memory_requirements(self.handle)
        };
        let index = placement.memory_type(&self.runtime, requirements.memory_type_bits)?;
        let request = MemoryRequest {
            requirements,
            type_index: index,
            tiling: ResourceTiling::Buffer,
        };
        let lease = self
            .memory
            .insert(MemoryLease::new(&self.runtime, request)?);
        // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
        unsafe {
            self.runtime
                .device()
                .bind_buffer_memory(self.handle, lease.memory, lease.offset)
        }
        .map_err(|e| vk_error("bind buffer memory", e))
    }
    pub fn write(&self, bytes: &[u8]) -> Result<(), RenderError> {
        self.write_at(0, bytes)
    }
    /// Fill an unused range of a mapped staging buffer before submitting the transfer.
    pub(super) fn write_at(&self, offset: usize, bytes: &[u8]) -> Result<(), RenderError> {
        if (offset as u64).saturating_add(bytes.len() as u64) > self.size {
            return Err(rejected((offset, bytes.len()), "a range within the buffer"));
        }
        let pointer = self.host_pointer()?;
        // Host-coherent memory stays mapped; writes happen only between submissions.
        // SAFETY: the destination is a live mapped allocation whose size was checked above, and it does not overlap the source.
        unsafe { std::ptr::copy_nonoverlapping(bytes.as_ptr(), pointer.add(offset), bytes.len()) };
        self.runtime
            .work
            .add(DeviceCount::UploadedBytes, bytes.len() as u64);
        Ok(())
    }

    pub(super) fn transfer_shared(&self) -> bool {
        self.transfer_shared
    }

    pub(super) fn size(&self) -> u64 {
        self.size
    }

    pub(super) fn allocation_size(&self) -> u64 {
        self.memory.as_ref().map_or(0, MemoryLease::size)
    }
    pub fn read(&self, size: usize) -> Result<Vec<u8>, RenderError> {
        self.read_at(0, size)
    }

    /// Read a written range after the owning submission fence.
    pub(super) fn read_at(&self, offset: usize, size: usize) -> Result<Vec<u8>, RenderError> {
        if offset
            .checked_add(size)
            .is_none_or(|end| end > isize::MAX as usize || end as u64 > self.size)
        {
            return Err(rejected(
                (offset, size),
                "read range within the buffer allocation",
            ));
        }
        let pointer = self.host_pointer()?;
        // SAFETY: the checked range stays within the live mapping, and readback callers
        // wait for the GPU writes and transfer-to-host dependency before reading these bytes.
        let bytes = unsafe { std::slice::from_raw_parts(pointer.add(offset), size) }.to_vec();
        self.runtime
            .work
            .add(DeviceCount::ReadBackBytes, size as u64);
        Ok(bytes)
    }
    /// Lend the first `size` mapped bytes to `read` without copying them;
    /// `buffer.with_mapped(64, |bytes| bytes[0])?`. A completed fence and a transfer-to-host
    /// barrier must precede the call, as for `read`.
    pub fn with_mapped<R>(
        &self,
        size: usize,
        read: impl FnOnce(&[u8]) -> R,
    ) -> Result<R, RenderError> {
        if size as u64 > self.size {
            return Err(rejected(size, "read size within the buffer allocation"));
        }
        let pointer = self.host_pointer()?;
        // SAFETY: the pointer and length describe this buffer's live mapped allocation, the
        // device finished writing it, and the borrow ends before this function returns.
        let bytes = unsafe { std::slice::from_raw_parts(pointer, size) };
        self.runtime
            .work
            .add(DeviceCount::ReadBackBytes, size as u64);
        Ok(read(bytes))
    }
    fn host_pointer(&self) -> Result<*mut u8, RenderError> {
        self.memory
            .as_ref()
            .and_then(MemoryLease::host_pointer)
            .ok_or_else(|| rejected(self.handle, "a buffer bound to mapped host memory"))
    }
}

#[derive(Clone, Copy)]
enum BufferMemory {
    Host(HostBufferAccess),
    DeviceLocal,
}

impl BufferMemory {
    fn memory_type(self, runtime: &VulkanRuntime, bits: u32) -> Result<u32, RenderError> {
        match self {
            Self::Host(access) => host_buffer_memory_type(&runtime.memory, bits, access)
                .ok_or_else(|| rejected(bits, "a host-visible coherent Vulkan memory type")),
            Self::DeviceLocal => runtime.memory_type(bits, vk::MemoryPropertyFlags::DEVICE_LOCAL),
        }
    }
}
impl Drop for DeviceBuffer {
    fn drop(&mut self) {
        // The lease field drops after this body, once the buffer no longer uses its range.
        // SAFETY: the handle belongs to this device and its owner releases it only after the GPU work that used it has completed.
        unsafe {
            self.runtime.device().destroy_buffer(self.handle, None);
        }
    }
}

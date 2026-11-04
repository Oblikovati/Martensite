//! Buffer allocations are released on every construction, recording and readback failure.
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
}
impl DeviceBuffer {
    pub fn new(
        runtime: &Arc<VulkanRuntime>,
        size: u64,
        usage: vk::BufferUsageFlags,
        access: HostBufferAccess,
    ) -> Result<Self, RenderError> {
        let info = vk::BufferCreateInfo::default()
            .size(size.max(4))
            .usage(usage)
            .sharing_mode(vk::SharingMode::EXCLUSIVE);
        // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
        let handle = unsafe { runtime.device().create_buffer(&info, None) }
            .map_err(|e| vk_error("create buffer", e))?;
        let mut buffer = Self {
            runtime: runtime.clone(),
            handle,
            memory: None,
            size: size.max(4),
        };
        buffer.allocate(access)?;
        Ok(buffer)
    }
    fn allocate(&mut self, access: HostBufferAccess) -> Result<(), RenderError> {
        // SAFETY: the instance, physical device and device handles stay valid while the runtime holds them, and the call only reads through out-parameters it owns.
        let requirements = unsafe {
            self.runtime
                .device()
                .get_buffer_memory_requirements(self.handle)
        };
        let index =
            host_buffer_memory_type(&self.runtime.memory, requirements.memory_type_bits, access)
                .ok_or_else(|| {
                    rejected(
                        requirements.memory_type_bits,
                        "a host-visible coherent Vulkan memory type",
                    )
                })?;
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
        if bytes.len() as u64 > self.size {
            return Err(rejected(bytes.len(), "bytes fitting allocated buffer"));
        }
        let pointer = self.host_pointer()?;
        // Host-coherent memory stays mapped; writes happen only between submissions.
        // SAFETY: the destination is a live mapped allocation whose size was checked above, and it does not overlap the source.
        unsafe { std::ptr::copy_nonoverlapping(bytes.as_ptr(), pointer, bytes.len()) };
        Ok(())
    }
    pub fn read(&self, size: usize) -> Result<Vec<u8>, RenderError> {
        if size as u64 > self.size {
            return Err(rejected(size, "read size within the buffer allocation"));
        }
        let pointer = self.host_pointer()?;
        // A completed fence and a transfer-to-host barrier precede all readbacks.
        // SAFETY: the pointer and length describe a live allocation or Vulkan output array of this scope, and the view does not outlive it.
        Ok(unsafe { std::slice::from_raw_parts(pointer, size) }.to_vec())
    }
    fn host_pointer(&self) -> Result<*mut u8, RenderError> {
        self.memory
            .as_ref()
            .and_then(MemoryLease::host_pointer)
            .ok_or_else(|| rejected(self.handle, "a buffer bound to mapped host memory"))
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

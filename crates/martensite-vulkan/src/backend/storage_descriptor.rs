//! Owned storage descriptors share upload and lifetime rules for sprites and clipping.
use super::{
    buffer::DeviceBuffer,
    memory_selection::HostBufferAccess,
    runtime::{VulkanRuntime, vk_error},
};
use crate::{RenderError, error::rejected};
use ash::vk;
use std::sync::Arc;
pub(super) struct StorageDescriptorLayout {
    runtime: Arc<VulkanRuntime>,
    pub handle: vk::DescriptorSetLayout,
}
impl StorageDescriptorLayout {
    pub fn new(
        runtime: &Arc<VulkanRuntime>,
        stages: vk::ShaderStageFlags,
    ) -> Result<Self, RenderError> {
        let bindings = [vk::DescriptorSetLayoutBinding::default()
            .binding(0)
            .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
            .descriptor_count(1)
            .stage_flags(stages)];
        let info = vk::DescriptorSetLayoutCreateInfo::default().bindings(&bindings);
        // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
        let handle = unsafe { runtime.device().create_descriptor_set_layout(&info, None) }
            .map_err(|e| vk_error("create storage layout", e))?;
        #[cfg(feature = "command-state-diagnostics")]
        super::replay_resource_diagnostic::descriptor_layout(handle, &info);
        Ok(Self {
            runtime: runtime.clone(),
            handle,
        })
    }
}
impl Drop for StorageDescriptorLayout {
    fn drop(&mut self) {
        // SAFETY: the handle belongs to this device and its owner releases it only after the GPU work that used it has completed.
        unsafe {
            self.runtime
                .device()
                .destroy_descriptor_set_layout(self.handle, None)
        }
    }
}
pub(super) struct UploadedStorage {
    runtime: Arc<VulkanRuntime>,
    pool: vk::DescriptorPool,
    pub set: vk::DescriptorSet,
    _pixels: DeviceBuffer,
    #[cfg(feature = "command-state-diagnostics")]
    diagnostic_bytes: Vec<u8>,
}
impl UploadedStorage {
    pub fn new(
        runtime: &Arc<VulkanRuntime>,
        layout: &StorageDescriptorLayout,
        bytes: &[u8],
    ) -> Result<Self, RenderError> {
        let (pixels, byte_count) = upload_storage_bytes(runtime, bytes)?;
        let pool = create_storage_pool(runtime)?;
        let mut sprite = Self {
            runtime: runtime.clone(),
            pool,
            set: vk::DescriptorSet::null(),
            _pixels: pixels,
            #[cfg(feature = "command-state-diagnostics")]
            diagnostic_bytes: Vec::new(),
        };
        sprite.bind(layout, byte_count)?;
        #[cfg(feature = "command-state-diagnostics")]
        {
            // Retain only successful uploads; this is the slice written to the actual buffer.
            sprite.diagnostic_bytes = bytes.to_vec();
            super::replay_command_diagnostic::storage(
                sprite.set,
                sprite._pixels.handle,
                bytes,
                false,
            );
        }
        Ok(sprite)
    }
    #[cfg(feature = "command-state-diagnostics")]
    pub(super) fn diagnostic_storage(&self) -> (vk::Buffer, &[u8]) {
        (self._pixels.handle, &self.diagnostic_bytes)
    }
    /// Read only after the owning submission fence and shader-to-host dependency completed.
    pub(super) fn read_after_fence(&self, size: usize) -> Result<Vec<u8>, RenderError> {
        let bytes = self._pixels.read(size)?;
        #[cfg(feature = "command-state-diagnostics")]
        super::replay_command_diagnostic::storage(self.set, self._pixels.handle, &bytes, true);
        Ok(bytes)
    }
    #[cfg(test)]
    #[cfg_attr(not(target_os = "linux"), allow(dead_code))] // Its users are Linux window tests.
    pub(super) fn test_readback(&self, size: usize) -> Result<Vec<u8>, RenderError> {
        // Caller records shader-to-host dependency and waits for its completed fence.
        self._pixels.read(size)
    }
    fn bind(&mut self, layout: &StorageDescriptorLayout, bytes: u64) -> Result<(), RenderError> {
        self.set = allocate_storage_set(&self.runtime, self.pool, layout.handle)?;
        #[cfg(feature = "command-state-diagnostics")]
        super::replay_resource_diagnostic::allocated(self.set, layout.handle);
        let buffer = [vk::DescriptorBufferInfo::default()
            .buffer(self._pixels.handle)
            .range(bytes)];
        let write = [vk::WriteDescriptorSet::default()
            .dst_set(self.set)
            .dst_binding(0)
            .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
            .buffer_info(&buffer)];
        // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
        unsafe { self.runtime.device().update_descriptor_sets(&write, &[]) };
        #[cfg(feature = "command-state-diagnostics")]
        super::replay_command_diagnostic::descriptor_writes(&write);
        Ok(())
    }
}
impl Drop for UploadedStorage {
    fn drop(&mut self) {
        // SAFETY: the handle belongs to this device and its owner releases it only after the GPU work that used it has completed.
        unsafe {
            self.runtime
                .device()
                .destroy_descriptor_pool(self.pool, None)
        }
    }
}
fn upload_storage_bytes(
    runtime: &Arc<VulkanRuntime>,
    bytes: &[u8],
) -> Result<(DeviceBuffer, u64), RenderError> {
    let size = bytes.len() as u64;
    if size > u64::from(runtime.limits.max_storage_buffer_range) {
        return Err(rejected(size, "storage bytes within maxStorageBufferRange"));
    }
    let pixels = DeviceBuffer::new(
        runtime,
        size,
        vk::BufferUsageFlags::STORAGE_BUFFER,
        HostBufferAccess::Upload,
    )?;
    pixels.write(bytes)?;
    Ok((pixels, size))
}
fn create_storage_pool(runtime: &VulkanRuntime) -> Result<vk::DescriptorPool, RenderError> {
    let sizes = [vk::DescriptorPoolSize {
        ty: vk::DescriptorType::STORAGE_BUFFER,
        descriptor_count: 1,
    }];
    // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
    unsafe {
        runtime.device().create_descriptor_pool(
            &vk::DescriptorPoolCreateInfo::default()
                .max_sets(1)
                .pool_sizes(&sizes),
            None,
        )
    }
    .map_err(|e| vk_error("create storage descriptor pool", e))
}
fn allocate_storage_set(
    runtime: &VulkanRuntime,
    pool: vk::DescriptorPool,
    layout: vk::DescriptorSetLayout,
) -> Result<vk::DescriptorSet, RenderError> {
    let layouts = [layout];
    // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
    unsafe {
        runtime.device().allocate_descriptor_sets(
            &vk::DescriptorSetAllocateInfo::default()
                .descriptor_pool(pool)
                .set_layouts(&layouts),
        )
    }
    .map(|sets| sets[0])
    .map_err(|e| vk_error("allocate storage descriptor", e))
}

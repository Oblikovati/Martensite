//! Test-only exact uploaded block; no scene descriptors or reconstructed camera inputs.
use crate::backend::{
    buffer::DeviceBuffer, memory_selection::HostBufferAccess, runtime::VulkanRuntime,
};
use ash::vk;
use std::sync::Arc;
pub(super) struct UniformInput {
    runtime: Arc<VulkanRuntime>,
    buffer: DeviceBuffer,
    pub layout: vk::DescriptorSetLayout,
    pool: vk::DescriptorPool,
    pub set: vk::DescriptorSet,
}
impl UniformInput {
    pub fn new(runtime: &Arc<VulkanRuntime>, bytes: &[u8]) -> Self {
        assert_eq!(bytes.len(), 416);
        let buffer = uploaded_uniform_buffer(runtime, bytes);
        let mut input = Self {
            runtime: runtime.clone(),
            buffer,
            layout: vk::DescriptorSetLayout::null(),
            pool: vk::DescriptorPool::null(),
            set: vk::DescriptorSet::null(),
        };
        input.create_layout();
        input.create_pool();
        input.allocate_set();
        input.bind_block();
        input
    }
    fn create_layout(&mut self) {
        let bindings = [vk::DescriptorSetLayoutBinding::default()
            .binding(0)
            .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
            .descriptor_count(1)
            .stage_flags(vk::ShaderStageFlags::FRAGMENT)];
        let info = vk::DescriptorSetLayoutCreateInfo::default().bindings(&bindings);
        // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
        self.layout = unsafe {
            self.runtime
                .device()
                .create_descriptor_set_layout(&info, None)
        }
        .unwrap();
    }
    fn create_pool(&mut self) {
        let sizes = [vk::DescriptorPoolSize::default()
            .ty(vk::DescriptorType::UNIFORM_BUFFER)
            .descriptor_count(1)];
        let info = vk::DescriptorPoolCreateInfo::default()
            .max_sets(1)
            .pool_sizes(&sizes);
        // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
        self.pool = unsafe { self.runtime.device().create_descriptor_pool(&info, None) }.unwrap();
    }
    fn allocate_set(&mut self) {
        let layouts = [self.layout];
        let info = vk::DescriptorSetAllocateInfo::default()
            .descriptor_pool(self.pool)
            .set_layouts(&layouts);
        // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
        self.set = unsafe { self.runtime.device().allocate_descriptor_sets(&info) }.unwrap()[0];
    }
    fn bind_block(&self) {
        let blocks = [vk::DescriptorBufferInfo::default()
            .buffer(self.buffer.handle)
            .range(vk::WHOLE_SIZE)];
        let writes = [vk::WriteDescriptorSet::default()
            .dst_set(self.set)
            .dst_binding(0)
            .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
            .buffer_info(&blocks)];
        // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
        unsafe { self.runtime.device().update_descriptor_sets(&writes, &[]) };
    }
}
impl Drop for UniformInput {
    fn drop(&mut self) {
        // SAFETY: the handle belongs to this device and its owner releases it only after the GPU work that used it has completed.
        unsafe {
            self.runtime
                .device()
                .destroy_descriptor_pool(self.pool, None);
            self.runtime
                .device()
                .destroy_descriptor_set_layout(self.layout, None);
        }
    }
}

fn uploaded_uniform_buffer(runtime: &Arc<VulkanRuntime>, bytes: &[u8]) -> DeviceBuffer {
    let buffer = DeviceBuffer::new(
        runtime,
        416,
        vk::BufferUsageFlags::UNIFORM_BUFFER,
        HostBufferAccess::Upload,
    )
    .unwrap();
    buffer.write(bytes).unwrap();
    assert_eq!(buffer.read(416).unwrap(), bytes);
    buffer
}

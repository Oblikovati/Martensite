//! Owned storage descriptors share upload and lifetime rules for sprites and clipping.
use super::{
    buffer::DeviceBuffer,
    host_upload_pages::{FrameUploadPages, UploadRange},
    memory_selection::HostBufferAccess,
    runtime::{VulkanRuntime, vk_error},
};
use crate::{RenderError, error::rejected};
use ash::vk;
use std::sync::Arc;

#[path = "storage_set_pool.rs"]
mod set_pool;
use set_pool::AllocatedStorageSet;
pub(super) use set_pool::{FrameStorageSets, StorageSetPool};

#[path = "lighting_binding_cache.rs"]
mod lighting_bindings;
pub(super) use lighting_bindings::{FrameLightingBindings, LightingBindingCache};

pub(super) struct StorageDescriptorLayout {
    runtime: Arc<VulkanRuntime>,
    pub handle: vk::DescriptorSetLayout,
    stages: vk::ShaderStageFlags,
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
            stages,
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
    // Retire the descriptor with its pool before releasing its buffer.
    owner: AllocatedStorageSet,
    pub set: vk::DescriptorSet,
    buffer: StorageBuffer,
    #[cfg(feature = "command-state-diagnostics")]
    diagnostic_bytes: Vec<u8>,
}
enum StorageBuffer {
    Dedicated(DeviceBuffer),
    FrameRange(UploadRange),
}

impl StorageBuffer {
    fn info(&self, size: u64) -> vk::DescriptorBufferInfo {
        let (buffer, offset) = match self {
            Self::Dedicated(buffer) => (buffer.handle, 0),
            Self::FrameRange(range) => (range.buffer.handle, range.offset),
        };
        vk::DescriptorBufferInfo::default()
            .buffer(buffer)
            .offset(offset)
            .range(size)
    }

    fn read(&self, size: usize) -> Result<Vec<u8>, RenderError> {
        match self {
            Self::Dedicated(buffer) => buffer.read(size),
            Self::FrameRange(range) => Err(rejected(
                range.offset,
                "a dedicated writable storage buffer for readback",
            )),
        }
    }
}

impl UploadedStorage {
    pub fn new(
        runtime: &Arc<VulkanRuntime>,
        layout: &StorageDescriptorLayout,
        bytes: &[u8],
    ) -> Result<Self, RenderError> {
        let size = storage_size(runtime, bytes)?;
        let buffer = upload_storage_bytes(runtime, bytes, size)?;
        let set = AllocatedStorageSet::dedicated(runtime, layout.handle)?;
        Ok(Self::bind_buffer(
            layout,
            bytes,
            size,
            StorageBuffer::Dedicated(buffer),
            set,
        ))
    }

    /// Bind read-only per-draw storage in the frame's aligned upload pages.
    pub(super) fn for_frame(
        runtime: &Arc<VulkanRuntime>,
        layout: &StorageDescriptorLayout,
        bytes: &[u8],
        pages: &mut FrameUploadPages<'_>,
    ) -> Result<Self, RenderError> {
        // Replay compares exact one-buffer descriptor payloads at offset zero.
        if cfg!(feature = "command-state-diagnostics") {
            return Self::new(runtime, layout, bytes);
        }
        let size = storage_size(runtime, bytes)?;
        let range = pages.write_storage(runtime, bytes)?;
        let set = pages.sets.allocate(runtime, layout.handle)?;
        Ok(Self::bind_buffer(
            layout,
            bytes,
            size,
            StorageBuffer::FrameRange(range),
            set,
        ))
    }

    fn bind_buffer(
        _layout: &StorageDescriptorLayout,
        bytes: &[u8],
        size: u64,
        buffer: StorageBuffer,
        owner: AllocatedStorageSet,
    ) -> Self {
        let set = owner.handle;
        let mut sprite = Self {
            owner,
            set,
            buffer,
            #[cfg(feature = "command-state-diagnostics")]
            diagnostic_bytes: Vec::new(),
        };
        #[cfg(feature = "command-state-diagnostics")]
        super::replay_resource_diagnostic::allocated(set, _layout.handle);
        sprite.bind(size);
        sprite.observe_upload(bytes);
        sprite
    }

    fn observe_upload(&mut self, _bytes: &[u8]) {
        #[cfg(feature = "command-state-diagnostics")]
        {
            // Retain only successful uploads; this is the slice written to the actual buffer.
            self.diagnostic_bytes = _bytes.to_vec();
            super::replay_command_diagnostic::storage(
                self.set,
                self.buffer.info(0).buffer,
                _bytes,
                false,
            );
        }
    }
    #[cfg(feature = "command-state-diagnostics")]
    pub(super) fn diagnostic_storage(&self) -> (vk::Buffer, &[u8]) {
        (self.buffer.info(0).buffer, &self.diagnostic_bytes)
    }
    /// Read only after the owning submission fence and shader-to-host dependency completed.
    pub(super) fn read_after_fence(&self, size: usize) -> Result<Vec<u8>, RenderError> {
        let bytes = self.buffer.read(size)?;
        #[cfg(feature = "command-state-diagnostics")]
        super::replay_command_diagnostic::storage(
            self.set,
            self.buffer.info(0).buffer,
            &bytes,
            true,
        );
        Ok(bytes)
    }
    #[cfg(test)]
    #[cfg_attr(not(target_os = "linux"), allow(dead_code))] // Its users are Linux window tests.
    pub(super) fn test_readback(&self, size: usize) -> Result<Vec<u8>, RenderError> {
        // Caller records shader-to-host dependency and waits for its completed fence.
        self.buffer.read(size)
    }
    fn bind(&self, bytes: u64) {
        let buffer = [self.buffer.info(bytes)];
        let write = [vk::WriteDescriptorSet::default()
            .dst_set(self.set)
            .dst_binding(0)
            .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
            .buffer_info(&buffer)];
        // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
        unsafe {
            self.owner
                .runtime()
                .device()
                .update_descriptor_sets(&write, &[])
        };
        #[cfg(feature = "command-state-diagnostics")]
        super::replay_command_diagnostic::descriptor_writes(&write);
    }
}
fn storage_size(runtime: &VulkanRuntime, bytes: &[u8]) -> Result<u64, RenderError> {
    let size = bytes.len() as u64;
    if size == 0 || size > u64::from(runtime.limits.max_storage_buffer_range) {
        return Err(rejected(
            size,
            "one or more storage bytes within maxStorageBufferRange",
        ));
    }
    Ok(size)
}

fn upload_storage_bytes(
    runtime: &Arc<VulkanRuntime>,
    bytes: &[u8],
    size: u64,
) -> Result<DeviceBuffer, RenderError> {
    let pixels = DeviceBuffer::new(
        runtime,
        size,
        vk::BufferUsageFlags::STORAGE_BUFFER,
        HostBufferAccess::Upload,
    )?;
    pixels.write(bytes)?;
    Ok(pixels)
}
#[cfg(all(test, not(feature = "command-state-diagnostics")))]
mod page_binding_tests {
    use super::set_pool::SETS_PER_PAGE;
    use super::{StorageBuffer, StorageDescriptorLayout, UploadedStorage};
    use crate::backend::{
        host_upload_pages::{FrameStoragePool, FrameUploadPages},
        resource_lifecycle_test_support::create_resource_renderer,
    };
    use crate::{PrimitiveDraw, RenderBackend, RenderFrame, RenderVertex, identity_clip};
    use ash::vk;

    #[test]
    fn device_binds_distinct_aligned_ranges_to_independent_descriptor_sets() {
        let renderer = create_resource_renderer();
        let runtime = &renderer.runtime;
        let pool = FrameStoragePool::default();
        let layout = StorageDescriptorLayout::new(runtime, vk::ShaderStageFlags::FRAGMENT).unwrap();
        let mut frame = FrameUploadPages::storage(&pool);
        let first = UploadedStorage::for_frame(runtime, &layout, &[41; 12], &mut frame).unwrap();
        let second = UploadedStorage::for_frame(runtime, &layout, &[93; 8], &mut frame).unwrap();
        let second_info = second.buffer.info(8);
        assert_ne!(first.set, second.set);
        assert_eq!(first.owner.pool_handle(), second.owner.pool_handle());
        assert_eq!(first.buffer.info(12).buffer, second_info.buffer);
        assert_frame_ranges(&second, runtime.limits.min_storage_buffer_offset_alignment);
        assert!(renderer.validation_errors().unwrap().is_empty());
    }

    fn assert_frame_ranges(second: &UploadedStorage, alignment: u64) {
        let StorageBuffer::FrameRange(range) = &second.buffer else {
            panic!("expected the aligned frame page");
        };
        assert!(range.offset >= 12 && range.offset.is_multiple_of(alignment));
        let offset = usize::try_from(range.offset).unwrap();
        let contents = range.buffer.read(offset + 8).unwrap();
        assert_eq!(&contents[..12], &[41; 12]);
        assert_eq!(&contents[offset..], &[93; 8]);
    }

    #[test]
    fn descriptor_owner_prevents_page_reuse_after_preparation_ends() {
        let renderer = create_resource_renderer();
        let runtime = &renderer.runtime;
        let pool = FrameStoragePool::default();
        let layout = StorageDescriptorLayout::new(runtime, vk::ShaderStageFlags::FRAGMENT).unwrap();
        let first = UploadedStorage::for_frame(
            runtime,
            &layout,
            &[11; 4],
            &mut FrameUploadPages::storage(&pool),
        )
        .unwrap();
        let retained = first.buffer.info(4).buffer;
        let retained_sets = first.owner.pool_handle();
        let concurrent = UploadedStorage::for_frame(
            runtime,
            &layout,
            &[22; 4],
            &mut FrameUploadPages::storage(&pool),
        )
        .unwrap();
        assert_ne!(retained, concurrent.buffer.info(4).buffer);
        assert_ne!(retained_sets, concurrent.owner.pool_handle());
        drop(first);
        let next = UploadedStorage::for_frame(
            runtime,
            &layout,
            &[33; 4],
            &mut FrameUploadPages::storage(&pool),
        )
        .unwrap();
        assert_eq!(retained, next.buffer.info(4).buffer);
        assert_eq!(retained_sets, next.owner.pool_handle());
        assert!(renderer.validation_errors().unwrap().is_empty());
    }

    #[test]
    fn descriptor_cursor_spills_to_a_second_page_when_full() {
        let renderer = create_resource_renderer();
        let runtime = &renderer.runtime;
        let pool = FrameStoragePool::default();
        let layout = StorageDescriptorLayout::new(runtime, vk::ShaderStageFlags::FRAGMENT).unwrap();
        let mut frame = FrameUploadPages::storage(&pool);
        let first_page = (0..SETS_PER_PAGE)
            .map(|_| UploadedStorage::for_frame(runtime, &layout, &[1; 4], &mut frame).unwrap())
            .collect::<Vec<_>>();
        assert!(
            first_page
                .iter()
                .all(|set| set.owner.pool_handle() == first_page[0].owner.pool_handle())
        );
        let spill = UploadedStorage::for_frame(runtime, &layout, &[2; 4], &mut frame).unwrap();
        assert_ne!(spill.owner.pool_handle(), first_page[0].owner.pool_handle());
        assert!(renderer.validation_errors().unwrap().is_empty());
    }

    #[test]
    fn separate_hatch_draws_read_their_own_frame_storage_ranges() {
        let mut renderer = create_resource_renderer();
        let red = hatch_draw([1., 0., 0., 1.], 0.5, u32::MAX);
        let green = hatch_draw([0., 1., 0., 1.], 0., 0);
        let mut frame = RenderFrame::new(4, 4, [0., 0., 1., 1.], vec![red, green]).unwrap();
        assert_eq!(
            renderer.render(&frame).unwrap().rgba,
            [255, 0, 0, 255].repeat(16)
        );
        frame.draws[0].hatch_rows = Some([0; 32]);
        frame.draws[1].hatch_rows = Some([u32::MAX; 32]);
        assert_eq!(
            renderer.render(&frame).unwrap().rgba,
            [0, 255, 0, 255].repeat(16)
        );
        assert!(renderer.validation_errors().unwrap().is_empty());
    }

    fn hatch_draw(color: [f32; 4], z: f32, mask: u32) -> PrimitiveDraw {
        let positions = [[-1., -1., z], [1., -1., z], [1., 1., z], [-1., 1., z]];
        let vertices = positions
            .map(|position| RenderVertex::new(position, color))
            .to_vec();
        let mut draw =
            PrimitiveDraw::new(vertices, vec![0, 1, 2, 0, 2, 3], identity_clip()).unwrap();
        draw.hatch_rows = Some([mask; 32]);
        draw
    }
}

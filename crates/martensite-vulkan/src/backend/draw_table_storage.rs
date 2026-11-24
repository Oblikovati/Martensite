//! The transform and material arrays of one raster table share a descriptor set.
use super::*;

pub(in crate::backend) struct DrawTableStorage {
    transforms: UploadedStorage,
    _materials: Option<UploadRange>,
}

impl std::ops::Deref for DrawTableStorage {
    type Target = UploadedStorage;

    fn deref(&self) -> &Self::Target {
        &self.transforms
    }
}

impl StorageDescriptorLayout {
    pub(in crate::backend) fn draw_table(
        runtime: &Arc<VulkanRuntime>,
        indexed_materials: bool,
    ) -> Result<Self, RenderError> {
        let vertex = vk::ShaderStageFlags::VERTEX;
        if !indexed_materials {
            return Self::new(runtime, vertex);
        }
        let storage = vk::DescriptorType::STORAGE_BUFFER;
        Self::for_bindings(
            runtime,
            &[
                storage_binding(0, storage, vertex),
                storage_binding(1, storage, vertex | vk::ShaderStageFlags::FRAGMENT),
            ],
        )
    }
}

impl DrawTableStorage {
    pub(in crate::backend) fn upload(
        runtime: &Arc<VulkanRuntime>,
        layout: &StorageDescriptorLayout,
        transforms: &[u8],
        materials: Option<&[u8]>,
        pages: &mut FrameUploadPages<'_>,
    ) -> Result<Self, RenderError> {
        let Some(materials) = materials else {
            return UploadedStorage::for_frame(runtime, layout, transforms, pages).map(
                |transforms| Self {
                    transforms,
                    _materials: None,
                },
            );
        };
        let matrices_size = storage_size(runtime, transforms)?;
        let materials_size = storage_size(runtime, materials)?;
        let matrices = pages.write_storage(runtime, transforms)?;
        let materials = pages.write_storage(runtime, materials)?;
        let set = pages.sets.allocate_bindings(runtime, layout.handle, 2)?;
        let transforms = UploadedStorage::bind_buffer(
            layout,
            transforms,
            matrices_size,
            StorageBuffer::FrameRange(matrices),
            set,
        );
        bind_materials(runtime, transforms.set, &materials, materials_size);
        Ok(Self {
            transforms,
            _materials: Some(materials),
        })
    }
}

fn bind_materials(runtime: &VulkanRuntime, set: vk::DescriptorSet, range: &UploadRange, size: u64) {
    let buffer = [vk::DescriptorBufferInfo::default()
        .buffer(range.buffer.handle)
        .offset(range.offset)
        .range(size)];
    let write = [vk::WriteDescriptorSet::default()
        .dst_set(set)
        .dst_binding(1)
        .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
        .buffer_info(&buffer)];
    // SAFETY: the frame has exclusive access to this set during construction and keeps both
    // buffer ranges alive through its fence. The descriptor write borrows live stack values.
    unsafe { runtime.device().update_descriptor_sets(&write, &[]) };
}

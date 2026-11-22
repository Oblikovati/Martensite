//! Stock raster draws share immutable lighting descriptors by exact encoded bytes.
use super::{AllocatedStorageSet, StorageDescriptorLayout, UploadedStorage};
use crate::backend::{host_upload_pages::FrameUploadPages, runtime::VulkanRuntime};
use crate::{
    PrimitiveDraw, RenderDeviceFailureClass, RenderError, RenderLighting, error::rejected,
};
use ash::vk;
use std::{
    collections::VecDeque,
    sync::{Arc, Mutex, PoisonError},
};

const MAX_BINDINGS: usize = 128;
const MAX_CACHED_LIGHTING_BYTES: usize = 1024;

pub(in crate::backend) fn uses_lighting_pages(draws: &[PrimitiveDraw]) -> bool {
    if cfg!(feature = "command-state-diagnostics") {
        return false;
    }
    // Common retained lighting already shares one static binding. Sample a bounded
    // prefix for placements whose matrix differences defeat that exact reuse.
    let mut stock = draws.iter().take(64).filter_map(stock_opaque_lighting);
    let Some(first) = stock.next() else {
        return false;
    };
    stock.any(|lighting| lighting.world != first.world)
}

fn stock_opaque_lighting(draw: &PrimitiveDraw) -> Option<&RenderLighting> {
    let lighting = draw.lighting.as_ref()?;
    (draw.topology() == crate::PrimitiveTopology::Triangles
        && !draw.surface_blend
        && !draw.surface_forced_blend
        && lighting.fixed.is_none()
        && !lighting.pbr)
        .then_some(lighting)
}

#[derive(Default)]
pub(in crate::backend) struct LightingBindingCache {
    bindings: Mutex<VecDeque<Arc<RetainedLightingBinding>>>,
    /// The fingerprint of the last candidate that missed. Bytes are retained only when they
    /// come back, so an orbit, whose lighting changes every frame, creates no buffer (#594).
    last_miss: Mutex<Option<u64>>,
}

struct RetainedLightingBinding {
    bytes: Box<[u8]>,
    stages: vk::ShaderStageFlags,
    descriptor: vk::DescriptorType,
    storage: Arc<UploadedStorage>,
}

struct LightingPage {
    set: AllocatedStorageSet,
    buffer: Arc<crate::backend::buffer::DeviceBuffer>,
    bytes: usize,
    stages: vk::ShaderStageFlags,
}

/// One candidate per frame keeps cold or heterogeneous scenes from churning the device cache.
#[derive(Default)]
pub(in crate::backend) struct FrameLightingBindings {
    scratch: Vec<u8>,
    candidate_checked: bool,
    candidate: Option<Arc<RetainedLightingBinding>>,
    first_static: Option<RetainedLightingBinding>,
    page: Option<LightingPage>,
}

impl LightingBindingCache {
    pub(in crate::backend) fn bind_draw(
        &self,
        runtime: &Arc<VulkanRuntime>,
        layout: &StorageDescriptorLayout,
        lighting: &RenderLighting,
        pages: &mut FrameUploadPages<'_>,
        retain: bool,
    ) -> Result<Arc<UploadedStorage>, RenderError> {
        let mut bytes = std::mem::take(&mut pages.lighting.scratch);
        lighting.encode_into(&mut bytes);
        let bound = self.bind_encoded(runtime, layout, &bytes, pages, retain);
        pages.lighting.scratch = bytes;
        bound
    }

    fn bind_encoded(
        &self,
        runtime: &Arc<VulkanRuntime>,
        layout: &StorageDescriptorLayout,
        bytes: &[u8],
        pages: &mut FrameUploadPages<'_>,
        retain: bool,
    ) -> Result<Arc<UploadedStorage>, RenderError> {
        let cacheable = retain
            && !cfg!(feature = "command-state-diagnostics")
            && bytes.len() <= MAX_CACHED_LIGHTING_BYTES;
        if cacheable {
            if let Some(binding) = self.candidate(runtime, layout, bytes, &mut pages.lighting)? {
                return Ok(binding);
            }
            if let Some(binding) = pages
                .lighting
                .first_static
                .as_ref()
                .filter(|entry| entry.matches(layout, bytes))
            {
                return Ok(Arc::clone(&binding.storage));
            }
        }
        if layout.uses_dynamic_offset() {
            return bind_lighting_range(runtime, layout, bytes, pages);
        }
        let binding = Arc::new(UploadedStorage::for_frame(runtime, layout, bytes, pages)?);
        if cacheable && pages.lighting.first_static.is_none() {
            // A camera orbit changes lighting each frame, even when all draws in that frame
            // share it. Keep the uploaded binding local so it never occupies the device cache.
            pages.lighting.first_static = Some(RetainedLightingBinding {
                bytes: bytes.into(),
                stages: layout.stages,
                descriptor: layout.descriptor,
                storage: Arc::clone(&binding),
            });
        }
        Ok(binding)
    }

    fn candidate(
        &self,
        runtime: &Arc<VulkanRuntime>,
        layout: &StorageDescriptorLayout,
        bytes: &[u8],
        frame: &mut FrameLightingBindings,
    ) -> Result<Option<Arc<UploadedStorage>>, RenderError> {
        if !frame.candidate_checked {
            frame.candidate_checked = true;
            frame.candidate = self.find_or_insert(runtime, layout, bytes)?;
        }
        Ok(frame
            .candidate
            .as_ref()
            .filter(|entry| entry.matches(layout, bytes))
            .map(|entry| Arc::clone(&entry.storage)))
    }

    fn find_or_insert(
        &self,
        runtime: &Arc<VulkanRuntime>,
        layout: &StorageDescriptorLayout,
        bytes: &[u8],
    ) -> Result<Option<Arc<RetainedLightingBinding>>, RenderError> {
        let mut bindings = self.bindings.lock().unwrap_or_else(PoisonError::into_inner);
        if let Some(hit) = promote_hit(&mut bindings, layout, bytes) {
            return Ok(Some(hit));
        }
        if !self.recurs(bytes) {
            return Ok(None);
        }
        let Some(binding) = allocate_binding(runtime, layout, bytes)? else {
            return Ok(None);
        };
        if bindings.len() == MAX_BINDINGS {
            bindings.pop_front();
        }
        bindings.push_back(Arc::clone(&binding));
        Ok(Some(binding))
    }
}

impl LightingBindingCache {
    /// Whether `bytes` missed the cache last time too; remembers them otherwise.
    fn recurs(&self, bytes: &[u8]) -> bool {
        use std::hash::{BuildHasher, BuildHasherDefault, DefaultHasher};
        let fingerprint = BuildHasherDefault::<DefaultHasher>::default().hash_one(bytes);
        let mut last = self
            .last_miss
            .lock()
            .unwrap_or_else(PoisonError::into_inner);
        last.replace(fingerprint) == Some(fingerprint)
    }
}

fn bind_lighting_range(
    runtime: &Arc<VulkanRuntime>,
    layout: &StorageDescriptorLayout,
    bytes: &[u8],
    pages: &mut FrameUploadPages<'_>,
) -> Result<Arc<UploadedStorage>, RenderError> {
    let size = super::storage_size(runtime, bytes)?;
    let range = pages.write_storage(runtime, bytes)?;
    u32::try_from(range.offset)
        .map_err(|_| rejected(range.offset, "a 32-bit dynamic lighting offset"))?;
    let candidate = &mut pages.lighting.page;
    if let Some(page) = candidate.as_ref()
        && Arc::ptr_eq(&page.buffer, &range.buffer)
        && page.bytes == bytes.len()
        && page.stages == layout.stages
    {
        return UploadedStorage::shared_frame_range(range, &page.set).map(Arc::new);
    }
    let page = LightingPage {
        set: pages.dynamic_sets.allocate(runtime, layout.handle)?,
        buffer: Arc::clone(&range.buffer),
        bytes: bytes.len(),
        stages: layout.stages,
    };
    let storage = UploadedStorage::shared_frame_range(range, &page.set)?;
    storage.bind(size, layout.descriptor);
    *candidate = Some(page);
    Ok(Arc::new(storage))
}

fn promote_hit(
    bindings: &mut VecDeque<Arc<RetainedLightingBinding>>,
    layout: &StorageDescriptorLayout,
    bytes: &[u8],
) -> Option<Arc<RetainedLightingBinding>> {
    let index = bindings
        .iter()
        .rposition(|entry| entry.matches(layout, bytes))?;
    let hit = bindings.remove(index)?;
    bindings.push_back(Arc::clone(&hit));
    Some(hit)
}

fn allocate_binding(
    runtime: &Arc<VulkanRuntime>,
    layout: &StorageDescriptorLayout,
    bytes: &[u8],
) -> Result<Option<Arc<RetainedLightingBinding>>, RenderError> {
    // Dedicated backing keeps the frame's reusable pages free. Those bytes are host-coherent
    // and written before publication, so the record never refers to an unfinished transfer.
    let storage = match UploadedStorage::new(runtime, layout, bytes) {
        Ok(storage) => storage,
        Err(error) if optional_allocation_refusal(&error) => return Ok(None),
        Err(error) => return Err(error),
    };
    Ok(Some(Arc::new(RetainedLightingBinding {
        bytes: bytes.into(),
        stages: layout.stages,
        descriptor: layout.descriptor,
        storage: Arc::new(storage),
    })))
}

fn optional_allocation_refusal(error: &RenderError) -> bool {
    error.device_failure().is_some_and(|failure| {
        failure.class() == RenderDeviceFailureClass::OutOfMemory
            || failure.result_code() == Some(vk::Result::ERROR_TOO_MANY_OBJECTS.as_raw())
    })
}

impl RetainedLightingBinding {
    fn matches(&self, layout: &StorageDescriptorLayout, bytes: &[u8]) -> bool {
        self.stages == layout.stages
            && self.descriptor == layout.descriptor
            && &*self.bytes == bytes
    }
}

#[cfg(all(test, not(feature = "command-state-diagnostics")))]
#[path = "lighting_binding_cache_tests.rs"]
mod tests;

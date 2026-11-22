//! Stock raster draws share immutable lighting descriptors by exact encoded bytes.
use super::{StorageDescriptorLayout, UploadedStorage};
use crate::backend::{host_upload_pages::FrameUploadPages, runtime::VulkanRuntime};
use crate::{RenderDeviceFailureClass, RenderError, RenderLighting};
use ash::vk;
use std::{
    collections::VecDeque,
    sync::{Arc, Mutex, PoisonError},
};

const MAX_BINDINGS: usize = 128;
const MAX_CACHED_LIGHTING_BYTES: usize = 1024;

#[derive(Default)]
pub(in crate::backend) struct LightingBindingCache {
    bindings: Mutex<VecDeque<Arc<RetainedLightingBinding>>>,
}

struct RetainedLightingBinding {
    bytes: Box<[u8]>,
    stages: vk::ShaderStageFlags,
    storage: Arc<UploadedStorage>,
}

/// One candidate per frame keeps cold or heterogeneous scenes from churning the device cache.
#[derive(Default)]
pub(in crate::backend) struct FrameLightingBindings {
    scratch: Vec<u8>,
    candidate_checked: bool,
    candidate: Option<Arc<RetainedLightingBinding>>,
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
        if retain
            && !cfg!(feature = "command-state-diagnostics")
            && bytes.len() <= MAX_CACHED_LIGHTING_BYTES
            && let Some(binding) = self.candidate(runtime, layout, bytes, &mut pages.lighting)?
        {
            return Ok(binding);
        }
        UploadedStorage::for_frame(runtime, layout, bytes, pages).map(Arc::new)
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
        // Stage visibility is the only variable in this layout's fixed one-storage-binding definition.
        self.stages == layout.stages && &*self.bytes == bytes
    }
}

#[cfg(all(test, not(feature = "command-state-diagnostics")))]
#[path = "lighting_binding_cache_tests.rs"]
mod tests;

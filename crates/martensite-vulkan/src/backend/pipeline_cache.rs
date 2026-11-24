//! One cache belongs to the logical device until its last pipeline owner retires.
#[path = "pipeline_cache_data.rs"]
mod data;

use ash::{Device, vk};
pub(super) use data::{CacheIdentity, CacheStorage};
use std::{
    ptr,
    sync::{Mutex, PoisonError},
};

#[derive(Default)]
pub(super) struct PipelineCache {
    state: Mutex<CacheState>,
    storage: Option<CacheStorage>,
    loaded_checksum: Option<u64>,
}

#[derive(Default)]
struct CacheState {
    handle: vk::PipelineCache,
    compiled: bool,
}

impl PipelineCache {
    /// Restore a cache for this device, falling back to an empty cache on storage failure.
    pub(super) fn new(device: &Device, storage: Option<CacheStorage>) -> Self {
        let loaded = storage.as_ref().and_then(CacheStorage::load);
        let restored = loaded
            .as_ref()
            .and_then(|blob| create_cache(device, &blob.bytes));
        let handle = restored
            .or_else(|| create_cache(device, &[]))
            .unwrap_or_default();
        let loaded_checksum = restored.and(loaded.map(|blob| blob.checksum));
        Self {
            state: Mutex::new(CacheState {
                handle,
                compiled: false,
            }),
            storage,
            loaded_checksum,
        }
    }

    /// Serialize native cache use, such as a graphics pipeline creation.
    pub(super) fn with_handle<T, E>(
        &self,
        create: impl FnOnce(vk::PipelineCache) -> Result<T, E>,
    ) -> Result<T, E> {
        let mut state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
        let result = create(state.handle);
        state.compiled |= result.is_ok();
        result
    }

    /// Save and release the cache before its logical device is destroyed.
    pub(super) fn close(&mut self, device: &Device) {
        let state = self.state.get_mut().unwrap_or_else(PoisonError::into_inner);
        let handle = std::mem::take(&mut state.handle);
        if handle == vk::PipelineCache::null() {
            return;
        }
        if state.compiled
            && let Some(storage) = &self.storage
        {
            save_cache(storage, device, handle, self.loaded_checksum);
        }
        // SAFETY: the runtime owns the device and holds exclusive access after all other owners retired.
        unsafe { device.destroy_pipeline_cache(handle, None) };
    }
}

fn create_cache(device: &Device, bytes: &[u8]) -> Option<vk::PipelineCache> {
    let info = vk::PipelineCacheCreateInfo::default().initial_data(bytes);
    // SAFETY: the device is live and the bytes are empty or verified data retrieved for this
    // cache identity by vkGetPipelineCacheData. The buffer outlives the create call.
    unsafe { device.create_pipeline_cache(&info, None) }.ok()
}

fn save_cache(
    storage: &CacheStorage,
    device: &Device,
    handle: vk::PipelineCache,
    loaded_checksum: Option<u64>,
) {
    let Some(bytes) = snapshot(device, handle) else {
        return;
    };
    if !storage.compatible(&bytes) {
        return;
    }
    let checksum = data::checksum(&bytes);
    if Some(checksum) != loaded_checksum {
        let _ = storage.save(&bytes, checksum);
    }
}

fn snapshot(device: &Device, cache: vk::PipelineCache) -> Option<Vec<u8>> {
    let mut size = 0;
    let query = device.fp_v1_0().get_pipeline_cache_data;
    // SAFETY: the cache belongs to this device. A null output buffer requests only its size.
    let status = unsafe { query(device.handle(), cache, &mut size, ptr::null_mut()) };
    if status != vk::Result::SUCCESS || !data::valid_size(size) {
        return None;
    }
    let mut bytes = Vec::new();
    bytes.try_reserve_exact(size).ok()?;
    bytes.resize(size, 0);
    // SAFETY: the same device and cache are live, the runtime serializes access, and bytes
    // owns a writable region of the supplied size.
    let status = unsafe { query(device.handle(), cache, &mut size, bytes.as_mut_ptr().cast()) };
    if status != vk::Result::SUCCESS || size > bytes.len() {
        return None;
    }
    bytes.truncate(size);
    Some(bytes)
}

//! Mapped upload ranges remain exclusive to one preparing or submitted frame.
use super::storage_descriptor::{FrameLightingBindings, FrameStorageSets, StorageSetPool};
use super::{buffer::DeviceBuffer, memory_selection::HostBufferAccess, runtime::VulkanRuntime};
use crate::{RenderError, error::rejected};
use ash::vk;
use std::sync::{Arc, Mutex, PoisonError};

const STORAGE_PAGE_BYTES: usize = 256 << 10;
/// A frame in flight holds its page while the next frame writes another one.
const RETAINED_PAGES: usize = super::frame_flight::FRAMES_IN_FLIGHT;

/// Reusable mapped pages, one per frame in flight (#594), with dedicated pages for larger uploads.
pub(super) struct HostUploadPool {
    cached: Mutex<Vec<Arc<DeviceBuffer>>>,
    page_bytes: usize,
    usage: vk::BufferUsageFlags,
}

impl HostUploadPool {
    /// Select the pool's usage and page size, for example a transfer source staging page.
    pub(super) fn new(page_bytes: usize, usage: vk::BufferUsageFlags) -> Self {
        Self {
            cached: Mutex::new(Vec::new()),
            page_bytes,
            usage,
        }
    }

    /// Capacity to record for ranges in a newly acquired page.
    pub(super) fn capacity_for(&self, bytes: usize) -> usize {
        bytes.max(self.page_bytes)
    }

    /// Lease an unwritten page, for example while building one frame's transfer commands.
    pub(super) fn acquire(
        &self,
        runtime: &Arc<VulkanRuntime>,
        bytes: usize,
    ) -> Result<Arc<DeviceBuffer>, RenderError> {
        if bytes > self.page_bytes {
            return self.allocate(runtime, bytes);
        }
        let mut cached = self.cached.lock().unwrap_or_else(PoisonError::into_inner);
        // The pool can only lend a cached page when every earlier owner has retired.
        if let Some(idle) = cached
            .iter_mut()
            .position(|page| Arc::get_mut(page).is_some())
        {
            return Ok(Arc::clone(&cached[idle]));
        }
        let page = self.allocate(runtime, self.page_bytes)?;
        if cached.len() < RETAINED_PAGES {
            cached.push(Arc::clone(&page));
        }
        Ok(page)
    }

    fn allocate(
        &self,
        runtime: &Arc<VulkanRuntime>,
        bytes: usize,
    ) -> Result<Arc<DeviceBuffer>, RenderError> {
        DeviceBuffer::new(runtime, bytes as u64, self.usage, HostBufferAccess::Upload).map(Arc::new)
    }
}

/// The storage upload cursor of a preparing frame. Descriptors also hold their page.
pub(super) struct FrameUploadPages<'a> {
    pool: &'a HostUploadPool,
    current: Option<UploadPage>,
    pub(super) sets: FrameStorageSets<'a>,
    pub(super) dynamic_sets: FrameStorageSets<'a>,
    pub(super) lighting: FrameLightingBindings,
    /// The last glyph atlas this frame bound, with its pixels and image layout.
    pub(super) glyph_atlas: Option<FrameGlyphAtlas>,
}

/// A glyph atlas binding that the following text draws of a frame reuse without the cache
/// lock; one lock per label took 5% of `frame_text_1k` (#594). It keeps the pixels, so their
/// address cannot name another atlas while it lives.
pub(super) struct FrameGlyphAtlas {
    pub pixels: Arc<Vec<[f32; 4]>>,
    pub extent: [u32; 2],
    pub layout: usize,
    pub binding: Arc<super::surface_image_descriptor::SurfaceImageBinding>,
}

/// The reusable data and set pages of one raster storage arena.
pub(super) struct FrameStoragePool {
    buffers: HostUploadPool,
    sets: StorageSetPool,
    dynamic_sets: StorageSetPool,
}

impl Default for FrameStoragePool {
    fn default() -> Self {
        Self {
            buffers: HostUploadPool::new(STORAGE_PAGE_BYTES, vk::BufferUsageFlags::STORAGE_BUFFER),
            sets: StorageSetPool::default(),
            dynamic_sets: StorageSetPool::new(vk::DescriptorType::STORAGE_BUFFER_DYNAMIC),
        }
    }
}

struct UploadPage {
    buffer: Arc<DeviceBuffer>,
    capacity: usize,
    end: usize,
}

pub(super) struct UploadRange {
    pub(super) buffer: Arc<DeviceBuffer>,
    pub(super) offset: u64,
}

impl<'a> FrameUploadPages<'a> {
    /// Open a frame cursor over a storage page pool, for example in raster setup.
    pub(super) fn storage(pool: &'a FrameStoragePool) -> Self {
        Self {
            pool: &pool.buffers,
            current: None,
            sets: FrameStorageSets::new(&pool.sets),
            dynamic_sets: FrameStorageSets::new(&pool.dynamic_sets),
            lighting: FrameLightingBindings::default(),
            glyph_atlas: None,
        }
    }

    /// Upload one descriptor payload at the device's required storage buffer alignment.
    pub(super) fn write_storage(
        &mut self,
        runtime: &Arc<VulkanRuntime>,
        bytes: &[u8],
    ) -> Result<UploadRange, RenderError> {
        let alignment = storage_alignment(runtime)?;
        if let Some(page) = self.current.as_mut()
            && let Some(offset) = page.offset_for(bytes.len(), alignment)
        {
            return page.write(offset, bytes);
        }
        let mut page = UploadPage {
            buffer: self.pool.acquire(runtime, bytes.len())?,
            capacity: self.pool.capacity_for(bytes.len()),
            end: 0,
        };
        let range = page.write(0, bytes)?;
        self.current = Some(page);
        Ok(range)
    }
}

impl UploadPage {
    fn offset_for(&self, bytes: usize, alignment: usize) -> Option<usize> {
        aligned_offset(self.end, self.capacity, bytes, alignment)
    }

    fn write(&mut self, offset: usize, bytes: &[u8]) -> Result<UploadRange, RenderError> {
        self.buffer.write_at(offset, bytes)?;
        self.end = offset + bytes.len();
        Ok(UploadRange {
            buffer: Arc::clone(&self.buffer),
            offset: offset as u64,
        })
    }
}

fn storage_alignment(runtime: &VulkanRuntime) -> Result<usize, RenderError> {
    let alignment = runtime.limits.min_storage_buffer_offset_alignment;
    usize::try_from(alignment)
        .ok()
        .filter(|value| *value > 0)
        .ok_or_else(|| rejected(alignment, "a positive host-sized storage offset alignment"))
}

fn aligned_offset(end: usize, capacity: usize, bytes: usize, alignment: usize) -> Option<usize> {
    let offset = end.checked_next_multiple_of(alignment)?;
    (offset.checked_add(bytes)? <= capacity).then_some(offset)
}

#[cfg(test)]
mod tests {
    use super::aligned_offset;

    #[test]
    fn storage_ranges_respect_device_alignment_and_capacity() {
        assert_eq!(aligned_offset(0, 256, 5, 16), Some(0));
        assert_eq!(aligned_offset(5, 256, 120, 16), Some(16));
        assert_eq!(aligned_offset(16, 256, 128, 128), Some(128));
        assert_eq!(aligned_offset(16, 256, 129, 128), None);
    }

    #[test]
    fn range_arithmetic_refuses_alignment_and_addition_overflow() {
        assert_eq!(aligned_offset(usize::MAX, usize::MAX, 1, 2), None);
        assert_eq!(aligned_offset(usize::MAX, usize::MAX, 1, 1), None);
    }
}

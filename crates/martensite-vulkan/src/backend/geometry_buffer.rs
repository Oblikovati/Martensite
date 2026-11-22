//! Small geometry streams share a buffer. A range is reusable only when its last owner drops.
use super::{
    buffer::DeviceBuffer, frame_work_ledger::DeviceCount, memory_arena::BlockSpace,
    runtime::VulkanRuntime,
};
use crate::{RenderDeviceFailureClass, RenderError};
use ash::vk;
use std::sync::{
    Arc, Mutex, PoisonError, Weak,
    atomic::{AtomicU64, Ordering},
};

const PAGE_BYTES: u64 = 256 << 10;
const MAX_SHARED_STREAM_BYTES: u64 = PAGE_BYTES / 8;

pub(super) struct GeometryBuffer {
    pub handle: vk::Buffer,
    pub offset: u64,
    size: u64,
    pub alignment: Option<u64>,
    backing: GeometryBacking,
}

enum GeometryBacking {
    Dedicated(DeviceBuffer),
    Shared(Arc<GeometryPage>),
}

struct GeometryPage {
    buffer: DeviceBuffer,
    space: Mutex<BlockSpace>,
    _charge: PageCharge,
}

struct PageCharge(Arc<AtomicU64>);

pub(super) struct GeometryBufferPool {
    vertices: Mutex<Weak<GeometryPage>>,
    indices: Mutex<Weak<GeometryPage>>,
    bytes: Arc<AtomicU64>,
    byte_limit: u64,
}

impl GeometryBufferPool {
    pub(super) fn new(byte_limit: u64) -> Self {
        Self {
            vertices: Mutex::default(),
            indices: Mutex::default(),
            bytes: Arc::default(),
            byte_limit,
        }
    }

    pub(super) fn allocate(
        &self,
        runtime: &Arc<VulkanRuntime>,
        usage: vk::BufferUsageFlags,
        size: u64,
        alignment: Option<u64>,
    ) -> Result<Arc<GeometryBuffer>, RenderError> {
        if let Some(alignment) = alignment.filter(|_| (1..=MAX_SHARED_STREAM_BYTES).contains(&size))
            && let Some(range) = self.shared_range(runtime, usage, size, alignment)?
        {
            return Ok(Arc::new(range));
        }
        let buffer = create_geometry_buffer(runtime, usage, size)?;
        Ok(Arc::new(GeometryBuffer::dedicated(buffer, alignment)))
    }

    fn shared_range(
        &self,
        runtime: &Arc<VulkanRuntime>,
        usage: vk::BufferUsageFlags,
        size: u64,
        alignment: u64,
    ) -> Result<Option<GeometryBuffer>, RenderError> {
        let slot = match usage {
            vk::BufferUsageFlags::VERTEX_BUFFER => &self.vertices,
            vk::BufferUsageFlags::INDEX_BUFFER => &self.indices,
            _ => return Ok(None),
        };
        let mut recent = slot.lock().unwrap_or_else(PoisonError::into_inner);
        if let Some(range) = recent
            .upgrade()
            .and_then(|page| page.reserve(size, alignment))
        {
            return Ok(Some(range));
        }
        let Some(page) = self.new_page(runtime, usage)? else {
            return Ok(None);
        };
        *recent = Arc::downgrade(&page);
        Ok(page.reserve(size, alignment))
    }

    fn new_page(
        &self,
        runtime: &Arc<VulkanRuntime>,
        usage: vk::BufferUsageFlags,
    ) -> Result<Option<Arc<GeometryPage>>, RenderError> {
        let Some(charge) = PageCharge::reserve(&self.bytes, self.byte_limit) else {
            return Ok(None);
        };
        let buffer = match create_geometry_buffer(runtime, usage, PAGE_BYTES) {
            Ok(buffer) => buffer,
            Err(error) if optional_page_refusal(&error) => return Ok(None),
            Err(error) => return Err(error),
        };
        Ok(Some(Arc::new(GeometryPage {
            buffer,
            space: Mutex::new(BlockSpace::new(PAGE_BYTES)),
            _charge: charge,
        })))
    }
}

impl GeometryBuffer {
    pub(super) fn dedicated(buffer: DeviceBuffer, alignment: Option<u64>) -> Self {
        Self {
            handle: buffer.handle,
            offset: 0,
            size: buffer.size(),
            alignment,
            backing: GeometryBacking::Dedicated(buffer),
        }
    }

    pub(super) fn size(&self) -> u64 {
        self.size
    }

    pub(super) fn allocation_size(&self) -> u64 {
        match &self.backing {
            GeometryBacking::Dedicated(buffer) => buffer.allocation_size(),
            GeometryBacking::Shared(_) => self.size,
        }
    }
}

impl Drop for GeometryBuffer {
    fn drop(&mut self) {
        if let GeometryBacking::Shared(page) = &self.backing {
            page.space
                .lock()
                .unwrap_or_else(PoisonError::into_inner)
                .release(self.offset, self.size);
        }
    }
}

impl GeometryPage {
    fn reserve(self: Arc<Self>, size: u64, alignment: u64) -> Option<GeometryBuffer> {
        let offset = self
            .space
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .reserve(size, alignment)?;
        Some(GeometryBuffer {
            handle: self.buffer.handle,
            offset,
            size,
            alignment: Some(alignment),
            backing: GeometryBacking::Shared(self),
        })
    }
}

impl PageCharge {
    fn reserve(bytes: &Arc<AtomicU64>, limit: u64) -> Option<Self> {
        bytes
            .try_update(Ordering::Relaxed, Ordering::Relaxed, |old| {
                old.checked_add(PAGE_BYTES).filter(|&new| new <= limit)
            })
            .ok()?;
        Some(Self(Arc::clone(bytes)))
    }
}

impl Drop for PageCharge {
    fn drop(&mut self) {
        self.0.fetch_sub(PAGE_BYTES, Ordering::Relaxed);
    }
}

fn create_geometry_buffer(
    runtime: &Arc<VulkanRuntime>,
    usage: vk::BufferUsageFlags,
    size: u64,
) -> Result<DeviceBuffer, RenderError> {
    let buffer = DeviceBuffer::device_local(runtime, size, usage)?;
    runtime.work.add(DeviceCount::GeometryBuffers, 1);
    Ok(buffer)
}

fn optional_page_refusal(error: &RenderError) -> bool {
    error.device_failure().is_some_and(|failure| {
        failure.class() == RenderDeviceFailureClass::OutOfMemory
            || failure.result_code() == Some(vk::Result::ERROR_TOO_MANY_OBJECTS.as_raw())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::resource_lifecycle_test_support::create_resource_renderer;

    #[test]
    fn shared_vertex_ranges_stay_disjoint_while_cloned_and_recycle_after_release() {
        let renderer = create_resource_renderer();
        let pool = GeometryBufferPool::new(PAGE_BYTES);
        let make = || {
            pool.allocate(
                &renderer.runtime,
                vk::BufferUsageFlags::VERTEX_BUFFER,
                132,
                Some(44),
            )
            .unwrap()
        };
        let first = make();
        let second = make();
        assert_eq!(first.handle, second.handle);
        assert_eq!((first.offset, second.offset), (0, 132));
        let submission = Arc::clone(&first);
        drop(first);
        assert_eq!(make().offset, 264);
        drop(submission);
        assert_eq!(make().offset, 0);
        assert_eq!(pool.bytes.load(Ordering::Relaxed), PAGE_BYTES);
        drop(second);
        assert_eq!(pool.bytes.load(Ordering::Relaxed), 0);
    }
}

//! A renderer keeps the host buffer of its last colour readback for the next one (#594).
//!
//! A new 8 MiB host buffer for each 1080p dump cost an allocation and a page fault per page
//! on its first touch. The peak memory does not grow: the dump needed the buffer anyway.
use super::{buffer::DeviceBuffer, memory_selection::HostBufferAccess, runtime::VulkanRuntime};
use crate::RenderError;
use ash::vk;
use std::sync::{Arc, Mutex, PoisonError};

/// The readback buffer a completed readback gave back; `cache.take(&runtime, bytes)?`.
#[derive(Default)]
pub(super) struct ReadbackBufferCache {
    kept: Mutex<Option<DeviceBuffer>>,
}

impl ReadbackBufferCache {
    /// A host readback buffer of at least `bytes`: the kept one when it is large enough,
    /// otherwise a new one; `cache.take(&runtime, 8_294_400)?`.
    pub fn take(
        &self,
        runtime: &Arc<VulkanRuntime>,
        bytes: u64,
    ) -> Result<DeviceBuffer, RenderError> {
        let kept = self
            .kept
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .take();
        if let Some(buffer) = kept.filter(|buffer| buffer.size() >= bytes) {
            return Ok(buffer);
        }
        new_readback_buffer(runtime, bytes)
    }
    /// Keep `buffer` for the next readback; its copy completed; `cache.give(buffer)`.
    pub fn give(&self, buffer: DeviceBuffer) {
        *self.kept.lock().unwrap_or_else(PoisonError::into_inner) = Some(buffer);
    }
}

/// A host buffer a transfer writes and the host reads; `new_readback_buffer(&runtime, 64)?`.
pub(super) fn new_readback_buffer(
    runtime: &Arc<VulkanRuntime>,
    bytes: u64,
) -> Result<DeviceBuffer, RenderError> {
    DeviceBuffer::new(
        runtime,
        bytes,
        vk::BufferUsageFlags::TRANSFER_DST,
        HostBufferAccess::Readback,
    )
}

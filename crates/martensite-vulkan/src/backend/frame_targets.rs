//! Attachment leases retain a bounded set of reusable raster and transparency targets.
use super::{
    image::{AttachmentSpec, TargetImage, attachment_properties},
    multisample_targets::MultisampleTargets,
    oit_targets::OitTargets,
    runtime::VulkanRuntime,
};
use crate::{RenderError, RenderTransparency};
use ash::vk;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex, PoisonError};

// Each family can reuse interleaved view and offscreen sizes without retaining a resize history.
const RETAINED_GENERATIONS: usize = 4;
const RETAINED_BYTES_PER_FAMILY: u64 = 64 << 20;

#[derive(Default)]
pub(super) struct FrameTargetPool {
    raster: Mutex<RasterPool>,
    transparency: Mutex<TargetSlot<TransparencyKey, OitTargets>>,
}

#[derive(Default)]
struct RasterPool {
    color: TargetSlot<ImageKey, TargetImage>,
    depth: TargetSlot<ImageKey, TargetImage>,
    multisample: TargetSlot<MultisampleKey, MultisampleTargets>,
}

struct TargetSlot<Key, Target> {
    generations: VecDeque<TargetGeneration<Key, Target>>,
    bytes: u64,
}

struct TargetGeneration<Key, Target> {
    key: Key,
    target: Arc<Target>,
    bytes: u64,
}

impl<Key, Target> Default for TargetSlot<Key, Target> {
    fn default() -> Self {
        Self {
            generations: VecDeque::new(),
            bytes: 0,
        }
    }
}

impl<Key: Eq, Target> TargetSlot<Key, Target> {
    fn lease(
        &mut self,
        key: Key,
        create: impl FnOnce() -> Result<Option<Target>, RenderError>,
        allocation_bytes: impl FnOnce(&Target) -> u64,
    ) -> Result<Option<Arc<Target>>, RenderError> {
        if let Some(target) = self.take_idle(&key) {
            return Ok(Some(target));
        }
        let Some(target) = create()? else {
            return Ok(None);
        };
        let bytes = allocation_bytes(&target);
        Ok(Some(self.retain(key, Arc::new(target), bytes)))
    }

    fn take_idle(&mut self, key: &Key) -> Option<Arc<Target>> {
        let index = self
            .generations
            .iter_mut()
            .rposition(|entry| &entry.key == key && Arc::get_mut(&mut entry.target).is_some())?;
        let entry = self.generations.remove(index)?;
        let target = Arc::clone(&entry.target);
        self.generations.push_back(entry);
        Some(target)
    }

    fn retain(&mut self, key: Key, target: Arc<Target>, bytes: u64) -> Arc<Target> {
        // An oversize frame can reuse its own allocation. It evicts all older generations.
        let allowance = RETAINED_BYTES_PER_FAMILY.saturating_sub(bytes);
        self.trim(allowance, RETAINED_GENERATIONS - 1);
        self.bytes += bytes;
        self.generations.push_back(TargetGeneration {
            key,
            target: Arc::clone(&target),
            bytes,
        });
        target
    }

    fn trim(&mut self, max_bytes: u64, max_generations: usize) {
        while self.bytes > max_bytes || self.generations.len() > max_generations {
            let Some(oldest) = self.generations.pop_front() else {
                break;
            };
            // A queued or captured frame independently pins every generation it references.
            self.bytes -= oldest.bytes;
        }
    }
}

pub(super) struct RasterTargetLease {
    pub color: Arc<TargetImage>,
    pub depth: Arc<TargetImage>,
    pub multisample: Option<Arc<MultisampleTargets>>,
}

#[derive(Clone, Copy, Eq, PartialEq)]
struct ImageKey {
    extent: vk::Extent2D,
    attachment: AttachmentSpec,
}

#[derive(Clone, Copy, Eq, PartialEq)]
struct MultisampleKey {
    extent: vk::Extent2D,
    stencil: bool,
    samples: vk::SampleCountFlags,
}

#[derive(Clone, Copy, Eq, PartialEq)]
struct TransparencyKey {
    extent: vk::Extent2D,
    samples: vk::SampleCountFlags,
    family: TransparencyFamily,
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum TransparencyFamily {
    Weighted,
    Peeling,
}

impl FrameTargetPool {
    pub(super) fn raster(
        &self,
        runtime: &Arc<VulkanRuntime>,
        extent: vk::Extent2D,
        stencil: bool,
        samples: vk::SampleCountFlags,
        subview_copy: bool,
    ) -> Result<RasterTargetLease, RenderError> {
        let mut pool = self.raster.lock().unwrap_or_else(PoisonError::into_inner);
        let multisample = pool.multisample_for(
            runtime,
            MultisampleKey {
                extent,
                stencil,
                samples,
            },
        )?;
        let color =
            pool.color
                .attachment(runtime, extent, color_attachment(runtime, subview_copy))?;
        let depth = pool
            .depth
            .attachment(runtime, extent, depth_attachment(runtime, stencil))?;
        Ok(RasterTargetLease {
            color,
            depth,
            multisample,
        })
    }

    pub(super) fn transparency(
        &self,
        runtime: &Arc<VulkanRuntime>,
        extent: vk::Extent2D,
        samples: vk::SampleCountFlags,
        mode: RenderTransparency,
    ) -> Result<Option<Arc<OitTargets>>, RenderError> {
        let mut slot = self
            .transparency
            .lock()
            .unwrap_or_else(PoisonError::into_inner);
        let (family, samples) = match mode {
            RenderTransparency::Unordered => {
                slot.trim(RETAINED_BYTES_PER_FAMILY, RETAINED_GENERATIONS);
                return Ok(None);
            }
            RenderTransparency::Weighted { .. } => (TransparencyFamily::Weighted, samples),
            RenderTransparency::DepthPeeling { .. } => {
                (TransparencyFamily::Peeling, vk::SampleCountFlags::TYPE_1)
            }
        };
        slot.lease(
            TransparencyKey {
                extent,
                samples,
                family,
            },
            || OitTargets::create(runtime, extent, mode, samples),
            OitTargets::allocation_bytes,
        )
    }
}

impl RasterPool {
    fn multisample_for(
        &mut self,
        runtime: &Arc<VulkanRuntime>,
        key: MultisampleKey,
    ) -> Result<Option<Arc<MultisampleTargets>>, RenderError> {
        if key.samples == vk::SampleCountFlags::TYPE_1 {
            self.multisample
                .trim(RETAINED_BYTES_PER_FAMILY, RETAINED_GENERATIONS);
            return Ok(None);
        }
        self.multisample.lease(
            key,
            || MultisampleTargets::new(runtime, key.extent, key.stencil, key.samples),
            MultisampleTargets::allocation_bytes,
        )
    }
}

impl TargetSlot<ImageKey, TargetImage> {
    fn attachment(
        &mut self,
        runtime: &Arc<VulkanRuntime>,
        extent: vk::Extent2D,
        attachment: AttachmentSpec,
    ) -> Result<Arc<TargetImage>, RenderError> {
        let key = ImageKey { extent, attachment };
        self.lease(
            key,
            || TargetImage::with_attachment(runtime, extent, attachment).map(Some),
            TargetImage::allocation_bytes,
        )?
        .ok_or_else(|| crate::error::rejected(extent, "an allocated render attachment"))
    }
}

fn color_attachment(runtime: &VulkanRuntime, subview_copy: bool) -> AttachmentSpec {
    let (format, mut usage, aspect) = attachment_properties(runtime.depth.formats, false, false);
    if subview_copy {
        usage |= vk::ImageUsageFlags::TRANSFER_DST;
    }
    AttachmentSpec {
        format,
        usage,
        aspect,
        samples: vk::SampleCountFlags::TYPE_1,
    }
}

fn depth_attachment(runtime: &VulkanRuntime, stencil: bool) -> AttachmentSpec {
    let (format, usage, aspect) = attachment_properties(runtime.depth.formats, true, stencil);
    AttachmentSpec {
        format,
        usage,
        aspect,
        samples: vk::SampleCountFlags::TYPE_1,
    }
}

#[cfg(test)]
#[path = "frame_target_pool_tests.rs"]
mod tests;

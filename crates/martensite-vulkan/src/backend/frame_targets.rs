//! Attachment leases retain a bounded set of reusable raster and transparency targets.
use super::{
    image::{AttachmentSpec, TargetImage, attachment_properties},
    immediate_main_record::{MainLayerImages, MainLayerTicket},
    multisample_targets::MultisampleTargets,
    oit_targets::OitTargets,
    runtime::VulkanRuntime,
};
use crate::{RenderError, RenderTransparency};
use ash::vk;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex, PoisonError, Weak};

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
    hiz_depth: TargetSlot<ImageKey, TargetImage>,
}

struct TargetSlot<Key, Target> {
    generations: VecDeque<TargetGeneration<Key, Target>>,
    bytes: u64,
}

struct TargetGeneration<Key, Target> {
    key: Key,
    target: Arc<Target>,
    bytes: u64,
    main: Option<Weak<()>>,
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
        if let Some(target) = self.take_idle(|candidate| candidate == &key) {
            return Ok(Some(target));
        }
        self.create_generation(key, create, allocation_bytes)
    }
    fn create_generation(
        &mut self,
        key: Key,
        create: impl FnOnce() -> Result<Option<Target>, RenderError>,
        allocation_bytes: impl FnOnce(&Target) -> u64,
    ) -> Result<Option<Arc<Target>>, RenderError> {
        let Some(target) = create()? else {
            return Ok(None);
        };
        let bytes = allocation_bytes(&target);
        Ok(Some(self.retain(key, Arc::new(target), bytes)))
    }

    fn take_idle(&mut self, matches: impl Fn(&Key) -> bool) -> Option<Arc<Target>> {
        let index = self.generations.iter_mut().rposition(|entry| {
            matches(&entry.key)
                && entry
                    .main
                    .as_ref()
                    .is_none_or(|mark| mark.strong_count() == 0)
                && Arc::get_mut(&mut entry.target).is_some()
        });
        let index = index.or_else(|| self.reclaim_main(matches))?;
        let mut entry = self.generations.remove(index)?;
        // A future restore may still carry the old ticket after this image gets new pixels.
        entry.main = None;
        let target = Arc::clone(&entry.target);
        self.generations.push_back(entry);
        Some(target)
    }

    fn reclaim_main(&mut self, matches: impl Fn(&Key) -> bool) -> Option<usize> {
        let full = self.generations.len() >= RETAINED_GENERATIONS;
        let bytes = self.bytes;
        self.generations.iter_mut().position(|entry| {
            matches(&entry.key)
                && (full || bytes.saturating_add(entry.bytes) > RETAINED_BYTES_PER_FAMILY)
                && Arc::get_mut(&mut entry.target).is_some()
        })
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
            main: None,
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
    pub main_publication: Option<MainLayerTicket>,
}

#[derive(Clone, Copy, Eq, PartialEq)]
struct ImageKey {
    extent: vk::Extent2D,
    attachment: AttachmentSpec,
}

impl ImageKey {
    fn without_copy_destination(mut self) -> Self {
        self.attachment.usage &= !MAIN_COPY_USAGE;
        self
    }

    fn satisfies(self, requested: Self) -> bool {
        self == requested || self.without_copy_destination() == requested
    }
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
    pub(super) fn can_reserve_main(bytes: u64) -> bool {
        // A restore needs an independently writable image of the same extent.
        bytes <= RETAINED_BYTES_PER_FAMILY / 2
    }

    pub(super) fn raster(
        &self,
        runtime: &Arc<VulkanRuntime>,
        extent: vk::Extent2D,
        stencil: bool,
        samples: vk::SampleCountFlags,
        copy_usage: [bool; 2],
        remember_main: bool,
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
        // A remembered pair may be restored by a fetch draw, so it is sampled too.
        let color = pool.color.attachment(
            runtime,
            extent,
            color_attachment(runtime, copy_usage[0] || remember_main),
        )?;
        let depth = pool.depth.attachment(
            runtime,
            extent,
            depth_attachment(
                runtime,
                stencil,
                copy_usage[1] || (remember_main && !stencil),
            ),
        )?;
        let main_publication = remember_main
            .then(|| pool.reserve_main(&color, &depth))
            .flatten();
        Ok(RasterTargetLease {
            color,
            depth,
            multisample,
            main_publication,
        })
    }

    pub(super) fn hiz_depth(
        &self,
        runtime: &Arc<VulkanRuntime>,
        extent: vk::Extent2D,
        attachment: AttachmentSpec,
    ) -> Result<Arc<TargetImage>, RenderError> {
        self.raster
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .hiz_depth
            .attachment(runtime, extent, attachment)
    }

    /// Read both generations under the same lock that admits writers to either image.
    pub(super) fn pin_main(&self, ticket: &MainLayerTicket) -> Option<MainLayerImages> {
        let pool = self.raster.lock().unwrap_or_else(PoisonError::into_inner);
        let color = pool.color.published(ticket)?;
        let depth = pool.depth.published(ticket)?;
        Some(MainLayerImages::new(Arc::clone(color), Arc::clone(depth)))
    }

    pub(super) fn revoke_main(&self, ticket: &MainLayerTicket) {
        let mut pool = self.raster.lock().unwrap_or_else(PoisonError::into_inner);
        pool.color.revoke(ticket);
        pool.depth.revoke(ticket);
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
    fn reserve_main(
        &mut self,
        color: &Arc<TargetImage>,
        depth: &Arc<TargetImage>,
    ) -> Option<MainLayerTicket> {
        let color = self.color.cacheable_generation(color)?;
        let depth = self.depth.cacheable_generation(depth)?;
        let ticket = MainLayerTicket::default();
        color.main = Some(ticket.witness());
        depth.main = Some(ticket.witness());
        Some(ticket)
    }

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
    fn cacheable_generation(
        &mut self,
        image: &Arc<TargetImage>,
    ) -> Option<&mut TargetGeneration<ImageKey, TargetImage>> {
        self.generations.iter_mut().find(|entry| {
            FrameTargetPool::can_reserve_main(entry.bytes) && Arc::ptr_eq(&entry.target, image)
        })
    }

    fn published(&self, ticket: &MainLayerTicket) -> Option<&Arc<TargetImage>> {
        self.generations.iter().find_map(|entry| {
            let mark = entry.main.as_ref()?;
            ticket.identifies(mark).then_some(&entry.target)
        })
    }

    fn revoke(&mut self, ticket: &MainLayerTicket) {
        for entry in &mut self.generations {
            if entry
                .main
                .as_ref()
                .is_some_and(|mark| ticket.identifies(mark))
            {
                entry.main = None;
            }
        }
    }

    fn attachment(
        &mut self,
        runtime: &Arc<VulkanRuntime>,
        extent: vk::Extent2D,
        attachment: AttachmentSpec,
    ) -> Result<Arc<TargetImage>, RenderError> {
        let key = ImageKey { extent, attachment };
        if let Some(target) = self.take_idle(|candidate| candidate.satisfies(key)) {
            return Ok(target);
        }
        self.release_idle_ordinary_target(key);
        self.create_generation(
            key,
            || TargetImage::with_attachment(runtime, extent, attachment).map(Some),
            TargetImage::allocation_bytes,
        )?
        .ok_or_else(|| crate::error::rejected(extent, "an allocated render attachment"))
    }

    fn release_idle_ordinary_target(&mut self, key: ImageKey) {
        let ordinary = key.without_copy_destination();
        if ordinary == key {
            return;
        }
        // A copy-capable image can serve later ordinary redraws. Free the obsolete
        // generation before allocation so the pair can reuse the same memory block.
        self.generations.retain_mut(|entry| {
            let idle = entry
                .main
                .as_ref()
                .is_none_or(|mark| mark.strong_count() == 0)
                && Arc::get_mut(&mut entry.target).is_some();
            if entry.key != ordinary || !idle {
                return true;
            }
            self.bytes -= entry.bytes;
            false
        });
    }
}

/// A copy destination is also sampled: a saved main image is drawn back by a fetch (#594).
const MAIN_COPY_USAGE: vk::ImageUsageFlags = vk::ImageUsageFlags::from_raw(
    vk::ImageUsageFlags::TRANSFER_DST.as_raw() | vk::ImageUsageFlags::SAMPLED.as_raw(),
);

fn color_attachment(runtime: &VulkanRuntime, subview_copy: bool) -> AttachmentSpec {
    let (format, mut usage, aspect) = attachment_properties(runtime.depth.formats, false, false);
    if subview_copy {
        usage |= MAIN_COPY_USAGE;
    }
    AttachmentSpec {
        format,
        usage,
        aspect,
        samples: vk::SampleCountFlags::TYPE_1,
    }
}

fn depth_attachment(runtime: &VulkanRuntime, stencil: bool, copy: bool) -> AttachmentSpec {
    let (format, mut usage, aspect) = attachment_properties(runtime.depth.formats, true, stencil);
    if copy {
        usage |= MAIN_COPY_USAGE;
    }
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

//! What a Vulkan device did for its views: the resources it created, the bytes it moved and
//! the CPU time of each phase of a redraw. Phase 0 of docs/features/vulkan-occt-parity/PLAN.md:
//! the parity harness and the retained-geometry tests read these counters, so a change that
//! keeps geometry on the device can prove that a static orbit uploads nothing.
use crate::frame_work_ledger::{FrameWorkLedger, WorkPhase};
use crate::{Graphic3dRenderDriver, Graphic3dRenderView};
use martensite_tkernel::osd::MonotonicInstant;
use std::sync::Arc;
use std::time::Duration;

/// Work of a Vulkan device, as totals or over one redraw; zero for injected backends.
/// Example: `render_view.last_frame_work().uploaded_bytes`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RenderDeviceWork {
    /// Redraws of the device's views.
    pub redraws: u64,
    /// Device buffers created (`vkCreateBuffer`).
    pub buffers_created: u64,
    /// Device-local vertex and index buffers created for raster draws.
    pub geometry_buffers_created: u64,
    /// Device images created and bound to memory.
    pub images_created: u64,
    /// Device memory objects allocated (`vkAllocateMemory`): shared blocks and dedicated ones.
    pub memory_allocations: u64,
    /// Graphics pipelines created.
    pub pipelines_created: u64,
    /// Descriptor sets allocated.
    pub descriptor_sets_allocated: u64,
    /// Queue submissions.
    pub submissions: u64,
    /// Bytes the host wrote into device buffers: geometry, uniforms and staging copies.
    pub uploaded_bytes: u64,
    /// Encoded vertex and index bytes queued for a raster geometry transfer.
    pub geometry_uploaded_bytes: u64,
    /// Bytes the host read back from device buffers.
    pub read_back_bytes: u64,
    /// CPU time per redraw phase.
    pub times: RenderWorkTimes,
}

/// CPU time of the phases of redraws. The phases exclude each other: a submission made while
/// frame resources are prepared counts as submission, not as upload. Time in no phase (frame
/// statistics, presentation set-up, resource release) is in `redraw` only.
/// Example: `work.times.scene + work.times.upload <= work.times.redraw`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RenderWorkTimes {
    /// Whole redraws, from the start of the scene walk to the returned frame.
    pub redraw: Duration,
    /// Walking the layers into the frame's draws. Reading the `Graphic3d` arrays and the CPU
    /// lowering are one pass today, so this phase holds both.
    pub scene: Duration,
    /// Creating and filling the frame's buffers, images and descriptor sets.
    pub upload: Duration,
    /// Recording command buffers.
    pub recording: Duration,
    /// Ending, submitting and presenting command buffers.
    pub submission: Duration,
    /// Waiting for fences and for swapchain images.
    pub waiting: Duration,
    /// Reading frames back and transforming the host copy.
    pub readback: Duration,
}

impl RenderDeviceWork {
    /// The work done after `earlier`; `driver.device_work().since(&before).submissions`.
    pub fn since(&self, earlier: &Self) -> Self {
        Self {
            redraws: self.redraws.saturating_sub(earlier.redraws),
            buffers_created: self.buffers_created.saturating_sub(earlier.buffers_created),
            geometry_buffers_created: self
                .geometry_buffers_created
                .saturating_sub(earlier.geometry_buffers_created),
            images_created: self.images_created.saturating_sub(earlier.images_created),
            memory_allocations: self
                .memory_allocations
                .saturating_sub(earlier.memory_allocations),
            pipelines_created: self
                .pipelines_created
                .saturating_sub(earlier.pipelines_created),
            descriptor_sets_allocated: (self.descriptor_sets_allocated)
                .saturating_sub(earlier.descriptor_sets_allocated),
            submissions: self.submissions.saturating_sub(earlier.submissions),
            uploaded_bytes: self.uploaded_bytes.saturating_sub(earlier.uploaded_bytes),
            geometry_uploaded_bytes: self
                .geometry_uploaded_bytes
                .saturating_sub(earlier.geometry_uploaded_bytes),
            read_back_bytes: self.read_back_bytes.saturating_sub(earlier.read_back_bytes),
            times: self.times.since(&earlier.times),
        }
    }
}

impl RenderWorkTimes {
    /// The time spent after `earlier`; `later.since(&earlier).upload`.
    pub fn since(&self, earlier: &Self) -> Self {
        Self {
            redraw: self.redraw.saturating_sub(earlier.redraw),
            scene: self.scene.saturating_sub(earlier.scene),
            upload: self.upload.saturating_sub(earlier.upload),
            recording: self.recording.saturating_sub(earlier.recording),
            submission: self.submission.saturating_sub(earlier.submission),
            waiting: self.waiting.saturating_sub(earlier.waiting),
            readback: self.readback.saturating_sub(earlier.readback),
        }
    }
}

impl Graphic3dRenderDriver {
    /// Everything the device counted since it was created; zero for injected backends.
    /// Example: `driver.device_work().since(&before).uploaded_bytes`.
    pub fn device_work(&self) -> RenderDeviceWork {
        let ledger = self.device.read().work_ledger();
        ledger.map_or_else(RenderDeviceWork::default, |ledger| ledger.totals())
    }
}

impl Graphic3dRenderView {
    /// The device work of this view's last redraw, failed or not.
    /// Example: `render_view.last_frame_work().buffers_created`.
    ///
    /// The counts are device-wide differences over the redraw, so a redraw that another thread
    /// runs on another view of the same driver at the same time adds to them.
    pub fn last_frame_work(&self) -> RenderDeviceWork {
        *self.retained.frame_work.read()
    }
    /// Brackets one redraw with the frame statistics and, inside them, the device work.
    /// Example: `self.with_measured_frame(|work| work.phase(WorkPhase::Scene, walk))`.
    pub fn with_measured_frame<T>(&self, redraw: impl FnOnce(&FrameWorkScope) -> T) -> T {
        self.with_frame_stats(|| self.with_frame_work(redraw))
    }
    /// Runs one redraw with the device work counted and keeps its share.
    fn with_frame_work<T>(&self, redraw: impl FnOnce(&FrameWorkScope) -> T) -> T {
        let scope = FrameWorkScope::open(self.device.read().work_ledger());
        let outcome = redraw(&scope);
        *self.retained.frame_work.write() = scope.close();
        outcome
    }
}

/// The device ledger over one redraw; `scope.phase(WorkPhase::Scene, || walk())`.
pub struct FrameWorkScope {
    ledger: Option<Arc<FrameWorkLedger>>,
    before: RenderDeviceWork,
    start: MonotonicInstant,
}

impl FrameWorkScope {
    fn open(ledger: Option<Arc<FrameWorkLedger>>) -> Self {
        let before = ledger.as_ref().map(|ledger| ledger.totals());
        Self {
            ledger,
            before: before.unwrap_or_default(),
            start: MonotonicInstant::now(),
        }
    }
    /// Runs `step` with its time charged to `phase`; `scope.phase(WorkPhase::Readback, step)`.
    pub fn phase<T>(&self, phase: WorkPhase, step: impl FnOnce() -> T) -> T {
        let _guard = self.ledger.as_ref().map(|ledger| ledger.enter(phase));
        step()
    }
    fn close(self) -> RenderDeviceWork {
        let Some(ledger) = self.ledger else {
            return RenderDeviceWork::default();
        };
        ledger.finish_redraw(self.start.elapsed());
        ledger.totals().since(&self.before)
    }
}

#[cfg(test)]
#[path = "frame_work_tests.rs"]
mod tests;

//! Device work counters and exclusive CPU phase times of one Vulkan device (phase 0 of
//! docs/features/vulkan-occt-parity/PLAN.md). A view reads the totals before and after a
//! redraw; the difference is `Graphic3dRenderView::last_frame_work`.
//!
//! The counts are atomics because every resource owner reaches the ledger through the shared
//! runtime. The phase clock is exclusive: entering a phase pauses the open one, so a transfer
//! submitted while frame resources are prepared counts as submission and waiting, not twice.
use crate::{RenderDeviceWork, RenderWorkTimes};
use ash::vk;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Mutex, PoisonError};
use std::time::{Duration, Instant};

/// What the ledger counts; `ledger.add(DeviceCount::UploadedBytes, 64)`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum DeviceCount {
    Buffers,
    Images,
    MemoryAllocations,
    Pipelines,
    DescriptorSets,
    Submissions,
    UploadedBytes,
    ReadBackBytes,
}
const COUNTS: usize = 8;

/// The CPU phases of a redraw; `ledger.enter(WorkPhase::Upload)`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum WorkPhase {
    Scene,
    Upload,
    Recording,
    Submission,
    Waiting,
    Readback,
}
const PHASES: usize = 6;

/// The counters and phase clock of one device; owned by its runtime.
pub(crate) struct FrameWorkLedger {
    origin: Instant,
    counts: [AtomicU64; COUNTS],
    redraws: AtomicU64,
    redraw_nanos: AtomicU64,
    phases: Mutex<PhaseAccounts>,
}

impl Default for FrameWorkLedger {
    fn default() -> Self {
        Self {
            origin: Instant::now(),
            counts: Default::default(),
            redraws: AtomicU64::new(0),
            redraw_nanos: AtomicU64::new(0),
            phases: Mutex::default(),
        }
    }
}

impl FrameWorkLedger {
    /// Count `amount` more of `count`; `runtime.work.add(DeviceCount::Buffers, 1)`.
    pub fn add(&self, count: DeviceCount, amount: u64) {
        self.counts[count as usize].fetch_add(amount, Ordering::Relaxed);
    }
    /// Count the sets of one successful allocation; `runtime.work.add_sets(&info)`.
    pub fn add_sets(&self, info: &vk::DescriptorSetAllocateInfo<'_>) {
        self.add(
            DeviceCount::DescriptorSets,
            u64::from(info.descriptor_set_count),
        );
    }
    /// Charge the time until the guard drops to `phase`; `let _upload = ledger.enter(Upload)`.
    pub fn enter(&self, phase: WorkPhase) -> PhaseGuard<'_> {
        self.accounts().enter(phase, self.origin.elapsed());
        PhaseGuard { ledger: self }
    }
    /// Count one finished redraw of `elapsed` wall time; `ledger.finish_redraw(elapsed)`.
    pub fn finish_redraw(&self, elapsed: Duration) {
        self.redraws.fetch_add(1, Ordering::Relaxed);
        let nanos = u64::try_from(elapsed.as_nanos()).unwrap_or(u64::MAX);
        self.redraw_nanos.fetch_add(nanos, Ordering::Relaxed);
    }
    /// Everything counted since the device was created; `ledger.totals().submissions`.
    pub fn totals(&self) -> RenderDeviceWork {
        let count = |kind: DeviceCount| self.counts[kind as usize].load(Ordering::Relaxed);
        RenderDeviceWork {
            redraws: self.redraws.load(Ordering::Relaxed),
            buffers_created: count(DeviceCount::Buffers),
            images_created: count(DeviceCount::Images),
            memory_allocations: count(DeviceCount::MemoryAllocations),
            pipelines_created: count(DeviceCount::Pipelines),
            descriptor_sets_allocated: count(DeviceCount::DescriptorSets),
            submissions: count(DeviceCount::Submissions),
            uploaded_bytes: count(DeviceCount::UploadedBytes),
            read_back_bytes: count(DeviceCount::ReadBackBytes),
            times: self.times(),
        }
    }
    fn times(&self) -> RenderWorkTimes {
        let redraw = Duration::from_nanos(self.redraw_nanos.load(Ordering::Relaxed));
        self.accounts().times(redraw)
    }
    /// A poisoned clock still holds valid sums: a panic inside a phase loses no earlier time.
    fn accounts(&self) -> std::sync::MutexGuard<'_, PhaseAccounts> {
        self.phases.lock().unwrap_or_else(PoisonError::into_inner)
    }
}

impl super::VulkanRenderer {
    /// The work counters of this device; `renderer.work_ledger().totals().submissions`.
    pub(crate) fn work_ledger(&self) -> std::sync::Arc<FrameWorkLedger> {
        self.runtime.work.clone()
    }
}

/// Closes the phase it was entered for; see `FrameWorkLedger::enter`.
pub(crate) struct PhaseGuard<'a> {
    ledger: &'a FrameWorkLedger,
}

impl Drop for PhaseGuard<'_> {
    fn drop(&mut self) {
        let now = self.ledger.origin.elapsed();
        self.ledger.accounts().leave(now);
    }
}

/// Exclusive phase sums over a stack of open phases, at explicit instants measured from the
/// ledger's origin; `accounts.enter(WorkPhase::Scene, now)`.
#[derive(Debug, Default)]
pub(crate) struct PhaseAccounts {
    sums: [Duration; PHASES],
    open: Vec<(WorkPhase, Duration)>,
}

impl PhaseAccounts {
    /// Pause the open phase at `now` and start `phase`.
    pub fn enter(&mut self, phase: WorkPhase, now: Duration) {
        self.charge_open(now);
        self.open.push((phase, now));
    }
    /// End the innermost phase at `now` and resume the one it paused.
    pub fn leave(&mut self, now: Duration) {
        self.charge_open(now);
        self.open.pop();
        if let Some((_, since)) = self.open.last_mut() {
            *since = now;
        }
    }
    /// Charge the open phase up to `now` and restart its interval there.
    fn charge_open(&mut self, now: Duration) {
        let Some((phase, since)) = self.open.last_mut() else {
            return;
        };
        self.sums[*phase as usize] += now.saturating_sub(*since);
        *since = now;
    }
    /// The phase sums with `redraw` as the whole-redraw time; `accounts.times(redraw)`.
    pub fn times(&self, redraw: Duration) -> RenderWorkTimes {
        let sum = |phase: WorkPhase| self.sums[phase as usize];
        RenderWorkTimes {
            redraw,
            scene: sum(WorkPhase::Scene),
            upload: sum(WorkPhase::Upload),
            recording: sum(WorkPhase::Recording),
            submission: sum(WorkPhase::Submission),
            waiting: sum(WorkPhase::Waiting),
            readback: sum(WorkPhase::Readback),
        }
    }
}

#[cfg(test)]
#[path = "frame_work_ledger_tests.rs"]
mod tests;

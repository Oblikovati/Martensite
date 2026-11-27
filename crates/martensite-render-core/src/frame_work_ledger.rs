//! Backend-agnostic device work counters and exclusive CPU phase times (#594, #633).
//!
//! Uses `martensite_tkernel::osd::MonotonicInstant` so frame-work accounting runs on both
//! native platforms and browser WebAssembly (`wasm32-unknown-unknown`).
use crate::{RenderDeviceWork, RenderWorkTimes};
use martensite_tkernel::osd::MonotonicInstant;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Mutex, PoisonError};
use std::time::Duration;

/// Resource and transfer counters recorded by a GPU backend.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeviceCount {
    /// Device buffers created.
    Buffers,
    /// Device-local vertex and index buffers created for raster draws.
    GeometryBuffers,
    /// Device images created and bound to memory.
    Images,
    /// Device memory allocations.
    MemoryAllocations,
    /// Graphics or compute pipelines created.
    Pipelines,
    /// Descriptor sets or bind groups allocated.
    DescriptorSets,
    /// Queue submissions.
    Submissions,
    /// Host bytes uploaded into device buffers.
    UploadedBytes,
    /// Encoded vertex and index bytes queued for a raster geometry transfer.
    GeometryUploadedBytes,
    /// Bytes read back from device buffers.
    ReadBackBytes,
}
const COUNTS: usize = 10;

/// Exclusive CPU phases of a single redraw.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorkPhase {
    /// Walking layers into frame draws.
    Scene,
    /// Creating and filling buffers, images, and bind groups.
    Upload,
    /// Recording command buffers.
    Recording,
    /// Ending, submitting, and presenting command buffers.
    Submission,
    /// Waiting for fences and swapchain images.
    Waiting,
    /// Reading frames back and transforming host copies.
    Readback,
}
const PHASES: usize = 6;

/// Atomic resource counters and exclusive CPU phase clock shared by one GPU device.
pub struct FrameWorkLedger {
    origin: MonotonicInstant,
    counts: [AtomicU64; COUNTS],
    redraws: AtomicU64,
    redraw_nanos: AtomicU64,
    phases: Mutex<PhaseAccounts>,
}

impl Default for FrameWorkLedger {
    fn default() -> Self {
        Self {
            origin: MonotonicInstant::now(),
            counts: Default::default(),
            redraws: AtomicU64::new(0),
            redraw_nanos: AtomicU64::new(0),
            phases: Mutex::default(),
        }
    }
}

impl FrameWorkLedger {
    /// Count `amount` more of `count`; `ledger.add(DeviceCount::Buffers, 1)`.
    pub fn add(&self, count: DeviceCount, amount: u64) {
        self.counts[count as usize].fetch_add(amount, Ordering::Relaxed);
    }
    /// Count descriptor-set or bind-group allocations; `ledger.add_descriptor_sets(count)`.
    pub fn add_descriptor_sets(&self, count: u64) {
        self.add(DeviceCount::DescriptorSets, count);
    }
    /// Charge elapsed time until the guard drops to `phase`; `let _g = ledger.enter(WorkPhase::Upload)`.
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
    /// Return everything counted since creation; `ledger.totals().submissions`.
    pub fn totals(&self) -> RenderDeviceWork {
        let count = |kind: DeviceCount| self.counts[kind as usize].load(Ordering::Relaxed);
        RenderDeviceWork {
            redraws: self.redraws.load(Ordering::Relaxed),
            buffers_created: count(DeviceCount::Buffers),
            geometry_buffers_created: count(DeviceCount::GeometryBuffers),
            images_created: count(DeviceCount::Images),
            memory_allocations: count(DeviceCount::MemoryAllocations),
            pipelines_created: count(DeviceCount::Pipelines),
            descriptor_sets_allocated: count(DeviceCount::DescriptorSets),
            submissions: count(DeviceCount::Submissions),
            uploaded_bytes: count(DeviceCount::UploadedBytes),
            geometry_uploaded_bytes: count(DeviceCount::GeometryUploadedBytes),
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
    /// Number of currently open nested phases (test helper).
    #[doc(hidden)]
    pub fn open_phase_count(&self) -> usize {
        self.accounts().open.len()
    }
}

/// Closes the phase entered by [`FrameWorkLedger::enter`].
pub struct PhaseGuard<'a> {
    ledger: &'a FrameWorkLedger,
}

impl Drop for PhaseGuard<'_> {
    fn drop(&mut self) {
        let now = self.ledger.origin.elapsed();
        self.ledger.accounts().leave(now);
    }
}

/// Exclusive phase sums over a stack of open phases at explicit instants.
#[derive(Debug, Default)]
pub struct PhaseAccounts {
    sums: [Duration; PHASES],
    open: Vec<(WorkPhase, Duration)>,
}

impl PhaseAccounts {
    /// Pause the open phase at `now` and start `phase`.
    pub fn enter(&mut self, phase: WorkPhase, now: Duration) {
        self.charge_open(now);
        self.open.push((phase, now));
    }
    /// End the innermost phase at `now` and resume the paused parent phase.
    pub fn leave(&mut self, now: Duration) {
        self.charge_open(now);
        self.open.pop();
        if let Some((_, since)) = self.open.last_mut() {
            *since = now;
        }
    }
    fn charge_open(&mut self, now: Duration) {
        let Some((phase, since)) = self.open.last_mut() else {
            return;
        };
        self.sums[*phase as usize] += now.saturating_sub(*since);
        *since = now;
    }
    /// Return accumulated phase times with `redraw` as the total wall duration.
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

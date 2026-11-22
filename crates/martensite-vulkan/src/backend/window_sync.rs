//! Presentation fences protect semaphore reuse and swapchain destruction (VK_EXT_swapchain_maintenance1).
use super::runtime::{VulkanRuntime, vk_error};
use super::window_fence_retirement::WindowFenceOwner;
use crate::RenderError;
use ash::vk;
use std::sync::Arc;
pub(super) struct WindowSync {
    runtime: Arc<VulkanRuntime>,
    pub acquired: vk::Semaphore,
    pub rendered: vk::Semaphore,
    pub present_fence: vk::Fence,
    pub pending: bool,
    /// The acquired semaphore has a pending signal that no queued work waits on yet.
    pub acquired_unconsumed: bool,
    device_retired: bool,
}
impl WindowSync {
    pub fn new(runtime: &Arc<VulkanRuntime>) -> Result<Self, RenderError> {
        let mut sync = Self {
            runtime: runtime.clone(),
            acquired: vk::Semaphore::null(),
            rendered: vk::Semaphore::null(),
            present_fence: vk::Fence::null(),
            pending: false,
            acquired_unconsumed: false,
            device_retired: false,
        };
        sync.initialize_handles()?;
        Ok(sync)
    }
    fn initialize_handles(&mut self) -> Result<(), RenderError> {
        let device = self.runtime.device();
        self.acquired =
            // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
            unsafe { device.create_semaphore(&vk::SemaphoreCreateInfo::default(), None) }
                .map_err(|e| vk_error("create acquire semaphore", e))?;
        self.rendered =
            // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
            unsafe { device.create_semaphore(&vk::SemaphoreCreateInfo::default(), None) }
                .map_err(|e| vk_error("create present semaphore", e))?;
        // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
        self.present_fence = unsafe { device.create_fence(&vk::FenceCreateInfo::default(), None) }
            .map_err(|e| vk_error("create presentation fence", e))?;
        Ok(())
    }

    pub fn wait_present(&mut self) -> Result<(), RenderError> {
        super::window_sync_retirement::wait_window_fence(
            &self.runtime,
            self.present_fence,
            &mut self.pending,
            WindowFenceOwner::Presentation,
        )
    }
    /// Try the presentation fence once and drain an unconsumed acquisition;
    /// `sync.try_retire()` preserves each unknown owner.
    pub fn try_retire(&mut self) -> super::retirement_attempt::RetirementAttempt {
        let presented = super::window_sync_retirement::attempt_window_fence(
            &self.runtime,
            self.present_fence,
            &mut self.pending,
            WindowFenceOwner::Presentation,
        );
        // Only an error between acquisition and submission leaves an unconsumed semaphore,
        // and every caller on that path already waits for the device; drain it here.
        if self.acquired_unconsumed {
            return presented.join(super::retirement_attempt::RetirementAttempt::from(
                self.drain_acquisition(),
            ));
        }
        presented
    }
    /// Remember that the chain's graphics submissions have completed.
    fn qualify_queue_work(&mut self) {
        // Fence ownership remains independently pending until its own actual signal is observed.
        self.device_retired = true;
    }
    /// Retire both exact pending fence owners; `sync.retire_blocking()` never infers presentation from idle.
    pub fn retire_blocking(&mut self) {
        let drained = self.drain_acquisition();
        let _ = self.runtime.lifecycle.observe(drained);
        let outcome = super::window_sync_retirement::retire_window_fence(
            &self.runtime,
            self.present_fence,
            &mut self.pending,
            WindowFenceOwner::Presentation,
        );
        let _ = self.runtime.lifecycle.observe(outcome);
    }
    /// Consume an acquired semaphore that no queued work waits on, so its pending signal from
    /// the presentation engine completes before the semaphore is reused or destroyed. Only an
    /// error between acquisition and submission leaves one (#594); `sync.drain_acquisition()`.
    fn drain_acquisition(&mut self) -> Result<(), RenderError> {
        if !self.acquired_unconsumed {
            return Ok(());
        }
        self.acquired_unconsumed = false;
        super::window_acquisition_drain::drain(&self.runtime, self.acquired)
    }
}

impl Drop for WindowSync {
    fn drop(&mut self) {
        self.retire_blocking();
        if !self.device_retired {
            self.runtime.retire_device_blocking();
        }
        // SAFETY: the handle belongs to this device and its owner releases it only after the GPU work that used it has completed.
        unsafe {
            if self.present_fence != vk::Fence::null() {
                self.runtime
                    .device()
                    .destroy_fence(self.present_fence, None);
            }
            for semaphore in [self.acquired, self.rendered] {
                if semaphore != vk::Semaphore::null() {
                    self.runtime.device().destroy_semaphore(semaphore, None);
                }
            }
        }
    }
}

/// One semaphore and fence set per frame in flight, used in turn, so a frame waits for the
/// presentation of the frame two before it instead of the previous one (#594).
pub(super) struct WindowSyncRing {
    slots: Vec<WindowSync>,
    current: usize,
}

impl WindowSyncRing {
    /// Create `count` sets; `WindowSyncRing::new(&runtime, FRAMES_IN_FLIGHT)?`.
    pub fn new(runtime: &Arc<VulkanRuntime>, count: usize) -> Result<Self, RenderError> {
        let slots = (0..count.max(1))
            .map(|_| WindowSync::new(runtime))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self { slots, current: 0 })
    }
    /// The set of the frame being prepared; `ring.current().acquired`.
    pub fn current(&self) -> &WindowSync {
        &self.slots[self.current]
    }
    /// The set of the frame being prepared, for its fence state; `ring.current_mut().wait_present()?`.
    pub fn current_mut(&mut self) -> &mut WindowSync {
        &mut self.slots[self.current]
    }
    /// Move to the next set once a frame was handed to presentation; `ring.advance()`.
    pub fn advance(&mut self) {
        self.current = (self.current + 1) % self.slots.len();
    }
    /// Try every set's fences once; `ring.try_retire()` keeps each unknown owner.
    pub fn try_retire(&mut self) -> super::retirement_attempt::RetirementAttempt {
        let mut attempt = super::retirement_attempt::RetirementAttempt::from(Ok(()));
        for sync in &mut self.slots {
            attempt = attempt.join(sync.try_retire());
        }
        attempt
    }
    /// Remember the chain's graphics completion proof for every set; `ring.qualify_queue_work()`.
    pub fn qualify_queue_work(&mut self) {
        self.slots
            .iter_mut()
            .for_each(WindowSync::qualify_queue_work);
    }
    /// Retire every set's pending fences; `ring.retire_blocking()`.
    pub fn retire_blocking(&mut self) {
        self.slots.iter_mut().for_each(WindowSync::retire_blocking);
    }
}

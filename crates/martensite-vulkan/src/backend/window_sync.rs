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
    pub acquire_fence: vk::Fence,
    pub present_fence: vk::Fence,
    pub pending: bool,
    pub acquire_pending: bool,
    device_retired: bool,
}
impl WindowSync {
    pub fn new(runtime: &Arc<VulkanRuntime>) -> Result<Self, RenderError> {
        let mut sync = Self {
            runtime: runtime.clone(),
            acquired: vk::Semaphore::null(),
            rendered: vk::Semaphore::null(),
            acquire_fence: vk::Fence::null(),
            present_fence: vk::Fence::null(),
            pending: false,
            acquire_pending: false,
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
        self.acquire_fence = unsafe { device.create_fence(&vk::FenceCreateInfo::default(), None) }
            .map_err(|e| vk_error("create acquisition fence", e))?;
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
    pub fn wait_acquire(&mut self) -> Result<(), RenderError> {
        super::window_sync_retirement::wait_window_fence(
            &self.runtime,
            self.acquire_fence,
            &mut self.acquire_pending,
            WindowFenceOwner::Acquisition,
        )
    }
    /// Try both required completion signals once; `sync.try_retire()` preserves each unknown owner.
    pub fn try_retire(&mut self) -> super::retirement_attempt::RetirementAttempt {
        let presented = super::window_sync_retirement::attempt_window_fence(
            &self.runtime,
            self.present_fence,
            &mut self.pending,
            WindowFenceOwner::Presentation,
        );
        let acquired = super::window_sync_retirement::attempt_window_fence(
            &self.runtime,
            self.acquire_fence,
            &mut self.acquire_pending,
            WindowFenceOwner::Acquisition,
        );
        presented.join(acquired)
    }
    /// Remember the owning close's device-idle proof; `sync.qualify_device_idle()` does not qualify fences.
    pub fn qualify_device_idle(&mut self) {
        // Fence ownership remains independently pending until its own actual signal is observed.
        self.device_retired = true;
    }
    /// Retire both exact pending fence owners; `sync.retire_blocking()` never infers presentation from idle.
    pub fn retire_blocking(&mut self) {
        let outcome = super::window_sync_retirement::retire_window_fence(
            &self.runtime,
            self.present_fence,
            &mut self.pending,
            WindowFenceOwner::Presentation,
        );
        let _ = self.runtime.lifecycle.observe(outcome);
        let outcome = super::window_sync_retirement::retire_window_fence(
            &self.runtime,
            self.acquire_fence,
            &mut self.acquire_pending,
            WindowFenceOwner::Acquisition,
        );
        let _ = self.runtime.lifecycle.observe(outcome);
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
            for fence in [self.acquire_fence, self.present_fence] {
                if fence != vk::Fence::null() {
                    self.runtime.device().destroy_fence(fence, None);
                }
            }
            for semaphore in [self.acquired, self.rendered] {
                if semaphore != vk::Semaphore::null() {
                    self.runtime.device().destroy_semaphore(semaphore, None);
                }
            }
        }
    }
}

//! Each submission owns its command pool and fence and finishes before frame resources drop.
use super::runtime::{VulkanRuntime, vk_error};
use super::submission_protocol::SubmissionProtocol;
use crate::RenderError;
use ash::vk;
use std::sync::Arc;
pub(super) struct CommandSubmission {
    pub(super) runtime: Arc<VulkanRuntime>,
    pool: vk::CommandPool,
    protocol: SubmissionProtocol,
    pub(super) fence: vk::Fence,
    pub command: vk::CommandBuffer,
}
impl CommandSubmission {
    pub fn new(runtime: &Arc<VulkanRuntime>) -> Result<Self, RenderError> {
        runtime.lifecycle.admit("create command submission")?;
        runtime.lifecycle.observe(Self::recording(runtime))
    }
    fn recording(runtime: &Arc<VulkanRuntime>) -> Result<Self, RenderError> {
        let info = vk::CommandPoolCreateInfo::default()
            .queue_family_index(runtime.queue_family)
            .flags(vk::CommandPoolCreateFlags::TRANSIENT);
        // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
        let pool = unsafe { runtime.device().create_command_pool(&info, None) }
            .map_err(|e| vk_error("create command pool", e))?;
        let mut submission = Self {
            runtime: runtime.clone(),
            pool,
            protocol: Default::default(),
            fence: vk::Fence::null(),
            command: vk::CommandBuffer::null(),
        };
        submission.allocate()?;
        submission.begin()?;
        #[cfg(feature = "command-state-diagnostics")]
        super::replay_raster_commands::submission_begin(
            runtime.device().handle(),
            submission.command,
            submission.fence,
            pool,
        );
        Ok(submission)
    }
    fn begin(&self) -> Result<(), RenderError> {
        let info = vk::CommandBufferBeginInfo::default()
            .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);
        // SAFETY: the queue, fences and command buffers belong to this device, and the runtime serializes access to them.
        unsafe {
            self.runtime
                .device()
                .begin_command_buffer(self.command, &info)
        }
        .map_err(|e| vk_error("begin command buffer", e))
    }
    fn allocate(&mut self) -> Result<(), RenderError> {
        let info = vk::CommandBufferAllocateInfo::default()
            .command_pool(self.pool)
            .level(vk::CommandBufferLevel::PRIMARY)
            .command_buffer_count(1);
        // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
        self.command = unsafe { self.runtime.device().allocate_command_buffers(&info) }
            .map_err(|e| vk_error("allocate command buffer", e))?[0];
        // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
        self.fence = unsafe {
            self.runtime
                .device()
                .create_fence(&vk::FenceCreateInfo::default(), None)
        }
        .map_err(|e| vk_error("create submit fence", e))?;
        Ok(())
    }
    /// Preserve the exact incomplete phase; `commands.retirement_incomplete()` never guesses from an error string.
    pub fn retirement_incomplete(&self) -> bool {
        let phase = self.protocol.phase();
        phase == super::submission_protocol::SubmissionPhase::RetirementIncomplete
    }
    /// Record the owning actual idle proof; `commands.qualify_device_idle()` leaves unrelated presentation ownership intact.
    pub fn qualify_device_idle(&self) {
        // Only the renderer retirement store calls this after a real successful/lost device wait.
        self.protocol.qualify_device_idle();
    }
    pub fn finish(&self) -> Result<(), RenderError> {
        let semaphores: &[vk::SemaphoreSubmitInfo<'_>] = &[];
        self.finish_synced(semaphores, semaphores)
    }
    pub fn finish_synced(
        &self,
        waits: &[vk::SemaphoreSubmitInfo<'_>],
        signals: &[vk::SemaphoreSubmitInfo<'_>],
    ) -> Result<(), RenderError> {
        let mut calls = super::submission_calls::DeviceSubmissionCalls::new(self, waits, signals);
        self.finish_dispatched(&mut calls)
    }
    /// Use one actual or named boundary dispatch; `commands.finish_dispatched(calls)` shares production admission/retirement.
    pub(super) fn finish_dispatched(
        &self,
        calls: &mut impl super::submission_protocol::SubmissionDispatch,
    ) -> Result<(), RenderError> {
        self.protocol
            .finish_admitted(&self.runtime.lifecycle, calls)
    }
}
impl Drop for CommandSubmission {
    fn drop(&mut self) {
        // Keep caller-owned frame/upload resources alive until this Drop returns.
        let mut calls = super::submission_calls::DeviceSubmissionCalls::new(self, &[], &[]);
        let outcome = self.protocol.retire_blocking(&mut calls);
        let _ = self.runtime.lifecycle.observe(outcome);
        // SAFETY: the handle belongs to this device and its owner releases it only after the GPU work that used it has completed.
        unsafe {
            if self.fence != vk::Fence::null() {
                self.runtime.device().destroy_fence(self.fence, None);
            }
            self.runtime.device().destroy_command_pool(self.pool, None);
        }
    }
}

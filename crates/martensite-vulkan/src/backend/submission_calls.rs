//! Thin device dispatch for the same one-shot submission protocol used by fault controls.
use super::device_retirement::{CompletionDispatch, DeviceIdleCalls, retirement_retry_pause};
use super::{
    runtime::vk_error, submission::CommandSubmission, submission_protocol::SubmissionDispatch,
};
use crate::RenderError;
use ash::vk;
pub(super) struct DeviceSubmissionCalls<'a, 'b> {
    submission: &'a CommandSubmission,
    waits: &'a [vk::SemaphoreSubmitInfo<'b>],
    signals: &'a [vk::SemaphoreSubmitInfo<'b>],
}
impl<'a, 'b> DeviceSubmissionCalls<'a, 'b> {
    /// Borrow the actual submission inputs; `DeviceSubmissionCalls::new(commands, waits, signals)`.
    pub fn new(
        submission: &'a CommandSubmission,
        waits: &'a [vk::SemaphoreSubmitInfo<'b>],
        signals: &'a [vk::SemaphoreSubmitInfo<'b>],
    ) -> Self {
        // Borrowing these arrays cannot outlive the lexical queue operation.
        Self {
            submission,
            waits,
            signals,
        }
    }
}
impl SubmissionDispatch for DeviceSubmissionCalls<'_, '_> {
    fn end_commands(&mut self) -> Result<(), RenderError> {
        let submission = self.submission;
        // SAFETY: the queue, fences and command buffers belong to this device, and the runtime serializes access to them.
        unsafe {
            submission
                .runtime
                .device()
                .end_command_buffer(submission.command)
        }
        .map_err(|error| vk_error("end command buffer", error))
    }
    fn submit_commands(&mut self) -> Result<(), RenderError> {
        let submission = self.submission;
        let commands = [vk::CommandBufferSubmitInfo::default().command_buffer(submission.command)];
        let submits = [vk::SubmitInfo2::default()
            .command_buffer_infos(&commands)
            .wait_semaphore_infos(self.waits)
            .signal_semaphore_infos(self.signals)];
        // SAFETY: the queue, fences and command buffers belong to this device, and the runtime serializes access to them.
        unsafe {
            submission.runtime.device().queue_submit2(
                submission.runtime.queue,
                &submits,
                submission.fence,
            )
        }
        .map_err(|error| vk_error("submit frame", error))?;
        #[cfg(feature = "command-state-diagnostics")]
        super::replay_raster_commands::submitted(
            submission.command,
            submission.fence,
            submission.runtime.queue,
            self.waits.len(),
            self.signals.len(),
        );
        Ok(())
    }
    fn wait_fence(&mut self) -> Result<(), RenderError> {
        let submission = self.submission;
        // SAFETY: the queue, fences and command buffers belong to this device, and the runtime serializes access to them.
        unsafe {
            submission
                .runtime
                .device()
                .wait_for_fences(&[submission.fence], true, u64::MAX)
        }
        .map_err(|error| vk_error("wait for readback", error))?;
        #[cfg(feature = "command-state-diagnostics")]
        super::replay_command_diagnostic::completed(submission.command, submission.fence);
        Ok(())
    }
}
impl CompletionDispatch for DeviceSubmissionCalls<'_, '_> {
    fn wait_completion(&mut self) -> Result<(), RenderError> {
        let runtime = &self.submission.runtime;
        DeviceIdleCalls::new(runtime.device()).wait_completion()
    }
    fn retry_pause(&mut self) {
        // Retry spacing is a teardown policy, never an alternative rendering path.
        retirement_retry_pause();
    }
}

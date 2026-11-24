//! Thin device dispatch for the same one-shot submission protocol used by fault controls.
use super::device_retirement::{CompletionDispatch, DeviceIdleCalls, retirement_retry_pause};
use super::frame_work_ledger::{DeviceCount, PhaseGuard, WorkPhase};
use super::queue_order::QueueTurn;
use super::{
    runtime::vk_error, submission::CommandSubmission, submission_protocol::SubmissionDispatch,
};
use crate::RenderError;
use ash::vk;
pub(super) struct DeviceSubmissionCalls<'a, 'b> {
    submission: &'a CommandSubmission,
    waits: &'a [vk::SemaphoreSubmitInfo<'b>],
    signals: &'a [vk::SemaphoreSubmitInfo<'b>],
    /// The queue turn of the presentation thread, which submits for the host: its calls take
    /// no turn of their own and leave the host's phase clock and counts alone (#594).
    held: Option<&'a QueueTurn>,
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
            held: None,
        }
    }
    /// Submit inside the presentation thread's turn;
    /// `DeviceSubmissionCalls::new(commands, waits, signals).within(&turn)`.
    pub fn within(mut self, turn: &'a QueueTurn) -> Self {
        self.held = Some(turn);
        self
    }
    fn phase(&self, phase: WorkPhase) -> Option<PhaseGuard<'_>> {
        let work = &self.submission.runtime.work;
        self.held.is_none().then(|| work.enter(phase))
    }

    fn queue_native(&self, submits: &[vk::SubmitInfo2<'_>]) -> Result<(), RenderError> {
        let submission = self.submission;
        #[cfg(test)]
        if submission.reject_graphics_allocation.get() {
            return Err(vk_error(
                "submit frame",
                vk::Result::ERROR_OUT_OF_DEVICE_MEMORY,
            ));
        }
        // SAFETY: the caller holds the queue turn. The command buffers, waits, signals and
        // fence are owned by the linked or primary submission until execution retires.
        unsafe {
            submission
                .runtime
                .device()
                .queue_submit2(submission.queue(), submits, submission.fence)
        }
        .map_err(|error| vk_error("submit frame", error))
    }

    fn retire_transfer_after_refusal(&self, primary: RenderError, turn: &QueueTurn) -> RenderError {
        if !primary
            .device_failure()
            .is_some_and(|failure| failure.class() == crate::RenderDeviceFailureClass::OutOfMemory)
        {
            return primary;
        }
        let Some(transfer) = self.submission.geometry_transfer() else {
            return primary;
        };
        match transfer.complete_after_refusal(turn) {
            Ok(()) => primary,
            Err(cleanup) => primary.with_observed_failure(cleanup),
        }
    }
}
impl SubmissionDispatch for DeviceSubmissionCalls<'_, '_> {
    fn end_commands(&mut self) -> Result<(), RenderError> {
        let submission = self.submission;
        let _submission = self.phase(WorkPhase::Submission);
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
        let _submission = self.phase(WorkPhase::Submission);
        let own_turn;
        let turn = match self.held {
            Some(turn) => turn,
            None => {
                own_turn = submission.runtime.queue_order.next_turn();
                &own_turn
            }
        };
        let mut linked_waits = Vec::new();
        let waits = if let Some(transfer) = submission.geometry_transfer() {
            linked_waits.extend_from_slice(self.waits);
            linked_waits.push(transfer.submit(turn)?);
            if self.held.is_none() {
                submission.runtime.work.add(DeviceCount::Submissions, 1);
            }
            &linked_waits
        } else {
            self.waits
        };
        let commands = [vk::CommandBufferSubmitInfo::default().command_buffer(submission.command)];
        let submits = [vk::SubmitInfo2::default()
            .command_buffer_infos(&commands)
            .wait_semaphore_infos(waits)
            .signal_semaphore_infos(self.signals)];
        self.queue_native(&submits)
            .map_err(|error| self.retire_transfer_after_refusal(error, turn))?;
        if self.held.is_none() {
            submission.runtime.work.add(DeviceCount::Submissions, 1);
        }
        #[cfg(feature = "command-state-diagnostics")]
        super::replay_raster_commands::submitted(
            submission.command,
            submission.fence,
            submission.queue(),
            waits.len(),
            self.signals.len(),
        );
        Ok(())
    }
    fn wait_fence(&mut self) -> Result<(), RenderError> {
        let submission = self.submission;
        let _waiting = self.phase(WorkPhase::Waiting);
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
        DeviceIdleCalls::new(runtime.device(), &runtime.queue_order)
            .within(self.held)
            .wait_completion()
    }
    fn retry_pause(&mut self) {
        // Retry spacing is a teardown policy, never an alternative rendering path.
        retirement_retry_pause();
    }
}

//! A fence proves retirement only after actual submission and successful completion.
use super::device_lifecycle::{DeviceLifecycle, contains_device_loss};
use super::device_retirement::{CompletionDispatch, retire_until_stopped};
use crate::{RenderError, error::rejected};
use std::cell::Cell;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum SubmissionPhase {
    Recording,
    NotSubmitted,
    Submitted,
    Retired,
    RetirementIncomplete,
}
pub(super) trait SubmissionDispatch: CompletionDispatch {
    fn end_commands(&mut self) -> Result<(), RenderError>;
    fn submit_commands(&mut self) -> Result<(), RenderError>;
    fn wait_fence(&mut self) -> Result<(), RenderError>;
}
pub(super) struct SubmissionProtocol(Cell<SubmissionPhase>);
impl Default for SubmissionProtocol {
    fn default() -> Self {
        // A newly recording buffer owns no queued execution or signalled fence.
        Self(Cell::new(SubmissionPhase::Recording))
    }
}
impl SubmissionProtocol {
    /// Inspect retirement proof; `protocol.phase()` distinguishes pending from stopped execution.
    pub fn phase(&self) -> SubmissionPhase {
        // Only this operation-local protocol mutates its submission state.
        self.0.get()
    }
    /// Recheck the actual dispatch boundary; `protocol.finish_admitted(lifecycle, calls)?` refuses old recordings.
    pub fn finish_admitted(
        &self,
        lifecycle: &DeviceLifecycle,
        calls: &mut impl SubmissionDispatch,
    ) -> Result<(), RenderError> {
        lifecycle.admit("submit frame")?;
        let outcome = self.finish(calls);
        if self.phase() == SubmissionPhase::RetirementIncomplete {
            lifecycle.block_retirement();
        }
        lifecycle.observe(outcome)
    }
    /// Submit once and qualify completion; `protocol.finish(&mut calls)?` uses actual dispatch.
    pub fn finish(&self, calls: &mut impl SubmissionDispatch) -> Result<(), RenderError> {
        if self.phase() != SubmissionPhase::Recording {
            return Err(rejected(self.phase(), "a recording one-shot submission"));
        }
        self.0.set(SubmissionPhase::NotSubmitted);
        calls.end_commands()?;
        self.submit_and_wait(calls)
    }
    fn submit_and_wait(&self, calls: &mut impl SubmissionDispatch) -> Result<(), RenderError> {
        if let Err(primary) = calls.submit_commands() {
            return self.after_failed_submit(calls, primary);
        }
        self.0.set(SubmissionPhase::Submitted);
        match calls.wait_fence() {
            Ok(()) => {
                self.0.set(SubmissionPhase::Retired);
                Ok(())
            }
            Err(primary) => self.after_failed_wait(calls, primary),
        }
    }
    fn after_failed_submit(
        &self,
        calls: &mut impl SubmissionDispatch,
        primary: RenderError,
    ) -> Result<(), RenderError> {
        let allocation_refusal = primary
            .device_failure()
            .is_some_and(|cause| cause.class() == crate::RenderDeviceFailureClass::OutOfMemory);
        if allocation_refusal {
            return Err(primary);
        }
        // Only the specified allocation errors guarantee an unchanged queue submission.
        self.0.set(SubmissionPhase::RetirementIncomplete);
        self.after_failed_wait(calls, primary)
    }
    fn after_failed_wait(
        &self,
        calls: &mut impl SubmissionDispatch,
        primary: RenderError,
    ) -> Result<(), RenderError> {
        if contains_device_loss(&primary) {
            self.0.set(SubmissionPhase::Retired);
            return Err(primary);
        }
        match self.retire(calls) {
            Ok(()) => Err(primary),
            Err(cleanup) => Err(primary.with_observed_failure(cleanup)),
        }
    }
    /// Record the owning actual device-idle proof; `protocol.qualify_device_idle()` cannot qualify window fences.
    pub fn qualify_device_idle(&self) {
        // The retirement store calls this only after its independently checked device attempt.
        self.0.set(SubmissionPhase::Retired);
    }
    /// Block last-owner teardown until stopped; `protocol.retire_blocking(calls)` never abandons owners.
    pub fn retire_blocking(&self, calls: &mut impl CompletionDispatch) -> Result<(), RenderError> {
        if !matches!(
            self.phase(),
            SubmissionPhase::Submitted | SubmissionPhase::RetirementIncomplete
        ) {
            return Ok(());
        }
        let outcome = retire_until_stopped(calls);
        self.0.set(SubmissionPhase::Retired);
        outcome
    }
    /// Retry only completion observation; `protocol.retire(&mut calls)?` never resubmits work.
    pub fn retire(&self, calls: &mut impl SubmissionDispatch) -> Result<(), RenderError> {
        if !matches!(
            self.phase(),
            SubmissionPhase::Submitted | SubmissionPhase::RetirementIncomplete
        ) {
            return Ok(());
        }
        let outcome = calls.wait_completion();
        let stopped = outcome.is_ok() || outcome.as_ref().err().is_some_and(contains_device_loss);
        self.0.set(if stopped {
            SubmissionPhase::Retired
        } else {
            SubmissionPhase::RetirementIncomplete
        });
        outcome
    }
}
#[cfg(test)]
#[path = "submission_protocol_tests.rs"]
mod tests;
